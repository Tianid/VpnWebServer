use std::net::TcpStream;

use crate::core::core_state::CoreState;
use crate::web_server::stream_processing::stream_data_processor::StreamDataProcessor;
use crate::web_server::connection_state::ConnectionState;
use crate::web_server::web_socket::message_transferring::parse_request;
use crate::web_server::web_socket::message_transferring::ws_request::RequestType;
use crate::web_server::web_socket::ser_deser::decode_message;
use crate::web_server::web_socket::sender;
use crate::logger;
use crate::core;

#[derive(Debug, Clone, Copy)]
pub struct WebSocketStreamDataProcessor { }

impl WebSocketStreamDataProcessor {

    pub fn new() -> Self { Self { } }
}

impl StreamDataProcessor for WebSocketStreamDataProcessor {

    fn process(self, stream: &mut TcpStream, size: usize, data: &[u8]) -> ConnectionState {
        let message = decode_message(&data[..size]);
        logger::info(format!("Receive message : {}", message).as_str());

        if message == "PING" {
            sender::send(stream, "PONG");
            return ConnectionState::KeepALive
        }
        
        let request = parse_request(message.clone());
        if request.is_none() {
            logger::info(format!("receive unsupported message: {}", message).as_str());
            return ConnectionState::KeepALive
        }
        
        match request.unwrap().request_type {
            RequestType::Connect    => core::connect_sync(),
            RequestType::Disconnect => core::disconnect_sync(),
            RequestType::Restart    => core::restart_sync(),
        }

        let status = core::calculate_state_sync();
        sender::send(stream, &self.get_strings_from_status(status));

        ConnectionState::KeepALive
    }
}

impl WebSocketStreamDataProcessor {
    fn get_strings_from_status(&self, status: CoreState) -> String {
        match status {
            CoreState::Connected        => String::from("connected"),
            CoreState::Connecting       => String::from("connecting"),
            CoreState::Disconnected     => String::from("disconnected"),
            CoreState::Reconnecting     => String::from("reconnecting"),
            CoreState::Disconnecting    => String::from("disconnecting"),
        }
    }
}
