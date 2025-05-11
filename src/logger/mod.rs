use time::OffsetDateTime;
use time::format_description;
use time::UtcOffset;

const DATE_FORMAT: &str = "[year]-[month]-[day] [hour]:[minute]:[second]";


pub fn info(message: &str) { println!("[{}] 🟦 {}", get_time(), message) }
pub fn debug(message: &str) { println!("[{}] 🟩 {}", get_time(), message) }
pub fn warn(message: &str) { println!("[{}] 🟧 {}", get_time(), message) }
pub fn error(message: &str) { println!("[{}] 🟥 {}", get_time(), message) }
pub fn trace(message: &str) { println!("[{}] 🟫 {}", get_time(), message) }

fn get_time() -> String {
    let now_time = OffsetDateTime::now_utc();

    let offset = match UtcOffset::from_hms(3, 0, 0) { 
        Ok(_offset) => _offset,
        Err(_)      => return String::new()
    };

    let utc_plus_three = now_time.to_offset(offset);
    let fmt = format_description::parse(DATE_FORMAT).unwrap_or_default();
    utc_plus_three.format(&fmt).unwrap_or_default()
}
