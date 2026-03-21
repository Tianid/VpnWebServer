mod terminal;
mod setup;
mod remove;

pub use crate::config::args::AutostartAction;

pub fn run(action: &AutostartAction, cfg: &crate::config::args::ServerConfig) {
    let result = match action {
        AutostartAction::Setup  => setup::setup(cfg),
        AutostartAction::Remove => remove::remove(),
    };
    if let Err(e) = result {
        eprintln!("[autostart] Error: {}", e);
        std::process::exit(1);
    }
}

fn resolve_home() -> Result<String, String> {
    if let Ok(h) = std::env::var("HOME") {
        if !h.is_empty() {
            return Ok(h);
        }
    }
    home_from_passwd()
}

fn home_from_passwd() -> Result<String, String> {
    let uid = getuid();
    let contents = std::fs::read_to_string("/etc/passwd")
        .map_err(|e| format!("Cannot read /etc/passwd: {}", e))?;
    for line in contents.lines() {
        let fields: Vec<&str> = line.splitn(7, ':').collect();
        if fields.len() >= 6 {
            if let Ok(entry_uid) = fields[2].parse::<u32>() {
                if entry_uid == uid {
                    return Ok(fields[5].to_string());
                }
            }
        }
    }
    Err("Cannot determine home directory: uid not found in /etc/passwd".to_string())
}

fn getuid() -> u32 {
    extern "C" {
        fn getuid() -> u32;
    }
    unsafe { getuid() }
}
