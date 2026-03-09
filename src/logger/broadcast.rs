use std::sync::Mutex;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct LogLine {
    pub timestamp: String,
    pub level: String,
    pub tag: String,
    pub pid: u32,
    pub tid: u64,
    pub message: String,
}

static SUBSCRIBERS: Mutex<Vec<SyncSender<LogLine>>> = Mutex::new(Vec::new());

pub fn subscribe() -> Receiver<LogLine> {
    subscribe_with_capacity(1024)
}

pub fn broadcast(line: &LogLine) {
    if let Ok(mut subs) = SUBSCRIBERS.try_lock() {
        subs.retain(|tx| tx.try_send(line.clone()).is_ok());
    }
}

fn subscribe_with_capacity(cap: usize) -> Receiver<LogLine> {
    let (tx, rx) = sync_channel(cap);
    SUBSCRIBERS.lock().unwrap().push(tx);
    rx
}
