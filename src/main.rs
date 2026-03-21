#[macro_use]
mod logger;
mod config;
mod autostart;
mod responses;
mod requests;
mod utils;
mod server;
mod core;

fn main() {
    let cfg = config::args::parse_args(std::env::args());

    if let Some(ref action) = cfg.autostart_action {
        autostart::run(action, &cfg);
        if !cfg.address_specified {
            std::process::exit(0);
        }
    }

    logger::init_time_offset();
    logger::set_level(cfg.log_level);
    log_info!("server", "haven {} starting", env!("CARGO_PKG_VERSION"));
    log_info!("server", "Bind address: {}:{}", cfg.address, cfg.port);
    log_info!("server", "Log level: {:?}", cfg.log_level);
    ctrlc::set_handler(|| {
        log_info!("server", "Shutdown signal received, exiting");
        std::process::exit(0);
    }).expect("Failed to register signal handler");

    let cache = core::LocationCache::new();
    cache.refresh_in_background();
    cache.refresh_periodically(std::time::Duration::from_secs(30 * 60));
    server::start(cfg, cache);
}
