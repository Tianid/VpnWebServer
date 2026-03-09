use crate::responses::response_builder::ResponseBuilder;
use crate::server::connection_state::ConnectionState;
use std::io::Write;
use std::net::TcpStream;

pub fn send<T: ResponseBuilder>(stream: &mut TcpStream, response_builder: T) -> ConnectionState {
    match stream.write_all(response_builder.build().as_bytes()) {
        Ok(_)   => { log_debug!("http", "Response sent") }
        Err(e)  => { log_error!("http", "Failed to send response: {}", e) }
    }

    ConnectionState::Close
}
