use serde::{Deserialize, Serialize};

use std::env;
use std::path::Path;

use crate::config::Config;
use crate::Module;

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Shell {
    #[serde(skip)]
    shell: Option<String>,

    header: String,
    /// Whether to show the full path of the shell
    show_path: bool,
}

fn get_shell() -> Option<String> {
    match env::var("SHELL") {
        Ok(s) => Some(s),
        Err(e) => {
            eprintln!("Failed to detect shell, {}", e);
            None
        }
    }
}

impl Default for Shell {
    fn default() -> Self {
        Self {
            shell: get_shell(),
            header: String::from("Shell"),
            show_path: false,
        }
    }
}

impl Module for Shell {
    fn print(&self, config: &Config) {
        if let Some(shell) = &self.shell {
            let mut shell = shell.clone();
            if !config.shell.show_path {
                let path = Path::new(&shell);
                shell = String::from(path.file_name().unwrap().to_str().unwrap());
            }
            println!(
                "{}: {}",
                config.color.bold().paint(&config.shell.header),
                shell
            );
        }
    }
}
