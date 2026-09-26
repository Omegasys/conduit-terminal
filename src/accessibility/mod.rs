//! Accessibility support for Conduit.
//!
//! This module provides platform-neutral accessibility state and behavior.
//! GUI and TUI frontends can consume the same accessibility configuration
//! without duplicating accessibility logic.

pub mod announcements;
pub mod font_scaling;
pub mod high_contrast;
pub mod keyboard_navigation;
pub mod reduced_motion;
pub mod screen_reader;

pub use announcements::{
    Announcement, AnnouncementKind, AnnouncementPriority, AnnouncementQueue,
};
pub use font_scaling::{FontScaling, FontScalingPreset};
pub use high_contrast::{ContrastMode, HighContrast};
pub use keyboard_navigation::{FocusDirection, KeyboardNavigation, NavigationAction};
pub use reduced_motion::ReducedMotion;
pub use screen_reader::{
    AccessibilityRole, AccessibilityState, ScreenReader, ScreenReaderNode,
};

/// Central accessibility configuration for a Conduit instance.
#[derive(Debug, Clone)]
pub struct AccessibilitySettings {
    pub screen_reader: ScreenReader,
    pub high_contrast: HighContrast,
    pub font_scaling: FontScaling,
    pub keyboard_navigation: KeyboardNavigation,
    pub reduced_motion: ReducedMotion,
    pub announcements: AnnouncementQueue,
}

impl Default for AccessibilitySettings {
    fn default() -> Self {
        Self {
            screen_reader: ScreenReader::default(),
            high_contrast: HighContrast::default(),
            font_scaling: FontScaling::default(),
            keyboard_navigation: KeyboardNavigation::default(),
            reduced_motion: ReducedMotion::default(),
            announcements: AnnouncementQueue::default(),
        }
    }
}

impl AccessibilitySettings {
    /// Returns whether any accessibility feature is currently active.
    pub fn enabled(&self) -> bool {
        self.screen_reader.enabled()
            || self.high_contrast.enabled()
            || self.font_scaling.is_scaled()
            || self.keyboard_navigation.enabled()
            || self.reduced_motion.enabled()
    }

    /// Reset all accessibility settings to their defaults.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Create a snapshot suitable for diagnostics or configuration UIs.
    pub fn snapshot(&self) -> AccessibilitySnapshot {
        AccessibilitySnapshot {
            enabled: self.enabled(),
            screen_reader: self.screen_reader.enabled(),
            high_contrast: self.high_contrast.enabled(),
            font_scale: self.font_scaling.scale(),
            keyboard_navigation: self.keyboard_navigation.enabled(),
            reduced_motion: self.reduced_motion.enabled(),
            pending_announcements: self.announcements.len(),
        }
    }
}

/// Read-only accessibility state for diagnostics and UI.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AccessibilitySnapshot {
    pub enabled: bool,
    pub screen_reader: bool,
    pub high_contrast: bool,
    pub font_scale: f32,
    pub keyboard_navigation: bool,
    pub reduced_motion: bool,
    pub pending_announcements: usize,
}
