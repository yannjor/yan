use ansi_term::Color;

use std::collections::HashMap;
use std::fs::read_to_string;

use crate::Module;

const CPU_INFO_PATH: &str = "/proc/cpuinfo";

pub struct Cpu {
    header: String,
    model: Option<String>,
}

/// Parses contents of /proc/cpuinfo into a HashMap.
/// Content is in the form:
/// KEY  : VALUE
fn parse(contents: &str) -> HashMap<String, String> {
    contents
        .lines()
        .map(|line| {
            let split = line.split(':').collect::<Vec<_>>();
            // Safe to unwrap, since data is almost guaranteed to be in a valid format.
            let key = split.get(0).unwrap().trim().to_string();
            let val = split.get(1).unwrap_or(&"").trim().to_string();
            (key, val)
        })
        .collect()
}

pub fn get_cpu_model() -> Option<String> {
    let contents = match read_to_string(CPU_INFO_PATH) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read {}, {}", CPU_INFO_PATH, e);
            return None;
        }
    };
    let cpu_info = parse(&contents);
    Some(cpu_info.get("model name")?.to_string())
}

impl Cpu {
    pub fn get() -> Self {
        Cpu {
            header: String::from("CPU"),
            model: get_cpu_model(),
        }
    }
}

impl Module for Cpu {
    fn print(&self, color: Color) {
        if let Some(m) = &self.model {
            println!("{}: {}", color.bold().paint(&self.header), m);
        }
    }
}
