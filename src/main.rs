mod info;

use ansi_term::Color;
use std::fmt;

use info::*;

/// Struct that stores an information field (line) in the output.
struct InfoField<'a> {
    info_name: &'a str,
    info_value: Option<String>,
}

impl<'a> InfoField<'a> {
    fn new(name: &str, value: Option<String>) -> InfoField {
        InfoField {
            info_name: name,
            info_value: value,
        }
    }
}

impl<'a> fmt::Display for InfoField<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(value) = &self.info_value {
            write!(f, "{}: {}", Color::Cyan.bold().paint(self.info_name), value)?
        }
        Ok(())
    }
}

/// Struct to store the detected system information.
struct SystemInfo<'a> {
    os_name: InfoField<'a>,
    arch: InfoField<'a>,
    kernel: InfoField<'a>,
    shell: InfoField<'a>,
    mem_usage: InfoField<'a>,
    cpu: InfoField<'a>,
}

impl<'a> SystemInfo<'a> {
    fn new() -> SystemInfo<'a> {
        SystemInfo {
            os_name: InfoField::new("OS", os::get_os()),
            arch: InfoField::new("Arch", os::get_os_architechture()),
            kernel: InfoField::new("Kernel", os::get_kernel()),
            shell: InfoField::new("Shell", shell::get_shell()),
            mem_usage: InfoField::new("Memory", memory::get_memory_usage()),
            cpu: InfoField::new("CPU", cpu::get_cpu()),
        }
    }
}

impl<'a> fmt::Display for SystemInfo<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for info in [
            &self.os_name,
            &self.arch,
            &self.kernel,
            &self.shell,
            &self.mem_usage,
            &self.cpu,
        ]
        .iter()
        {
            writeln!(f, "{}", info)?
        }
        Ok(())
    }
}

fn main() {
    let sys_info = SystemInfo::new();
    println!("{}", sys_info);
}
