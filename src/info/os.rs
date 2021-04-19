use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

const OS_RELEASE_PATH: &str = "/etc/os-release";
const OS_KERNEL_PATH: &str = "/proc/sys/kernel/osrelease";

/// Parses the contents of /etc/os-release into a HashMap.
fn parse_os_release(contents: &str) -> Option<HashMap<String, String>> {
    contents
        .lines()
        .map(|line| {
            let split = line.split('=').collect::<Vec<_>>();
            let key = split.get(0)?.to_string();
            let val = split.get(1)?.trim_matches('"').to_string();
            Some((key, val))
        })
        .collect()
}

pub fn get_os() -> Option<String> {
    let os_info = match read_to_string(OS_RELEASE_PATH) {
        Ok(info) => info,
        Err(e) => {
            eprintln!("Failed to read {}, {}", OS_RELEASE_PATH, e);
            return None;
        }
    };
    let release_map = parse_os_release(&os_info)?;
    let distro_name = release_map.get("NAME")?.to_string();
    if let Some(version) = release_map.get("VERSION") {
        return Some(format!("OS: {} {}", distro_name, version));
    }
    Some(distro_name)
}

pub fn get_os_architechture() -> Option<String> {
    Some(env::consts::ARCH.to_string())
}

pub fn get_kernel() -> Option<String> {
    match read_to_string(OS_KERNEL_PATH) {
        Ok(kernel) => Some(kernel.trim().to_string()),
        Err(e) => {
            eprintln!("Failed to read kernel version, {}", e);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_os_release() {
        let input = "NAME=\"Arch Linux\"
PRETTY_NAME=\"Arch Linux\"
ID=arch
BUILD_ID=rolling
ANSI_COLOR=\"38;2;23;147;209\"
HOME_URL=\"https://www.archlinux.org/\"
DOCUMENTATION_URL=\"https://wiki.archlinux.org/\"
SUPPORT_URL=\"https://bbs.archlinux.org/\"
BUG_REPORT_URL=\"https://bugs.archlinux.org/\"
LOGO=archlinux
";
        let release_map = parse_os_release(input).unwrap();
        assert_eq!(release_map.get("NAME"), Some(&"Arch Linux".to_string()));
        assert_eq!(release_map.get("BUILD_ID"), Some(&"rolling".to_string()));
    }
}
