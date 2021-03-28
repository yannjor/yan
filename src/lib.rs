use ::std::env;
use std::collections::HashMap;
use std::fs::read_to_string;

const OS_RELEASE_PATH: &str = "/etc/os-release";
const MEM_USAGE_PATH: &str = "/proc/meminfo";

/// Parses a file containing key - value pairs into a HashMap.
fn parse_to_hashmap(filepath: &str, delimiter: char) -> Result<HashMap<String, String>, String> {
    let mut release_dict = HashMap::new();
    if let Ok(contents) = read_to_string(filepath) {
        for line in contents.lines() {
            let split = line.split(delimiter).collect::<Vec<_>>();
            if split.len() < 2 {
                return Err(format!("Failed to parse line in {}.", filepath));
            }
            release_dict.insert(split[0].to_owned(), split[1].to_owned());
        }
        Ok(release_dict)
    } else {
        Err(format!("Failed to read {}.", filepath))
    }
}

/// Retrieves the distro name from /etc/os-release
pub fn get_os() -> Option<String> {
    let release_map = match parse_to_hashmap(OS_RELEASE_PATH, '=') {
        Ok(map) => map,
        Err(e) => {
            eprintln!("{}", e);
            return None;
        }
    };
    let distro_name = release_map.get("NAME")?.trim_matches('"');
    if let Some(version) = release_map.get("VERSION") {
        return Some(format!("OS: {} {}", distro_name, version));
    }
    Some(format!("OS: {}", distro_name))
}

pub fn get_cpu_architechture() -> Option<String> {
    Some(format!("Arch: {}", env::consts::ARCH))
}

pub fn get_shell() -> Option<String> {
    match env::var("SHELL") {
        Ok(shell) => Some(format!("Shell: {}", shell)),
        Err(e) => {
            eprintln!("Failed to detect shell, {}", e);
            None
        }
    }
}

/// Parses a string value from /proc/meminfo into a u32 containing the size
/// in MiB.
fn parse_mem_value(value: &str) -> u32 {
    let kb_value = value
        .trim_matches(|c: char| c == 'k' || c == 'B' || c.is_whitespace())
        .parse::<u32>()
        .unwrap();
    let mb_value = kb_value / 1024;
    mb_value
}

pub fn get_memory_usage() -> Option<String> {
    let mem_map = match parse_to_hashmap(MEM_USAGE_PATH, ':') {
        Ok(map) => map,
        Err(e) => {
            eprintln!("{}", e);
            return None;
        }
    };
    let mem_total = parse_mem_value(mem_map.get("MemTotal")?);
    let mem_available = parse_mem_value(mem_map.get("MemAvailable")?);

    Some(format!(
        "Memory: {}MiB / {}MiB",
        mem_total - mem_available,
        mem_total
    ))
}
