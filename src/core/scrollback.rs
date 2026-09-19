//! Terminal scrollback storage.
//!
//! Scrollback is deliberately separate from the visible screen buffer.
//! This allows the renderer and terminal screen to remain bounded while
//! preserving a configurable history of previous terminal output.

/// A line stored in scrollback.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScrollbackLine {
    pub text: String,
}

impl ScrollbackLine {
    pub fn new<S: Into<String>>(text: S) -> Self {
        Self { text: text.into() }
    }
}

/// Scrollback buffer.
#[derive(Debug, Clone)]
pub struct Scrollback {
    lines: Vec<ScrollbackLine>,
    capacity: usize,
    viewport: usize,
}

impl Scrollback {
    /// Creates a scrollback buffer.
    ///
    /// A capacity of zero means scrollback is disabled.
    pub fn new(capacity: usize) -> Self {
        Self {
            lines: Vec::with_capacity(capacity.min(1024)),
            capacity,
            viewport: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.lines.len()
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn lines(&self) -> &[ScrollbackLine] {
        &self.lines
    }

    pub fn viewport(&self) -> usize {
        self.viewport
    }

    /// Adds a completed line to scrollback.
    pub fn push<S: Into<String>>(&mut self, text: S) {
        if self.capacity == 0 {
            return;
        }

        self.lines.push(ScrollbackLine::new(text));

        while self.lines.len() > self.capacity {
            self.lines.remove(0);
        }

        self.viewport = 0;
    }

    /// Returns a line counting backward from the newest line.
    pub fn from_end(&self, offset: usize) -> Option<&ScrollbackLine> {
        if offset >= self.lines.len() {
            return None;
        }

        self.lines.get(self.lines.len() - 1 - offset)
    }

    /// Scrolls upward by the requested number of lines.
    pub fn scroll_up(&mut self, amount: usize) {
        self.viewport = self
            .viewport
            .saturating_add(amount)
            .min(self.max_viewport());
    }

    /// Scrolls downward toward the live terminal.
    pub fn scroll_down(&mut self, amount: usize) {
        self.viewport = self.viewport.saturating_sub(amount);
    }

    /// Returns to the live terminal position.
    pub fn scroll_to_bottom(&mut self) {
        self.viewport = 0;
    }

    /// Removes all stored history.
    pub fn clear(&mut self) {
        self.lines.clear();
        self.viewport = 0;
    }

    /// Changes the maximum number of stored lines.
    pub fn set_capacity(&mut self, capacity: usize) {
        self.capacity = capacity;

        if capacity == 0 {
            self.lines.clear();
            self.viewport = 0;
            return;
        }

        if self.lines.len() > capacity {
            let remove_count = self.lines.len() - capacity;
            self.lines.drain(0..remove_count);
        }

        self.viewport = self.viewport.min(self.max_viewport());
    }

    fn max_viewport(&self) -> usize {
        self.lines.len().saturating_sub(1)
    }
}

impl Default for Scrollback {
    fn default() -> Self {
        Self::new(10_000)
    }
}
