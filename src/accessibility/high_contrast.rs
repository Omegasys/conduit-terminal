//! High-contrast display support.

/// High-contrast rendering modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContrastMode {
    Normal,
    High,
    ExtraHigh,
}

impl Default for ContrastMode {
    fn default() -> Self {
        Self::Normal
    }
}

impl ContrastMode {
    pub fn enabled(self) -> bool {
        self != Self::Normal
    }
}

/// High-contrast configuration.
#[derive(Debug, Clone, Copy)]
pub struct HighContrast {
    mode: ContrastMode,
    minimum_ratio: f32,
    force_borders: bool,
    suppress_transparency: bool,
}

impl Default for HighContrast {
    fn default() -> Self {
        Self {
            mode: ContrastMode::Normal,
            minimum_ratio: 4.5,
            force_borders: false,
            suppress_transparency: false,
        }
    }
}

impl HighContrast {
    pub fn mode(&self) -> ContrastMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: ContrastMode) {
        self.mode = mode;

        match mode {
            ContrastMode::Normal => {
                self.minimum_ratio = 4.5;
                self.force_borders = false;
                self.suppress_transparency = false;
            }
            ContrastMode::High => {
                self.minimum_ratio = 7.0;
                self.force_borders = true;
                self.suppress_transparency = true;
            }
            ContrastMode::ExtraHigh => {
                self.minimum_ratio = 10.0;
                self.force_borders = true;
                self.suppress_transparency = true;
            }
        }
    }

    pub fn enable(&mut self) {
        self.set_mode(ContrastMode::High);
    }

    pub fn disable(&mut self) {
        self.set_mode(ContrastMode::Normal);
    }

    pub fn enabled(&self) -> bool {
        self.mode.enabled()
    }

    pub fn minimum_ratio(&self) -> f32 {
        self.minimum_ratio
    }

    pub fn force_borders(&self) -> bool {
        self.force_borders
    }

    pub fn suppress_transparency(&self) -> bool {
        self.suppress_transparency
    }

    /// Returns whether a contrast ratio meets the current requirement.
    pub fn meets_ratio(&self, ratio: f32) -> bool {
        ratio >= self.minimum_ratio
    }
}
