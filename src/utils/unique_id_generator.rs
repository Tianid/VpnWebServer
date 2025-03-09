use std::collections::HashSet;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct UniqueIdGenerator {
    used_ids: Mutex<HashSet<String>>,
}

impl UniqueIdGenerator {
    pub fn new() -> UniqueIdGenerator {
        Self { used_ids: Mutex::new(HashSet::new()) }
    }

    pub fn generate(&self) -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        const ID_LENGTH: usize = 10;

        loop {
            let mut id = String::with_capacity(ID_LENGTH);
            let nanos = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .subsec_nanos();

            let mut seed = nanos as u64;

            for _ in 0..ID_LENGTH {
                let index = (seed % (CHARSET.len() as u64)) as usize;
                id.push(CHARSET[index] as char);
                seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            }

            let mut used_ids = self.used_ids.lock().unwrap();

            if used_ids.insert(id.clone()) {
                return id;
            }
        }
    }
}
