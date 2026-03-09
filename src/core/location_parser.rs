use regex::Regex;

use crate::core::location::Location;

pub fn parse_locations(output: &str) -> Vec<Location> {
    let re = Regex::new(r"\s{2,}").unwrap();
    output
        .lines()
        .filter(|l| {
            let b = l.as_bytes();
            b.len() >= 3
                && b[0].is_ascii_uppercase()
                && b[1].is_ascii_uppercase()
                && b[2].is_ascii_whitespace()
        })
        .filter_map(|line| {
            let parts: Vec<&str> = re.split(line.trim()).collect();
            if parts.len() != 4 {
                log_warn!("core", "location_parser: unexpected column count {} in {:?}", parts.len(), line);
                return None;
            }
            let ping = parts[3].parse::<i32>().unwrap_or(-1);
            Some(Location {
                iso:     parts[0].to_string(),
                country: parts[1].to_string(),
                city:    parts[2].to_string(),
                ping_ms: ping,
            })
        })
        .collect()
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input_returns_empty() {
        assert_eq!(parse_locations("").len(), 0);
    }

    #[test]
    fn header_only_is_skipped() {
        let input = "ISO   COUNTRY              CITY                           PING ESTIMATE";
        assert_eq!(parse_locations(input).len(), 0);
    }

    #[test]
    fn valid_row_parsed_correctly() {
        let input = "FI    Finland              Helsinki                       60";
        let locs = parse_locations(input);
        assert_eq!(locs.len(), 1);
        assert_eq!(locs[0].iso,     "FI");
        assert_eq!(locs[0].country, "Finland");
        assert_eq!(locs[0].city,    "Helsinki");
        assert_eq!(locs[0].ping_ms, 60);
    }

    #[test]
    fn header_and_multiple_rows_parsed() {
        let input = "ISO   COUNTRY              CITY                           PING ESTIMATE\n\
                     FI    Finland              Helsinki                       60\n\
                     DE    Germany              Frankfurt                      41";
        let locs = parse_locations(input);
        assert_eq!(locs.len(), 2);
        assert_eq!(locs[0].iso, "FI");
        assert_eq!(locs[1].iso, "DE");
    }

    #[test]
    fn non_numeric_ping_becomes_minus_one() {
        let input = "XX    Country              City                           n/a";
        let locs = parse_locations(input);
        assert_eq!(locs.len(), 1);
        assert_eq!(locs[0].ping_ms, -1);
    }

    #[test]
    fn wrong_column_count_is_skipped() {
        let input = "FI    Finland";
        assert_eq!(parse_locations(input).len(), 0);
    }

    #[test]
    fn empty_lines_are_skipped() {
        let input = "\n\nFI    Finland              Helsinki                       60\n\n";
        assert_eq!(parse_locations(input).len(), 1);
    }

    #[test]
    fn footer_line_is_skipped() {
        let input = "FI    Finland              Helsinki                       60\nYou can connect to a location by running `adguardvpn-cli connect -l 'city, country or ISO code'`";
        let locs = parse_locations(input);
        assert_eq!(locs.len(), 1);
        assert_eq!(locs[0].iso, "FI");
    }

    #[test]
    fn city_with_virtual_suffix_parsed() {
        let input = "IR    Iran                 Tehran (Virtual)               102       ";
        let locs = parse_locations(input);
        assert_eq!(locs.len(), 1);
        assert_eq!(locs[0].iso,     "IR");
        assert_eq!(locs[0].country, "Iran");
        assert_eq!(locs[0].city,    "Tehran (Virtual)");
        assert_eq!(locs[0].ping_ms, 102);
    }

    #[test]
    fn city_with_internal_spaces_parsed() {
        let input = "US    United States        New York                       109       ";
        let locs = parse_locations(input);
        assert_eq!(locs.len(), 1);
        assert_eq!(locs[0].iso,     "US");
        assert_eq!(locs[0].country, "United States");
        assert_eq!(locs[0].city,    "New York");
        assert_eq!(locs[0].ping_ms, 109);
    }

    #[test]
    fn trailing_spaces_on_ping_are_ignored() {
        let input = "DE    Germany              Frankfurt                      38        ";
        let locs = parse_locations(input);
        assert_eq!(locs.len(), 1);
        assert_eq!(locs[0].ping_ms, 38);
    }
}
