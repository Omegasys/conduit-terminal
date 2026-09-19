//! Aggregated terminal state.
//!
//! This type provides a stable state model for the renderer, UI,
//! diagnostics system, event bus, and future session persistence.

use super::{
    Cursor,
    Scrollback,
};

/// High-level terminal mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalMode {
    Normal,
    AlternateScreen,
}

/// Terminal state snapshot.
#[derive(Debug, Clone)]
pub struct TerminalState {
    pub rows: usize,
    pub columns: usize,

    pub cursor: Cursor,
    pub scrollback: Scrollback,

    pub mode: TerminalMode,

    pub application_cursor_keys: bool,
    pub application_keypad: bool,
    pub bracketed_paste: bool,
    pub mouse_reporting: bool,

    pub title: String,

    pub focused: bool,
    pub bell_pending: bool,
}

impl TerminalState {
    pub fn new(rows: usize, columns: usize) -> Self {
        Self {
            rows: rows.max(1),
            columns: columns.max(1),

            cursor: Cursor::new(),
            scrollback: Scrollback::default(),

            mode: TerminalMode::Normal,

            application_cursor_keys: false,
            application_keypad: false,
            bracketed_paste: false,
            mouse_reporting: false,

            title: String::from("Conduit"),

            focused: true,
            bell_pending: false,
        }
    }

    pub fn resize(&mut self, rows: usize, columns: usize) {
        self.rows = rows.max(1);
        self.columns = columns.max(1);
        self.cursor.clamp(self.rows, self.columns);
    }

    pub fn enter_alternate_screen(&mut self) {
        self.mode = TerminalMode::AlternateScreen;
    }

    pub fn leave_alternate_screen(&mut self) {
        self.mode = TerminalMode::Normal;
    }

    pub fn is_alternate_screen(&self) -> bool {
        self.mode == TerminalMode::AlternateScreen
    }

    pub fn set_title<S: Into<String>>(&mut self, title: S) {
        self.title = title.into();
    }

    pub fn set_focus(&mut self, focused: bool) {
        self.focused = focused;
    }

    pub fn trigger_bell(&mut self) {
        self.bell_pending = true;
    }

    pub fn consume_bell(&mut self) -> bool {
        let pending = self.bell_pending;
        self.bell_pending = false;
        pending
    }

    pub fn reset(&mut self) {
        self.cursor.reset();
        self.mode = TerminalMode::Normal;

        self.application_cursor_keys = false;
        self.application_keypad = false;
        self.bracketed_paste = false;
        self.mouse_reporting = false;

        self.title = String::from("Conduit");
        self.bell_pending = false;
    }
}

impl Default for TerminalState {
    fn default() -> Self {
        Self::new(24, 80)
    }
}
