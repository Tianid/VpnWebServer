use std::sync::{Arc, RwLock};
use std::time::Instant;

use crate::core::location::Location;

#[derive(Debug, Clone)]
pub struct LocationCache {
    locations:    Arc<RwLock<Vec<Location>>>,
    last_updated: Arc<RwLock<Option<Instant>>>,
}

impl LocationCache {
    pub fn new() -> Self {
        Self {
            locations:    Arc::new(RwLock::new(Vec::new())),
            last_updated: Arc::new(RwLock::new(None)),
        }
    }

    pub fn refresh_in_background(&self) {
        let cache = Arc::clone(&self.locations);
        let ts    = Arc::clone(&self.last_updated);
        std::thread::spawn(move || {
            match super::list_locations() {
                Ok(locs) => {
                    *cache.write().unwrap() = locs;
                    *ts.write().unwrap()    = Some(Instant::now());
                    log_info!("core", "Location cache refreshed");
                }
                Err(e) => log_error!("core", "Failed to refresh location cache: {}", e),
            }
        });
    }

    pub fn get(&self) -> Vec<Location> {
        self.locations.read().unwrap().clone()
    }
}
