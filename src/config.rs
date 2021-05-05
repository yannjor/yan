use serde::{Deserialize, Serialize};
use std::fs;
use std::io::ErrorKind::NotFound;

const BINARY_NAME: &str = env!("CARGO_PKG_NAME");

/// Unit used when outputting memory usage
#[derive(Serialize, Deserialize)]
pub enum MemoryUnit {
    KiB,
    MiB,
    GiB,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    /// Unit used for memory usage
    pub mem_unit: MemoryUnit,
    /// Show memory usage percentage
    pub mem_percentage: bool,
    /// Show full path for shell
    pub shell_path: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mem_unit: MemoryUnit::GiB,
            mem_percentage: true,
            shell_path: false,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, &'static str> {
        let config_dir = match dirs::config_dir() {
            Some(dir) => dir.join(BINARY_NAME),
            None => return Err("Failed to get config directory"),
        };

        let config_file = config_dir.join("config.toml");
        let config = match fs::read_to_string(&config_file) {
            Ok(c) => toml::from_str(&c).expect("Failed to parse toml in configuration file"),

            // Create default config file if it doesn't exist
            Err(ref e) if e.kind() == NotFound => {
                let config = Config::default();

                let toml =
                    toml::to_string_pretty(&config).expect("Failed to convert config to toml");

                fs::create_dir_all(config_dir).expect("Failed to create config directory");
                fs::write(&config_file, toml).expect("Failed to write config file");

                config
            }

            Err(_) => return Err("Failed to load configuration file"),
        };

        Ok(config)
    }
}