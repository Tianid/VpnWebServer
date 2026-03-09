#[macro_use]
mod logger;
mod config;
mod responses;
mod requests;
mod utils;
mod server;
mod core;

fn main() {
    logger::init_time_offset();
    let cfg = config::args::parse_args(std::env::args());
    logger::set_level(cfg.log_level);
    log_info!("server", "haven {} starting", env!("CARGO_PKG_VERSION"));
    log_info!("server", "Bind address: {}:{}", cfg.address, cfg.port);
    log_info!("server", "Log level: {:?}", cfg.log_level);
    let cache = core::LocationCache::new();
    cache.refresh_in_background();
    server::start(cfg, cache);
}
