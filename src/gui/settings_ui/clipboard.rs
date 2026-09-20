use std::collections::BTreeMap;

use crate::config_engine::ConfigValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipboardBackend {
    System,
    X11,
    Wayland,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipboardSecurity {
    Unrestricted,
    ConfirmExternalReads,
    Strict,
}

#[derive(Debug, Clone)]
pub struct ClipboardSettings {
    backend: ClipboardBackend,
    security: ClipboardSecurity,
    read_enabled: bool,
    write_enabled: bool,
    primary_selection: bool,
    middle_click_paste: bool,
    copy_on_select: bool,
    trim_trailing_whitespace: bool,
    normalize_line_endings: bool,
    preserve_selection: bool,
    allow_large_paste: bool,
    large_paste_threshold: usize,
    confirm_multiline_paste: bool,
    confirm_large_paste: bool,
}

impl Default for ClipboardSettings {
    fn default() -> Self {
        Self {
            backend: ClipboardBackend::System,
            security: ClipboardSecurity::ConfirmExternalReads,
            read_enabled: true,
            write_enabled: true,
            primary_selection: true,
            middle_click_paste: true,
            copy_on_select: false,
            trim_trailing_whitespace: false,
            normalize_line_endings: true,
            preserve_selection: false,
            allow_large_paste: true,
            large_paste_threshold: 1_048_576,
            confirm_multiline_paste: true,
            confirm_large_paste: true,
        }
    }
}

impl ClipboardSettings {
    pub fn backend(&self) -> ClipboardBackend {
        self.backend
    }

    pub fn set_backend(&mut self, value: ClipboardBackend) {
        self.backend = value;
    }

    pub fn security(&self) -> ClipboardSecurity {
        self.security
    }

    pub fn set_security(&mut self, value: ClipboardSecurity) {
        self.security = value;
    }

    pub fn read_enabled(&self) -> bool {
        self.read_enabled
    }

    pub fn set_read_enabled(&mut self, value: bool) {
        self.read_enabled = value;
    }

    pub fn write_enabled(&self) -> bool {
        self.write_enabled
    }

    pub fn set_write_enabled(&mut self, value: bool) {
        self.write_enabled = value;
    }

    pub fn primary_selection(&self) -> bool {
        self.primary_selection
    }

    pub fn set_primary_selection(&mut self, value: bool) {
        self.primary_selection = value;
    }

    pub fn middle_click_paste(&self) -> bool {
        self.middle_click_paste
    }

    pub fn set_middle_click_paste(&mut self, value: bool) {
        self.middle_click_paste = value;
    }

    pub fn copy_on_select(&self) -> bool {
        self.copy_on_select
    }

    pub fn set_copy_on_select(&mut self, value: bool) {
        self.copy_on_select = value;
    }

    pub fn trim_trailing_whitespace(&self) -> bool {
        self.trim_trailing_whitespace
    }

    pub fn set_trim_trailing_whitespace(&mut self, value: bool) {
        self.trim_trailing_whitespace = value;
    }

    pub fn normalize_line_endings(&self) -> bool {
        self.normalize_line_endings
    }

    pub fn set_normalize_line_endings(&mut self, value: bool) {
        self.normalize_line_endings = value;
    }

    pub fn preserve_selection(&self) -> bool {
        self.preserve_selection
    }

    pub fn set_preserve_selection(&mut self, value: bool) {
        self.preserve_selection = value;
    }

    pub fn allow_large_paste(&self) -> bool {
        self.allow_large_paste
    }

    pub fn set_allow_large_paste(&mut self, value: bool) {
        self.allow_large_paste = value;
    }

    pub fn large_paste_threshold(&self) -> usize {
        self.large_paste_threshold
    }

    pub fn set_large_paste_threshold(&mut self, value: usize) {
        self.large_paste_threshold = value.clamp(1024, 1 << 30);
    }

    pub fn confirm_multiline_paste(&self) -> bool {
        self.confirm_multiline_paste
    }

    pub fn set_confirm_multiline_paste(&mut self, value: bool) {
        self.confirm_multiline_paste = value;
    }

    pub fn confirm_large_paste(&self) -> bool {
        self.confirm_large_paste
    }

    pub fn set_confirm_large_paste(&mut self, value: bool) {
        self.confirm_large_paste = value;
    }

    pub fn to_config(&self) -> BTreeMap<String, ConfigValue> {
        let mut values = BTreeMap::new();

        values.insert(
            "backend".into(),
            ConfigValue::String(format!("{:?}", self.backend).to_lowercase()),
        );
        values.insert(
            "security".into(),
            ConfigValue::String(format!("{:?}", self.security).to_lowercase()),
        );
        values.insert("read_enabled".into(), ConfigValue::Boolean(self.read_enabled));
        values.insert(
            "write_enabled".into(),
            ConfigValue::Boolean(self.write_enabled),
        );
        values.insert(
            "primary_selection".into(),
            ConfigValue::Boolean(self.primary_selection),
        );
        values.insert(
            "middle_click_paste".into(),
            ConfigValue::Boolean(self.middle_click_paste),
        );
        values.insert(
            "copy_on_select".into(),
            ConfigValue::Boolean(self.copy_on_select),
        );
        values.insert(
            "trim_trailing_whitespace".into(),
            ConfigValue::Boolean(self.trim_trailing_whitespace),
        );
        values.insert(
            "normalize_line_endings".into(),
            ConfigValue::Boolean(self.normalize_line_endings),
        );
        values.insert(
            "preserve_selection".into(),
            ConfigValue::Boolean(self.preserve_selection),
        );
        values.insert(
            "allow_large_paste".into(),
            ConfigValue::Boolean(self.allow_large_paste),
        );
        values.insert(
            "large_paste_threshold".into(),
            ConfigValue::Integer(self.large_paste_threshold as i64),
        );
        values.insert(
            "confirm_multiline_paste".into(),
            ConfigValue::Boolean(self.confirm_multiline_paste),
        );
        values.insert(
            "confirm_large_paste".into(),
            ConfigValue::Boolean(self.confirm_large_paste),
        );

        values
    }
}
