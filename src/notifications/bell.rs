use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BellAction {
    None,
    Audible,
    Visual,
    Both,
}

#[derive(Debug, Clone, Copy)]
pub struct BellConfig {
    pub enabled: bool,
    pub action: BellAction,
    pub cooldown: Duration,
}

impl Default for BellConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            action: BellAction::Audible,
            cooldown: Duration::from_millis(100),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BellHandler {
    config: BellConfig,
    last_ring: Option<Instant>,
    ring_count: u64,
}

impl BellHandler {
    pub fn new(config: BellConfig) -> Self {
        Self {
            config,
            last_ring: None,
            ring_count: 0,
        }
    }

    pub fn ring(&mut self) -> bool {
        if !self.config.enabled {
            return false;
        }

        let now = Instant::now();

        if let Some(last) = self.last_ring {
            if now.duration_since(last) < self.config.cooldown {
                return false;
            }
        }

        self.last_ring = Some(now);
        self.ring_count = self.ring_count.saturating_add(1);

        true
    }

    pub fn update(&mut self) {}

    pub fn config(&self) -> BellConfig {
        self.config
    }

    pub fn set_config(&mut self, config: BellConfig) {
        self.config = config;
    }

    pub fn enabled(&self) -> bool {
        self.config.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.config.enabled = enabled;
    }

    pub fn action(&self) -> BellAction {
        self.config.action
    }

    pub fn ring_count(&self) -> u64 {
        self.ring_count
    }
}
