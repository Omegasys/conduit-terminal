use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct CommandDuration {
    started_at: Option<Instant>,
    elapsed: Option<Duration>,
}

impl CommandDuration {
    pub fn new() -> Self {
        Self {
            started_at: None,
            elapsed: None,
        }
    }

    pub fn start(&mut self) {
        self.started_at = Some(Instant::now());
        self.elapsed = None;
    }

    pub fn stop(&mut self) -> Option<Duration> {
        let elapsed = self.started_at.map(|started| started.elapsed());

        if let Some(duration) = elapsed {
            self.elapsed = Some(duration);
        }

        self.started_at = None;

        elapsed
    }

    pub fn reset(&mut self) {
        self.started_at = None;
        self.elapsed = None;
    }

    pub fn is_running(&self) -> bool {
        self.started_at.is_some()
    }

    pub fn elapsed(&self) -> Option<Duration> {
        match self.started_at {
            Some(started) => Some(started.elapsed()),
            None => self.elapsed,
        }
    }

    pub fn elapsed_millis(&self) -> Option<u128> {
        self.elapsed().map(|duration| duration.as_millis())
    }

    pub fn elapsed_secs(&self) -> Option<f64> {
        self.elapsed()
            .map(|duration| duration.as_secs_f64())
    }
}

impl Default for CommandDuration {
    fn default() -> Self {
        Self::new()
    }
}
