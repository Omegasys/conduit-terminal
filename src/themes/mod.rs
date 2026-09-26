//! Conduit theme system.
//!
//! Themes define colors, typography, and visual appearance for the GUI,
//! TUI, terminal renderer, and other frontends.

pub mod appearance;
pub mod colors;
pub mod fonts;
pub mod loader;
pub mod manager;
pub mod theme;
pub mod watcher;

pub use appearance::{Appearance, AppearanceMode};
pub use colors::{AnsiPalette, Color, ColorPalette};
pub use fonts::{FontFamily, FontSet, FontStyle, FontWeight};
pub use loader::{ThemeLoadError, ThemeLoader};
pub use manager::{ThemeError, ThemeManager};
pub use theme::{Theme, ThemeId, ThemeMetadata};
pub use watcher::{ThemeChange, ThemeWatcher};

/// Result type used by the theme subsystem.
pub type ThemeResult<T> = Result<T, ThemeError>;
