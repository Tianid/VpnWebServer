mod command_builder;
mod command_parser;
mod commands;
pub mod core_state;

use std::process::Stdio;
use std::thread::sleep;
use std::time::Duration;

use crate::core::command_builder::CommandBuilder;
use crate::core::commands::command::CoreCommand;
use crate::core::commands::location::Location;
use crate::core::commands::restart_time::RestartTime;
use crate::core::core_state::CoreState;
use crate::logger;

pub fn connect_sync() {
    let mut command = CommandBuilder::new()
        .set_command(CoreCommand::Connect(Location::FI))
        .build();

    match command.output() {
        Ok(_) => logger::debug("Successfully execute connect command"),
        Err(error) => {
            logger::error(format!("Failed to execute connect command, error: {}", error).as_str())
        }
    }
}

pub fn disconnect_sync() {
    let mut command = CommandBuilder::new()
        .set_command(CoreCommand::Disconnect)
        .build();

    match command.output() {
        Ok(_) => logger::debug("Successfully execute disconnect command"),
        Err(error) => logger::error(
            format!("Failed to execute disconnect command, error: {}", error).as_str(),
        ),
    }
}

pub fn restart_sync() {
    let mut command = CommandBuilder::new()
        .set_command(CoreCommand::Restart(RestartTime::Now))
        .build();

    match command.output() {
        Ok(_) => logger::debug("Successfully execute restart command"),
        Err(error) => {
            logger::error(format!("Failed to execute restart command, error: {}", error).as_str())
        }
    }
}

pub fn reconnect_to_wifi() {
    let ssid = get_ssid();
    if ssid.is_none() {
        logger::error("Failed to execute command restart to wifi, ssid is missing");
        return;
    }

    let mut command = CommandBuilder::new()
        .set_command(CoreCommand::ReconnectToSSID(ssid.clone().unwrap()))
        .build();

    match command.output() {
        Ok(_) => {
            await_reconnection_to_ssid(ssid.unwrap().as_str(), 10, Duration::from_millis(500));
            logger::debug("Successfully execute restart to wifi command")
        }
        Err(error) => logger::error(
            format!(
                "Failed to execute command restart to wifi, error: {}",
                error
            )
            .as_str(),
        ),
    }
}

pub fn calculate_state_sync() -> CoreState {
    execute_status_command()
}

fn execute_status_command() -> CoreState {
    let mut command = CommandBuilder::new()
        .set_command(CoreCommand::Status)
        .build();

    match command.output() {
        Ok(output) => {
            logger::debug("Successfully execute status command");
            let res = String::from_utf8_lossy(&output.stdout);
            let status = command_parser::parse_status(res.to_string());
            logger::debug(format!("Receive status from system: {:?}", status).as_str());
            status
        }
        Err(error) => {
            logger::error(format!("Failed to execute status command error: {}", error).as_str());
            CoreState::Disconnected
        }
    }
}

fn get_ssid() -> Option<String> {
    let command = CommandBuilder::new()
        .set_command(CoreCommand::GetSSID)
        .build()
        .stdout(Stdio::piped())
        .output();

    match command {
        Ok(output) => {
            let ssid = String::from_utf8_lossy(&output.stdout).trim().to_string();
            logger::debug(format!("Successfully get SSID from system = {}", ssid).as_str());
            Some(ssid)
        }
        Err(error) => {
            logger::error(format!("Failed to get SSID from system, error = {}", error).as_str());
            None
        }
    }
}

fn await_reconnection_to_ssid(current_ssid: &str, timeout_secs: u64, try_check_in_ms: Duration) {
    let start = std::time::Instant::now();
    while start.elapsed().as_secs() < timeout_secs {
        match !(get_ssid().is_none_or(|ssid| ssid.is_empty() || current_ssid != ssid)) {
            true => {
                sleep(Duration::from_millis(1_500));
                return;
            }
            false => sleep(try_check_in_ms),
        }
    }
}
