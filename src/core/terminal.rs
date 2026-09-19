//! High-level terminal state.
//!
//! `Terminal` sits between the session/PTY and the logical screen.
//! Future protocol implementations can feed parsed terminal events
//! into this layer.

use super::Screen;

/// High-level terminal state.
pub struct Terminal {
    screen: Screen,

    /// Whether the alternate screen buffer is currently active.
    alternate_screen: bool,

    /// Whether bracketed paste mode is enabled.
    bracketed_paste: bool,

    /// Whether mouse reporting is enabled.
    mouse_reporting: bool,

    /// Current terminal title.
    title: String,
}

impl Terminal {
    /// Creates a terminal around an existing screen.
    pub fn new(screen: Screen) -> Self {
        Self {
            screen,
            alternate_screen: false,
            bracketed_paste: false,
            mouse_reporting: false,
            title: String::from("Conduit"),

        }
    }

    /// Processes output received from the PTY.
    ///
    /// This currently delegates to the bootstrap screen parser.
    /// The dedicated protocol parser can later replace this path
    /// while preserving the public Terminal interface.
    pub fn process_output(&mut self, data: &[u8]) {
        self.screen.process_bytes(data);
    }

    /// Returns the current screen.
    pub fn screen(&self) -> &Screen {
        &self.screen
    }

    /// Returns mutable screen state.
    pub fn screen_mut(&mut self) -> &mut Screen {
        &mut self.screen
    }

    /// Resizes the terminal screen.
    pub fn resize(&mut self, rows: usize, columns: usize) {
        self.screen.resize(rows, columns);
    }

    /// Returns whether the alternate screen is active.
    pub fn alternate_screen(&self) -> bool {
        self.alternate_screen
    }

    /// Sets alternate-screen state.
    pub fn set_alternate_screen(&mut self, enabled: bool) {
        self.alternate_screen = enabled;
    }

    /// Returns whether bracketed paste mode is active.
    pub fn bracketed_paste(&self) -> bool {
        self.bracketed_paste
    }

    /// Enables or disables bracketed paste mode.
    pub fn set_bracketed_paste(&mut self, enabled: bool) {
        self.bracketed_paste = enabled;
    }

    /// Returns whether mouse reporting is enabled.
    pub fn mouse_reporting(&self) -> bool {
        self.mouse_reporting
    }

    /// Enables or disables mouse reporting.
    pub fn set_mouse_reporting(&mut self, enabled: bool) {
        self.mouse_reporting = enabled;
    }

    /// Returns the terminal title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Changes the terminal title.
    pub fn set_title<S: Into<String>>(&mut self, title: S) {
        self.title = title.into();
    }
}
