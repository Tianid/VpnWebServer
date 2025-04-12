pub mod core_state;
mod command_builder;
mod command_parser;
mod commands;

use crate::logger;
use crate::core::core_state::CoreState;
use crate::core::commands::restart_time::RestartTime;
use crate::core::commands::location::Location;
use crate::core::commands::command::CoreCommand;
use crate::core::command_builder::CommandBuilder;

pub fn connect_sync() {
    let mut command = CommandBuilder::new()
        .set_command(CoreCommand::Connect(Location::FI))
        .build();

    match command.output() {
        Ok(_)       => logger::debug("Successfully execute connect command"),
        Err(error)  => logger::error(format!("Failed to execute connect command, error: {}", error).as_str())
    }
}

pub fn disconnect_sync() {
    let mut command = CommandBuilder::new()
        .set_command(CoreCommand::Disconnect)
        .build();

    match command.output() {
        Ok(_)       => logger::debug("Successfully execute disconnect command"),
        Err(error)  => logger::error(format!("Failed to execute disconnect command, error: {}", error).as_str()),
    }
}

pub fn restart_sync() {
    let mut command = CommandBuilder::new()
        .set_command(CoreCommand::Restart(RestartTime::Now))
        .build();

    match command.output() {
        Ok(_)       => logger::debug("Successfully execute restart command"),
        Err(error)  => logger::error(format!("Failed to execute restart command, error: {}", error).as_str()),
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
        Ok(output)       => {
            logger::debug("Successfully execute status command");
            let res = String::from_utf8_lossy(&output.stdout);
            let status = command_parser::parse_status(res.to_string());
            logger::debug(format!("Receive status from system: {:?}", status).as_str());
            status
        }
        Err(error)  => {
            logger::error(format!("Failed to execute status command error: {}", error).as_str());
            CoreState::Disconnected
        }
    }
}
