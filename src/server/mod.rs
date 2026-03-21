mod connection_state;
mod request_handler;
mod reader;
mod http_handler;
mod pages;
mod router;
mod sender;
pub mod ws;

use reader::read_stream;
use http_handler::HttpHandler;
use std::net::{TcpListener, TcpStream};
use std::sync::OnceLock;
use std::time::Instant;
use crate::config::args::ServerConfig;
use crate::core::LocationCache;

static SERVER_START: OnceLock<Instant> = OnceLock::new();

pub fn uptime_secs() -> u64 {
    SERVER_START.get().map(|t| t.elapsed().as_secs()).unwrap_or(0)
}

pub fn start(configuration: ServerConfig, cache: LocationCache) {
    SERVER_START.get_or_init(Instant::now);
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

const READ_BUFFER_SIZE: usize = 4096;

fn listen_stream(stream: &mut TcpStream, cache: LocationCache) {
    let handler = HttpHandler::new(cache);
    read_stream(READ_BUFFER_SIZE, stream, &handler);
    log_info!("server", "Client connection closed");
}

