use std::time::Duration;

use super::format::{
    RecordingEvent,
    RecordingEventKind,
    RecordingFile,
};

#[derive(Debug, Clone)]
pub struct ReplayEvent {
    pub event: RecordingEvent,
    pub due_at: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayState {
    Idle,
    Playing,
    Paused,
    Finished,
}

#[derive(Debug)]
pub struct RecordingReplay {
    recording: RecordingFile,
    state: ReplayState,
    position: usize,
    started_at: Option<std::time::Instant>,
    paused_elapsed: Duration,
}

impl RecordingReplay {
    pub fn new(recording: RecordingFile) -> Self {
        Self {
            recording,
            state: ReplayState::Idle,
            position: 0,
            started_at: None,
            paused_elapsed: Duration::ZERO,
        }
    }

    pub fn start(&mut self) {
        self.position = 0;
        self.paused_elapsed = Duration::ZERO;
        self.started_at = Some(std::time::Instant::now());
        self.state = ReplayState::Playing;
    }

    pub fn pause(&mut self) {
        if self.state == ReplayState::Playing {
            self.state = ReplayState::Paused;
        }
    }

    pub fn resume(&mut self) {
        if self.state == ReplayState::Paused {
            self.state = ReplayState::Playing;
        }
    }

    pub fn stop(&mut self) {
        self.state = ReplayState::Idle;
        self.position = 0;
        self.started_at = None;
    }

    pub fn update(&mut self) -> Vec<ReplayEvent> {
        if self.state != ReplayState::Playing {
            return Vec::new();
        }

        let elapsed = self.elapsed();
        let mut ready = Vec::new();

        while let Some(event) = self.recording.events().get(self.position) {
            if event.timestamp() > elapsed {
                break;
            }

            ready.push(ReplayEvent {
                event: event.clone(),
                due_at: event.timestamp(),
            });

            self.position += 1;
        }

        if self.position >= self.recording.events().len() {
            self.state = ReplayState::Finished;
        }

        ready
    }

    pub fn elapsed(&self) -> Duration {
        self.started_at
            .map(|started| started.elapsed())
            .unwrap_or_default()
    }

    pub fn progress(&self) -> f32 {
        let total = self.recording.duration();

        if total.is_zero() {
            return if self.state == ReplayState::Finished {
                1.0
            } else {
                0.0
            };
        }

        (self.elapsed().as_secs_f32() / total.as_secs_f32())
            .min(1.0)
    }

    pub fn state(&self) -> ReplayState {
        self.state
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn recording(&self) -> &RecordingFile {
        &self.recording
    }

    pub fn current_event(&self) -> Option<&RecordingEvent> {
        self.recording.events().get(self.position)
    }

    pub fn output_events(&self) -> impl Iterator<Item = &RecordingEvent> {
        self.recording
            .events()
            .iter()
            .filter(|event| {
                matches!(event.kind(), RecordingEventKind::Output)
            })
    }
}
