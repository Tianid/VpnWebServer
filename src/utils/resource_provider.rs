use std::fs;

pub fn read_content(path: &str) -> Option<String> {
    match fs::read_to_string(&path) {
        Ok(content) => { Some(content) }
        Err(error) => {
            log_error!("server", "Failed to read file {}: {}", path, error);
            None
        }
    }
}
