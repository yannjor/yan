use std::fs;
use std::io::ErrorKind::NotFound;

use ansi_term::Color;
use serde::{Deserialize, Serialize};

use crate::memory::Memory;
use crate::shell::Shell;
use crate::uptime::Uptime;

const BINARY_NAME: &str = env!("CARGO_PKG_NAME");

pub trait Printable {
    fn print(&self, config: &Config);
}

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Accent color in output
    #[serde(skip)]
    pub color: Color,

    pub memory: Memory,

    pub shell: Shell,

    pub uptime: Uptime,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            color: Color::Cyan,
            memory: Memory::default(),
            shell: Shell::default(),
            uptime: Uptime::default(),
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
