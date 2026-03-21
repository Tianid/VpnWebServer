use std::sync::{Arc, RwLock};
use std::time::Duration;

use crate::core::location::Location;

#[derive(Debug, Clone)]
pub struct LocationCache {
    locations: Arc<RwLock<Vec<Location>>>,
}

impl LocationCache {
    pub fn new() -> Self {
        Self {
            locations: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn refresh_in_background(&self) {
        let cache = Arc::clone(&self.locations);
        std::thread::spawn(move || {
            match super::list_locations() {
                Ok(locs) => {
                    *cache.write().unwrap() = locs;
                    log_info!("core", "Location cache refreshed");
                }
                Err(e) => log_error!("core", "Failed to refresh location cache: {}", e),
            }
        });
    }

    pub fn refresh_periodically(&self, interval: Duration) {
        let cache = Arc::clone(&self.locations);
        std::thread::spawn(move || {
            loop {
                std::thread::sleep(interval);
                match super::list_locations() {
                    Ok(locs) => {
                        *cache.write().unwrap() = locs;
                        log_info!("core", "Location cache auto-refreshed");
                    }
                    Err(e) => log_error!("core", "Location cache auto-refresh failed: {}", e),
                }
            }
        });
    }

    pub fn get(&self) -> Vec<Location> {
        self.locations.read().unwrap().clone()
    }
}
