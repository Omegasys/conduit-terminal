use crate::config_engine::ConfigValue;
use std::collections::BTreeMap;

/// Controls the amount of diagnostic information exposed by developer mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeveloperLogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl DeveloperLogLevel {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Trace => "trace",
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Error => "error",
        }
    }
}

/// Developer and diagnostic settings.
///
/// These settings are intended for development, debugging, profiling,
/// protocol investigation, and diagnosing Conduit installations.
#[derive(Debug, Clone)]
pub struct DeveloperSettings {
    developer_mode: bool,
    log_level: DeveloperLogLevel,
    enable_debug_overlay: bool,
    show_event_debugger: bool,
    show_command_debugger: bool,
    show_resource_debugger: bool,
    show_configuration_debugger: bool,
    show_terminal_protocol_debugger: bool,
    trace_events: bool,
    trace_commands: bool,
    trace_configuration: bool,
    trace_resources: bool,
    trace_plugins: bool,
    trace_terminal_io: bool,
    collect_performance_metrics: bool,
    collect_memory_metrics: bool,
    collect_render_metrics: bool,
    enable_profiling: bool,
    profile_startup: bool,
    profile_rendering: bool,
    profile_terminal_io: bool,
    expose_internal_commands: bool,
    enable_experimental_commands: bool,
    allow_test_resources: bool,
    keep_debug_logs: bool,
    debug_log_directory: String,
}

impl Default for DeveloperSettings {
    fn default() -> Self {
        Self {
            developer_mode: false,
            log_level: DeveloperLogLevel::Info,
            enable_debug_overlay: false,
            show_event_debugger: false,
            show_command_debugger: false,
            show_resource_debugger: false,
            show_configuration_debugger: false,
            show_terminal_protocol_debugger: false,
            trace_events: false,
            trace_commands: false,
            trace_configuration: false,
            trace_resources: false,
            trace_plugins: false,
            trace_terminal_io: false,
            collect_performance_metrics: false,
            collect_memory_metrics: false,
            collect_render_metrics: false,
            enable_profiling: false,
            profile_startup: false,
            profile_rendering: false,
            profile_terminal_io: false,
            expose_internal_commands: false,
            enable_experimental_commands: false,
            allow_test_resources: false,
            keep_debug_logs: true,
            debug_log_directory: "~/.local/share/conduit/debug".to_string(),
        }
    }
}

