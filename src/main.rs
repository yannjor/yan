mod config;
mod cpu;
mod memory;
mod os;
mod packages;
mod shell;
mod title;
mod uptime;

use config::Config;

pub trait Module {
    fn print(&self, config: &Config);
}

/// Struct to store the detected system information.
struct SystemInfo {
    modules: Vec<Box<dyn Module>>,
}

impl SystemInfo {
    fn load() -> Self {
        let modules: Vec<Box<dyn Module>> = vec![
            Box::new(title::Title::default()),
            Box::new(os::Distro::default()),
            Box::new(os::Architechture::default()),
            Box::new(os::Kernel::default()),
            Box::new(uptime::Uptime::default()),
            Box::new(packages::Packages::default()),
            Box::new(shell::Shell::default()),
            Box::new(memory::Memory::default()),
            Box::new(cpu::Cpu::default()),
        ];
        SystemInfo { modules }
    }
}

fn main() {
    let config = match Config::load() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Configuration error: {}", e);
            std::process::exit(1)
        }
    };

    let sys_info = SystemInfo::load();

    for module in sys_info.modules.iter() {
        module.print(&config);
    }
}
