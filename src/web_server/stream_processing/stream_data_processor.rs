use std::net::TcpStream;

use crate::web_server::connection_state::ConnectionState;

pub trait StreamDataProcessor : Copy {
    fn process(self, stream: &mut TcpStream, size:  usize, data: &[u8]) -> ConnectionState;
}
