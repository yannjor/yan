use ansi_term::Color;

use std::collections::HashMap;
use std::env::consts;
use std::fs;

use crate::Module;

const OS_RELEASE_PATH: &str = "/etc/os-release";
const OS_KERNEL_PATH: &str = "/proc/sys/kernel/osrelease";

pub struct Distro {
    header: String,
    name: Option<String>,
}

/// Parse contents in form of:
/// KEY="VALUE"
/// KEY="VALUE"
/// ...
/// into a HashMap.
fn parse(contents: &str) -> HashMap<String, String> {
    contents
        .lines()
        .map(|line| {
            let split = line.split('=').collect::<Vec<_>>();
            // Safe to unwrap here as it's very unlikely this data is not in a valid format.
            let key = split.get(0).unwrap().to_string();
            let val = split.get(1).unwrap().trim_matches('"').to_string();
            (key, val)
        })
        .collect()
}

fn get_os_name() -> Option<String> {
    let contents = match fs::read_to_string(OS_RELEASE_PATH) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read {}, {}", OS_RELEASE_PATH, e);
            return None;
        }
    };
    let parsed = parse(&contents);
    let distro_name = parsed.get("NAME")?.to_string();

    Some(distro_name)
}

impl Distro {
    pub fn get() -> Self {
        Distro {
            header: String::from("OS"),
            name: get_os_name(),
        }
    }
}

impl Module for Distro {
    fn print(&self, color: Color) {
        if let Some(n) = &self.name {
            println!("{}: {}", color.bold().paint(&self.header), n);
        }
    }
}

pub struct Architechture {
    header: String,
    architechture: String,
}

impl Architechture {
    pub fn get() -> Self {
        Architechture {
            header: String::from("Arch"),
            architechture: consts::ARCH.to_string(),
        }
    }
}

impl Module for Architechture {
    fn print(&self, color: Color) {
        println!(
            "{}: {}",
            color.bold().paint(&self.header),
            self.architechture
        );
    }
}

pub struct Kernel {
    header: String,
    version: Option<String>,
}

impl Kernel {
    pub fn get() -> Self {
        let kernel = match fs::read_to_string(OS_KERNEL_PATH) {
            Ok(k) => Some(k.trim().to_string()),
            Err(e) => {
                eprintln!("Failed to read kernel version, {}", e);
                None
            }
        };

        Kernel {
            header: String::from("Kernel"),
            version: kernel,
        }
    }
}

impl Module for Kernel {
    fn print(&self, color: Color) {
        if let Some(v) = &self.version {
            println!("{}: {}", color.bold().paint(&self.header), v);
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
        let release_map = parse(input);
        assert_eq!(release_map.get("NAME"), Some(&"Arch Linux".to_string()));
        assert_eq!(release_map.get("BUILD_ID"), Some(&"rolling".to_string()));
    }
}
