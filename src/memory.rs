use std::collections::HashMap;
use std::fs::read_to_string;

use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::Module;

const MEM_USAGE_PATH: &str = "/proc/meminfo";

/// Conversion factor between Kibibyte and Mebibyte
const MEBIBYTE: f32 = 1024.0;
/// Conversion factor between Kibibyte and Gibibyte
const GIBIBYTE: f32 = MEBIBYTE * MEBIBYTE;

/// Unit used when outputting memory usage
#[derive(Serialize, Deserialize)]
enum MemoryUnit {
    KiB,
    MiB,
    GiB,
}

impl MemoryUnit {
    /// Converts a KiB value to MiB or GiB and returns a formatted string
    /// like: "15.55 GiB"
    pub fn to_unit_str(&self, kib_value: u32) -> String {
        match self {
            Self::KiB => format!("{} KiB", kib_value),
            Self::MiB => format!("{:.0} MiB", kib_value as f32 / MEBIBYTE),
            Self::GiB => format!("{:.2} GiB", kib_value as f32 / GIBIBYTE),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Memory {
    /// Format: (used, total)
    #[serde(skip)]
    usage: (u32, u32),

    header: String,
    /// Unit used for memory usage. Possible values include KiB, MiB and GiB
    unit: MemoryUnit,
    /// Whether to show memory usage as percentage
    show_percentage: bool,
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

/// Returns usage in format: (used, total) in KiB
fn get_usage() -> (u32, u32) {
    let contents = read_to_string(MEM_USAGE_PATH).expect("Failed to read /proc/meminfo");

    let map = parse(&contents);
    let total = *map.get("MemTotal").unwrap();
    let available = *map.get("MemAvailable").unwrap();
    let used = total - available;
    (used, total)
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            usage: get_usage(),
            header: String::from("Memory"),
            unit: MemoryUnit::GiB,
            show_percentage: true,
        }
    }
}

impl Module for Memory {
    fn print(&self, config: &Config) {
        let used = self.usage.0;
        let total = self.usage.1;

        let mut usage = format!(
            "{} / {}",
            MemoryUnit::to_unit_str(&config.memory.unit, used),
            MemoryUnit::to_unit_str(&config.memory.unit, total)
        );
        if config.memory.show_percentage {
            let used_percent = (used as f32 / total as f32) * 100.0;
            usage.push_str(&format!(" ({:.0}%)", used_percent));
        }

        println!(
            "{}: {}",
            config.color.bold().paint(&config.memory.header),
            usage
        );
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
