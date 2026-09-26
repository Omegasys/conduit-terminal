use std::collections::HashMap;
use std::time::{Duration, Instant};

use super::manager::NotificationId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UrgencyLevel {
    None,
    Low,
    Normal,
    High,
    Critical,
}

impl Default for UrgencyLevel {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Copy)]
struct UrgencyEntry {
    level: UrgencyLevel,
    timestamp: Instant,
}

#[derive(Debug)]
pub struct UrgencyTracker {
    entries: HashMap<NotificationId, UrgencyEntry>,
    timeout: Duration,
}

impl Default for UrgencyTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl UrgencyTracker {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            timeout: Duration::from_secs(30),
        }
    }

    pub fn set(
        &mut self,
        id: NotificationId,
        level: UrgencyLevel,
    ) {
        self.entries.insert(
            id,
            UrgencyEntry {
                level,
                timestamp: Instant::now(),
            },
        );
    }

    pub fn get(
        &self,
        id: NotificationId,
    ) -> UrgencyLevel {
        self.entries
            .get(&id)
            .map(|entry| entry.level)
            .unwrap_or(UrgencyLevel::None)
    }

    pub fn highest(&self) -> UrgencyLevel {
        self.entries
            .values()
            .map(|entry| entry.level)
            .max()
            .unwrap_or(UrgencyLevel::None)
    }

    pub fn clear(&mut self, id: NotificationId) {
        self.entries.remove(&id);
    }

    pub fn clear_all(&mut self) {
        self.entries.clear();
    }

    pub fn update(&mut self) {
        let timeout = self.timeout;
        let now = Instant::now();

        self.entries
            .retain(|_, entry| now.duration_since(entry.timestamp) < timeout);
    }

    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }

    pub fn is_urgent(&self) -> bool {
        self.highest() >= UrgencyLevel::High
    }
}
