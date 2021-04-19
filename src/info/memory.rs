use std::collections::HashMap;
use std::fs::read_to_string;

const MEM_USAGE_PATH: &str = "/proc/meminfo";

/// Parses a string value from /proc/meminfo into a u32 containing the size
/// in MiB.
fn parse_mem_value(value: &str) -> u32 {
    let kb_value = value
        .trim_matches(|c: char| c == 'k' || c == 'B' || c.is_whitespace())
        .parse::<u32>()
        .unwrap_or(0);
    let mb_value = kb_value / 1024;
    mb_value
}

/// Parses the contents of /proc/meminfo into a HashMap.
fn parse_proc_meminfo(contents: &str) -> Option<HashMap<String, u32>> {
    contents
        .lines()
        .map(|line| {
            let split = line.split(':').collect::<Vec<_>>();
            let key = split.get(0)?.to_string();
            let val = parse_mem_value(split.get(1)?);
            Some((key, val))
        })
        .collect()
}

pub fn get_memory_usage() -> Option<String> {
    let mem_info = match read_to_string(MEM_USAGE_PATH) {
        Ok(info) => info,
        Err(e) => {
            eprintln!("Failed to read {}, {}", MEM_USAGE_PATH, e);
            return None;
        }
    };

    let meminfo_map = parse_proc_meminfo(&mem_info)?;
    let mem_total = meminfo_map.get("MemTotal")?;
    let mem_available = meminfo_map.get("MemAvailable")?;

    Some(format!(
        "{}MiB / {}MiB",
        mem_total - mem_available,
        mem_total
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_proc_meminfo() {
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
        let meminfo_map = parse_proc_meminfo(input).unwrap();
        assert_eq!(meminfo_map.len(), 53);
        assert_eq!(meminfo_map.get("MemTotal"), Some(&15950));
        assert_eq!(meminfo_map.get("PageTables"), Some(&38));
    }
}
