use serde::{Deserialize, Serialize};

use crate::core::{CoreState, Location};
use crate::logger::LogLine;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    Status,
    Connect {
        #[serde(default)]
        location: Option<String>,
    },
    Disconnect,
    ReconnectWifi,
    Restart,
    RefreshLocations,
    SetLogLevel { level: String },
    GetSystemInfo,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    StatusUpdate {
        state: CoreState,
        location: Option<String>,
    },
    LocationList {
        locations: Vec<Location>,
    },
    Error {
        code: String,
        message: String,
    },
    LogLine {
        timestamp: String,
        level: String,
        tag: String,
        pid: u32,
        tid: u64,
        message: String,
    },
    LogLevelChanged {
        level: String,
    },
    SystemInfo {
        cpu_temp_c:   Option<f32>,
        uptime_s:     u64,
        mem_free_kb:  u64,
        mem_total_kb: u64,
    },
}

impl ServerMessage {
    pub fn from_log_line(line: &LogLine) -> Self {
        ServerMessage::LogLine {
            timestamp: line.timestamp.clone(),
            level:     line.level.clone(),
            tag:       line.tag.clone(),
            pid:       line.pid,
            tid:       line.tid,
            message:   line.message.clone(),
        }
    }
}





#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{CoreState, Location};

    fn deser<'a, T: serde::de::DeserializeOwned>(json: &str) -> T {
        serde_json::from_str(json).expect("deserialize failed")
    }

    fn ser(msg: &ServerMessage) -> String {
        serde_json::to_string(msg).expect("serialize failed")
    }

    #[test]
    fn client_status_deserializes() {
        let msg: ClientMessage = deser(r#"{"type":"Status"}"#);
        assert!(matches!(msg, ClientMessage::Status));
    }

    #[test]
    fn client_connect_with_location_deserializes() {
        let msg: ClientMessage = deser(r#"{"type":"Connect","location":"Helsinki"}"#);
        assert!(matches!(msg, ClientMessage::Connect { location: Some(ref l) } if l == "Helsinki"));
    }

    #[test]
    fn client_connect_without_location_deserializes() {
        let msg: ClientMessage = deser(r#"{"type":"Connect"}"#);
        assert!(matches!(msg, ClientMessage::Connect { location: None }));
    }

    #[test]
    fn client_connect_null_location_deserializes() {
        let msg: ClientMessage = deser(r#"{"type":"Connect","location":null}"#);
        assert!(matches!(msg, ClientMessage::Connect { location: None }));
    }

    #[test]
    fn client_disconnect_deserializes() {
        let msg: ClientMessage = deser(r#"{"type":"Disconnect"}"#);
        assert!(matches!(msg, ClientMessage::Disconnect));
    }

    #[test]
    fn client_reconnect_wifi_deserializes() {
        let msg: ClientMessage = deser(r#"{"type":"ReconnectWifi"}"#);
        assert!(matches!(msg, ClientMessage::ReconnectWifi));
    }

    #[test]
    fn client_restart_deserializes() {
        let msg: ClientMessage = deser(r#"{"type":"Restart"}"#);
        assert!(matches!(msg, ClientMessage::Restart));
    }

    #[test]
    fn client_refresh_locations_deserializes() {
        let msg: ClientMessage = deser(r#"{"type":"RefreshLocations"}"#);
        assert!(matches!(msg, ClientMessage::RefreshLocations));
    }

    #[test]
    fn client_set_log_level_deserializes() {
        let msg: ClientMessage = deser(r#"{"type":"SetLogLevel","level":"debug"}"#);
        assert!(matches!(msg, ClientMessage::SetLogLevel { ref level } if level == "debug"));
    }

    #[test]
    fn server_status_update_connected_serializes() {
        let json = ser(&ServerMessage::StatusUpdate {
            state:    CoreState::Connected,
            location: Some("Helsinki, Finland".to_string()),
        });
        assert!(json.contains(r#""type":"StatusUpdate""#));
        assert!(json.contains(r#""state":"Connected""#));
        assert!(json.contains("Helsinki, Finland"));
    }

    #[test]
    fn server_status_update_disconnected_no_location_serializes() {
        let json = ser(&ServerMessage::StatusUpdate {
            state:    CoreState::Disconnected,
            location: None,
        });
        assert!(json.contains(r#""state":"Disconnected""#));
        assert!(json.contains(r#""location":null"#));
    }

    #[test]
    fn server_location_list_serializes() {
        let json = ser(&ServerMessage::LocationList {
            locations: vec![
                Location { iso: "FI".into(), city: "Helsinki".into(), country: "Finland".into(), ping_ms: 60 },
            ],
        });
        assert!(json.contains(r#""type":"LocationList""#));
        assert!(json.contains(r#""id":"FI""#));
        assert!(json.contains("Helsinki"));
        assert!(json.contains(r#""ping_ms":60"#));
    }

    #[test]
    fn server_error_serializes() {
        let json = ser(&ServerMessage::Error {
            code:    "CommandFailed".to_string(),
            message: "exit code 1".to_string(),
        });
        assert!(json.contains(r#""type":"Error""#));
        assert!(json.contains(r#""code":"CommandFailed""#));
        assert!(json.contains("exit code 1"));
    }

    #[test]
    fn server_log_level_changed_serializes() {
        let json = ser(&ServerMessage::LogLevelChanged { level: "debug".to_string() });
        assert!(json.contains(r#""type":"LogLevelChanged""#));
        assert!(json.contains(r#""level":"debug""#));
    }

    #[test]
    fn server_log_line_serializes() {
        let json = ser(&ServerMessage::LogLine {
            timestamp: "2026-03-09 14:23:01:042".to_string(),
            level:     "INFO".to_string(),
            tag:       "ws".to_string(),
            pid:       1234,
            tid:       2,
            message:   "test message".to_string(),
        });
        assert!(json.contains(r#""type":"LogLine""#));
        assert!(json.contains(r#""level":"INFO""#));
        assert!(json.contains(r#""tag":"ws""#));
        assert!(json.contains("test message"));
    }

    #[test]
    fn client_get_system_info_deserializes() {
        let msg: ClientMessage = deser(r#"{"type":"GetSystemInfo"}"#);
        assert!(matches!(msg, ClientMessage::GetSystemInfo));
    }

    #[test]
    fn server_system_info_with_temp_serializes() {
        let json = ser(&ServerMessage::SystemInfo {
            cpu_temp_c:   Some(52.3),
            uptime_s:     3600,
            mem_free_kb:  1_024_000,
            mem_total_kb: 4_096_000,
        });
        assert!(json.contains(r#""type":"SystemInfo""#));
        assert!(json.contains(r#""uptime_s":3600"#));
        assert!(json.contains(r#""mem_free_kb":1024000"#));
        assert!(json.contains(r#""mem_total_kb":4096000"#));
        assert!(json.contains(r#""cpu_temp_c":52"#));
    }

    #[test]
    fn server_system_info_without_temp_serializes() {
        let json = ser(&ServerMessage::SystemInfo {
            cpu_temp_c:   None,
            uptime_s:     0,
            mem_free_kb:  0,
            mem_total_kb: 0,
        });
        assert!(json.contains(r#""type":"SystemInfo""#));
        assert!(json.contains(r#""cpu_temp_c":null"#));
        assert!(json.contains(r#""mem_total_kb":0"#));
    }
}
