use super::{ModeState, UiElement, UiMode};

#[derive(Debug, Clone)]
pub struct FullscreenMode {
    state: ModeState,
}

impl Default for FullscreenMode {
    fn default() -> Self {
        Self::new()
    }
}

impl FullscreenMode {
    pub fn new() -> Self {
        Self {
            state: ModeState::new(UiMode::Fullscreen),
        }
    }

    pub fn state(&self) -> &ModeState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut ModeState {
        &mut self.state
    }

    pub fn enter(&mut self) {
        self.state.set_mode(UiMode::Fullscreen);
        self.state.set_fullscreen(true);

        self.state.set_visible(UiElement::WindowDecorations, false);
        self.state.set_visible(UiElement::MenuBar, false);
        self.state.set_visible(UiElement::Toolbar, false);
        self.state.set_visible(UiElement::Terminal, true);
        self.state.set_visible(UiElement::TabBar, true);
    }

    pub fn exit(&mut self) {
        self.state.set_fullscreen(false);
        self.state.set_mode(UiMode::Full);
    }

    pub fn is_active(&self) -> bool {
        self.state.mode() == UiMode::Fullscreen
            && self.state.fullscreen()
    }
}
