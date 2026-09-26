//! Keyboard-only navigation support.

/// Direction used when moving keyboard focus.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusDirection {
    Previous,
    Next,
    Up,
    Down,
    Left,
    Right,
    First,
    Last,
}

/// High-level keyboard navigation action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationAction {
    Move(FocusDirection),
    Activate,
    Cancel,
    Select,
    Toggle,
    Expand,
    Collapse,
    PageUp,
    PageDown,
    Home,
    End,
}

/// Keyboard navigation configuration.
#[derive(Debug, Clone, Copy)]
pub struct KeyboardNavigation {
    enabled: bool,
    wrap: bool,
    focus_indicators: bool,
    tab_moves_focus: bool,
    escape_clears_focus: bool,
}

impl Default for KeyboardNavigation {
    fn default() -> Self {
        Self {
            enabled: true,
            wrap: true,
            focus_indicators: true,
            tab_moves_focus: true,
            escape_clears_focus: false,
        }
    }
}

impl KeyboardNavigation {
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn wrap(&self) -> bool {
        self.wrap
    }

    pub fn set_wrap(&mut self, wrap: bool) {
        self.wrap = wrap;
    }

    pub fn focus_indicators(&self) -> bool {
        self.focus_indicators
    }

    pub fn set_focus_indicators(&mut self, enabled: bool) {
        self.focus_indicators = enabled;
    }

    pub fn tab_moves_focus(&self) -> bool {
        self.tab_moves_focus
    }

    pub fn set_tab_moves_focus(&mut self, enabled: bool) {
        self.tab_moves_focus = enabled;
    }

    pub fn escape_clears_focus(&self) -> bool {
        self.escape_clears_focus
    }

    pub fn set_escape_clears_focus(&mut self, enabled: bool) {
        self.escape_clears_focus = enabled;
    }

    /// Translate a logical key name into a navigation action.
    ///
    /// Frontends can translate their toolkit-specific key events into these
    /// names before handing them to the accessibility layer.
    pub fn action_for_key(&self, key: &str) -> Option<NavigationAction> {
        if !self.enabled {
            return None;
        }

        match key {
            "Tab" if self.tab_moves_focus => {
                Some(NavigationAction::Move(FocusDirection::Next))
            }
            "Shift+Tab" if self.tab_moves_focus => {
                Some(NavigationAction::Move(FocusDirection::Previous))
            }
            "Up" => Some(NavigationAction::Move(FocusDirection::Up)),
            "Down" => Some(NavigationAction::Move(FocusDirection::Down)),
            "Left" => Some(NavigationAction::Move(FocusDirection::Left)),
            "Right" => Some(NavigationAction::Move(FocusDirection::Right)),
            "Home" => Some(NavigationAction::Home),
            "End" => Some(NavigationAction::End),
            "PageUp" => Some(NavigationAction::PageUp),
            "PageDown" => Some(NavigationAction::PageDown),
            "Enter" | "Space" => Some(NavigationAction::Activate),
            "Escape" if self.escape_clears_focus => Some(NavigationAction::Cancel),
            _ => None,
        }
    }
}
