use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandDuration {
    duration: Duration,
}

impl CommandDuration {
    pub fn new(duration: Duration) -> Self {
        Self { duration }
    }

    pub fn duration(&self) -> Duration {
        self.duration
    }

    pub fn as_secs_f64(&self) -> f64 {
        self.duration.as_secs_f64()
    }

    pub fn is_long_running(&self, threshold: Duration) -> bool {
        self.duration >= threshold
    }
}

#[derive(Debug, Default)]
pub struct CommandDurationTracker {
    active: HashMap<u64, Instant>,
    completed: HashMap<u64, CommandDuration>,
}

impl CommandDurationTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&mut self, command_id: u64) {
        self.active.insert(command_id, Instant::now());
    }

    pub fn finish(&mut self, command_id: u64) -> Option<CommandDuration> {
        let start = self.active.remove(&command_id)?;
        let duration = CommandDuration::new(start.elapsed());

        self.completed.insert(command_id, duration);

        Some(duration)
    }

    pub fn cancel(&mut self, command_id: u64) -> Option<CommandDuration> {
        self.finish(command_id)
    }

    pub fn elapsed(&self, command_id: u64) -> Option<Duration> {
        self.active.get(&command_id).map(Instant::elapsed)
    }

    pub fn completed(&self, command_id: u64) -> Option<CommandDuration> {
        self.completed.get(&command_id).copied()
    }

    pub fn remove(&mut self, command_id: u64) {
        self.active.remove(&command_id);
        self.completed.remove(&command_id);
    }

    pub fn clear(&mut self) {
        self.active.clear();
        self.completed.clear();
    }

    pub fn active_count(&self) -> usize {
        self.active.len()
    }

    pub fn completed_count(&self) -> usize {
        self.completed.len()
    }
}
