mod connection_state;
mod stream_processing;
pub mod web_server_configuration;
mod http_requests_processing;
mod sender;
mod web_socket;

use connection_state::ConnectionState;
use stream_processing::{http_requests_stream_data_processor::HttpRequestsStreamDataProcesspr, stream_reader::read_from_stream};
use std::net::{TcpListener, TcpStream};
use crate::web_server::web_server_configuration::WebServerConfiguration;
use crate::logger;

pub fn start(configuration: WebServerConfiguration) {
    let upstream = format!("{}:{}", configuration.address, configuration.port);
    let listener = TcpListener::bind(upstream.clone())
        .expect(format!("[ERROR] Failed to bind to upstream {}", upstream).as_str());
    logger::info(format!("WebSocket server started at ws://{}/ws", upstream).as_str());

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("!!!! STREAM CREATED");
                std::thread::spawn(move || {
                    listen_stream(&mut stream);
                });
            }
            Err(error) => {
               logger::error(format!("Stream error occured, error: {}", error).as_str());
            }
        }
    }
}

fn listen_stream(stream: &mut TcpStream) {
    let processor = HttpRequestsStreamDataProcesspr::new();
    loop {
        match read_from_stream(1024, stream, &processor) {
            ConnectionState::KeepALive  => { continue; }
            ConnectionState::Close      => {
               logger::info("Client connection will be closed");
               break
            }
        }
    }
}
