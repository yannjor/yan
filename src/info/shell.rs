use std::env;

pub fn get_shell() -> Option<String> {
    match env::var("SHELL") {
        Ok(shell_path) => Some(format!("Shell: {}", shell_path)),
        Err(err) => {
            eprintln!("Failed to detect shell, {}", err);
            None
        }
    }
}
