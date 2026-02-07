use std::net::TcpStream;

use crate::web_server::connection_state::ConnectionState;
use crate::web_server::stream_processing::stream_reader::read_from_stream;
use crate::web_server::stream_processing::web_socket_stream_data_processor::WebSocketStreamDataProcessor;

pub fn listen_web_socket_messages(stream: &mut TcpStream) -> ConnectionState {
    let processor = WebSocketStreamDataProcessor::new();
    loop {
        match read_from_stream(4096, stream, &processor) {
            ConnectionState::KeepALive  => { continue; }
            ConnectionState::Close      => { break }
        }
    }

    ConnectionState::Close
}
