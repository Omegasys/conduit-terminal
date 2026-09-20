use super::{ModeState, UiElement, UiMode};

#[derive(Debug, Clone)]
pub struct DistractionFreeMode {
    state: ModeState,
    hide_tabs: bool,
    hide_notifications: bool,
}

impl Default for DistractionFreeMode {
    fn default() -> Self {
        Self::new()
    }
}

impl DistractionFreeMode {
    pub fn new() -> Self {
        Self {
            state: ModeState::new(UiMode::DistractionFree),
            hide_tabs: false,
            hide_notifications: true,
        }
    }

    pub fn state(&self) -> &ModeState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut ModeState {
        &mut self.state
    }

    pub fn enter(&mut self) {
        self.state.set_mode(UiMode::DistractionFree);

        self.state.set_visible(UiElement::MenuBar, false);
        self.state.set_visible(UiElement::Toolbar, false);
        self.state.set_visible(UiElement::Sidebar, false);
        self.state.set_visible(UiElement::StatusBar, false);
        self.state.set_visible(UiElement::FlowView, false);
        self.state.set_visible(
            UiElement::Notifications,
            !self.hide_notifications,
        );

        self.state.set_visible(
            UiElement::TabBar,
            !self.hide_tabs,
        );

        self.state.set_visible(UiElement::Terminal, true);
        self.state.set_visible(UiElement::SecurityIndicators, true);
    }

    pub fn hide_tabs(&self) -> bool {
        self.hide_tabs
    }

    pub fn set_hide_tabs(&mut self, hide: bool) {
        self.hide_tabs = hide;

        if self.is_active() {
            self.state.set_visible(UiElement::TabBar, !hide);
        }
    }

    pub fn hide_notifications(&self) -> bool {
        self.hide_notifications
    }

    pub fn set_hide_notifications(&mut self, hide: bool) {
        self.hide_notifications = hide;

        if self.is_active() {
            self.state
                .set_visible(UiElement::Notifications, !hide);
        }
    }

    pub fn is_active(&self) -> bool {
        self.state.mode() == UiMode::DistractionFree
    }
}
