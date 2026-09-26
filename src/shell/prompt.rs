//! Prompt integration.
//!
//! Conduit does not own shell prompts. Instead, this module describes optional
//! integration that allows a shell to report prompt boundaries and state back
//! to Conduit.

/// Prompt integration mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptMode {
    Disabled,
    Passive,
    Integrated,
}

impl Default for PromptMode {
    fn default() -> Self {
        Self::Integrated
    }
}

/// Prompt integration state.
#[derive(Debug, Clone)]
pub struct PromptIntegration {
    mode: PromptMode,
    command_marker: String,
    prompt_marker: String,
    continuation_marker: String,
    sequence: u64,
}

impl Default for PromptIntegration {
    fn default() -> Self {
        Self {
            mode: PromptMode::Integrated,
            command_marker: "CONDUIT_COMMAND".into(),
            prompt_marker: "CONDUIT_PROMPT".into(),
            continuation_marker: "CONDUIT_CONTINUATION".into(),
            sequence: 0,
        }
    }
}

impl PromptIntegration {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mode(&self) -> PromptMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: PromptMode) {
        self.mode = mode;
    }

    pub fn enabled(&self) -> bool {
        self.mode != PromptMode::Disabled
    }

    pub fn set_markers(
        &mut self,
        command: impl Into<String>,
        prompt: impl Into<String>,
        continuation: impl Into<String>,
    ) {
        self.command_marker = command.into();
        self.prompt_marker = prompt.into();
        self.continuation_marker = continuation.into();
    }

    pub fn command_marker(&self) -> &str {
        &self.command_marker
    }

    pub fn prompt_marker(&self) -> &str {
        &self.prompt_marker
    }

    pub fn continuation_marker(&self) -> &str {
        &self.continuation_marker
    }

    /// Create a unique marker for a shell integration event.
    pub fn next_marker(&mut self) -> String {
        self.sequence += 1;

        format!("{}-{}", self.command_marker, self.sequence)
    }

    /// Generate a shell-neutral marker payload.
    pub fn marker_payload(&mut self) -> String {
        let marker = self.next_marker();

        format!(
            "{}:{}:{}",
            self.prompt_marker,
            marker,
            self.continuation_marker
        )
    }

    pub fn reset(&mut self) {
        self.sequence = 0;
    }
}
