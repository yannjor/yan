use minifetch::parse_string_to_hashmap;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

const OS_RELEASE_PATH: &str = "/etc/os-release";

/// Parses the contents of /etc/os-release into a HashMap.
fn parse_os_release(contents: &str) -> Option<HashMap<String, String>> {
    let mut release_map: HashMap<String, String> = parse_string_to_hashmap(contents, '=')?;
    for (_, val) in release_map.iter_mut() {
        *val = val.trim_matches('"').to_string();
    }
    Some(release_map)
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
    let distro_name = release_map.get("NAME")?;
    if let Some(version) = release_map.get("VERSION") {
        return Some(format!("OS: {} {}", distro_name, version));
    }
    Some(format!("OS: {}", distro_name))
}

pub fn get_cpu_architechture() -> Option<String> {
    Some(format!("Arch: {}", env::consts::ARCH))
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
