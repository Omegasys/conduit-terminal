#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecKeyboardMode {
    Normal,
    Application,
    Cursor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecKeyboardProtocol {
    mode: DecKeyboardMode,
    application_keypad: bool,
}

impl Default for DecKeyboardProtocol {
    fn default() -> Self {
        Self {
            mode: DecKeyboardMode::Normal,
            application_keypad: false,
        }
    }
}

impl DecKeyboardProtocol {
    pub fn mode(&self) -> DecKeyboardMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: DecKeyboardMode) {
        self.mode = mode;
    }

    pub fn application_keypad(&self) -> bool {
        self.application_keypad
    }

    pub fn set_application_keypad(&mut self, enabled: bool) {
        self.application_keypad = enabled;
    }
}
