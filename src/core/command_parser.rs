use crate::core::core_state::CoreState;

pub fn parse_status(output: String) -> CoreState {
    let lowercased = output.to_lowercase();

    match lowercased {
        _ if { lowercased.contains("reconnecting") } => CoreState::Reconnecting,
        _ if { lowercased.contains("disconnected") } => CoreState::Disconnected,
        _ if { lowercased.contains("connected") }    => CoreState::Connected,
        _                                            => CoreState::Disconnected,
    }
}

