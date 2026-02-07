use std::net::Ipv4Addr;

use super::args_reader::{self, RawArgs};
use crate::generated::constants::{WEB_SERVER_DEFAULT_ADDRESS, WEB_SERVER_DEFAULT_PORT};

/// Validated and finalized server arguments (with defaults applied).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerArgs {
    pub address: String,
    pub port: String,
}

/// Validate raw arguments and apply defaults.
/// Rules:
/// - address must be a valid IPv4 (e.g., "127.0.0.1"); otherwise default is used.
/// - port must be a valid u16; otherwise default is used.
/// - empty strings are treated as missing and replaced by defaults.
pub fn validate(raw: RawArgs) -> ServerArgs {
    let address = match raw.address {
        Some(address_str) => {
            if !address_str.is_empty() && address_str.parse::<Ipv4Addr>().is_ok() {
                address_str
            } else {
                WEB_SERVER_DEFAULT_ADDRESS.to_string()
            }
        }
        None => WEB_SERVER_DEFAULT_ADDRESS.to_string(),
    };

    let port = match raw.port {
        Some(port_str) => {
            if !port_str.is_empty() && port_str.parse::<u16>().is_ok() {
                port_str
            } else {
                WEB_SERVER_DEFAULT_PORT.to_string()
            }
        }
        None => WEB_SERVER_DEFAULT_PORT.to_string(),
    };

    ServerArgs { address, port }
}

/// Convenience helper: parse from an iterator and then validate/apply defaults.
pub fn parse_and_validate<T>(args: T) -> ServerArgs
where
    T: IntoIterator<Item = String>,
{
    let raw_args = args_reader::read_args(args);
    validate(raw_args)
}
