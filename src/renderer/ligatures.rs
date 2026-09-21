/// Ligature rendering mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LigatureMode {
    Disabled,
    Standard,
    Contextual,
    Discretionary,
    All,
}

impl Default for LigatureMode {
    fn default() -> Self {
        Self::Standard
    }
}

/// A sequence that may be rendered as a ligature.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ligature {
    sequence: String,
    replacement: String,
    enabled: bool,
}

impl Ligature {
    pub fn new<S1, S2>(sequence: S1, replacement: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            sequence: sequence.into(),
            replacement: replacement.into(),
            enabled: true,
        }
    }

    pub fn sequence(&self) -> &str {
        &self.sequence
    }

    pub fn replacement(&self) -> &str {
        &self.replacement
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

/// Ligature configuration for terminal text.
#[derive(Debug, Clone)]
pub struct LigatureSettings {
    mode: LigatureMode,
    custom: Vec<Ligature>,
    allow_calt: bool,
    allow_liga: bool,
    allow_clig: bool,
    preserve_cell_width: bool,
}

impl Default for LigatureSettings {
    fn default() -> Self {
        Self {
            mode: LigatureMode::Standard,
            custom: Vec::new(),
            allow_calt: true,
            allow_liga: true,
            allow_clig: true,
            preserve_cell_width: true,
        }
    }
}

impl LigatureSettings {
    pub fn mode(&self) -> LigatureMode {
        self.mode
    }

    pub fn custom(&self) -> &[Ligature] {
        &self.custom
    }

    pub fn allow_calt(&self) -> bool {
        self.allow_calt
    }

    pub fn allow_liga(&self) -> bool {
        self.allow_liga
    }

    pub fn allow_clig(&self) -> bool {
        self.allow_clig
    }

    pub fn preserve_cell_width(&self) -> bool {
        self.preserve_cell_width
    }

    pub fn set_mode(&mut self, mode: LigatureMode) {
        self.mode = mode;
    }

    pub fn set_allow_calt(&mut self, enabled: bool) {
        self.allow_calt = enabled;
    }

    pub fn set_allow_liga(&mut self, enabled: bool) {
        self.allow_liga = enabled;
    }

    pub fn set_allow_clig(&mut self, enabled: bool) {
        self.allow_clig = enabled;
    }

    pub fn set_preserve_cell_width(&mut self, enabled: bool) {
        self.preserve_cell_width = enabled;
    }

    pub fn add_custom(&mut self, ligature: Ligature) {
        self.custom.push(ligature);
    }

    pub fn remove_custom(&mut self, sequence: &str) {
        self.custom.retain(|item| item.sequence() != sequence);
    }

    pub fn clear_custom(&mut self) {
        self.custom.clear();
    }

    pub fn enabled(&self) -> bool {
        self.mode != LigatureMode::Disabled
    }
}
