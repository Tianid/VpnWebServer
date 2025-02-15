mod vpn_core;

use std::{env, thread};
use std::fs;
use std::io::{Read, Write};
use std::net::{Ipv4Addr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use crate::vpn_core::{VpnManager, VpnStatus};

const DEFAULT_SEVER_ADDRESS: &str = "127.0.0.1";
const DEFAULT_PORT: &str = "9000";

fn main() {
    let (address, port) = get_server_params();
    println!("Running server on {}:{}", address, port);

    let vpn_manager = Arc::new(Mutex::new(VpnManager::new()));

    let listener = TcpListener::bind(format!("{}:{}", address, port)).expect("Failed to bind to Tcp listener");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {

                let vpn_manager = Arc::clone(&vpn_manager);
                thread::spawn(move || {
                    handle_client_requests(stream, vpn_manager);
                });

            }
            Err(e) => { println!("Error: {}", e) }
        }
    }
}


fn get_server_params() ->  (String, String) {
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

    (address, port)
}

fn is_valid_ipv4(ip: &str) -> bool {
    ip.parse::<Ipv4Addr>().is_ok()
}

fn handle_client_requests(mut stream: TcpStream, vpn_manager: Arc<Mutex<VpnManager>>) {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap_or(0);

    if bytes_read == 0 { return }

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);

    if request.starts_with("GET / HTTP/1.1") {
        println!("Receive GET request");
        send_response(&mut stream, "index.html", 200);
    } else if request.starts_with("GET /status HTTP/1.1") {
        println!("Receive GET /status request");
        send_status(&mut stream, vpn_manager.clone());
    } else if request.starts_with("POST /start HTTP/1.1") {
        println!("Receive POST /start request");
        vpn_manager.lock().unwrap().start();
        send_response_text(&mut stream, "Ok", 200);
    } else if request.starts_with("POST /stop HTTP/1.1") {
        println!("Receive POST /stop request");
        vpn_manager.lock().unwrap().stop();
        send_response_text(&mut stream, "Ok", 200);
    } else {
        println!("Restricted request {}", request.to_string());
        send_response_text(&mut stream, "404 Not Found", 404);
    }
}

fn send_response(stream: &mut TcpStream, filename: &str, status_code: u16) {
    let file_content = fs::read_to_string(format!("Resources/{}", filename));
    let response = match file_content {
        Ok(content) => format!(
            "HTTP/1.1 {} OK\r\nContent-Type: text/html\r\n\r\n{}",
            status_code, content
        ),
        Err(_) => "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\n\r\nФайл не найден!".to_string(),
    };
    stream.write_all(response.as_bytes()).unwrap();
}

fn send_response_text(stream: &mut TcpStream, text: &str, status_code: u16) {
    let response = format!(
        "HTTP/1.1 {} OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        status_code,
        text.len(),
        text
    );
    stream.write_all(response.as_bytes()).unwrap();
}


fn send_status(stream: &mut TcpStream, manager: Arc<Mutex<VpnManager>>) {
    let status = match manager.lock().unwrap().get_status() {
        VpnStatus::Disconnecting => "Disconnecting",
        VpnStatus::Disconnected => "Disconnected",
        VpnStatus::Connecting => "Connecting",
        VpnStatus::Connected => "Connected",
        VpnStatus::Reconnecting => "Reconnecting",
    };
    send_response_text(stream, status, 200);
}
