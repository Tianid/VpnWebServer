use crate::requests::http_request::HttpRequest;
use crate::responses::ws_connection_response_builder::WsConnectionResponseBuilder;
use crate::web_server::connection_state::ConnectionState;
use crate::web_server::sender::http_sender;
use crate::web_server::web_socket::listener::listen_web_socket_messages;
use crate::logger;
use base64::encode;
use sha1::{Digest, Sha1};
use std::io::Write;
use std::net::TcpStream;

pub fn process(stream: &mut TcpStream, http_request: &HttpRequest) -> ConnectionState {
    if http_request.headers["Upgrade"] != "websocket" {
        logger::error("[ERROR] Failed to initialize WebSocket handshake, wrong WS connection");
        return ConnectionState::Close
    }

    init_ws_handshake(stream, http_request);
    listen_web_socket_messages(stream)
}

fn init_ws_handshake(stream: &mut TcpStream, http_request: &HttpRequest) {
    let default = String::new();
    let sec_websocket_key = http_request
        .headers
        .get("Sec-WebSocket-Key")
        .unwrap_or(&default)
        .trim();

    let builder = WsConnectionResponseBuilder::new()
        .response_key(generate_websocket_accept_key(sec_websocket_key).as_str());

    http_sender::send(stream, builder);
    stream.flush().expect("[ERROR] Failed to flush stream"); // TODO improve it
    logger::info("WebSocket handshake has been successfully completed");
}

fn generate_websocket_accept_key(key: &str) -> String {
    let mut hasher = Sha1::new();
    let magic_string = format!("{}258EAFA5-E914-47DA-95CA-C5AB0DC85B11", key);
    hasher.update(magic_string.as_bytes());
    let result = hasher.finalize();
    encode(result)
}
