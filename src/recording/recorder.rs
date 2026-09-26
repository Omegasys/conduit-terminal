use std::time::{Duration, Instant};

use super::format::{
    RecordingEvent,
    RecordingFile,
};
use super::metadata::RecordingMetadata;
use super::privacy::RecordingPrivacyPolicy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordingState {
    Idle,
    Recording,
    Paused,
    Finished,
}

#[derive(Debug)]
pub enum RecordingError {
    NotRecording,
    AlreadyRecording,
    Paused,
}

pub struct RecordingRecorder {
    state: RecordingState,
    started_at: Option<Instant>,
    paused_at: Option<Instant>,
    paused_duration: Duration,
    recording: Option<RecordingFile>,
    privacy: RecordingPrivacyPolicy,
}

impl RecordingRecorder {
    pub fn new(metadata: RecordingMetadata) -> Self {
        Self {
            state: RecordingState::Idle,
            started_at: None,
            paused_at: None,
            paused_duration: Duration::ZERO,
            recording: Some(RecordingFile::new(metadata)),
            privacy: RecordingPrivacyPolicy::default(),
        }
    }

    pub fn start(&mut self) -> Result<(), RecordingError> {
        if self.state == RecordingState::Recording {
            return Err(RecordingError::AlreadyRecording);
        }

        self.started_at = Some(Instant::now());
        self.paused_at = None;
        self.paused_duration = Duration::ZERO;
        self.state = RecordingState::Recording;

        Ok(())
    }

    pub fn pause(&mut self) -> Result<(), RecordingError> {
        if self.state != RecordingState::Recording {
            return Err(RecordingError::NotRecording);
        }

        self.paused_at = Some(Instant::now());
        self.state = RecordingState::Paused;

        Ok(())
    }

    pub fn resume(&mut self) -> Result<(), RecordingError> {
        if self.state != RecordingState::Paused {
            return Err(RecordingError::NotRecording);
        }

        if let Some(paused) = self.paused_at.take() {
            self.paused_duration += paused.elapsed();
        }

        self.state = RecordingState::Recording;

        Ok(())
    }

    pub fn finish(&mut self) -> Result<RecordingFile, RecordingError> {
        if self.state == RecordingState::Idle {
            return Err(RecordingError::NotRecording);
        }

        self.state = RecordingState::Finished;

        self.recording
            .take()
            .ok_or(RecordingError::NotRecording)
    }

    pub fn record_output(
        &mut self,
        data: impl Into<Vec<u8>>,
    ) -> Result<(), RecordingError> {
        self.record(
            RecordingEvent::output(self.elapsed(), data),
        )
    }

    pub fn record_input(
        &mut self,
        data: impl Into<Vec<u8>>,
    ) -> Result<(), RecordingError> {
        if !self.privacy.record_input() {
            return Ok(());
        }

        self.record(
            RecordingEvent::input(self.elapsed(), data),
        )
    }

    pub fn record_resize(
        &mut self,
        columns: u16,
        rows: u16,
    ) -> Result<(), RecordingError> {
        self.record(
            RecordingEvent::resize(
                self.elapsed(),
                columns,
                rows,
            ),
        )
    }

    pub fn record_bell(&mut self) -> Result<(), RecordingError> {
        self.record(RecordingEvent::bell(self.elapsed()))
    }

    pub fn record(
        &mut self,
        event: RecordingEvent,
    ) -> Result<(), RecordingError> {
        if self.state == RecordingState::Idle
            || self.state == RecordingState::Finished
        {
            return Err(RecordingError::NotRecording);
        }

        if self.state == RecordingState::Paused {
            return Err(RecordingError::Paused);
        }

        if !self.privacy.allow_event(event.kind()) {
            return Ok(());
        }

        if let Some(recording) = self.recording.as_mut() {
            recording.push(event);
        }

        Ok(())
    }

    pub fn elapsed(&self) -> Duration {
        match self.started_at {
            Some(started) => {
                let elapsed = started.elapsed();

                elapsed
                    .checked_sub(self.paused_duration)
                    .unwrap_or_default()
            }
            None => Duration::ZERO,
        }
    }

    pub fn state(&self) -> RecordingState {
        self.state
    }

    pub fn is_recording(&self) -> bool {
        self.state == RecordingState::Recording
    }

    pub fn recording(&self) -> Option<&RecordingFile> {
        self.recording.as_ref()
    }

    pub fn recording_mut(&mut self) -> Option<&mut RecordingFile> {
        self.recording.as_mut()
    }

    pub fn privacy(&self) -> &RecordingPrivacyPolicy {
        &self.privacy
    }

    pub fn privacy_mut(&mut self) -> &mut RecordingPrivacyPolicy {
        &mut self.privacy
    }
}
