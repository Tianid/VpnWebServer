use std::net::TcpStream;
use crate::web_server::sender::http_sender;
use crate::generated::constants::{HTML_INDEX_PAGE_PATH, WEB_SOCKET_JS_PATH};
use crate::requests::http_request::HttpRequest;
use crate::responses::http_response_builder::HttpResponseBuilder;
use crate::utils::resource_provider::read_content;
use crate::web_server::connection_state::ConnectionState;
use crate::web_server::web_socket::messaging;
use crate::logger;

pub fn process(stream: &mut TcpStream, http_request: &HttpRequest) -> ConnectionState {
    let path = http_request.path.as_str();
    match path {
        "/"                                                     => { http_sender::send(stream, create_builder(HTML_INDEX_PAGE_PATH, "text/html")) }
        WS_JS if WS_JS == format!("/{}", WEB_SOCKET_JS_PATH)    => { http_sender::send(stream, create_builder(WEB_SOCKET_JS_PATH, "application/javascript")) }
        "/ws"                                                   => { messaging::process(stream, http_request) }
        _                                                       => {
            // FIXME return 404 text page
            logger::error(format!("Failed to process GET request, undefined path occurred {}", path).as_str());
            http_sender::send(stream, create_bad_access_builder("404 Not Found", 404))
        }
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

    create_bad_access_builder("404 Not Found", 404)
}

fn create_bad_access_builder(text: &str, status_code: u16) -> HttpResponseBuilder {
    HttpResponseBuilder::new()
        .content_type("text/plain")
        .status_code(status_code)
        .text(text)
}