impl DeveloperSettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn developer_mode(&self) -> bool {
        self.developer_mode
    }

    pub fn log_level(&self) -> DeveloperLogLevel {
        self.log_level
    }

    pub fn enable_debug_overlay(&self) -> bool {
        self.enable_debug_overlay
    }

    pub fn show_event_debugger(&self) -> bool {
        self.show_event_debugger
    }

    pub fn show_command_debugger(&self) -> bool {
        self.show_command_debugger
    }

    pub fn show_resource_debugger(&self) -> bool {
        self.show_resource_debugger
    }

    pub fn show_configuration_debugger(&self) -> bool {
        self.show_configuration_debugger
    }

    pub fn show_terminal_protocol_debugger(&self) -> bool {
        self.show_terminal_protocol_debugger
    }

    pub fn trace_events(&self) -> bool {
        self.trace_events
    }

    pub fn trace_commands(&self) -> bool {
        self.trace_commands
    }

    pub fn trace_configuration(&self) -> bool {
        self.trace_configuration
    }

    pub fn trace_resources(&self) -> bool {
        self.trace_resources
    }

    pub fn trace_plugins(&self) -> bool {
        self.trace_plugins
    }

    pub fn trace_terminal_io(&self) -> bool {
        self.trace_terminal_io
    }

    pub fn collect_performance_metrics(&self) -> bool {
        self.collect_performance_metrics
    }

    pub fn collect_memory_metrics(&self) -> bool {
        self.collect_memory_metrics
    }

    pub fn collect_render_metrics(&self) -> bool {
        self.collect_render_metrics
    }

    pub fn enable_profiling(&self) -> bool {
        self.enable_profiling
    }

    pub fn profile_startup(&self) -> bool {
        self.profile_startup
    }

    pub fn profile_rendering(&self) -> bool {
        self.profile_rendering
    }

    pub fn profile_terminal_io(&self) -> bool {
        self.profile_terminal_io
    }

    pub fn expose_internal_commands(&self) -> bool {
        self.expose_internal_commands
    }

    pub fn enable_experimental_commands(&self) -> bool {
        self.enable_experimental_commands
    }

    pub fn allow_test_resources(&self) -> bool {
        self.allow_test_resources
    }

    pub fn keep_debug_logs(&self) -> bool {
        self.keep_debug_logs
    }

    pub fn debug_log_directory(&self) -> &str {
        &self.debug_log_directory
    }

    pub fn set_developer_mode(&mut self, value: bool) {
        self.developer_mode = value;

        if !value {
            self.enable_debug_overlay = false;
            self.show_event_debugger = false;
            self.show_command_debugger = false;
            self.show_resource_debugger = false;
            self.show_configuration_debugger = false;
            self.show_terminal_protocol_debugger = false;
            self.enable_profiling = false;
        }
    }

    pub fn set_log_level(&mut self, value: DeveloperLogLevel) {
        self.log_level = value;
    }

    pub fn set_enable_debug_overlay(&mut self, value: bool) {
        self.enable_debug_overlay = value;
    }

    pub fn set_show_event_debugger(&mut self, value: bool) {
        self.show_event_debugger = value;
    }

    pub fn set_show_command_debugger(&mut self, value: bool) {
        self.show_command_debugger = value;
    }

    pub fn set_show_resource_debugger(&mut self, value: bool) {
        self.show_resource_debugger = value;
    }

    pub fn set_show_configuration_debugger(&mut self, value: bool) {
        self.show_configuration_debugger = value;
    }

    pub fn set_show_terminal_protocol_debugger(&mut self, value: bool) {
        self.show_terminal_protocol_debugger = value;
    }

    pub fn set_trace_events(&mut self, value: bool) {
        self.trace_events = value;
    }

    pub fn set_trace_commands(&mut self, value: bool) {
        self.trace_commands = value;
    }

    pub fn set_trace_configuration(&mut self, value: bool) {
        self.trace_configuration = value;
    }

    pub fn set_trace_resources(&mut self, value: bool) {
        self.trace_resources = value;
    }

    pub fn set_trace_plugins(&mut self, value: bool) {
        self.trace_plugins = value;
    }

    pub fn set_trace_terminal_io(&mut self, value: bool) {
        self.trace_terminal_io = value;
    }

    pub fn set_collect_performance_metrics(&mut self, value: bool) {
        self.collect_performance_metrics = value;
    }

    pub fn set_collect_memory_metrics(&mut self, value: bool) {
        self.collect_memory_metrics = value;
    }

    pub fn set_collect_render_metrics(&mut self, value: bool) {
        self.collect_render_metrics = value;
    }

    pub fn set_enable_profiling(&mut self, value: bool) {
        self.enable_profiling = value;
    }

    pub fn set_profile_startup(&mut self, value: bool) {
        self.profile_startup = value;
    }

    pub fn set_profile_rendering(&mut self, value: bool) {
        self.profile_rendering = value;
    }

    pub fn set_profile_terminal_io(&mut self, value: bool) {
        self.profile_terminal_io = value;
    }

    pub fn set_expose_internal_commands(&mut self, value: bool) {
        self.expose_internal_commands = value;
    }

    pub fn set_enable_experimental_commands(&mut self, value: bool) {
        self.enable_experimental_commands = value;
    }

    pub fn set_allow_test_resources(&mut self, value: bool) {
        self.allow_test_resources = value;
    }

    pub fn set_keep_debug_logs(&mut self, value: bool) {
        self.keep_debug_logs = value;
    }

    pub fn set_debug_log_directory<S: Into<String>>(&mut self, value: S) {
        let value = value.into();

        if !value.trim().is_empty() {
            self.debug_log_directory = value;
        }
    }

    pub fn to_config(&self) -> BTreeMap<String, ConfigValue> {
        let mut values = BTreeMap::new();

        values.insert(
            "developer_mode".into(),
            ConfigValue::Boolean(self.developer_mode),
        );
        values.insert(
            "log_level".into(),
            ConfigValue::String(self.log_level.as_str().into()),
        );
        values.insert(
            "enable_debug_overlay".into(),
            ConfigValue::Boolean(self.enable_debug_overlay),
        );
        values.insert(
            "show_event_debugger".into(),
            ConfigValue::Boolean(self.show_event_debugger),
        );
        values.insert(
            "show_command_debugger".into(),
            ConfigValue::Boolean(self.show_command_debugger),
        );
        values.insert(
            "show_resource_debugger".into(),
            ConfigValue::Boolean(self.show_resource_debugger),
        );
        values.insert(
            "show_configuration_debugger".into(),
            ConfigValue::Boolean(self.show_configuration_debugger),
        );
        values.insert(
            "show_terminal_protocol_debugger".into(),
            ConfigValue::Boolean(self.show_terminal_protocol_debugger),
        );
        values.insert(
            "trace_events".into(),
            ConfigValue::Boolean(self.trace_events),
        );
        values.insert(
            "trace_commands".into(),
            ConfigValue::Boolean(self.trace_commands),
        );
        values.insert(
            "trace_configuration".into(),
            ConfigValue::Boolean(self.trace_configuration),
        );
        values.insert(
            "trace_resources".into(),
            ConfigValue::Boolean(self.trace_resources),
        );
        values.insert(
            "trace_plugins".into(),
            ConfigValue::Boolean(self.trace_plugins),
        );
        values.insert(
            "trace_terminal_io".into(),
            ConfigValue::Boolean(self.trace_terminal_io),
        );
        values.insert(
            "collect_performance_metrics".into(),
            ConfigValue::Boolean(self.collect_performance_metrics),
        );
        values.insert(
            "collect_memory_metrics".into(),
            ConfigValue::Boolean(self.collect_memory_metrics),
        );
        values.insert(
            "collect_render_metrics".into(),
            ConfigValue::Boolean(self.collect_render_metrics),
        );
        values.insert(
            "enable_profiling".into(),
            ConfigValue::Boolean(self.enable_profiling),
        );
        values.insert(
            "profile_startup".into(),
            ConfigValue::Boolean(self.profile_startup),
        );
        values.insert(
            "profile_rendering".into(),
            ConfigValue::Boolean(self.profile_rendering),
        );
        values.insert(
            "profile_terminal_io".into(),
            ConfigValue::Boolean(self.profile_terminal_io),
        );
        values.insert(
            "expose_internal_commands".into(),
            ConfigValue::Boolean(self.expose_internal_commands),
        );
        values.insert(
            "enable_experimental_commands".into(),
            ConfigValue::Boolean(self.enable_experimental_commands),
        );
        values.insert(
            "allow_test_resources".into(),
            ConfigValue::Boolean(self.allow_test_resources),
        );
        values.insert(
            "keep_debug_logs".into(),
            ConfigValue::Boolean(self.keep_debug_logs),
        );
        values.insert(
            "debug_log_directory".into(),
            ConfigValue::String(self.debug_log_directory.clone()),
        );

        values
    }
}
