mod responses;
mod requests;
mod generated;
mod utils;
mod web_server;
mod logger;
mod core;

fn main() {
    web_server::start(web_server::web_server_configuration::WebServerConfiguration::new("127.0.0.1".as_ref(), "9000".as_ref()));
}
