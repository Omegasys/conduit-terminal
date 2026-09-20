pub mod distraction_free;
pub mod full;
pub mod fullscreen;
pub mod minimal;
pub mod safe_mode;
pub mod terminal_only;

pub use distraction_free::DistractionFreeMode;
pub use full::FullMode;
pub use fullscreen::FullscreenMode;
pub use minimal::MinimalMode;
pub use safe_mode::SafeMode;
pub use terminal_only::TerminalOnlyMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiMode {
    Full,
    Minimal,
    Fullscreen,
    TerminalOnly,
    DistractionFree,
    SafeMode,
}

impl Default for UiMode {
    fn default() -> Self {
        Self::Full
    }
}

impl UiMode {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Full => "Full",
            Self::Minimal => "Minimal",
            Self::Fullscreen => "Fullscreen",
            Self::TerminalOnly => "Terminal Only",
            Self::DistractionFree => "Distraction Free",
            Self::SafeMode => "Safe Mode",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Full => "Full Conduit interface with all enabled UI components.",
            Self::Minimal => "Reduced interface with essential controls.",
            Self::Fullscreen => "Full-screen presentation with desktop decorations removed.",
            Self::TerminalOnly => "Terminal-focused interface with surrounding UI hidden.",
            Self::DistractionFree => "A clean workspace with nonessential interface elements hidden.",
            Self::SafeMode => "Restricted presentation and interaction mode for safer operation.",
        }
    }

    pub fn is_restricted(&self) -> bool {
        matches!(self, Self::SafeMode)
    }

    pub fn is_fullscreen(&self) -> bool {
        matches!(self, Self::Fullscreen)
    }

    pub fn is_terminal_focused(&self) -> bool {
        matches!(
            self,
            Self::TerminalOnly | Self::DistractionFree
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiElement {
    MenuBar,
    Toolbar,
    Sidebar,
    TabBar,
    StatusBar,
    CommandPalette,
    FlowView,
    Terminal,
    WindowDecorations,
    Notifications,
    SecurityIndicators,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementVisibility {
    Visible,
    Hidden,
}

#[derive(Debug, Clone)]
pub struct ModeState {
    mode: UiMode,
    previous_mode: Option<UiMode>,
    fullscreen: bool,
    elements: std::collections::BTreeMap<UiElement, ElementVisibility>,
}

impl ModeState {
    pub fn new(mode: UiMode) -> Self {
        let mut state = Self {
            mode,
            previous_mode: None,
            fullscreen: false,
            elements: std::collections::BTreeMap::new(),
        };

        state.apply_defaults();
        state
    }

    pub fn mode(&self) -> UiMode {
        self.mode
    }

    pub fn previous_mode(&self) -> Option<UiMode> {
        self.previous_mode
    }

    pub fn set_mode(&mut self, mode: UiMode) {
        if self.mode != mode {
            self.previous_mode = Some(self.mode);
            self.mode = mode;
            self.apply_defaults();
        }
    }

    pub fn restore_previous_mode(&mut self) -> bool {
        let Some(previous) = self.previous_mode.take() else {
            return false;
        };

        self.mode = previous;
        self.apply_defaults();
        true
    }

    pub fn is_visible(&self, element: UiElement) -> bool {
        self.elements
            .get(&element)
            .copied()
            .unwrap_or(ElementVisibility::Visible)
            == ElementVisibility::Visible
    }

    pub fn set_visible(&mut self, element: UiElement, visible: bool) {
        self.elements.insert(
            element,
            if visible {
                ElementVisibility::Visible
            } else {
                ElementVisibility::Hidden
            },
        );
    }

    pub fn fullscreen(&self) -> bool {
        self.fullscreen
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        self.fullscreen = fullscreen;
    }

    pub fn toggle_fullscreen(&mut self) {
        self.fullscreen = !self.fullscreen;
    }

    pub fn elements(
        &self,
    ) -> impl Iterator<Item = (&UiElement, &ElementVisibility)> {
        self.elements.iter()
    }

    fn apply_defaults(&mut self) {
        use UiElement::*;

        self.elements.clear();

        for element in [
            MenuBar,
            Toolbar,
            Sidebar,
            TabBar,
            StatusBar,
            CommandPalette,
            FlowView,
            Terminal,
            WindowDecorations,
            Notifications,
            SecurityIndicators,
        ] {
            self.set_visible(element, true);
        }

        self.fullscreen = matches!(self.mode, UiMode::Fullscreen);

        match self.mode {
            UiMode::Full => {}

            UiMode::Minimal => {
                self.set_visible(Toolbar, false);
                self.set_visible(Sidebar, false);
                self.set_visible(StatusBar, false);
                self.set_visible(FlowView, false);
            }

            UiMode::Fullscreen => {
                self.set_visible(WindowDecorations, false);
                self.set_visible(MenuBar, false);
                self.set_visible(Toolbar, false);
            }

            UiMode::TerminalOnly => {
                self.set_visible(MenuBar, false);
                self.set_visible(Toolbar, false);
                self.set_visible(Sidebar, false);
                self.set_visible(StatusBar, false);
                self.set_visible(FlowView, false);
                self.set_visible(Notifications, false);
            }

            UiMode::DistractionFree => {
                self.set_visible(MenuBar, false);
                self.set_visible(Toolbar, false);
                self.set_visible(Sidebar, false);
                self.set_visible(StatusBar, false);
                self.set_visible(FlowView, false);
            }

            UiMode::SafeMode => {
                self.set_visible(Toolbar, false);
                self.set_visible(FlowView, false);
                self.set_visible(CommandPalette, false);
            }
        }
    }
}

impl Default for ModeState {
    fn default() -> Self {
        Self::new(UiMode::Full)
    }
}
