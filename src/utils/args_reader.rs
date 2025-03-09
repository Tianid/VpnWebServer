use std::env;
use std::net::Ipv4Addr;
use crate::entities::web_server_config::WebServerConfig;

const DEFAULT_SEVER_ADDRESS: &str = "127.0.0.1";
const DEFAULT_PORT: &str = "9000";

pub fn get_server_config() -> WebServerConfig {
    let args: Vec<String> = env::args().collect();

    let mut address = String::new();
    let mut port = String::new();

    let mut iter = args.iter().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--address" => {
                if let Some(value) = iter.next() {
                    address = value.to_string();
                }
            }
            "-a" => {
                if let Some(value) = iter.next() {
                    address = value.to_string();
                }
            }
            "--port" => {
                if let Some(value) = iter.next() {
                    port = value.to_string();
                }
            }
            "-p" => {
                if let Some(value) = iter.next() {
                    port = value.to_string();
                }
            }
            _ => {}
        }
    }

    if address.is_empty() || !is_valid_ipv4(&address) {
        address = DEFAULT_SEVER_ADDRESS.to_string();
    }

    if port.is_empty() || !port.parse::<u8>().is_ok() {
        port = DEFAULT_PORT.to_string();
    }

    WebServerConfig { address, port }
}

fn is_valid_ipv4(ip: &str) -> bool {
    ip.parse::<Ipv4Addr>().is_ok()
}
