use std::net::TcpStream;

use crate::web_server::stream_processing::stream_data_processor::StreamDataProcessor;
use crate::web_server::connection_state::ConnectionState;
use crate::web_server::web_socket::ser_deser::decode_message;
use crate::web_server::web_socket::sender;
use crate::logger;

#[derive(Debug, Clone, Copy)]
pub struct WebSocketStreamDataProcessor { }

impl WebSocketStreamDataProcessor {

    pub fn new() -> Self { Self { } }
}

impl StreamDataProcessor for WebSocketStreamDataProcessor {

    fn process(self, stream: &mut TcpStream, size: usize, data: &[u8]) -> ConnectionState {
        let message = decode_message(&data[..size]);
        logger::info(format!("Receive message : {}", message).as_str());
        sender::send(stream, message.as_str());
        // FIXME process message
        // FIXME send message
        ConnectionState::KeepALive
    }
}
