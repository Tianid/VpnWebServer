use utils::args_reader::get_server_config;

mod responses;
mod requests;
mod generated;
mod utils;
mod web_server;
mod logger;
mod core;

fn main() {
    web_server::start(get_server_config());
}
