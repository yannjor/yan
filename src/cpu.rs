use std::collections::HashMap;
use std::fs::read_to_string;

use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::Module;

const CPU_INFO_PATH: &str = "/proc/cpuinfo";

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Cpu {
    header: String,
    #[serde(skip)]
    model: String,
    #[serde(skip)]
    cores: u32,

    /// Whether to add core count to cpu output
    show_core_count: bool,
    /// Whether to remove extra branding like 'Quad-Core' from the model name
    shorten_model: bool,
}

/// Parses contents of /proc/cpuinfo into a HashMap.
/// Content is in the form:
/// KEY  : VALUE
fn parse(contents: &str) -> HashMap<&str, &str> {
    contents
        .lines()
        .map(|line| {
            let split = line.split(':').collect::<Vec<_>>();
            // Safe to unwrap, since data is almost guaranteed to be in a valid format.
            let key = split.get(0).unwrap().trim();
            let val = split.get(1).unwrap_or(&"").trim();
            (key, val)
        })
        .collect()
}

/// Gets cpu model name and core count and returns result as tuple
fn get_cpu() -> (String, u32) {
    let contents = read_to_string(CPU_INFO_PATH).expect("Failed to read /proc/cpuinfo");
    let cpu_info = parse(&contents);
    let model = cpu_info
        .get("model name")
        .expect("Failed to parse CPU model")
        .to_string();

    let cores = cpu_info
        .get("cpu cores")
        .expect("Failed to parse CPU cores")
        .parse::<u32>()
        .unwrap();

    (model, cores)
}

/// Removes some extra branding from the cpu model
fn shorten_model_name(model: String) -> String {
    let patterns = [
        "(TM)",
        "(tm)",
        "(R)",
        "(r)",
        " Core",
        " CPU",
        " Processor",
        " Dual-Core",
        " Quad-Core",
        " Six-Core",
        " Eight-Core",
    ];
    patterns.iter().fold(model, |m, p| m.replace(p, ""))
}

impl Default for Cpu {
    fn default() -> Self {
        let cpu = get_cpu();
        Self {
            header: String::from("CPU"),
            model: cpu.0,
            cores: cpu.1,
            show_core_count: true,
            shorten_model: true,
        }
    }
}

impl Module for Cpu {
    fn print(&self, config: &Config) {
        let mut cpu = self.model.clone();

        if config.cpu.shorten_model {
            cpu = shorten_model_name(cpu);
        }

        if config.cpu.show_core_count {
            cpu.push_str(&format!(" ({})", self.cores));
        }

        println!("{}: {}", config.color.bold().paint(&config.cpu.header), cpu);
    }
}
