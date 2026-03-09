mod connection_state;
mod request_handler;
mod reader;
mod http_handler;
mod pages;
mod router;
mod sender;
pub mod ws;

use connection_state::ConnectionState;
use reader::read_stream;
use http_handler::HttpHandler;
use std::net::{TcpListener, TcpStream};
use crate::config::args::ServerConfig;
use crate::core::LocationCache;

pub fn start(configuration: ServerConfig, cache: LocationCache) {
    let upstream = format!("{}:{}", configuration.address, configuration.port);
    let listener = TcpListener::bind(upstream.clone())
        .expect(format!("[ERROR] Failed to bind to upstream {}", upstream).as_str());
    log_info!("server", "Listening on {}", upstream);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let peer = stream.peer_addr().map(|a| a.to_string()).unwrap_or_else(|_| "unknown".to_string());
                log_info!("server", "New TCP connection from {}", peer);
                let thread_cache = cache.clone();
                std::thread::spawn(move || {
                    listen_stream(&mut stream, thread_cache);
                });
            }
            Err(error) => {
                log_error!("server", "Stream error: {}", error);
            }
        }
    }
}

fn listen_stream(stream: &mut TcpStream, cache: LocationCache) {
    let handler = HttpHandler::new(cache);
    loop {
        match read_stream(1024, stream, &handler) {
            ConnectionState::KeepAlive  => { continue; }
            ConnectionState::Close      => {
                log_info!("server", "Client connection closed");
                break
            }
        }
    }
}

