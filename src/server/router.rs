use serde::Deserialize;
use std::net::TcpStream;

use crate::core::{self, CoreState, LocationCache};
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
    log_debug!("http", "{:?} {}", req.method, path);
    match &req.method {
        HttpMethod::GET => match path {
            "/" => {
                let ua = req.headers.get("User-Agent").map(String::as_str).unwrap_or("");
                let page = if is_mobile_ua(ua) { Page::Mobile } else { Page::Desktop };
                serve_file(stream, page.path(), "text/html")
            }
            "/health" => handle_health(stream),
            "/ws" => ws_handler::handle(stream, req, cache),
            p if p.starts_with("/resources/") => {
                let decoded = percent_decode(p.trim_start_matches('/'));
                let ct = content_type(&decoded);
                serve_file(stream, &decoded, ct)
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

fn handle_health(stream: &mut TcpStream) -> ConnectionState {
    let (state, _) = core::status().unwrap_or((CoreState::Disconnected, None));
    let vpn = match state {
        CoreState::Connected    => "connected",
        CoreState::Disconnected => "disconnected",
        CoreState::Reconnecting => "reconnecting",
    };
    let body = format!(
        r#"{{"status":"ok","vpn":"{}","uptime_s":{}}}"#,
        vpn,
        super::uptime_secs()
    );
    sender::send(stream, ok_json(&body))
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

fn is_mobile_ua(ua: &str) -> bool {
    ua.contains("Mobile") || ua.contains("Android") || ua.contains("iPhone")
        || ua.contains("iPad") || ua.contains("iPod")
}

fn content_type(path: &str) -> &'static str {
    if path.ends_with(".js")        { "application/javascript" }
    else if path.ends_with(".html") { "text/html" }
    else if path.ends_with(".css")  { "text/css" }
    else if path.ends_with(".svg")  { "image/svg+xml" }
    else                            { "application/octet-stream" }
}

fn percent_decode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let b = s.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%' && i + 2 < b.len() {
            if let (Some(h), Some(l)) = (hex_val(b[i + 1]), hex_val(b[i + 2])) {
                out.push((h << 4 | l) as char);
                i += 3;
                continue;
            }
        }
        out.push(b[i] as char);
        i += 1;
    }
    out
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
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
    fn is_mobile_ua_desktop_returns_false() {
        assert!(!is_mobile_ua("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36"));
        assert!(!is_mobile_ua("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"));
        assert!(!is_mobile_ua(""));
    }

    #[test]
    fn is_mobile_ua_mobile_keyword_returns_true() {
        assert!(is_mobile_ua("Mozilla/5.0 (Linux; Android 14) Mobile"));
        assert!(is_mobile_ua("Mozilla/5.0 (Linux; Android 14; Pixel 8)"));
        assert!(is_mobile_ua("Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X)"));
        assert!(is_mobile_ua("Mozilla/5.0 (iPad; CPU OS 17_0 like Mac OS X)"));
        assert!(is_mobile_ua("Mozilla/5.0 (iPod touch; CPU iPhone OS 17_0 like Mac OS X)"));
    }

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
    fn content_type_svg() {
        assert_eq!(content_type("flag.svg"), "image/svg+xml");
        assert_eq!(content_type("resources/web_resources/assets/flags/US.svg"), "image/svg+xml");
    }

    #[test]
    fn content_type_unknown_falls_back_to_octet_stream() {
        assert_eq!(content_type("file.bin"), "application/octet-stream");
        assert_eq!(content_type("data.json"), "application/octet-stream");
        assert_eq!(content_type("noextension"), "application/octet-stream");
    }

    #[test]
    fn percent_decode_plain_passthrough() {
        assert_eq!(
            percent_decode("resources/web_resources/assets/flags/US.svg"),
            "resources/web_resources/assets/flags/US.svg"
        );
    }

    #[test]
    fn percent_decode_space() {
        assert_eq!(percent_decode("some%20path.svg"), "some path.svg");
        assert_eq!(percent_decode("a%20b%20c"), "a b c");
    }

    #[test]
    fn percent_decode_invalid_sequence_passed_through() {
        assert_eq!(percent_decode("a%2Zb"), "a%2Zb");
        assert_eq!(percent_decode("a%"), "a%");
    }

    #[test]
    fn content_type_path_with_directories() {
        assert_eq!(content_type("resources/page_scripts/client.js"), "application/javascript");
        assert_eq!(content_type("resources/html_pages/index.html"), "text/html");
    }
}
