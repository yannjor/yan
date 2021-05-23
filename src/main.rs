mod config;
mod cpu;
mod memory;
mod os;
mod shell;
mod uptime;

use config::Config;
use config::Printable;

fn main() {
    let config = match Config::load() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Configuration error: {}", e);
            std::process::exit(1)
        }
    };

    for module in &config.modules {
        module.print(&config);
    }
}
