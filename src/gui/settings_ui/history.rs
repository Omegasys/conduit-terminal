use crate::config_engine::ConfigValue;
use std::collections::BTreeMap;

/// Controls shell history integration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoryMode {
    ShellOnly,
    ConduitMetadata,
    Unified,
    Disabled,
}

impl HistoryMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ShellOnly => "shell_only",
            Self::ConduitMetadata => "conduit_metadata",
            Self::Unified => "unified",
            Self::Disabled => "disabled",
        }
    }
}

/// Controls how duplicate commands are handled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DuplicateHistoryMode {
    KeepAll,
    IgnoreConsecutive,
    IgnoreAll,
}

impl DuplicateHistoryMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::KeepAll => "keep_all",
            Self::IgnoreConsecutive => "ignore_consecutive",
            Self::IgnoreAll => "ignore_all",
        }
    }
}

/// Command history settings.
#[derive(Debug, Clone)]
pub struct HistorySettings {
    enabled: bool,
    mode: HistoryMode,
    maximum_entries: u64,
    save_history: bool,
    share_between_sessions: bool,
    share_between_workspaces: bool,
    share_between_profiles: bool,
    record_failed_commands: bool,
    record_empty_commands: bool,
    record_timestamps: bool,
    record_exit_codes: bool,
    record_working_directory: bool,
    record_terminal_identity: bool,
    duplicate_mode: DuplicateHistoryMode,
    ignore_space_prefix: bool,
    ignore_patterns: Vec<String>,
    search_history: bool,
    fuzzy_search: bool,
    persist_search_queries: bool,
}

impl Default for HistorySettings {
    fn default() -> Self {
        Self {
            enabled: true,
            mode: HistoryMode::ConduitMetadata,
            maximum_entries: 10000,
            save_history: true,
            share_between_sessions: true,
            share_between_workspaces: false,
            share_between_profiles: false,
            record_failed_commands: true,
            record_empty_commands: false,
            record_timestamps: true,
            record_exit_codes: true,
            record_working_directory: true,
            record_terminal_identity: true,
            duplicate_mode: DuplicateHistoryMode::IgnoreConsecutive,
            ignore_space_prefix: true,
            ignore_patterns: Vec::new(),
            search_history: true,
            fuzzy_search: true,
            persist_search_queries: false,
        }
    }
}

