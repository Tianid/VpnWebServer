mod commands;
mod location_parser;
mod state_parser;
pub mod location;
pub mod location_cache;
pub mod sysinfo;

pub use location::Location;
pub use location_cache::LocationCache;

use std::fmt;
use std::io;
use std::process::Stdio;
use std::thread::sleep;
use std::time::Duration;

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum CoreState {
    Connected,
    Disconnected,
    Reconnecting,
}

#[derive(Debug)]
pub enum CoreError {
    CommandFailed { cmd: String, stderr: String },
    ParseError    { context: String, raw: String },
    IoError(io::Error),
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoreError::CommandFailed { cmd, stderr } => {
                write!(f, "Command '{}' failed: {}", cmd, stderr)
            }
            CoreError::ParseError { context, raw } => {
                write!(f, "Parse error in {}: {:?}", context, raw)
            }
            CoreError::IoError(e) => write!(f, "I/O error: {}", e),
        }
    }
}

pub type CoreResult<T> = Result<T, CoreError>;

pub fn connect(location: Option<String>) -> CoreResult<()> {
    let output = commands::vpn_connect(location.as_deref())
        .output()
        .map_err(CoreError::IoError)?;
    if output.status.success() {
        log_info!("core", "VPN connect command succeeded");
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        log_error!("core", "VPN connect command failed: {}", stderr);
        Err(CoreError::CommandFailed { cmd: "adguardvpn-cli connect".to_string(), stderr })
    }
}

pub fn disconnect() -> CoreResult<()> {
    let output = commands::vpn_disconnect()
        .output()
        .map_err(CoreError::IoError)?;
    if output.status.success() {
        log_info!("core", "VPN disconnect command succeeded");
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        log_error!("core", "VPN disconnect command failed: {}", stderr);
        Err(CoreError::CommandFailed { cmd: "adguardvpn-cli disconnect".to_string(), stderr })
    }
}

pub fn restart() -> CoreResult<()> {
    let output = commands::system_restart()
        .output()
        .map_err(CoreError::IoError)?;
    if output.status.success() {
        log_info!("core", "Restart command invoked");
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        log_error!("core", "Restart command failed: {}", stderr);
        Err(CoreError::CommandFailed { cmd: "shutdown -r now".to_string(), stderr })
    }
}

pub fn status() -> CoreResult<(CoreState, Option<String>)> {
    let output = commands::vpn_status()
        .output()
        .map_err(CoreError::IoError)?;
    let stdout   = String::from_utf8_lossy(&output.stdout);
    let state    = state_parser::parse_status(&stdout);
    let location = if state == CoreState::Connected {
        state_parser::parse_location(&stdout)
    } else {
        None
    };
    log_debug!("core", "adguardvpn-cli status → {:?}", state);
    Ok((state, location))
}

pub fn list_locations() -> CoreResult<Vec<Location>> {
    let output = commands::vpn_list_locations(100)
        .output()
        .map_err(CoreError::IoError)?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(CoreError::CommandFailed {
            cmd: "adguardvpn-cli list-locations".to_string(),
            stderr,
        });
    }
    let stdout   = String::from_utf8_lossy(&output.stdout);
    let mut locs = location_parser::parse_locations(&stdout);
    locs.sort_by_key(|l| if l.ping_ms < 0 { i32::MAX } else { l.ping_ms });
    log_debug!("core", "Fetched {} locations", locs.len());
    Ok(locs)
}

pub fn reconnect_wifi() -> CoreResult<()> {
    let ssid = get_ssid()?;
    if ssid.is_empty() {
        return Err(CoreError::ParseError {
            context: "get_ssid".to_string(),
            raw:     "(empty)".to_string(),
        });
    }
    let output = commands::wifi_reconnect(&ssid)
        .output()
        .map_err(CoreError::IoError)?;
    if output.status.success() {
        await_reconnection_to_ssid(&ssid, 10, Duration::from_millis(500));
        log_info!("wifi", "Wi-Fi reconnect succeeded");
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        log_error!("wifi", "Wi-Fi reconnect command failed: {}", stderr);
        Err(CoreError::CommandFailed {
            cmd: format!("nmcli connection up id {}", ssid),
            stderr,
        })
    }
}

fn get_ssid() -> CoreResult<String> {
    let output = commands::wifi_get_ssid()
        .stdout(Stdio::piped())
        .output()
        .map_err(CoreError::IoError)?;
    let ssid = String::from_utf8_lossy(&output.stdout).trim().to_string();
    log_debug!("wifi", "Detected SSID: {}", ssid);
    Ok(ssid)
}

fn await_reconnection_to_ssid(current_ssid: &str, timeout_secs: u64, check_interval: Duration) {
    let start = std::time::Instant::now();
    while start.elapsed().as_secs() < timeout_secs {
        match get_ssid() {
            Ok(ssid) if ssid == current_ssid && !ssid.is_empty() => {
                sleep(Duration::from_millis(1_500));
                return;
            }
            _ => sleep(check_interval),
        }
    }
    log_warn!("wifi", "SSID reconnect confirmation timed out after {}s", timeout_secs);
}
