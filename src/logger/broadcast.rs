use std::collections::VecDeque;
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

pub const HISTORY_CAPACITY: usize = 1000;

struct BroadcastState {
    subscribers: Vec<SyncSender<LogLine>>,
    history:     VecDeque<LogLine>,
}

static STATE: Mutex<BroadcastState> = Mutex::new(BroadcastState {
    subscribers: Vec::new(),
    history:     VecDeque::new(),
});

pub fn subscribe_with_history() -> (Receiver<LogLine>, Vec<LogLine>) {
    let (tx, rx) = sync_channel(1024);
    let mut state = STATE.lock().unwrap();
    state.subscribers.push(tx);
    let history: Vec<LogLine> = state.history.iter().cloned().collect();
    (rx, history)
}

#[cfg(test)]
pub fn get_history() -> Vec<LogLine> {
    STATE.lock()
        .map(|s| s.history.iter().cloned().collect())
        .unwrap_or_default()
}

pub fn broadcast(line: &LogLine) {
    if let Ok(mut state) = STATE.lock() {
        state.subscribers.retain(|tx| tx.try_send(line.clone()).is_ok());
        if state.history.len() >= HISTORY_CAPACITY {
            state.history.pop_front();
        }
        state.history.push_back(line.clone());
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    fn make_line(message: &str) -> LogLine {
        LogLine {
            timestamp: "2026-01-01 00:00:00:000".to_string(),
            level:     "INFO".to_string(),
            tag:       "test".to_string(),
            pid:       0,
            tid:       0,
            message:   message.to_string(),
        }
    }

    #[test]
    fn broadcast_adds_to_history() {
        let unique = "adds_to_history_sentinel_a1b2c3";
        broadcast(&make_line(unique));
        assert!(get_history().iter().any(|l| l.message == unique));
    }

    #[test]
    fn history_preserves_insertion_order() {
        let tag_a = "order_test_first_zzz1";
        let tag_b = "order_test_second_zzz2";
        broadcast(&make_line(tag_a));
        broadcast(&make_line(tag_b));
        let hist = get_history();
        let pos_a = hist.iter().rposition(|l| l.message == tag_a).expect("tag_a missing");
        let pos_b = hist.iter().rposition(|l| l.message == tag_b).expect("tag_b missing");
        assert!(pos_a < pos_b, "tag_a must appear before tag_b");
    }

    #[test]
    fn history_evicts_oldest_at_capacity() {
        let unique = "eviction_marker_unique_xyzzy7";
        broadcast(&make_line(unique));
        for i in 0..HISTORY_CAPACITY {
            broadcast(&make_line(&format!("eviction_filler_{}", i)));
        }
        let hist = get_history();
        assert_eq!(hist.len(), HISTORY_CAPACITY);
        assert!(!hist.iter().any(|l| l.message == unique), "oldest entry should have been evicted");
    }

    #[test]
    fn get_history_returns_independent_clone() {
        let unique = "clone_test_sentinel_xyz789";
        broadcast(&make_line(unique));
        let mut snap = get_history();
        snap.clear();
        assert!(
            get_history().iter().any(|l| l.message == unique),
            "clearing returned Vec must not affect the buffer"
        );
    }
}

