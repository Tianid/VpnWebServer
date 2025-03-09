use std::time::SystemTime;
use std::fmt::Write;

static d: i32 = 1; // TODO use for logger

pub fn info(message: &str) { println!("[{}] 🟦 {}", get_time(), message) }
pub fn debug(message: &str) { println!("[{}] 🟩 {}", get_time(), message) }
pub fn warn(message: &str) { println!("[{}] 🟧 {}", get_time(), message) }
pub fn error(message: &str) { println!("[{}] 🟥 {}", get_time(), message) }
pub fn trace(message: &str) { println!("[{}] 🟫 {}", get_time(), message) }

fn get_time() -> String {
    let now = SystemTime::now();
    let duration_since_epoch = now.duration_since(SystemTime::UNIX_EPOCH).expect("Time went backwards");

    let total_seconds = duration_since_epoch.as_secs();
    let millis = duration_since_epoch.subsec_millis();

    let days_since_epoch = total_seconds / 86400;
    let seconds_in_day = total_seconds % 86400;

    let year = 1970 + days_since_epoch / 365;
    let days_in_year = days_since_epoch % 365;
    let month = 1 + days_in_year / 30;
    let day = 1 + days_in_year % 30;

    let hours = seconds_in_day / 3600;
    let minutes = (seconds_in_day % 3600) / 60;
    let seconds = seconds_in_day % 60;

    let mut timestamp = String::new();
    write!(
        &mut timestamp,
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
        year, month, day, hours, minutes, seconds, millis
    ).unwrap();

    timestamp
}
