mod info;
use info::memory::get_memory_usage;
use info::os::{get_cpu_architechture, get_os};
use info::shell::get_shell;

fn main() {
    let os = get_os();
    let arch = get_cpu_architechture();
    let shell = get_shell();
    let memory = get_memory_usage();
    for val in [os, arch, shell, memory].iter() {
        if let Some(value) = val {
            println!("{}", value);
        }
    }
}
