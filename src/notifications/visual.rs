use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub struct VisualNotificationConfig {
    pub enabled: bool,
    pub duration: Duration,
}

impl Default for VisualNotificationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            duration: Duration::from_millis(150),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VisualNotification {
    pub started_at: Instant,
    pub duration: Duration,
}

impl VisualNotification {
    pub fn new(duration: Duration) -> Self {
        Self {
            started_at: Instant::now(),
            duration,
        }
    }

    pub fn expired(&self) -> bool {
        self.started_at.elapsed() >= self.duration
    }

    pub fn progress(&self) -> f32 {
        let elapsed = self.started_at.elapsed();

        if elapsed >= self.duration {
            return 1.0;
        }

        elapsed.as_secs_f32() / self.duration.as_secs_f32()
    }
}

impl VisualNotificationConfig {
    pub fn trigger(&mut self) {
        self.enabled = true;
    }

    pub fn update(&mut self) {}

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}
