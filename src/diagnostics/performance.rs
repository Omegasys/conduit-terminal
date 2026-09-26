use std::time::{Duration, Instant};

#[derive(Debug, Clone, Default)]
pub struct PerformanceDiagnostics {
    frame_count: u64,
    dropped_frames: u64,
    command_count: u64,
    event_count: u64,
    render_time: Duration,
    last_frame: Option<Instant>,
}

impl PerformanceDiagnostics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_frame(&mut self, duration: Duration) {
        self.frame_count = self.frame_count.saturating_add(1);
        self.render_time += duration;
        self.last_frame = Some(Instant::now());
    }

    pub fn record_dropped_frame(&mut self) {
        self.dropped_frames = self.dropped_frames.saturating_add(1);
    }

    pub fn record_command(&mut self) {
        self.command_count = self.command_count.saturating_add(1);
    }

    pub fn record_event(&mut self) {
        self.event_count = self.event_count.saturating_add(1);
    }

    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }

    pub fn dropped_frames(&self) -> u64 {
        self.dropped_frames
    }

    pub fn command_count(&self) -> u64 {
        self.command_count
    }

    pub fn event_count(&self) -> u64 {
        self.event_count
    }

    pub fn total_render_time(&self) -> Duration {
        self.render_time
    }

    pub fn average_frame_time(&self) -> Option<Duration> {
        if self.frame_count == 0 {
            return None;
        }

        Some(self.render_time / self.frame_count as u32)
    }

    pub fn estimated_fps(&self) -> Option<f64> {
        let average = self.average_frame_time()?;

        if average.is_zero() {
            return None;
        }

        Some(1.0 / average.as_secs_f64())
    }

    pub fn last_frame_age(&self) -> Option<Duration> {
        self.last_frame.map(|frame| frame.elapsed())
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
