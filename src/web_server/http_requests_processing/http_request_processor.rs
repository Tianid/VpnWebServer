use std::net::TcpStream;
use crate::requests::http_method::HttpMethod;
use crate::requests::http_request::HttpRequest;
use crate::web_server::http_requests_processing::get_processor;
use crate::web_server::connection_state::ConnectionState;
use crate::logger;

pub fn process(stream: &mut TcpStream, http_request: &HttpRequest) -> ConnectionState {
    match http_request.method {
        HttpMethod::GET     => { get_processor::process(stream, http_request) }
        HttpMethod::POST    => { logger::warn("POST method is no supported to be processed"); ConnectionState::Close }
        HttpMethod::DELETE  => { logger::warn("DELETE method is no supported to be processed"); ConnectionState::Close }
        HttpMethod::UPDATE  => { logger::warn("UPDATE method is no supported to be processed"); ConnectionState::Close }
        HttpMethod::HEAD    => { logger::warn("HEAD method is no supported to be processed"); ConnectionState::Close }
        HttpMethod::PUT     => { logger::warn("PUT method is no supported to be processed"); ConnectionState::Close }
        HttpMethod::OPTIONS => { logger::warn("OPTIONS method is no supported to be processed"); ConnectionState::Close }
        HttpMethod::CONNECT => { logger::warn("CONNECT method is no supported to be processed"); ConnectionState::Close }
        HttpMethod::TRACE   => { logger::warn("TRACE method is no supported to be processed"); ConnectionState::Close }
        HttpMethod::PATCH   => { logger::warn("PATCH method is no supported to be processed"); ConnectionState::Close }
    }
}
