use crate::core::response_sender::send;
use crate::core::ws_message_coding::{decode_message, encode_message};
use crate::requests::http_request::HttpRequest;
use crate::responses::ws_connection_response_builder::WsConnectionResponseBuilder;
use base64::{encode, Engine};
use sha1::{Digest, Sha1};
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn process(stream: &mut TcpStream, http_request: &HttpRequest) {
    if http_request.headers["Upgrade"] != "websocket" {
        println!("[ERROR] Failed to initialize WebSocket handshake, wrong WS connection");
        return;
    }

    init_ws_handshake(stream, http_request);
    handle_ws_message(stream);
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

    send(stream, builder);
    stream.flush().expect("[ERROR] Failed to flush stream");
    println!("[INFO] WebSocket handshake has been successfully completed");
}

fn generate_websocket_accept_key(key: &str) -> String {
    let mut hasher = Sha1::new();
    let magic_string = format!("{}258EAFA5-E914-47DA-95CA-C5AB0DC85B11", key);
    hasher.update(magic_string.as_bytes());
    let result = hasher.finalize();
    encode(result)
}

fn send_ws_message(stream: &mut TcpStream, message: &str) {
    match stream.write_all(&encode_message(message)) {
        Ok(_) => {
            println!("[DEBUG] Successfully send message to WebSocket")
        }
        Err(error) => {
            println!("[ERROR] Failed to send message to WebSocket: {}", error)
        }
    }
}

fn handle_ws_message(stream: &mut TcpStream) {
    let mut buffer = [0; 1024];

    while let Ok(size) = stream.read(&mut buffer) {
        println!("!!!! HANDLE IN WORK");
        if size == 0 {
            println!("[INFO] Client disconnected");
            break;
        }

        let message = decode_message(&buffer[..size]);
        println!("[INFO] Receive message from WebSocket: {}", message);

        if message == "exit" {
            println!("[INFO] Closing connection");
            break;
        }

        send_ws_message(stream, &message)
    }
    println!("!!!! HANDLE SCOPE IS OVER")
}
