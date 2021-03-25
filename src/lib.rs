use std::collections::HashMap;
use std::fs::read_to_string;

const OS_RELEASE_PATH: &str = "/etc/os-release";

/// If successful returns a HashMap containing key value pairs found in
/// /etc/os-release
fn parse_os_release_file<'a>() -> Result<HashMap<String, String>, &'a str> {
    let mut release_dict = HashMap::new();
    if let Ok(contents) = read_to_string(OS_RELEASE_PATH) {
        for line in contents.lines() {
            let split = line.split('=').collect::<Vec<_>>();
            if split.len() < 2 {
                return Err("Failed to parse line in os-release file.");
            }
            release_dict.insert(split[0].to_owned(), split[1].trim_matches('"').to_owned());
        }
        Ok(release_dict)
    } else {
        Err("Failed to read os-release file.")
    }
}

pub fn get_distro() -> String {
    let default_name = String::from("Unknown");
    if let Ok(release_dict) = parse_os_release_file() {
        let distro_name = release_dict.get("NAME").unwrap_or(&default_name);
        if let Some(version) = release_dict.get("VERSION") {
            return format!("{} {}", distro_name, version);
        }
        distro_name.to_string()
    } else {
        default_name
    }
}
