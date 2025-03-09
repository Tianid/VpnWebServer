use std::fs;
use crate::logger;

pub fn read_content(path: &str) -> Option<String> {
    match fs::read_to_string(&path) {
        Ok(content) => { Some(content) }
        Err(error) => {
            logger::error(format!("Failed to read content of {}: {}", path, error).as_str());
            None
        }
    }
}
