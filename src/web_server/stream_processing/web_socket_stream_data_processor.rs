use std::net::TcpStream;

use crate::core::core_state::CoreState;
use crate::web_server::stream_processing::stream_data_processor::StreamDataProcessor;
use crate::web_server::connection_state::ConnectionState;
use crate::web_server::web_socket::message_transferring::create_response;
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
            self.send_status(stream);
            return ConnectionState::KeepALive
        }
        
        let request = parse_request(message.clone());
        if request.is_none() {
            logger::info(format!("receive unsupported message: {}", message).as_str());
            return ConnectionState::KeepALive
        }
        
        match request.unwrap().request_type {
            RequestType::Connect            => core::connect_sync(),
            RequestType::Disconnect         => core::disconnect_sync(),
            RequestType::Restart            => core::restart_sync(),
            RequestType::ReconnectToWiFi    => core::reconnect_to_wifi(),
            RequestType::Status             => { /* Do nothing */ }
        }

        self.send_status(stream);

        ConnectionState::KeepALive
    }
}

impl WebSocketStreamDataProcessor {

    fn send_status(&self, stream: &mut TcpStream) {
        let status = core::calculate_state_sync();
        if let Some(response) = create_response(status) {
            sender::send(stream, response.as_str());
            return
        }

        logger::error("Failed to send status, status is missing");
    }
}
