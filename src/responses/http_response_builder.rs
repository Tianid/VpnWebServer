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
            "HTTP/1.1 {} OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
            status_code,
            content_type,
            text.len(),
            text
        )
    }
}
