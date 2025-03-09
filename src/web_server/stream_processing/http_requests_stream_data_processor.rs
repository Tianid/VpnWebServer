use std::net::TcpStream;

use crate::requests::http_request::HttpRequest;
use crate::web_server::connection_state::ConnectionState;
use crate::web_server::stream_processing::stream_data_processor::StreamDataProcessor;
use crate::web_server::http_requests_processing::http_request_processor;
use crate::logger;

#[derive(Debug, Clone, Copy)]
pub struct HttpRequestsStreamDataProcesspr {}

impl HttpRequestsStreamDataProcesspr {
    pub fn new() -> Self {
        Self {}
    }
}

impl StreamDataProcessor for HttpRequestsStreamDataProcesspr {
    fn process(self, stream: &mut TcpStream, size: usize, data: &[u8]) -> ConnectionState {
        let request = String::from_utf8_lossy(&data[..size]);
        match HttpRequest::new(request.as_ref()) {
            Some(http_request)  => {
                logger::info(format!("Receive HTTP-request: {:?}", http_request).as_str());
                http_request_processor::process(stream, &http_request)
            }
            None                => {
                logger::info(format!("Receive not valid HTTP-request='{}', close connection",request).as_str());
                ConnectionState::Close
            }
        }
    }
}
