use serde::Deserialize;
use std::net::TcpStream;

use crate::core::LocationCache;
use crate::logger;
use crate::requests::http_method::HttpMethod;
use crate::requests::http_request::HttpRequest;
use crate::responses::http_response_builder::HttpResponseBuilder;
use crate::utils::resource_provider::read_content;
use crate::server::connection_state::ConnectionState;
use crate::server::sender;
use crate::server::ws::handler as ws_handler;
use super::pages::Page;

pub fn route(stream: &mut TcpStream, req: &HttpRequest, cache: LocationCache) -> ConnectionState {
    let path = req.path.as_str();
    match &req.method {
        HttpMethod::GET => match path {
            "/"   => serve_file(stream, Page::Index.path(), "text/html"),
            "/ws" => ws_handler::handle(stream, req, cache),
            p if p.starts_with("/resources/") => {
                let file_path = p.trim_start_matches('/');
                let ct = content_type(file_path);
                serve_file(stream, file_path, ct)
            }
            _ => {
                log_warn!("http", "404 Not Found: GET {}", path);
                sender::send(stream, not_found())
            }
        },
        HttpMethod::POST => match path {
            "/api/config" => handle_api_config(stream, req),
            _ => {
                log_warn!("http", "404 Not Found: POST {}", path);
                sender::send(stream, not_found())
            }
        },
        method => {
            log_warn!("http", "Method {:?} not supported for {}", method, path);
            sender::send(stream, not_found())
        }
    }
}

#[derive(Deserialize)]
struct ConfigRequest {
    log_level: String,
}

fn handle_api_config(stream: &mut TcpStream, req: &HttpRequest) -> ConnectionState {
    let body = match &req.body {
        Some(b) => b.as_str(),
        None => {
            log_warn!("http", "POST /api/config: missing body");
            return sender::send(stream, bad_request("missing body"));
        }
    };
    let cfg = match serde_json::from_str::<ConfigRequest>(body) {
        Ok(c)  => c,
        Err(e) => {
            log_warn!("http", "POST /api/config: JSON parse error: {}", e);
            return sender::send(stream, bad_request("invalid JSON"));
        }
    };
    match logger::LogLevel::from_str(&cfg.log_level) {
        Some(level) => {
            logger::set_level(level);
            let resp = format!("{{\"log_level\":\"{}\"}}", cfg.log_level);
            sender::send(stream, ok_json(&resp))
        }
        None => {
            log_warn!("http", "POST /api/config: invalid log_level '{}'", cfg.log_level);
            sender::send(stream, bad_request(&format!("invalid log_level: {}", cfg.log_level)))
        }
    }
}

fn serve_file(stream: &mut TcpStream, path: &str, ct: &str) -> ConnectionState {
    match read_content(path) {
        Some(content) => sender::send(
            stream,
            HttpResponseBuilder::new().status_code(200).content_type(ct).text(&content),
        ),
        None => {
            log_warn!("http", "Static file not found: {}", path);
            sender::send(stream, not_found())
        }
    }
}

fn content_type(path: &str) -> &'static str {
    if path.ends_with(".js")   { "application/javascript" }
    else if path.ends_with(".html") { "text/html" }
    else if path.ends_with(".css")  { "text/css" }
    else                            { "application/octet-stream" }
}

fn not_found() -> HttpResponseBuilder {
    HttpResponseBuilder::new().status_code(404).content_type("text/plain").text("404 Not Found")
}

fn bad_request(msg: &str) -> HttpResponseBuilder {
    HttpResponseBuilder::new().status_code(400).content_type("text/plain").text(msg)
}

fn ok_json(body: &str) -> HttpResponseBuilder {
    HttpResponseBuilder::new().status_code(200).content_type("application/json").text(body)
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_type_js() {
        assert_eq!(content_type("script.js"), "application/javascript");
    }

    #[test]
    fn content_type_html() {
        assert_eq!(content_type("page.html"), "text/html");
    }

    #[test]
    fn content_type_css() {
        assert_eq!(content_type("style.css"), "text/css");
    }

    #[test]
    fn content_type_unknown_falls_back_to_octet_stream() {
        assert_eq!(content_type("file.bin"), "application/octet-stream");
        assert_eq!(content_type("data.json"), "application/octet-stream");
        assert_eq!(content_type("noextension"), "application/octet-stream");
    }

    #[test]
    fn content_type_path_with_directories() {
        assert_eq!(content_type("resources/page_scripts/client.js"), "application/javascript");
        assert_eq!(content_type("resources/html_pages/index.html"), "text/html");
    }
}
