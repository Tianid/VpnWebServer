use crate::logger::LogLevel;

#[derive(Debug, Clone, PartialEq)]
pub enum AutostartAction {
    Setup,
    Remove,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub address:            String,
    pub port:               u16,
    pub log_level:          LogLevel,
    pub autostart_action:   Option<AutostartAction>,
    pub address_specified:  bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            address:            "127.0.0.1".to_string(),
            port:               9000,
            log_level:          LogLevel::Info,
            autostart_action:   None,
            address_specified:  false,
        }
    }
}

pub fn parse_args<I: IntoIterator<Item = String>>(args: I) -> ServerConfig {
    let mut config = ServerConfig::default();
    let mut iter = args.into_iter().peekable();

    if let Some(first) = iter.peek() {
        if !first.starts_with('-') {
            iter.next();
        }
    }

    while let Some(arg) = iter.next() {
        if let Some((key, value)) = arg.split_once('=') {
            apply_arg(&mut config, key.trim(), value.trim());
            continue;
        }
        match arg.as_str() {
            "-A" | "--setup-autostart" => {
                config.autostart_action = Some(AutostartAction::Setup);
            }
            "-R" | "--remove-autostart" => {
                config.autostart_action = Some(AutostartAction::Remove);
            }
            "--address" | "-a" => {
                if let Some(v) = iter.next() { apply_arg(&mut config, "--address", v.trim()); }
            }
            "--port" | "-p" => {
                if let Some(v) = iter.next() { apply_arg(&mut config, "--port", v.trim()); }
            }
            "--log-level" | "-l" => {
                if let Some(v) = iter.next() { apply_arg(&mut config, "--log-level", v.trim()); }
            }
            "--help" | "-h" => { print_usage(); std::process::exit(0); }
            "--version" | "-V" => {
                println!("haven {}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
            _ => {}
        }
    }

    config
}

fn apply_arg(config: &mut ServerConfig, key: &str, value: &str) {
    match key {
        "--address" | "-a" => {
            if value.parse::<std::net::Ipv4Addr>().is_ok() {
                config.address = value.to_string();
                config.address_specified = true;
            } else {
                eprintln!("[WARN] Invalid address '{}', using default '{}'", value, config.address);
            }
        }
        "--port" | "-p" => {
            match value.parse::<u16>() {
                Ok(p) if p > 0 => config.port = p,
                _ => eprintln!("[WARN] Invalid port '{}', using default {}", value, config.port),
            }
        }
        "--log-level" | "-l" => {
            match LogLevel::from_str(value) {
                Some(lvl) => config.log_level = lvl,
                None => eprintln!("[WARN] Invalid log level '{}', using default {:?}", value, config.log_level),
            }
        }
        _ => {}
    }
}

fn print_usage() {
    println!(
        "Usage: haven [OPTIONS]\n\
         \n\
         Options:\n\
           -a, --address <ADDR>      Bind address (default: 127.0.0.1)\n\
           -p, --port <PORT>         Bind port (default: 9000)\n\
           -l, --log-level <LEVEL>   Log level: trace|debug|info|warn|error|off (default: info)\n\
           -A, --setup-autostart     Create XDG autostart entries and exit\n\
           -R, --remove-autostart    Remove XDG autostart entries and exit\n\
           -h, --help                Print this help and exit\n\
           -V, --version             Print version and exit\n\
         \n\
         Autostart flags may be combined with -a to configure autostart\n\
         and immediately start the server in the same invocation.\n"
    );
}





#[cfg(test)]
mod tests {
    use super::*;
    use crate::logger::LogLevel;

    fn args(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn no_args_gives_defaults() {
        let cfg = parse_args(args(&[]));
        assert_eq!(cfg.address,   "127.0.0.1");
        assert_eq!(cfg.port,      9000);
        assert_eq!(cfg.log_level, LogLevel::Info);
    }

    #[test]
    fn long_flags_are_parsed() {
        let cfg = parse_args(args(&["--address", "0.0.0.0", "--port", "8080", "--log-level", "debug"]));
        assert_eq!(cfg.address,   "0.0.0.0");
        assert_eq!(cfg.port,      8080);
        assert_eq!(cfg.log_level, LogLevel::Debug);
    }

    #[test]
    fn short_flags_are_parsed() {
        let cfg = parse_args(args(&["-a", "192.168.1.1", "-p", "1234", "-l", "warn"]));
        assert_eq!(cfg.address,   "192.168.1.1");
        assert_eq!(cfg.port,      1234);
        assert_eq!(cfg.log_level, LogLevel::Warn);
    }

    #[test]
    fn equals_form_is_parsed() {
        let cfg = parse_args(args(&["--address=0.0.0.0", "--port=8080", "--log-level=error"]));
        assert_eq!(cfg.address,   "0.0.0.0");
        assert_eq!(cfg.port,      8080);
        assert_eq!(cfg.log_level, LogLevel::Error);
    }

    #[test]
    fn binary_name_is_skipped() {
        let cfg = parse_args(args(&["haven", "--port", "1111"]));
        assert_eq!(cfg.port, 1111);
    }

    #[test]
    fn invalid_ip_uses_default() {
        let cfg = parse_args(args(&["--address", "not-an-ip"]));
        assert_eq!(cfg.address, "127.0.0.1");
    }

    #[test]
    fn invalid_port_zero_uses_default() {
        let cfg = parse_args(args(&["--port", "0"]));
        assert_eq!(cfg.port, 9000);
    }

    #[test]
    fn invalid_port_text_uses_default() {
        let cfg = parse_args(args(&["--port", "notaport"]));
        assert_eq!(cfg.port, 9000);
    }

    #[test]
    fn unknown_flag_is_ignored() {
        let cfg = parse_args(args(&["--unknown", "value"]));
        assert_eq!(cfg.address, "127.0.0.1");
        assert_eq!(cfg.port,    9000);
    }

    #[test]
    fn setup_autostart_long_flag() {
        let cfg = parse_args(args(&["--setup-autostart"]));
        assert_eq!(cfg.autostart_action, Some(AutostartAction::Setup));
        assert!(!cfg.address_specified);
    }

    #[test]
    fn setup_autostart_short_flag() {
        let cfg = parse_args(args(&["-A"]));
        assert_eq!(cfg.autostart_action, Some(AutostartAction::Setup));
    }

    #[test]
    fn remove_autostart_long_flag() {
        let cfg = parse_args(args(&["--remove-autostart"]));
        assert_eq!(cfg.autostart_action, Some(AutostartAction::Remove));
        assert!(!cfg.address_specified);
    }

    #[test]
    fn remove_autostart_short_flag() {
        let cfg = parse_args(args(&["-R"]));
        assert_eq!(cfg.autostart_action, Some(AutostartAction::Remove));
    }

    #[test]
    fn autostart_with_valid_address_sets_address_specified() {
        let cfg = parse_args(args(&["-A", "-a", "0.0.0.0"]));
        assert_eq!(cfg.autostart_action, Some(AutostartAction::Setup));
        assert!(cfg.address_specified);
        assert_eq!(cfg.address, "0.0.0.0");
    }

    #[test]
    fn autostart_with_invalid_address_leaves_address_specified_false() {
        let cfg = parse_args(args(&["-A", "-a", "not-an-ip"]));
        assert_eq!(cfg.autostart_action, Some(AutostartAction::Setup));
        assert!(!cfg.address_specified);
    }

    #[test]
    fn all_log_levels_parse() {
        for (s, expected) in &[
            ("trace", LogLevel::Trace),
            ("debug", LogLevel::Debug),
            ("info",  LogLevel::Info),
            ("warn",  LogLevel::Warn),
            ("error", LogLevel::Error),
            ("off",   LogLevel::Off),
        ] {
            let cfg = parse_args(args(&["-l", s]));
            assert_eq!(cfg.log_level, *expected, "failed for level '{}'", s);
        }
    }
}
