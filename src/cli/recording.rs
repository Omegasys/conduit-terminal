use std::fmt;
use std::path::{Path, PathBuf};

/// Recording output formats supported by the CLI layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordingFormat {
    Asciicast,
    PlainText,
    Json,
    Binary,
}

impl RecordingFormat {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Asciicast => "asciicast",
            Self::PlainText => "text",
            Self::Json => "json",
            Self::Binary => "binary",
        }
    }

    pub fn from_name(value: &str) -> Option<Self> {
        match value {
            "asciicast" | "cast" => Some(Self::Asciicast),
            "text" | "txt" | "plain" => Some(Self::PlainText),
            "json" => Some(Self::Json),
            "binary" | "bin" => Some(Self::Binary),
            _ => None,
        }
    }
}

impl fmt::Display for RecordingFormat {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

/// Actions supported by the `conduit recording` command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordingCommandAction {
    Status,
    Start,
    Pause,
    Resume,
    Stop,
    List,
    Show,
    Delete,
    Export,
    Import,
}

impl RecordingCommandAction {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Status => "status",
            Self::Start => "start",
            Self::Pause => "pause",
            Self::Resume => "resume",
            Self::Stop => "stop",
            Self::List => "list",
            Self::Show => "show",
            Self::Delete => "delete",
            Self::Export => "export",
            Self::Import => "import",
        }
    }

    pub fn from_name(value: &str) -> Option<Self> {
        match value {
            "status" => Some(Self::Status),
            "start" => Some(Self::Start),
            "pause" => Some(Self::Pause),
            "resume" => Some(Self::Resume),
            "stop" => Some(Self::Stop),
            "list" => Some(Self::List),
            "show" => Some(Self::Show),
            "delete" => Some(Self::Delete),
            "export" => Some(Self::Export),
            "import" => Some(Self::Import),
            _ => None,
        }
    }
}

impl fmt::Display for RecordingCommandAction {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

/// Recording state exposed by the CLI layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordingState {
    Inactive,
    Recording,
    Paused,
    Stopping,
    Completed,
    Failed,
}

impl RecordingState {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Inactive => "inactive",
            Self::Recording => "recording",
            Self::Paused => "paused",
            Self::Stopping => "stopping",
            Self::Completed => "completed",
            Self::Failed => "failed",
        }
    }
}

impl fmt::Display for RecordingState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

/// A parsed recording command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordingCommand {
    action: RecordingCommandAction,
    recording_id: Option<String>,
    output_path: Option<PathBuf>,
    input_path: Option<PathBuf>,
    format: Option<RecordingFormat>,
    include_input: bool,
    include_output: bool,
    include_timestamps: bool,
    compress: bool,
    encrypt: bool,
}

impl RecordingCommand {
    pub fn new(action: RecordingCommandAction) -> Self {
        Self {
            action,
            recording_id: None,
            output_path: None,
            input_path: None,
            format: None,
            include_input: false,
            include_output: true,
            include_timestamps: true,
            compress: false,
            encrypt: false,
        }
    }

    pub fn action(&self) -> &RecordingCommandAction {
        &self.action
    }

    pub fn recording_id(&self) -> Option<&str> {
        self.recording_id.as_deref()
    }

    pub fn output_path(&self) -> Option<&Path> {
        self.output_path.as_deref()
    }

    pub fn input_path(&self) -> Option<&Path> {
        self.input_path.as_deref()
    }

    pub fn format(&self) -> Option<RecordingFormat> {
        self.format
    }

    pub fn include_input(&self) -> bool {
        self.include_input
    }

    pub fn include_output(&self) -> bool {
        self.include_output
    }

    pub fn include_timestamps(&self) -> bool {
        self.include_timestamps
    }

    pub fn compress(&self) -> bool {
        self.compress
    }

    pub fn encrypt(&self) -> bool {
        self.encrypt
    }

    pub fn set_recording_id<S>(&mut self, recording_id: S)
    where
        S: Into<String>,
    {
        self.recording_id = Some(recording_id.into());
    }

    pub fn set_output_path<P>(&mut self, path: P)
    where
        P: Into<PathBuf>,
    {
        self.output_path = Some(path.into());
    }

    pub fn set_input_path<P>(&mut self, path: P)
    where
        P: Into<PathBuf>,
    {
        self.input_path = Some(path.into());
    }

    pub fn set_format(&mut self, format: RecordingFormat) {
        self.format = Some(format);
    }

    pub fn set_include_input(&mut self, enabled: bool) {
        self.include_input = enabled;
    }

    pub fn set_include_output(&mut self, enabled: bool) {
        self.include_output = enabled;
    }

    pub fn set_include_timestamps(&mut self, enabled: bool) {
        self.include_timestamps = enabled;
    }

    pub fn set_compress(&mut self, enabled: bool) {
        self.compress = enabled;
    }

    pub fn set_encrypt(&mut self, enabled: bool) {
        self.encrypt = enabled;
    }

    pub fn requires_recording_id(&self) -> bool {
        matches!(
            self.action,
            RecordingCommandAction::Pause
                | RecordingCommandAction::Resume
                | RecordingCommandAction::Stop
                | RecordingCommandAction::Show
                | RecordingCommandAction::Delete
                | RecordingCommandAction::Export
        )
    }

    pub fn requires_output_path(&self) -> bool {
        matches!(
            self.action,
            RecordingCommandAction::Start
                | RecordingCommandAction::Export
        )
    }

    pub fn requires_input_path(&self) -> bool {
        matches!(self.action, RecordingCommandAction::Import)
    }
}
