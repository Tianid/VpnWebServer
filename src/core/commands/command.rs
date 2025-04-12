use crate::core::commands::location::Location;
use crate::core::commands::restart_time::RestartTime;

#[derive(Clone, Copy)]
pub enum CoreCommand {
    Connect(Location),
    Disconnect,
    Restart(RestartTime),
    Status
}
