use minifetch::{get_cpu_architechture, get_memory_usage, get_os, get_shell};

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
