use serde::{Serialize};

#[derive(Clone, Copy, Debug, Serialize)]
pub enum CoreState {
    Connected,
    Disconnected,
    Connecting,
    Disconnecting,
    Reconnecting,
}
