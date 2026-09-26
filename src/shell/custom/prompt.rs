//! Prompt integration for custom shells.

use super::errors::{CustomShellError, CustomShellResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CustomPromptMode {
    Disabled,
    Passive,
    Integrated,
}

impl Default for CustomPromptMode {
    fn default() -> Self {
        Self::Passive
    }
}

#[derive(Debug, Clone)]
pub struct CustomPromptIntegration {
    mode: CustomPromptMode,
    prompt_marker: String,
    command_marker: String,
    continuation_marker: String,
    sequence: u64,
}

impl Default for CustomPromptIntegration {
    fn default() -> Self {
        Self {
            mode: CustomPromptMode::Passive,
            prompt_marker: "\x1b]133;A\x07".into(),
            command_marker: "\x1b]133;C\x07".into(),
            continuation_marker: "\x1b]133;A;continuation\x07".into(),
            sequence: 0,
        }
    }
}

impl CustomPromptIntegration {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mode(&self) -> CustomPromptMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: CustomPromptMode) {
        self.mode = mode;
    }

    pub fn enabled(&self) -> bool {
        self.mode != CustomPromptMode::Disabled
    }

    pub fn prompt_marker(&self) -> &str {
        &self.prompt_marker
    }

    pub fn command_marker(&self) -> &str {
        &self.command_marker
    }

    pub fn continuation_marker(&self) -> &str {
        &self.continuation_marker
    }

    pub fn set_markers(
        &mut self,
        prompt: impl Into<String>,
        command: impl Into<String>,
        continuation: impl Into<String>,
    ) {
        self.prompt_marker = prompt.into();
        self.command_marker = command.into();
        self.continuation_marker = continuation.into();
    }

    pub fn next_sequence(&mut self) -> u64 {
        self.sequence = self.sequence.wrapping_add(1);
        self.sequence
    }

    pub fn validate(&self) -> CustomShellResult<()> {
        if self.mode == CustomPromptMode::Integrated
            && self.prompt_marker.is_empty()
        {
            return Err(CustomShellError::InvalidPrompt(
                "integrated prompts require a prompt marker".into(),
            ));
        }

        Ok(())
    }
}
