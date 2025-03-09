use std::net::TcpStream;
use crate::core::response_sender;
use crate::generated::constants::{HTML_INDEX_PAGE_PATH, WEB_SOCKET_JS_PATH};
use crate::requests::http_request::HttpRequest;
use crate::responses::http_response_builder::HttpResponseBuilder;
use crate::utils::resource_provider::read_content;
use crate::core::ws_messaging_processor;

pub fn process(stream: &mut TcpStream, http_request: &HttpRequest) {
    let path = http_request.path.as_str();
    match path {
        "/"                                                         => { response_sender::send(stream, create_builder(HTML_INDEX_PAGE_PATH, "text/html")) }
        WS_JS if WS_JS == format!("/{}", WEB_SOCKET_JS_PATH)   => { response_sender::send(stream, create_builder(WEB_SOCKET_JS_PATH, "application/javascript")) }
        "/ws"                                                       => { ws_messaging_processor::process(stream, http_request) }
        _                                                           => { println!("[ERROR] Failed to process GET request, undefined path occurred {}", path) }
    }
}

fn create_builder(
    path: &str,
    content_type: &str,
) -> HttpResponseBuilder {
    if let Some(content) = read_content(path) {
        return HttpResponseBuilder::new()
            .content_type(content_type)
            .status_code(200)
            .text(content.as_str())
    }

    HttpResponseBuilder::new()
        .content_type("text/plain")
        .status_code(404)
        .text("404 Not Found")
}
