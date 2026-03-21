use crate::responses::response_builder::ResponseBuilder;

pub struct HttpResponseBuilder {
    status_code: Option<u16>,
    content_type: Option<String>,
    text: Option<String>
}

impl HttpResponseBuilder {

    pub fn new() -> Self { HttpResponseBuilder {
        status_code: None,
        content_type: None,
        text: None
    } }

    pub fn status_code(mut self, status_code: u16) -> Self {
        self.status_code = Some(status_code);
        self
    }

    pub fn content_type(mut self, content_type: &str) -> Self {
        self.content_type = Some(content_type.to_string());
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }
}

impl ResponseBuilder for HttpResponseBuilder {
    fn build(self) -> String {
        let status_code = if let Some(status) = self.status_code { status } else { return String::new() };
        let content_type = if let Some(content_type) = self.content_type { content_type } else { return String::new() };
        let text = if let Some(text) = self.text { text } else { return String::new() };

        format!(
            "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
            status_code,
            reason_phrase(status_code),
            content_type,
            text.len(),
            text
        )
    }
}

fn reason_phrase(code: u16) -> &'static str {
    match code {
        200 => "OK",
        201 => "Created",
        204 => "No Content",
        301 => "Moved Permanently",
        302 => "Found",
        304 => "Not Modified",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        500 => "Internal Server Error",
        503 => "Service Unavailable",
        _   => "Unknown",
    }
}





#[cfg(test)]
mod tests {
    use super::*;
    use crate::responses::response_builder::ResponseBuilder;

    #[test]
    fn build_200_contains_correct_status_line() {
        let result = HttpResponseBuilder::new()
            .status_code(200)
            .content_type("text/html")
            .text("<h1>OK</h1>")
            .build();
        assert!(result.starts_with("HTTP/1.1 200 OK\r\n"));
    }

    #[test]
    fn build_contains_content_type_header() {
        let result = HttpResponseBuilder::new()
            .status_code(200)
            .content_type("application/json")
            .text("{}")
            .build();
        assert!(result.contains("Content-Type: application/json\r\n"));
    }

    #[test]
    fn build_contains_correct_content_length() {
        let body = "hello";
        let result = HttpResponseBuilder::new()
            .status_code(200)
            .content_type("text/plain")
            .text(body)
            .build();
        assert!(result.contains(&format!("Content-Length: {}\r\n", body.len())));
    }

    #[test]
    fn build_body_appears_after_blank_line() {
        let result = HttpResponseBuilder::new()
            .status_code(200)
            .content_type("text/plain")
            .text("body-content")
            .build();
        assert!(result.ends_with("\r\n\r\nbody-content"));
    }

    #[test]
    fn build_missing_status_code_returns_empty() {
        let result = HttpResponseBuilder::new()
            .content_type("text/plain")
            .text("hello")
            .build();
        assert!(result.is_empty());
    }

    #[test]
    fn build_missing_content_type_returns_empty() {
        let result = HttpResponseBuilder::new()
            .status_code(200)
            .text("hello")
            .build();
        assert!(result.is_empty());
    }

    #[test]
    fn build_missing_text_returns_empty() {
        let result = HttpResponseBuilder::new()
            .status_code(200)
            .content_type("text/plain")
            .build();
        assert!(result.is_empty());
    }

    #[test]
    fn build_404_uses_correct_reason_phrase() {
        let result = HttpResponseBuilder::new()
            .status_code(404)
            .content_type("text/plain")
            .text("Not Found")
            .build();
        assert!(result.starts_with("HTTP/1.1 404 Not Found\r\n"));
        assert!(result.ends_with("Not Found"));
    }

    #[test]
    fn build_400_uses_correct_reason_phrase() {
        let result = HttpResponseBuilder::new()
            .status_code(400)
            .content_type("text/plain")
            .text("bad")
            .build();
        assert!(result.starts_with("HTTP/1.1 400 Bad Request\r\n"));
    }
}
