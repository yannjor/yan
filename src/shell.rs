use ansi_term::Color;
use std::env;

use crate::Module;

pub struct Shell {
    header: String,
    shell: Option<String>,
}

fn get_shell() -> Option<String> {
    match env::var("SHELL") {
        Ok(shell_path) => Some(shell_path),
        Err(e) => {
            eprintln!("Failed to detect shell, {}", e);
            None
        }
    }
}

impl Shell {
    pub fn get() -> Self {
        Shell {
            header: String::from("Shell"),
            shell: get_shell(),
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
