use super::{ModeState, UiElement, UiMode};

#[derive(Debug, Clone)]
pub struct MinimalMode {
    state: ModeState,
}

impl Default for MinimalMode {
    fn default() -> Self {
        Self::new()
    }
}

impl MinimalMode {
    pub fn new() -> Self {
        Self {
            state: ModeState::new(UiMode::Minimal),
        }
    }

    pub fn state(&self) -> &ModeState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut ModeState {
        &mut self.state
    }

    pub fn enter(&mut self) {
        self.state.set_mode(UiMode::Minimal);

        self.state.set_visible(UiElement::MenuBar, true);
        self.state.set_visible(UiElement::TabBar, true);
        self.state.set_visible(UiElement::Terminal, true);
        self.state.set_visible(UiElement::SecurityIndicators, true);

        self.state.set_visible(UiElement::Toolbar, false);
        self.state.set_visible(UiElement::Sidebar, false);
        self.state.set_visible(UiElement::StatusBar, false);
        self.state.set_visible(UiElement::FlowView, false);
    }

    pub fn is_active(&self) -> bool {
        self.state.mode() == UiMode::Minimal
    }
}
