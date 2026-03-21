use super::CoreState;

pub fn parse_status(output: &str) -> CoreState {
    let lower = output.to_lowercase();
    if lower.contains("reconnecting") {
        CoreState::Reconnecting
    } else if lower.contains("disconnected") {
        CoreState::Disconnected
    } else if lower.contains("connected") {
        CoreState::Connected
    } else {
        CoreState::Disconnected
    }
}

pub fn parse_location(output: &str) -> Option<String> {
    let lower = output.to_lowercase();
    if let Some(pos) = lower.find("connected to ") {
        let rest = &output[pos + "connected to ".len()..];
        let end = rest
            .find(" in ")
            .or_else(|| rest.find(|c: char| c == ',' || c == '\n' || c == '\r' || c == '.'))
            .unwrap_or(rest.len());
        let city = strip_ansi(rest[..end].trim());
        if !city.is_empty() {
            return Some(city);
        }
    }
    if let Some(pos) = lower.find("location:") {
        let rest = &output[pos + "location:".len()..];
        let end = rest
            .find(|c: char| c == '\n' || c == '\r' || c == ',')
            .unwrap_or(rest.len());
        let city = strip_ansi(rest[..end].trim());
        if !city.is_empty() {
            return Some(city);
        }
    }
    None
}

fn strip_ansi(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            for c2 in chars.by_ref() {
                if c2.is_ascii_alphabetic() {
                    break;
                }
            }
        } else {
            result.push(c);
        }
    }
    result
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connected_returns_connected() {
        assert_eq!(parse_status("VPN is connected"), CoreState::Connected);
    }

    #[test]
    fn disconnected_returns_disconnected() {
        assert_eq!(parse_status("VPN is disconnected"), CoreState::Disconnected);
    }

    #[test]
    fn reconnecting_returns_reconnecting() {
        assert_eq!(parse_status("reconnecting to server"), CoreState::Reconnecting);
    }

    #[test]
    fn reconnecting_wins_over_connected_substring() {
        assert_eq!(parse_status("reconnecting"), CoreState::Reconnecting);
    }

    #[test]
    fn unknown_output_returns_disconnected() {
        assert_eq!(parse_status("some unexpected output"), CoreState::Disconnected);
    }

    #[test]
    fn empty_input_returns_disconnected() {
        assert_eq!(parse_status(""), CoreState::Disconnected);
    }

    #[test]
    fn case_insensitive_connected() {
        assert_eq!(parse_status("CONNECTED"), CoreState::Connected);
    }

    #[test]
    fn case_insensitive_disconnected() {
        assert_eq!(parse_status("DISCONNECTED"), CoreState::Disconnected);
    }

    #[test]
    fn case_insensitive_reconnecting() {
        assert_eq!(parse_status("RECONNECTING"), CoreState::Reconnecting);
    }

    #[test]
    fn parse_location_connected_to_pattern() {
        assert_eq!(parse_location("VPN is connected to Amsterdam"), Some("Amsterdam".to_string()));
    }

    #[test]
    fn parse_location_stops_at_comma() {
        assert_eq!(parse_location("Connected to Amsterdam, Netherlands"), Some("Amsterdam".to_string()));
    }

    #[test]
    fn parse_location_stops_at_period() {
        assert_eq!(parse_location("Connected to Amsterdam."), Some("Amsterdam".to_string()));
    }

    #[test]
    fn parse_location_actual_cli_format() {
        assert_eq!(
            parse_location("Connected to FRANKFURT in TUN mode, running on tun0"),
            Some("FRANKFURT".to_string())
        );
    }

    #[test]
    fn parse_location_strips_ansi_codes() {
        assert_eq!(
            parse_location("Connected to \x1b[1mLEMESOS\x1b[0m in TUN mode, running on tun0"),
            Some("LEMESOS".to_string())
        );
    }

    #[test]
    fn parse_location_location_field_pattern() {
        assert_eq!(parse_location("Status: Connected\nLocation: Frankfurt"), Some("Frankfurt".to_string()));
    }

    #[test]
    fn parse_location_disconnected_returns_none() {
        assert_eq!(parse_location("VPN is disconnected"), None);
    }

    #[test]
    fn parse_location_empty_returns_none() {
        assert_eq!(parse_location(""), None);
    }
}
