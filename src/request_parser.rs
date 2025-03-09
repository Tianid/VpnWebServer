use std::io::Read;
use std::net::TcpStream;

pub fn parse_request(mut stream: &TcpStream) -> Option<Requests> {
    let mut buffer = [0; 1024];
    let bytes_read = stream.peek(&mut buffer).unwrap_or(0);

    if bytes_read == 0 { return None }

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);

    println!("!!!! {}", request);

    if request.starts_with("GET / HTTP/1.1") {
        Some(Requests::Get)
    } else if request.starts_with("GET /web_js/update_status.js HTTP/1.1") {
        Some(Requests::GetJs)
    } else if request.starts_with("GET /ws HTTP/1.1") {
        Some(Requests::GetWs)
    } else if request.starts_with("POST /start HTTP/1.1") {
        Some(Requests::PostStart)
    } else if request.starts_with("POST /stop HTTP/1.1") {
        Some(Requests::PostStop)
    } else {
        None
    }
}


#[derive(Debug)]
pub enum Requests {
    Get,
    GetJs,
    GetWs,
    PostStart,
    PostStop
}
