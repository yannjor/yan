use serde::{Deserialize, Serialize};

use std::process::Command;

use crate::Config;
use crate::Module;

const PACKAGE_MANAGERS: &[PackageManager] = &[
    PackageManager::new("pacman", &["--query", "--quiet"]),
    PackageManager::new("dpkg", &["--get-selections"]),
    PackageManager::new("rpm", &["--query", "--all"]),
    PackageManager::new("apk", &["info"]),
    PackageManager::new("xbps-query", &["--list-pkgs"]),
    PackageManager::new("flatpak", &["list"]),
];

struct PackageManager<'a> {
    name: &'a str,
    /// Arguments to list installed packages
    args: &'a [&'a str],
}

impl<'a> PackageManager<'a> {
    const fn new(name: &'a str, args: &'a [&'a str]) -> Self {
        Self { name, args }
    }

    fn is_installed(&self) -> bool {
        Command::new(self.name).output().is_ok()
    }

    fn get_package_count(&self) -> Option<u32> {
        let command_output = Command::new(self.name).args(self.args).output();
        let package_list = match command_output {
            Ok(output) => output.stdout,
            Err(_) => return None,
        };
        let count = String::from_utf8_lossy(&package_list).lines().count() as u32;
        Some(count)
    }
}

struct PackageCount {
    count: u32,
    package_manager: String,
}

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct Packages {
    #[serde(skip)]
    package_counts: Vec<PackageCount>,

    header: String,
    /// Whether to show the package manager names. Format: "count (pkg_manager)"
    show_package_managers: bool,
}

impl Default for Packages {
    fn default() -> Self {
        let mut counts: Vec<PackageCount> = Vec::new();
        for pkg_manager in PACKAGE_MANAGERS.iter() {
            if pkg_manager.is_installed() {
                match pkg_manager.get_package_count() {
                    Some(count) => counts.push(PackageCount {
                        count,
                        package_manager: pkg_manager.name.to_string(),
                    }),
                    None => {
                        eprintln!("Failed to get package count from {}", pkg_manager.name);
                        continue;
                    }
                }
            }
        }

        Self {
            package_counts: counts,
            header: String::from("Packages"),
            show_package_managers: true,
        }
    }
}

impl Module for Packages {
    fn print(&self, config: &Config) {
        if !self.package_counts.is_empty() {
            let package_string = if config.packages.show_package_managers {
                self.package_counts
                    .iter()
                    .map(|c| format!("{} ({})", c.count, c.package_manager))
                    .collect::<Vec<_>>()
                    .join(", ")
            } else {
                self.package_counts
                    .iter()
                    .fold(0, |acc, pkg_count| acc + pkg_count.count)
                    .to_string()
            };

            println!(
                "{}: {}",
                config.color.bold().paint(&self.header),
                package_string
            );
        }
    }
}
