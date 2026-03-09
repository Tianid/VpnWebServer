use std::net::TcpStream;

use crate::core::LocationCache;
use crate::requests::http_request::HttpRequest;
use crate::server::connection_state::ConnectionState;
use crate::server::request_handler::RequestHandler;
use crate::server::router;

#[derive(Debug, Clone)]
pub struct HttpHandler {
    cache: LocationCache,
}

impl HttpHandler {
    pub fn new(cache: LocationCache) -> Self {
        Self { cache }
    }
}

impl RequestHandler for HttpHandler {
    fn handle(&self, stream: &mut TcpStream, size: usize, data: &[u8]) -> ConnectionState {
        let request = String::from_utf8_lossy(&data[..size]);
        match HttpRequest::new(request.as_ref()) {
            Some(http_request) => {
                log_debug!("http", "HTTP request: {:?}", http_request);
                router::route(stream, &http_request, self.cache.clone())
            }
            None => {
                log_warn!("http", "Invalid HTTP request, closing connection");
                ConnectionState::Close
            }
        }
    }
}
