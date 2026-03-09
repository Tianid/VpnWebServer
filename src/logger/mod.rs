pub mod broadcast;
pub use broadcast::{subscribe, LogLine};

use std::cell::Cell;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU64, AtomicU8, Ordering};
use time::format_description;
use time::{OffsetDateTime, UtcOffset};

const DATE_FORMAT: &str = "[year]-[month]-[day] [hour]:[minute]:[second]:[subsecond digits:3]";

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info  = 2,
    Warn  = 3,
    Error = 4,
    Off   = 5,
}

impl LogLevel {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "trace" => Some(Self::Trace),
            "debug" => Some(Self::Debug),
            "info"  => Some(Self::Info),
            "warn"  => Some(Self::Warn),
            "error" => Some(Self::Error),
            "off"   => Some(Self::Off),
            _       => None,
        }
    }
}

impl From<u8> for LogLevel {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::Trace,
            1 => Self::Debug,
            2 => Self::Info,
            3 => Self::Warn,
            4 => Self::Error,
            _ => Self::Off,
        }
    }
}

static GLOBAL_LOG_LEVEL: AtomicU8 = AtomicU8::new(LogLevel::Info as u8);

pub fn set_level(level: LogLevel) {
    GLOBAL_LOG_LEVEL.store(level as u8, Ordering::Relaxed);
    info("server", &format!("Log level changed to {:?}", level));
}

pub fn current_level() -> LogLevel {
    LogLevel::from(GLOBAL_LOG_LEVEL.load(Ordering::Relaxed))
}

static CACHED_OFFSET: OnceLock<UtcOffset> = OnceLock::new();

pub fn init_time_offset() {
    let offset = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);
    CACHED_OFFSET.get_or_init(|| offset);
}

static NEXT_THREAD_ID: AtomicU64 = AtomicU64::new(1);

thread_local! {
    static LOCAL_THREAD_ID: Cell<u64> = const { Cell::new(0) };
}

#[allow(dead_code)]
pub fn trace(tag: &str, msg: &str) { emit(LogLevel::Trace, tag, msg); }
pub fn debug(tag: &str, msg: &str) { emit(LogLevel::Debug, tag, msg); }
pub fn info (tag: &str, msg: &str) { emit(LogLevel::Info,  tag, msg); }
pub fn warn (tag: &str, msg: &str) { emit(LogLevel::Warn,  tag, msg); }
pub fn error(tag: &str, msg: &str) { emit(LogLevel::Error, tag, msg); }

#[macro_export]
macro_rules! log_trace {
    ($tag:expr, $($arg:tt)*) => {
        $crate::logger::trace($tag, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_debug {
    ($tag:expr, $($arg:tt)*) => {
        $crate::logger::debug($tag, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_info {
    ($tag:expr, $($arg:tt)*) => {
        $crate::logger::info($tag, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_warn {
    ($tag:expr, $($arg:tt)*) => {
        $crate::logger::warn($tag, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_error {
    ($tag:expr, $($arg:tt)*) => {
        $crate::logger::error($tag, &format!($($arg)*))
    };
}

fn current_thread_id() -> u64 {
    LOCAL_THREAD_ID.with(|id| {
        let v = id.get();
        if v == 0 {
            let new_id = NEXT_THREAD_ID.fetch_add(1, Ordering::Relaxed);
            id.set(new_id);
            new_id
        } else {
            v
        }
    })
}

fn get_time() -> String {
    let offset = CACHED_OFFSET.get().copied().unwrap_or(UtcOffset::UTC);
    let now = OffsetDateTime::now_utc().to_offset(offset);
    let fmt = format_description::parse(DATE_FORMAT).unwrap_or_default();
    now.format(&fmt).unwrap_or_default()
}

fn level_icon(level: LogLevel) -> &'static str {
    match level {
        LogLevel::Trace => "🟫",
        LogLevel::Debug => "🟩",
        LogLevel::Info  => "🟦",
        LogLevel::Warn  => "🟧",
        LogLevel::Error => "🟥",
        LogLevel::Off   => "",
    }
}

fn emit(level: LogLevel, tag: &str, msg: &str) {
    if level < current_level() {
        return;
    }
    if level == LogLevel::Off {
        return;
    }
    let ts = get_time();
    let icon = level_icon(level);
    let pid = std::process::id();
    let tid = current_thread_id();
    println!(
        "[{}] {} [{:>5}] [PID:{} TID:{:>3}] [{}] {}",
        ts,
        icon,
        format!("{:?}", level).to_uppercase(),
        pid,
        tid,
        tag,
        msg
    );
    let line = LogLine {
        timestamp: ts,
        level: format!("{:?}", level).to_uppercase(),
        tag: tag.to_string(),
        pid,
        tid,
        message: msg.to_string(),
    };
    broadcast::broadcast(&line);
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_all_valid_lowercase() {
        assert_eq!(LogLevel::from_str("trace"), Some(LogLevel::Trace));
        assert_eq!(LogLevel::from_str("debug"), Some(LogLevel::Debug));
        assert_eq!(LogLevel::from_str("info"),  Some(LogLevel::Info));
        assert_eq!(LogLevel::from_str("warn"),  Some(LogLevel::Warn));
        assert_eq!(LogLevel::from_str("error"), Some(LogLevel::Error));
        assert_eq!(LogLevel::from_str("off"),   Some(LogLevel::Off));
    }

    #[test]
    fn from_str_case_insensitive() {
        assert_eq!(LogLevel::from_str("INFO"),  Some(LogLevel::Info));
        assert_eq!(LogLevel::from_str("Debug"), Some(LogLevel::Debug));
        assert_eq!(LogLevel::from_str("WARN"),  Some(LogLevel::Warn));
        assert_eq!(LogLevel::from_str("ERROR"), Some(LogLevel::Error));
    }

    #[test]
    fn from_str_invalid_returns_none() {
        assert_eq!(LogLevel::from_str("verbose"), None);
        assert_eq!(LogLevel::from_str(""),         None);
        assert_eq!(LogLevel::from_str("INFO2"),    None);
        assert_eq!(LogLevel::from_str("log"),      None);
    }

    #[test]
    fn partial_ord_ordering_is_correct() {
        assert!(LogLevel::Trace < LogLevel::Debug);
        assert!(LogLevel::Debug < LogLevel::Info);
        assert!(LogLevel::Info  < LogLevel::Warn);
        assert!(LogLevel::Warn  < LogLevel::Error);
        assert!(LogLevel::Error < LogLevel::Off);
    }

    #[test]
    fn set_and_get_level_round_trip() {
        set_level(LogLevel::Debug);
        assert_eq!(current_level(), LogLevel::Debug);
        set_level(LogLevel::Info);
        assert_eq!(current_level(), LogLevel::Info);
    }
}
