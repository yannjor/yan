use std::fs::read_to_string;

const CPU_INFO_PATH: &str = "/proc/cpuinfo";

fn parse_proc_cpuinfo(contents: &str) -> Option<String> {
    for line in contents.lines() {
        if line.contains("model name") {
            let split = line.split(':').collect::<Vec<_>>();
            let cpu_model = split.get(1)?.trim().to_string();
            return Some(cpu_model);
        }
    }
    None
}

pub fn get_cpu() -> Option<String> {
    let cpu_info = match read_to_string(CPU_INFO_PATH) {
        Ok(info) => info,
        Err(e) => {
            eprintln!("Failed to read {}, {}", CPU_INFO_PATH, e);
            return None;
        }
    };
    parse_proc_cpuinfo(&cpu_info)
}
