use crate::responses::response_builder::ResponseBuilder;
use std::io::Write;
use std::net::TcpStream;

pub fn send<T: ResponseBuilder>(stream: &mut TcpStream, response_builder: T) {
    match stream.write_all(response_builder.build().as_bytes()) {
        Ok(_) => println!("[DEBUG] Successfully send response"),
        Err(e) => panic!("[ERROR] Failed to send response, error {}", e),
    }
}
