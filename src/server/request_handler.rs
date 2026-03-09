use std::net::TcpStream;

use crate::server::connection_state::ConnectionState;

pub trait RequestHandler {
    fn handle(&self, stream: &mut TcpStream, size: usize, data: &[u8]) -> ConnectionState;
}
