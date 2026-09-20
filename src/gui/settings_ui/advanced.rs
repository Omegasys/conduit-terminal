use crate::config_engine::ConfigValue;
use std::collections::BTreeMap;

/// Controls how aggressively Conduit reloads components.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReloadMode {
    Disabled,
    Manual,
    Automatic,
    Aggressive,
}

impl ReloadMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::Manual => "manual",
            Self::Automatic => "automatic",
            Self::Aggressive => "aggressive",
        }
    }
}

/// Advanced runtime configuration.
#[derive(Debug, Clone)]
pub struct AdvancedSettings {
    reload_mode: ReloadMode,
    hot_reload_resources: bool,
    hot_reload_configuration: bool,
    hot_reload_themes: bool,
    hot_reload_plugins: bool,
    watch_configuration_files: bool,
    preserve_state_on_reload: bool,
    automatic_recovery: bool,
    recovery_attempts: u32,
    recovery_delay_ms: u64,
    worker_threads: u32,
    io_threads: u32,
    max_event_queue: usize,
    max_concurrent_sessions: usize,
    cache_enabled: bool,
    cache_size_mb: u32,
    cache_ttl_seconds: u64,
    preallocate_resources: bool,
    lazy_loading: bool,
    startup_parallelism: bool,
    experimental_features: bool,
    developer_features: bool,
    strict_configuration: bool,
}

impl Default for AdvancedSettings {
    fn default() -> Self {
        Self {
            reload_mode: ReloadMode::Automatic,
            hot_reload_resources: true,
            hot_reload_configuration: true,
            hot_reload_themes: true,
            hot_reload_plugins: true,
            watch_configuration_files: true,
            preserve_state_on_reload: true,
            automatic_recovery: true,
            recovery_attempts: 3,
            recovery_delay_ms: 1000,
            worker_threads: 0,
            io_threads: 2,
            max_event_queue: 10_000,
            max_concurrent_sessions: 64,
            cache_enabled: true,
            cache_size_mb: 256,
            cache_ttl_seconds: 3600,
            preallocate_resources: false,
            lazy_loading: true,
            startup_parallelism: true,
            experimental_features: false,
            developer_features: false,
            strict_configuration: false,
        }
    }
}

