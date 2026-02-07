use crate::responses::response_builder::ResponseBuilder;

pub struct WsConnectionResponseBuilder {
    response_key: Option<String>,
}

impl WsConnectionResponseBuilder {

    pub fn new() -> Self { WsConnectionResponseBuilder {
        response_key: None
    } }

    pub fn response_key(mut self, response_key: &str) -> Self {
        self.response_key = Some(response_key.to_owned());
        self
    }
}

impl ResponseBuilder for WsConnectionResponseBuilder {
    fn build(self) -> String {
        let response_key = if let Some(response_key) = self.response_key { response_key } else { String::new() };

        format!(
            "HTTP/1.1 101 Switching Protocols\r\n\
            Upgrade: websocket\r\n\
            Connection: Upgrade\r\n\
            Sec-WebSocket-Accept: {}\r\n\r\n",
            response_key
        )
    }
}
