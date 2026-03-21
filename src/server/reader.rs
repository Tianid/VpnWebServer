use std::io::Read;
use std::net::TcpStream;
use crate::server::connection_state::ConnectionState;
use crate::server::request_handler::RequestHandler;
pub fn read_stream<T: RequestHandler>(
    buffer_size: usize,
    stream: &mut TcpStream,
    handler: &T,
) -> ConnectionState {
    let mut buffer = vec![0; buffer_size];

    match stream.read(&mut buffer) {
        Ok(0)       => {
            log_debug!("server", "Client closed connection");
            ConnectionState::Close
        }
        Ok(size)    => handler.handle(stream, size, &buffer),
        Err(error)  => {
            log_error!("server", "Stream read error: {}", error);
            ConnectionState::Close
        }
    }
}
