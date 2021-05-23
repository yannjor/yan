use std::fs::read_to_string;

use serde::{Deserialize, Serialize};

use crate::Config;
use crate::config::Printable;

/// /proc/uptime contains two values, the first one represents the time the
/// system has been on (in seconds) and the second value is the sum of how much
/// time each core has spent idle.
const UPTIME_PATH: &str = "/proc/uptime";

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Uptime {
    header: String,

    #[serde(skip)]
    /// Uptime in seconds
    uptime: u32,

    /// When true, uptime will be outputted like: 2d 10h 3m
    short_output: bool,
}

struct Duration {
    days: u32,
    hours: u32,
    mins: u32,
    secs: u32,
}

impl Duration {
    fn from_secs(secs: u32) -> Self {
        Self {
            days: secs / 60 / 60 / 24,
            hours: secs / 60 / 60 % 24,
            mins: secs / 60 % 60,
            secs,
        }
    }

    fn to_string(&self, short: bool) -> String {
        let mut result = String::new();

        match self.days {
            0 => (),
            1 => result.push_str("1 day, "),
            _ => result.push_str(&format!("{} days, ", self.days)),
        };
        match self.hours {
            0 => (),
            1 => result.push_str("1 hour, "),
            _ => result.push_str(&format!("{} hours, ", self.hours)),
        };
        match self.mins {
            0 => (),
            1 => result.push_str("1 min"),
            _ => result.push_str(&format!("{} mins, ", self.mins)),
        };

        if result.is_empty() {
            result.push_str(&format!("{} secs", self.secs));
        }

        if short {
            result = result
                .replace(" days,", "d")
                .replace(" day,", "d")
                .replace(" hours,", "h")
                .replace(" hour,", "h")
                .replace(" mins", "m")
                .replace(" min", "m")
                .replace(" secs", "s");
        }
        // In case duration is exactly some amount of days or hours
        result
            .trim_end_matches(|c: char| c == ',' || c.is_whitespace())
            .to_string()
    }
}

/// Returns system uptime in seconds
fn get_uptime() -> u32 {
    read_to_string(UPTIME_PATH)
        .expect("Failed to read uptime file")
        .split_once(' ')
        .unwrap()
        .0
        .parse::<f32>()
        .expect("Failed to parse uptime") as u32
}

impl Default for Uptime {
    fn default() -> Self {
        Self {
            header: String::from("Uptime"),
            uptime: get_uptime(),
            short_output: false,
        }
    }
}

impl Printable for Uptime {
    fn print(&self, config: &Config) {
        let uptime = Duration::from_secs(self.uptime).to_string(config.uptime.short_output);
        println!(
            "{}: {}",
            config.color.bold().paint(&config.uptime.header),
            uptime
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_duration_to_string() {
        let duration = Duration::from_secs(199980);
        assert_eq!(
            duration.to_string(false),
            String::from("2 days, 7 hours, 33 mins")
        );
        // Exactly 1 day
        let duration = Duration::from_secs(86400);
        assert_eq!(duration.to_string(false), String::from("1 day"));
        // Only seconds
        let duration = Duration::from_secs(33);
        assert_eq!(duration.to_string(false), String::from("33 secs"));
        // Short output
        let duration = Duration::from_secs(888888);
        assert_eq!(duration.to_string(true), String::from("10d 6h 54m"));
    }
}
