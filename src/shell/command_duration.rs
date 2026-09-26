use std::time::{Duration, Instant, SystemTime};

/// Timing information for a completed command.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandDuration {
    duration: Duration,
}

impl CommandDuration {
    pub const fn new(duration: Duration) -> Self {
        Self { duration }
    }

    pub const fn zero() -> Self {
        Self {
            duration: Duration::ZERO,
        }
    }

    pub const fn duration(self) -> Duration {
        self.duration
    }

    pub fn as_secs_f64(self) -> f64 {
        self.duration.as_secs_f64()
    }

    pub fn as_millis(self) -> u128 {
        self.duration.as_millis()
    }

    pub fn is_zero(self) -> bool {
        self.duration.is_zero()
    }
}

impl From<Duration> for CommandDuration {
    fn from(duration: Duration) -> Self {
        Self::new(duration)
    }
}

impl From<CommandDuration> for Duration {
    fn from(duration: CommandDuration) -> Self {
        duration.duration
    }
}

/// A running command timer.
#[derive(Debug, Clone)]
pub struct CommandTimer {
    started_at: Instant,
    wall_clock: SystemTime,
}

impl CommandTimer {
    pub fn start() -> Self {
        Self {
            started_at: Instant::now(),
            wall_clock: SystemTime::now(),
        }
    }

    pub fn elapsed(&self) -> CommandDuration {
        CommandDuration::new(self.started_at.elapsed())
    }

    pub fn started_at(&self) -> SystemTime {
        self.wall_clock
    }

    pub fn finish(self) -> CommandDuration {
        CommandDuration::new(self.started_at.elapsed())
    }
}

/// Stores the duration of the most recently completed command.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandDurationTracker {
    last: Option<CommandDuration>,
    total: Duration,
    command_count: u64,
}

impl CommandDurationTracker {
    pub fn new() -> Self {
        Self {
            last: None,
            total: Duration::ZERO,
            command_count: 0,
        }
    }

    pub fn last(&self) -> Option<CommandDuration> {
        self.last
    }

    pub fn total(&self) -> Duration {
        self.total
    }

    pub fn command_count(&self) -> u64 {
        self.command_count
    }

    pub fn average(&self) -> Option<CommandDuration> {
        if self.command_count == 0 {
            return None;
        }

        Some(CommandDuration::new(
            self.total / self.command_count as u32,
        ))
    }

    pub fn record(&mut self, duration: CommandDuration) {
        self.last = Some(duration);
        self.total = self.total.saturating_add(duration.duration());
        self.command_count = self.command_count.saturating_add(1);
    }

    pub fn reset(&mut self) {
        self.last = None;
        self.total = Duration::ZERO;
        self.command_count = 0;
    }
}

impl Default for CommandDurationTracker {
    fn default() -> Self {
        Self::new()
    }
}
