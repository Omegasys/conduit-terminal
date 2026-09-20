use crate::config_engine::ConfigValue;
use std::collections::BTreeMap;

/// Controls what terminal activity is recorded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordingMode {
    Disabled,
    Manual,
    Automatic,
    OnOutput,
    OnSessionStart,
}

impl RecordingMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::Manual => "manual",
            Self::Automatic => "automatic",
            Self::OnOutput => "on_output",
            Self::OnSessionStart => "on_session_start",
        }
    }
}

/// Recording output format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordingFormat {
    PlainText,
    Asciicast,
    Conduit,
    Json,
}

impl RecordingFormat {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PlainText => "plain_text",
            Self::Asciicast => "asciicast",
            Self::Conduit => "conduit",
            Self::Json => "json",
        }
    }
}

/// Terminal recording settings.
#[derive(Debug, Clone)]
pub struct RecordingSettings {
    enabled: bool,
    mode: RecordingMode,
    format: RecordingFormat,
    directory: String,
    include_input: bool,
    include_output: bool,
    include_timestamps: bool,
    include_window_size: bool,
    include_environment: bool,
    include_shell_metadata: bool,
    compress_recordings: bool,
    compression_level: u8,
    maximum_file_size_mb: u32,
    automatic_stop_minutes: u32,
    overwrite_existing: bool,
    encrypt_recordings: bool,
    encrypt_metadata: bool,
    keep_failed_recordings: bool,
}

impl Default for RecordingSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            mode: RecordingMode::Manual,
            format: RecordingFormat::Conduit,
            directory: "~/.local/share/conduit/recordings".to_string(),
            include_input: true,
            include_output: true,
            include_timestamps: true,
            include_window_size: true,
            include_environment: false,
            include_shell_metadata: true,
            compress_recordings: true,
            compression_level: 6,
            maximum_file_size_mb: 1024,
            automatic_stop_minutes: 0,
            overwrite_existing: false,
            encrypt_recordings: false,
            encrypt_metadata: false,
            keep_failed_recordings: true,
        }
    }
}

impl RecordingSettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn mode(&self) -> RecordingMode {
        self.mode
    }

    pub fn format(&self) -> RecordingFormat {
        self.format
    }

    pub fn directory(&self) -> &str {
        &self.directory
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

    pub fn include_window_size(&self) -> bool {
        self.include_window_size
    }

    pub fn include_environment(&self) -> bool {
        self.include_environment
    }

    pub fn include_shell_metadata(&self) -> bool {
        self.include_shell_metadata
    }

    pub fn compress_recordings(&self) -> bool {
        self.compress_recordings
    }

    pub fn compression_level(&self) -> u8 {
        self.compression_level
    }

    pub fn maximum_file_size_mb(&self) -> u32 {
        self.maximum_file_size_mb
    }

    pub fn automatic_stop_minutes(&self) -> u32 {
        self.automatic_stop_minutes
    }

    pub fn overwrite_existing(&self) -> bool {
        self.overwrite_existing
    }

    pub fn encrypt_recordings(&self) -> bool {
        self.encrypt_recordings
    }

    pub fn encrypt_metadata(&self) -> bool {
        self.encrypt_metadata
    }

    pub fn keep_failed_recordings(&self) -> bool {
        self.keep_failed_recordings
    }

    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = value;
    }

    pub fn set_mode(&mut self, value: RecordingMode) {
        self.mode = value;
    }

    pub fn set_format(&mut self, value: RecordingFormat) {
        self.format = value;
    }

    pub fn set_directory<S: Into<String>>(&mut self, value: S) {
        let value = value.into();
        if !value.trim().is_empty() {
            self.directory = value;
        }
    }

    pub fn set_include_input(&mut self, value: bool) {
        self.include_input = value;
    }

    pub fn set_include_output(&mut self, value: bool) {
        self.include_output = value;
    }

    pub fn set_include_timestamps(&mut self, value: bool) {
        self.include_timestamps = value;
    }

    pub fn set_include_window_size(&mut self, value: bool) {
        self.include_window_size = value;
    }

    pub fn set_include_environment(&mut self, value: bool) {
        self.include_environment = value;
    }

    pub fn set_include_shell_metadata(&mut self, value: bool) {
        self.include_shell_metadata = value;
    }

    pub fn set_compress_recordings(&mut self, value: bool) {
        self.compress_recordings = value;
    }

    pub fn set_compression_level(&mut self, value: u8) {
        self.compression_level = value.clamp(0, 9);
    }

    pub fn set_maximum_file_size_mb(&mut self, value: u32) {
        self.maximum_file_size_mb = value.clamp(1, 16384);
    }

    pub fn set_automatic_stop_minutes(&mut self, value: u32) {
        self.automatic_stop_minutes = value.min(7 * 24 * 60);
    }

    pub fn set_overwrite_existing(&mut self, value: bool) {
        self.overwrite_existing = value;
    }

    pub fn set_encrypt_recordings(&mut self, value: bool) {
        self.encrypt_recordings = value;
    }

    pub fn set_encrypt_metadata(&mut self, value: bool) {
        self.encrypt_metadata = value;
    }

    pub fn set_keep_failed_recordings(&mut self, value: bool) {
        self.keep_failed_recordings = value;
    }

    pub fn to_config(&self) -> BTreeMap<String, ConfigValue> {
        let mut values = BTreeMap::new();

        values.insert("enabled".into(), ConfigValue::Boolean(self.enabled));
        values.insert(
            "mode".into(),
            ConfigValue::String(self.mode.as_str().into()),
        );
        values.insert(
            "format".into(),
            ConfigValue::String(self.format.as_str().into()),
        );
        values.insert(
            "directory".into(),
            ConfigValue::String(self.directory.clone()),
        );
        values.insert(
            "include_input".into(),
            ConfigValue::Boolean(self.include_input),
        );
        values.insert(
            "include_output".into(),
            ConfigValue::Boolean(self.include_output),
        );
        values.insert(
            "include_timestamps".into(),
            ConfigValue::Boolean(self.include_timestamps),
        );
        values.insert(
            "include_window_size".into(),
            ConfigValue::Boolean(self.include_window_size),
        );
        values.insert(
            "include_environment".into(),
            ConfigValue::Boolean(self.include_environment),
        );
        values.insert(
            "include_shell_metadata".into(),
            ConfigValue::Boolean(self.include_shell_metadata),
        );
        values.insert(
            "compress_recordings".into(),
            ConfigValue::Boolean(self.compress_recordings),
        );
        values.insert(
            "compression_level".into(),
            ConfigValue::Integer(self.compression_level as i64),
        );
        values.insert(
            "maximum_file_size_mb".into(),
            ConfigValue::Integer(self.maximum_file_size_mb as i64),
        );
        values.insert(
            "automatic_stop_minutes".into(),
            ConfigValue::Integer(self.automatic_stop_minutes as i64),
        );
        values.insert(
            "overwrite_existing".into(),
            ConfigValue::Boolean(self.overwrite_existing),
        );
        values.insert(
            "encrypt_recordings".into(),
            ConfigValue::Boolean(self.encrypt_recordings),
        );
        values.insert(
            "encrypt_metadata".into(),
            ConfigValue::Boolean(self.encrypt_metadata),
        );
        values.insert(
            "keep_failed_recordings".into(),
            ConfigValue::Boolean(self.keep_failed_recordings),
        );

        values
    }
}
