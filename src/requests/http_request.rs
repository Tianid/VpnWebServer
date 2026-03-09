use std::collections::HashMap;
use std::str::FromStr;
use crate::requests::http_method::HttpMethod;

#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>
}

impl HttpRequest {
    pub fn new(request: &str) -> Option<HttpRequest> {
        let mut lines = request.lines();

        let first_line = lines.next()?;
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        if parts.len() < 2 { return None; }

        let method = if let Ok(method) = HttpMethod::from_str(parts[0]) { method } else { return None };
        let path = parts[1].to_string();

        let mut headers = HashMap::new();
        for line in lines.by_ref() {
            if line.is_empty() { break; }

            if let Some((key, value)) = line.split_once(": ") {
                headers.insert(key.to_string(), value.to_string());
            }
        }

        let body = lines.collect::<Vec<&str>>().join("\n");
        let body = if body.is_empty() { None } else { Some(body) };

        Some(HttpRequest {
            method,
            path,
            headers,
            body
        })
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_get_request_parses() {
        let req = HttpRequest::new("GET / HTTP/1.1\r\nHost: localhost\r\n\r\n");
        assert!(req.is_some());
        let req = req.unwrap();
        assert!(matches!(req.method, HttpMethod::GET));
        assert_eq!(req.path, "/");
        assert_eq!(req.body, None);
    }

    #[test]
    fn valid_post_request_with_body_parses() {
        let raw = "POST /api/config HTTP/1.1\r\nContent-Type: application/json\r\n\r\n{\"log_level\":\"debug\"}";
        let req = HttpRequest::new(raw).unwrap();
        assert!(matches!(req.method, HttpMethod::POST));
        assert_eq!(req.path, "/api/config");
        assert_eq!(req.body, Some("{\"log_level\":\"debug\"}".to_string()));
    }

    #[test]
    fn empty_input_returns_none() {
        assert!(HttpRequest::new("").is_none());
    }

    #[test]
    fn unknown_method_returns_none() {
        assert!(HttpRequest::new("FOOBAR / HTTP/1.1\r\n\r\n").is_none());
    }

    #[test]
    fn headers_are_parsed_into_map() {
        let raw = "GET /ws HTTP/1.1\r\nUpgrade: websocket\r\nSec-WebSocket-Key: abc123\r\n\r\n";
        let req = HttpRequest::new(raw).unwrap();
        assert_eq!(req.headers.get("Upgrade").map(String::as_str), Some("websocket"));
        assert_eq!(req.headers.get("Sec-WebSocket-Key").map(String::as_str), Some("abc123"));
    }

    #[test]
    fn request_with_no_headers_parses() {
        let req = HttpRequest::new("GET /health HTTP/1.1\r\n\r\n");
        assert!(req.is_some());
        let req = req.unwrap();
        assert_eq!(req.path, "/health");
        assert!(req.headers.is_empty());
    }

    #[test]
    fn request_line_missing_path_returns_none() {
        assert!(HttpRequest::new("GET\r\n\r\n").is_none());
    }
}
