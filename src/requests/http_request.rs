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
