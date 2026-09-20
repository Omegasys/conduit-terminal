use super::{ModeState, UiElement, UiMode};

#[derive(Debug, Clone)]
pub struct FullMode {
    state: ModeState,
}

impl Default for FullMode {
    fn default() -> Self {
        Self::new()
    }
}

impl FullMode {
    pub fn new() -> Self {
        Self {
            state: ModeState::new(UiMode::Full),
        }
    }

    pub fn state(&self) -> &ModeState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut ModeState {
        &mut self.state
    }

    pub fn enter(&mut self) {
        self.state.set_mode(UiMode::Full);
        self.state.set_fullscreen(false);

        for element in [
            UiElement::MenuBar,
            UiElement::Toolbar,
            UiElement::Sidebar,
            UiElement::TabBar,
            UiElement::StatusBar,
            UiElement::CommandPalette,
            UiElement::FlowView,
            UiElement::Terminal,
            UiElement::WindowDecorations,
            UiElement::Notifications,
            UiElement::SecurityIndicators,
        ] {
            self.state.set_visible(element, true);
        }
    }

    pub fn is_active(&self) -> bool {
        self.state.mode() == UiMode::Full
    }
}
