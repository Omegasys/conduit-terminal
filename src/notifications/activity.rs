use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivityKind {
    Output,
    Input,
    CommandStarted,
    CommandFinished,
    Bell,
    Notification,
    Error,
}

#[derive(Debug, Clone, Copy)]
pub struct ActivityState {
    pub kind: ActivityKind,
    pub timestamp: Instant,
    pub active: bool,
}

#[derive(Debug)]
pub struct ActivityTracker {
    last_activity: Option<ActivityState>,
    activity_count: u64,
}

impl Default for ActivityTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl ActivityTracker {
    pub fn new() -> Self {
        Self {
            last_activity: None,
            activity_count: 0,
        }
    }

    pub fn record(&mut self, kind: ActivityKind) {
        self.last_activity = Some(ActivityState {
            kind,
            timestamp: Instant::now(),
            active: true,
        });

        self.activity_count = self.activity_count.saturating_add(1);
    }

    pub fn update(&mut self) {}

    pub fn last_activity(&self) -> Option<ActivityState> {
        self.last_activity
    }

    pub fn activity_count(&self) -> u64 {
        self.activity_count
    }

    pub fn is_active(&self) -> bool {
        self.last_activity
            .map(|activity| activity.active)
            .unwrap_or(false)
    }

    pub fn clear(&mut self) {
        self.last_activity = None;
    }
}
