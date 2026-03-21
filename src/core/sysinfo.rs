#[derive(Debug)]
pub struct SystemInfo {
    pub cpu_temp_c:   Option<f32>,
    pub uptime_s:     u64,
    pub mem_free_kb:  u64,
    pub mem_total_kb: u64,
}

pub fn get() -> SystemInfo {
    SystemInfo {
        cpu_temp_c:   read_cpu_temp(),
        uptime_s:     read_uptime(),
        mem_free_kb:  read_mem_free(),
        mem_total_kb: read_mem_total(),
    }
}

fn read_cpu_temp() -> Option<f32> {
    std::fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")
        .ok()
        .and_then(|s| s.trim().parse::<i32>().ok())
        .map(|millideg| millideg as f32 / 1000.0)
}

fn read_uptime() -> u64 {
    std::fs::read_to_string("/proc/uptime")
        .ok()
        .and_then(|s| {
            s.split_whitespace()
                .next()
                .and_then(|v| v.parse::<f64>().ok())
        })
        .map(|s| s as u64)
        .unwrap_or(0)
}

fn read_mem_free() -> u64 {
    std::fs::read_to_string("/proc/meminfo")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("MemAvailable:"))
                .and_then(|l| l.split_whitespace().nth(1))
                .and_then(|v| v.parse::<u64>().ok())
        })
        .unwrap_or(0)
}

fn read_mem_total() -> u64 {
    std::fs::read_to_string("/proc/meminfo")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("MemTotal:"))
                .and_then(|l| l.split_whitespace().nth(1))
                .and_then(|v| v.parse::<u64>().ok())
        })
        .unwrap_or(0)
}





#[cfg(test)]
mod tests {
    #[test]
    fn read_cpu_temp_invalid_content_returns_none() {
        assert_eq!(parse_cpu_temp("not_a_number\n"), None);
        assert_eq!(parse_cpu_temp(""), None);
    }

    #[test]
    fn read_cpu_temp_valid_millidegrees_converts_correctly() {
        assert!((parse_cpu_temp("48000\n").unwrap() - 48.0).abs() < 0.001);
        assert!((parse_cpu_temp("55500").unwrap() - 55.5).abs() < 0.001);
    }

    #[test]
    fn read_uptime_invalid_content_returns_zero() {
        assert_eq!(parse_uptime(""), 0);
        assert_eq!(parse_uptime("not a number"), 0);
    }

    #[test]
    fn read_uptime_valid_content_parses_first_field() {
        assert_eq!(parse_uptime("3600.42 1234.56\n"), 3600);
        assert_eq!(parse_uptime("120.0 60.0"), 120);
    }

    #[test]
    fn read_mem_free_missing_key_returns_zero() {
        assert_eq!(parse_mem_free("MemTotal: 8000000 kB\n"), 0);
        assert_eq!(parse_mem_free(""), 0);
    }

    #[test]
    fn read_mem_free_valid_content_returns_kb() {
        let content = "MemTotal:       8000000 kB\nMemAvailable:   3500000 kB\n";
        assert_eq!(parse_mem_free(content), 3_500_000);
    }

    #[test]
    fn read_mem_total_missing_key_returns_zero() {
        assert_eq!(parse_mem_total("MemAvailable: 3500000 kB\n"), 0);
        assert_eq!(parse_mem_total(""), 0);
    }

    #[test]
    fn read_mem_total_valid_content_returns_kb() {
        let content = "MemTotal:       8000000 kB\nMemAvailable:   3500000 kB\n";
        assert_eq!(parse_mem_total(content), 8_000_000);
    }

    fn parse_cpu_temp(s: &str) -> Option<f32> {
        s.trim().parse::<i32>().ok().map(|v| v as f32 / 1000.0)
    }

    fn parse_uptime(s: &str) -> u64 {
        s.split_whitespace()
            .next()
            .and_then(|v| v.parse::<f64>().ok())
            .map(|v| v as u64)
            .unwrap_or(0)
    }

    fn parse_mem_free(s: &str) -> u64 {
        s.lines()
            .find(|l| l.starts_with("MemAvailable:"))
            .and_then(|l| l.split_whitespace().nth(1))
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(0)
    }

    fn parse_mem_total(s: &str) -> u64 {
        s.lines()
            .find(|l| l.starts_with("MemTotal:"))
            .and_then(|l| l.split_whitespace().nth(1))
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(0)
    }
}
