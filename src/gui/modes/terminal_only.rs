use super::{ModeState, UiElement, UiMode};

#[derive(Debug, Clone)]
pub struct TerminalOnlyMode {
    state: ModeState,
}

impl Default for TerminalOnlyMode {
    fn default() -> Self {
        Self::new()
    }
}

impl TerminalOnlyMode {
    pub fn new() -> Self {
        Self {
            state: ModeState::new(UiMode::TerminalOnly),
        }
    }

    pub fn state(&self) -> &ModeState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut ModeState {
        &mut self.state
    }

    pub fn enter(&mut self) {
        self.state.set_mode(UiMode::TerminalOnly);

        for element in [
            UiElement::MenuBar,
            UiElement::Toolbar,
            UiElement::Sidebar,
            UiElement::StatusBar,
            UiElement::FlowView,
            UiElement::Notifications,
        ] {
            self.state.set_visible(element, false);
        }

        self.state.set_visible(UiElement::Terminal, true);
        self.state.set_visible(UiElement::TabBar, true);
        self.state.set_visible(UiElement::SecurityIndicators, true);
    }

    pub fn is_active(&self) -> bool {
        self.state.mode() == UiMode::TerminalOnly
    }
}
