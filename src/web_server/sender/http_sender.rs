use crate::responses::response_builder::ResponseBuilder;
use crate::web_server::connection_state::ConnectionState;
use crate::logger;
use std::io::Write;
use std::net::TcpStream;

pub fn send<T: ResponseBuilder>(stream: &mut TcpStream, response_builder: T) -> ConnectionState {
    match stream.write_all(response_builder.build().as_bytes()) {
        Ok(_)   => { logger::debug("Successfully send response") }
        Err(e)  => { logger::error(format!("Failed to send response, error {}", e).as_str()) }
    }

    ConnectionState::Close
}
