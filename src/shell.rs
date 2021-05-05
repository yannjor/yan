use ansi_term::Color;
use std::env;
use std::path::Path;

use crate::config::Config;
use crate::Module;

pub struct Shell {
    header: String,
    shell: Option<String>,
}

fn get_shell(config: &Config) -> Option<String> {
    let shell = match env::var("SHELL") {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to detect shell, {}", e);
            return None;
        }
    };

    if config.shell_path {
        Some(shell)
    } else {
        let shell_path = Path::new(&shell);
        Some(String::from(shell_path.file_name()?.to_str()?))
    }
}

impl Shell {
    pub fn get(config: &Config) -> Self {
        Shell {
            header: String::from("Shell"),
            shell: get_shell(config),
        }
    }
}

impl Module for Shell {
    fn print(&self, color: Color) {
        if let Some(s) = &self.shell {
            println!("{}: {}", color.bold().paint(&self.header), s);
        }
    }
}