impl AdvancedSettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reload_mode(&self) -> ReloadMode {
        self.reload_mode
    }

    pub fn hot_reload_resources(&self) -> bool {
        self.hot_reload_resources
    }

    pub fn hot_reload_configuration(&self) -> bool {
        self.hot_reload_configuration
    }

    pub fn hot_reload_themes(&self) -> bool {
        self.hot_reload_themes
    }

    pub fn hot_reload_plugins(&self) -> bool {
        self.hot_reload_plugins
    }

    pub fn watch_configuration_files(&self) -> bool {
        self.watch_configuration_files
    }

    pub fn preserve_state_on_reload(&self) -> bool {
        self.preserve_state_on_reload
    }

    pub fn automatic_recovery(&self) -> bool {
        self.automatic_recovery
    }

    pub fn recovery_attempts(&self) -> u32 {
        self.recovery_attempts
    }

    pub fn recovery_delay_ms(&self) -> u64 {
        self.recovery_delay_ms
    }

    pub fn worker_threads(&self) -> u32 {
        self.worker_threads
    }

    pub fn io_threads(&self) -> u32 {
        self.io_threads
    }

    pub fn max_event_queue(&self) -> usize {
        self.max_event_queue
    }

    pub fn max_concurrent_sessions(&self) -> usize {
        self.max_concurrent_sessions
    }

    pub fn cache_enabled(&self) -> bool {
        self.cache_enabled
    }

    pub fn cache_size_mb(&self) -> u32 {
        self.cache_size_mb
    }

    pub fn cache_ttl_seconds(&self) -> u64 {
        self.cache_ttl_seconds
    }

    pub fn preallocate_resources(&self) -> bool {
        self.preallocate_resources
    }

    pub fn lazy_loading(&self) -> bool {
        self.lazy_loading
    }

    pub fn startup_parallelism(&self) -> bool {
        self.startup_parallelism
    }

    pub fn experimental_features(&self) -> bool {
        self.experimental_features
    }

    pub fn developer_features(&self) -> bool {
        self.developer_features
    }

    pub fn strict_configuration(&self) -> bool {
        self.strict_configuration
    }

    pub fn set_reload_mode(&mut self, value: ReloadMode) {
        self.reload_mode = value;
    }

    pub fn set_hot_reload_resources(&mut self, value: bool) {
        self.hot_reload_resources = value;
    }

    pub fn set_hot_reload_configuration(&mut self, value: bool) {
        self.hot_reload_configuration = value;
    }

    pub fn set_hot_reload_themes(&mut self, value: bool) {
        self.hot_reload_themes = value;
    }

    pub fn set_hot_reload_plugins(&mut self, value: bool) {
        self.hot_reload_plugins = value;
    }

    pub fn set_watch_configuration_files(&mut self, value: bool) {
        self.watch_configuration_files = value;
    }

    pub fn set_preserve_state_on_reload(&mut self, value: bool) {
        self.preserve_state_on_reload = value;
    }

    pub fn set_automatic_recovery(&mut self, value: bool) {
        self.automatic_recovery = value;
    }

    pub fn set_recovery_attempts(&mut self, value: u32) {
        self.recovery_attempts = value.clamp(1, 20);
    }

    pub fn set_recovery_delay_ms(&mut self, value: u64) {
        self.recovery_delay_ms = value.clamp(100, 60_000);
    }

    pub fn set_worker_threads(&mut self, value: u32) {
        self.worker_threads = value.min(256);
    }

    pub fn set_io_threads(&mut self, value: u32) {
        self.io_threads = value.clamp(1, 128);
    }

    pub fn set_max_event_queue(&mut self, value: usize) {
        self.max_event_queue = value.clamp(100, 1_000_000);
    }

    pub fn set_max_concurrent_sessions(&mut self, value: usize) {
        self.max_concurrent_sessions = value.clamp(1, 4096);
    }

    pub fn set_cache_enabled(&mut self, value: bool) {
        self.cache_enabled = value;
    }

    pub fn set_cache_size_mb(&mut self, value: u32) {
        self.cache_size_mb = value.clamp(16, 16_384);
    }

    pub fn set_cache_ttl_seconds(&mut self, value: u64) {
        self.cache_ttl_seconds = value.clamp(60, 604_800);
    }

    pub fn set_preallocate_resources(&mut self, value: bool) {
        self.preallocate_resources = value;
    }

    pub fn set_lazy_loading(&mut self, value: bool) {
        self.lazy_loading = value;
    }

    pub fn set_startup_parallelism(&mut self, value: bool) {
        self.startup_parallelism = value;
    }

    pub fn set_experimental_features(&mut self, value: bool) {
        self.experimental_features = value;
    }

    pub fn set_developer_features(&mut self, value: bool) {
        self.developer_features = value;
    }

    pub fn set_strict_configuration(&mut self, value: bool) {
        self.strict_configuration = value;
    }

    pub fn to_config(&self) -> BTreeMap<String, ConfigValue> {
        let mut values = BTreeMap::new();

        values.insert(
            "reload_mode".into(),
            ConfigValue::String(self.reload_mode.as_str().into()),
        );
        values.insert(
            "hot_reload_resources".into(),
            ConfigValue::Boolean(self.hot_reload_resources),
        );
        values.insert(
            "hot_reload_configuration".into(),
            ConfigValue::Boolean(self.hot_reload_configuration),
        );
        values.insert(
            "hot_reload_themes".into(),
            ConfigValue::Boolean(self.hot_reload_themes),
        );
        values.insert(
            "hot_reload_plugins".into(),
            ConfigValue::Boolean(self.hot_reload_plugins),
        );
        values.insert(
            "watch_configuration_files".into(),
            ConfigValue::Boolean(self.watch_configuration_files),
        );
        values.insert(
            "preserve_state_on_reload".into(),
            ConfigValue::Boolean(self.preserve_state_on_reload),
        );
        values.insert(
            "automatic_recovery".into(),
            ConfigValue::Boolean(self.automatic_recovery),
        );
        values.insert(
            "recovery_attempts".into(),
            ConfigValue::Integer(self.recovery_attempts as i64),
        );
        values.insert(
            "recovery_delay_ms".into(),
            ConfigValue::Integer(self.recovery_delay_ms as i64),
        );
        values.insert(
            "worker_threads".into(),
            ConfigValue::Integer(self.worker_threads as i64),
        );
        values.insert(
            "io_threads".into(),
            ConfigValue::Integer(self.io_threads as i64),
        );
        values.insert(
            "max_event_queue".into(),
            ConfigValue::Integer(self.max_event_queue as i64),
        );
        values.insert(
            "max_concurrent_sessions".into(),
            ConfigValue::Integer(self.max_concurrent_sessions as i64),
        );
        values.insert(
            "cache_enabled".into(),
            ConfigValue::Boolean(self.cache_enabled),
        );
        values.insert(
            "cache_size_mb".into(),
            ConfigValue::Integer(self.cache_size_mb as i64),
        );
        values.insert(
            "cache_ttl_seconds".into(),
            ConfigValue::Integer(self.cache_ttl_seconds as i64),
        );
        values.insert(
            "preallocate_resources".into(),
            ConfigValue::Boolean(self.preallocate_resources),
        );
        values.insert(
            "lazy_loading".into(),
            ConfigValue::Boolean(self.lazy_loading),
        );
        values.insert(
            "startup_parallelism".into(),
            ConfigValue::Boolean(self.startup_parallelism),
        );
        values.insert(
            "experimental_features".into(),
            ConfigValue::Boolean(self.experimental_features),
        );
        values.insert(
            "developer_features".into(),
            ConfigValue::Boolean(self.developer_features),
        );
        values.insert(
            "strict_configuration".into(),
            ConfigValue::Boolean(self.strict_configuration),
        );

        values
    }
}
