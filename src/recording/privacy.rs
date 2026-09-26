use super::format::RecordingEventKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordingPrivacyMode {
    Normal,
    Private,
    InputDisabled,
    OutputDisabled,
    Disabled,
}

impl Default for RecordingPrivacyMode {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Debug, Clone)]
pub struct RecordingPrivacyPolicy {
    mode: RecordingPrivacyMode,
    record_input: bool,
    record_output: bool,
    record_metadata: bool,
    record_working_directory: bool,
}

impl Default for RecordingPrivacyPolicy {
    fn default() -> Self {
        Self {
            mode: RecordingPrivacyMode::Normal,
            record_input: true,
            record_output: true,
            record_metadata: true,
            record_working_directory: true,
        }
    }
}

impl RecordingPrivacyPolicy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mode(&self) -> RecordingPrivacyMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: RecordingPrivacyMode) {
        self.mode = mode;

        match mode {
            RecordingPrivacyMode::Normal => {
                self.record_input = true;
                self.record_output = true;
                self.record_metadata = true;
                self.record_working_directory = true;
            }

            RecordingPrivacyMode::Private => {
                self.record_input = false;
                self.record_output = true;
                self.record_metadata = false;
                self.record_working_directory = false;
            }

            RecordingPrivacyMode::InputDisabled => {
                self.record_input = false;
                self.record_output = true;
                self.record_metadata = true;
                self.record_working_directory = true;
            }

            RecordingPrivacyMode::OutputDisabled => {
                self.record_input = true;
                self.record_output = false;
                self.record_metadata = true;
                self.record_working_directory = true;
            }

            RecordingPrivacyMode::Disabled => {
                self.record_input = false;
                self.record_output = false;
                self.record_metadata = false;
                self.record_working_directory = false;
            }
        }
    }

    pub fn record_input(&self) -> bool {
        self.record_input
    }

    pub fn record_output(&self) -> bool {
        self.record_output
    }

    pub fn record_metadata(&self) -> bool {
        self.record_metadata
    }

    pub fn record_working_directory(&self) -> bool {
        self.record_working_directory
    }

    pub fn allow_event(&self, kind: &RecordingEventKind) -> bool {
        match kind {
            RecordingEventKind::Input => self.record_input,
            RecordingEventKind::Output => self.record_output,

            RecordingEventKind::WorkingDirectoryChanged(_) => {
                self.record_metadata && self.record_working_directory
            }

            RecordingEventKind::Resize { .. }
            | RecordingEventKind::Bell
            | RecordingEventKind::TitleChanged(_)
            | RecordingEventKind::Marker(_) => self.record_metadata,
        }
    }
}
