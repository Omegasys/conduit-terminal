//! Reduced-motion accessibility preferences.

/// Controls animated UI behavior.
#[derive(Debug, Clone, Copy)]
pub struct ReducedMotion {
    enabled: bool,
    disable_transitions: bool,
    disable_cursor_animation: bool,
    disable_blinking: bool,
    shorten_scroll_animations: bool,
}

impl Default for ReducedMotion {
    fn default() -> Self {
        Self {
            enabled: false,
            disable_transitions: false,
            disable_cursor_animation: false,
            disable_blinking: false,
            shorten_scroll_animations: false,
        }
    }
}

impl ReducedMotion {
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        self.apply_defaults();
    }

    pub fn enable(&mut self) {
        self.set_enabled(true);
    }

    pub fn disable(&mut self) {
        self.set_enabled(false);
    }

    fn apply_defaults(&mut self) {
        if self.enabled {
            self.disable_transitions = true;
            self.disable_cursor_animation = true;
            self.disable_blinking = true;
            self.shorten_scroll_animations = true;
        } else {
            self.disable_transitions = false;
            self.disable_cursor_animation = false;
            self.disable_blinking = false;
            self.shorten_scroll_animations = false;
        }
    }

    pub fn disable_transitions(&self) -> bool {
        self.disable_transitions
    }

    pub fn disable_cursor_animation(&self) -> bool {
        self.disable_cursor_animation
    }

    pub fn disable_blinking(&self) -> bool {
        self.disable_blinking
    }

    pub fn shorten_scroll_animations(&self) -> bool {
        self.shorten_scroll_animations
    }

    /// Returns a duration suitable for an animation.
    ///
    /// A zero duration means the frontend should perform the transition
    /// immediately.
    pub fn animation_duration(&self, normal_ms: u64) -> u64 {
        if !self.enabled {
            normal_ms
        } else if self.disable_transitions {
            0
        } else {
            normal_ms.min(50)
        }
    }
}