impl HistorySettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn mode(&self) -> HistoryMode {
        self.mode
    }

    pub fn maximum_entries(&self) -> u64 {
        self.maximum_entries
    }

    pub fn save_history(&self) -> bool {
        self.save_history
    }

    pub fn share_between_sessions(&self) -> bool {
        self.share_between_sessions
    }

    pub fn share_between_workspaces(&self) -> bool {
        self.share_between_workspaces
    }

    pub fn share_between_profiles(&self) -> bool {
        self.share_between_profiles
    }

    pub fn record_failed_commands(&self) -> bool {
        self.record_failed_commands
    }

    pub fn record_empty_commands(&self) -> bool {
        self.record_empty_commands
    }

    pub fn record_timestamps(&self) -> bool {
        self.record_timestamps
    }

    pub fn record_exit_codes(&self) -> bool {
        self.record_exit_codes
    }

    pub fn record_working_directory(&self) -> bool {
        self.record_working_directory
    }

    pub fn record_terminal_identity(&self) -> bool {
        self.record_terminal_identity
    }

    pub fn duplicate_mode(&self) -> DuplicateHistoryMode {
        self.duplicate_mode
    }

    pub fn ignore_space_prefix(&self) -> bool {
        self.ignore_space_prefix
    }

    pub fn ignore_patterns(&self) -> &[String] {
        &self.ignore_patterns
    }

    pub fn search_history(&self) -> bool {
        self.search_history
    }

    pub fn fuzzy_search(&self) -> bool {
        self.fuzzy_search
    }

    pub fn persist_search_queries(&self) -> bool {
        self.persist_search_queries
    }

    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = value;
    }

    pub fn set_mode(&mut self, value: HistoryMode) {
        self.mode = value;
    }

    pub fn set_maximum_entries(&mut self, value: u64) {
        self.maximum_entries = value.clamp(100, 10_000_000);
    }

    pub fn set_save_history(&mut self, value: bool) {
        self.save_history = value;
    }

    pub fn set_share_between_sessions(&mut self, value: bool) {
        self.share_between_sessions = value;
    }

    pub fn set_share_between_workspaces(&mut self, value: bool) {
        self.share_between_workspaces = value;
    }

    pub fn set_share_between_profiles(&mut self, value: bool) {
        self.share_between_profiles = value;
    }

    pub fn set_record_failed_commands(&mut self, value: bool) {
        self.record_failed_commands = value;
    }

    pub fn set_record_empty_commands(&mut self, value: bool) {
        self.record_empty_commands = value;
    }

    pub fn set_record_timestamps(&mut self, value: bool) {
        self.record_timestamps = value;
    }

    pub fn set_record_exit_codes(&mut self, value: bool) {
        self.record_exit_codes = value;
    }

    pub fn set_record_working_directory(&mut self, value: bool) {
        self.record_working_directory = value;
    }

    pub fn set_record_terminal_identity(&mut self, value: bool) {
        self.record_terminal_identity = value;
    }

    pub fn set_duplicate_mode(&mut self, value: DuplicateHistoryMode) {
        self.duplicate_mode = value;
    }

    pub fn set_ignore_space_prefix(&mut self, value: bool) {
        self.ignore_space_prefix = value;
    }

    pub fn add_ignore_pattern<S: Into<String>>(&mut self, pattern: S) {
        let pattern = pattern.into();

        if !pattern.trim().is_empty() && !self.ignore_patterns.contains(&pattern) {
            self.ignore_patterns.push(pattern);
        }
    }

    pub fn remove_ignore_pattern(&mut self, pattern: &str) {
        self.ignore_patterns.retain(|item| item != pattern);
    }

    pub fn clear_ignore_patterns(&mut self) {
        self.ignore_patterns.clear();
    }

    pub fn set_search_history(&mut self, value: bool) {
        self.search_history = value;
    }

    pub fn set_fuzzy_search(&mut self, value: bool) {
        self.fuzzy_search = value;
    }

    pub fn set_persist_search_queries(&mut self, value: bool) {
        self.persist_search_queries = value;
    }

    pub fn to_config(&self) -> BTreeMap<String, ConfigValue> {
        let mut values = BTreeMap::new();

        values.insert("enabled".into(), ConfigValue::Boolean(self.enabled));
        values.insert(
            "mode".into(),
            ConfigValue::String(self.mode.as_str().into()),
        );
        values.insert(
            "maximum_entries".into(),
            ConfigValue::Integer(self.maximum_entries as i64),
        );
        values.insert(
            "save_history".into(),
            ConfigValue::Boolean(self.save_history),
        );
        values.insert(
            "share_between_sessions".into(),
            ConfigValue::Boolean(self.share_between_sessions),
        );
        values.insert(
            "share_between_workspaces".into(),
            ConfigValue::Boolean(self.share_between_workspaces),
        );
        values.insert(
            "share_between_profiles".into(),
            ConfigValue::Boolean(self.share_between_profiles),
        );
        values.insert(
            "record_failed_commands".into(),
            ConfigValue::Boolean(self.record_failed_commands),
        );
        values.insert(
            "record_empty_commands".into(),
            ConfigValue::Boolean(self.record_empty_commands),
        );
        values.insert(
            "record_timestamps".into(),
            ConfigValue::Boolean(self.record_timestamps),
        );
        values.insert(
            "record_exit_codes".into(),
            ConfigValue::Boolean(self.record_exit_codes),
        );
        values.insert(
            "record_working_directory".into(),
            ConfigValue::Boolean(self.record_working_directory),
        );
        values.insert(
            "record_terminal_identity".into(),
            ConfigValue::Boolean(self.record_terminal_identity),
        );
        values.insert(
            "duplicate_mode".into(),
            ConfigValue::String(self.duplicate_mode.as_str().into()),
        );
        values.insert(
            "ignore_space_prefix".into(),
            ConfigValue::Boolean(self.ignore_space_prefix),
        );
        values.insert(
            "ignore_patterns".into(),
            ConfigValue::Array(
                self.ignore_patterns
                    .iter()
                    .cloned()
                    .map(ConfigValue::String)
                    .collect(),
            ),
        );
        values.insert(
            "search_history".into(),
            ConfigValue::Boolean(self.search_history),
        );
        values.insert(
            "fuzzy_search".into(),
            ConfigValue::Boolean(self.fuzzy_search),
        );
        values.insert(
            "persist_search_queries".into(),
            ConfigValue::Boolean(self.persist_search_queries),
        );

        values
    }
}
