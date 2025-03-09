use std::{io::Write, net::TcpStream};
use crate::web_server::web_socket::ser_deser::encode_message;
use crate::logger;

pub fn send(stream: &mut TcpStream, message: &str) {
    match stream.write_all(&encode_message(message)) {
        Ok(_)       => logger::debug("Successfully send message to WebSocket"),
        Err(error)  => logger::error(format!("Failed to send message to WebSocket: {}", error).as_str())
    }
}
