mod info;

use info::*;

fn main() {
    let os = os::get_os();
    let arch = os::get_cpu_architechture();
    let kernel = os::get_kernel();
    let shell = shell::get_shell();
    let memory = memory::get_memory_usage();
    for info in [os, arch, kernel, shell, memory].iter() {
        if let Some(value) = info {
            println!("{}", value);
        }
    }
}
