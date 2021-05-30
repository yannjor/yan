use std::env;
use std::fs::read_to_string;

use crate::config::Config;
use crate::Module;

const HOSTNAME_PATH: &str = "/etc/hostname";

pub struct Title {
    user: Option<String>,
    hostname: Option<String>,
}

impl Default for Title {
    fn default() -> Self {
        let user = match env::var("USER") {
            Ok(user) => Some(user),
            Err(e) => {
                eprintln!("Failed to get user, {}", e);
                None
            }
        };
        let hostname = match read_to_string(HOSTNAME_PATH) {
            Ok(hostname) => Some(hostname),
            Err(e) => {
                eprintln!("Failed to get hostname, {}", e);
                None
            }
        };
        Self { user, hostname }
    }
}

impl Module for Title {
    fn print(&self, config: &Config) {
        if let (Some(user), Some(hostname)) = (&self.user, &self.hostname) {
            print!(
                "{}@{}",
                config.color.bold().paint(user),
                config.color.bold().paint(hostname)
            );
            // Print title separator
            println!("{:-<1$}", "", user.len() + hostname.len());
        }
    }
}
