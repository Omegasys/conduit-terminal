use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DecMode {
    ApplicationCursorKeys,
    Origin,
    AutoWrap,
    CursorVisible,
    ReverseVideo,
    BracketedPaste,
    FocusReporting,
    Mouse1000,
    Mouse1002,
    Mouse1003,
    Mouse1006,
    SynchronizedOutput,
    AlternateScreen,
    SaveCursor,
    Unknown(u16),
}

impl DecMode {
    pub fn from_number(number: u16) -> Self {
        match number {
            1 => Self::ApplicationCursorKeys,
            6 => Self::Origin,
            7 => Self::AutoWrap,
            25 => Self::CursorVisible,
            5 => Self::ReverseVideo,
            2004 => Self::BracketedPaste,
            1004 => Self::FocusReporting,
            1000 => Self::Mouse1000,
            1002 => Self::Mouse1002,
            1003 => Self::Mouse1003,
            1006 => Self::Mouse1006,
            2026 => Self::SynchronizedOutput,
            1049 => Self::AlternateScreen,
            1048 => Self::SaveCursor,
            other => Self::Unknown(other),
        }
    }
}

#[derive(Debug, Default)]
pub struct DecModeManager {
    modes: HashMap<DecMode, bool>,
}

impl DecModeManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, mode: DecMode, enabled: bool) {
        self.modes.insert(mode, enabled);
    }

    pub fn enable(&mut self, mode: DecMode) {
        self.set(mode, true);
    }

    pub fn disable(&mut self, mode: DecMode) {
        self.set(mode, false);
    }

    pub fn is_enabled(&self, mode: DecMode) -> bool {
        self.modes.get(&mode).copied().unwrap_or(false)
    }

    pub fn all(&self) -> &HashMap<DecMode, bool> {
        &self.modes
    }

    pub fn reset(&mut self) {
        self.modes.clear();
    }
}
