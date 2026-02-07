use crate::core::commands::location::Location;
use crate::core::commands::restart_time::RestartTime;

#[derive(Clone)]
pub enum CoreCommand {
    Connect(Location),
    Disconnect,
    Restart(RestartTime),
    Status,
    GetSSID,
    ReconnectToSSID(String),
}
