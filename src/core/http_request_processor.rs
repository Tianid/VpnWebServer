use std::net::TcpStream;
use crate::requests::http_method::HttpMethod;
use crate::requests::http_request::HttpRequest;
use crate::core::http_request_get_processor;

pub fn process(stream: &mut TcpStream, http_request: &HttpRequest) {
    match http_request.method {
        HttpMethod::GET     => { http_request_get_processor::process(stream, http_request) }
        HttpMethod::POST    => { println!("[WARN] POST method is no supported to be processed") }
        HttpMethod::DELETE  => { println!("[WARN] DELETE method is no supported to be processed") }
        HttpMethod::UPDATE  => { println!("[WARN] UPDATE method is no supported to be processed") }
        HttpMethod::HEAD    => { println!("[WARN] HEAD method is no supported to be processed") }
        HttpMethod::PUT     => { println!("[WARN] PUT method is no supported to be processed") }
        HttpMethod::OPTIONS => { println!("[WARN] OPTIONS method is no supported to be processed") }
        HttpMethod::CONNECT => { println!("[WARN] CONNECT method is no supported to be processed") }
        HttpMethod::TRACE   => { println!("[WARN] TRACE method is no supported to be processed") }
        HttpMethod::PATCH   => { println!("[WARN] PATCH method is no supported to be processed") }
    }
}
