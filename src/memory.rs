use ansi_term::Color;
use std::collections::HashMap;
use std::fs::read_to_string;

use crate::config::{Config, MemoryUnit};

use crate::Module;

const MEM_USAGE_PATH: &str = "/proc/meminfo";

pub struct Memory {
    header: String,
    usage: Option<String>,
}

/// Parses a string value from /proc/meminfo into a u32 containing the size
/// in KiB
fn parse_mem_value(value: &str) -> u32 {
    value
        .trim_matches(|c: char| c == 'k' || c == 'B' || c.is_whitespace())
        .parse::<u32>()
        .unwrap_or(0)
}

/// Parses the contents of /proc/meminfo into a HashMap.
/// Content is in form:
/// KEY:    VALUE KB
fn parse(contents: &str) -> HashMap<String, u32> {
    contents
        .lines()
        .map(|line| {
            let split = line.split(':').collect::<Vec<_>>();
            // Safe to unwrap, since data is almost guaranteed to be in a valid format.
            let key = split.get(0).unwrap().to_string();
            let val = parse_mem_value(split.get(1).unwrap());
            (key, val)
        })
        .collect()
}

pub fn get_memory_usage(config: &Config) -> Option<String> {
    let contents = match read_to_string(MEM_USAGE_PATH) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read {}, {}", MEM_USAGE_PATH, e);
            return None;
        }
    };

    let meminfo_map = parse(&contents);
    let mem_total = *meminfo_map.get("MemTotal")?;
    let mem_available = *meminfo_map.get("MemAvailable")?;
    let mem_used = mem_total - mem_available;

    let mut usage = match config.mem_unit {
        MemoryUnit::KiB => format!("{} KiB / {} KiB", mem_used, mem_total),
        MemoryUnit::MiB => {
            format!("{} MiB / {} MiB", mem_used / 1024, mem_total / 1024)
        }
        MemoryUnit::GiB => {
            format!(
                "{:.2} GiB / {:.2} GiB",
                mem_used as f32 / 1048576.0,
                mem_total as f32 / 1048576.0
            )
        }
    };

    if config.mem_percentage {
        usage.push_str(&format!(
            " ({:.0}%)",
            (mem_used as f32 / mem_total as f32) * 100.0
        ));
    }

    Some(usage)
}

impl Memory {
    pub fn get(config: &Config) -> Self {
        Self {
            header: String::from("Memory"),
            usage: get_memory_usage(config),
        }
    }
}

impl Module for Memory {
    fn print(&self, color: Color) {
        if let Some(u) = &self.usage {
            println!("{}: {}", color.bold().paint(&self.header), u);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let input = "MemTotal:       16333740 kB
MemFree:         8542972 kB
MemAvailable:   11875280 kB
Buffers:          194000 kB
Cached:          3742856 kB
SwapCached:            0 kB
Active:          1697548 kB
Inactive:        5577196 kB
Active(anon):      33452 kB
Inactive(anon):  3717896 kB
Active(file):    1664096 kB
Inactive(file):  1859300 kB
Unevictable:          16 kB
Mlocked:              16 kB
SwapTotal:             0 kB
SwapFree:              0 kB
Dirty:               132 kB
Writeback:             0 kB
AnonPages:       3337928 kB
Mapped:          1285136 kB
Shmem:            423092 kB
KReclaimable:     131000 kB
Slab:             236936 kB
SReclaimable:     131000 kB
SUnreclaim:       105936 kB
KernelStack:       16896 kB
PageTables:        39732 kB
NFS_Unstable:          0 kB
Bounce:                0 kB
WritebackTmp:          0 kB
CommitLimit:     8166868 kB
Committed_AS:   10551436 kB
VmallocTotal:   34359738367 kB
VmallocUsed:       77072 kB
VmallocChunk:          0 kB
Percpu:             5952 kB
HardwareCorrupted:     0 kB
AnonHugePages:         0 kB
ShmemHugePages:        0 kB
ShmemPmdMapped:        0 kB
FileHugePages:         0 kB
FilePmdMapped:         0 kB
CmaTotal:              0 kB
CmaFree:               0 kB
HugePages_Total:       0
HugePages_Free:        0
HugePages_Rsvd:        0
HugePages_Surp:        0
Hugepagesize:       2048 kB
Hugetlb:               0 kB
DirectMap4k:      465568 kB
DirectMap2M:     9965568 kB
DirectMap1G:     6291456 kB
";
        let meminfo_map = parse(input);
        assert_eq!(meminfo_map.len(), 53);
        assert_eq!(meminfo_map.get("MemTotal"), Some(&16333740));
        assert_eq!(meminfo_map.get("PageTables"), Some(&39732));
    }
}
