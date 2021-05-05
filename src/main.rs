mod config;
mod cpu;
mod memory;
mod os;
mod shell;

use ansi_term::Color;
use config::Config;

pub trait Module {
    fn print(&self, color: Color);
}

/// Struct to store the detected system information.
struct SystemInfo {
    modules: Vec<Box<dyn Module>>,
}

impl SystemInfo {
    fn load() -> Self {
        let config = match Config::load() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Configuration error: {}", e);
                std::process::exit(1)
            }
        };

        let modules: Vec<Box<dyn Module>> = vec![
            Box::new(os::Distro::get()),
            Box::new(os::Architechture::get()),
            Box::new(os::Kernel::get()),
            Box::new(shell::Shell::get(&config)),
            Box::new(memory::Memory::get(&config)),
            Box::new(cpu::Cpu::get()),
        ];

        SystemInfo { modules }
    }
}

fn main() {
    let sys_info = SystemInfo::load();
    for module in sys_info.modules.iter() {
        module.print(Color::Cyan);
    }
}
