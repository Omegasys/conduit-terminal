use super::{ModeState, UiElement, UiMode};

#[derive(Debug, Clone)]
pub struct SafeMode {
    state: ModeState,
    plugins_disabled: bool,
    external_commands_disabled: bool,
    network_access_disabled: bool,
    configuration_writes_disabled: bool,
    confirmation_required: bool,
}

impl Default for SafeMode {
    fn default() -> Self {
        Self::new()
    }
}

impl SafeMode {
    pub fn new() -> Self {
        Self {
            state: ModeState::new(UiMode::SafeMode),
            plugins_disabled: true,
            external_commands_disabled: true,
            network_access_disabled: true,
            configuration_writes_disabled: true,
            confirmation_required: true,
        }
    }

    pub fn state(&self) -> &ModeState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut ModeState {
        &mut self.state
    }

    pub fn enter(&mut self) {
        self.state.set_mode(UiMode::SafeMode);

        self.state.set_visible(UiElement::MenuBar, true);
        self.state.set_visible(UiElement::TabBar, true);
        self.state.set_visible(UiElement::Terminal, true);
        self.state.set_visible(UiElement::SecurityIndicators, true);

        self.state.set_visible(UiElement::Toolbar, false);
        self.state.set_visible(UiElement::FlowView, false);
        self.state.set_visible(UiElement::CommandPalette, false);
    }

    pub fn exit(&mut self) {
        self.state.set_mode(UiMode::Full);
    }

    pub fn plugins_disabled(&self) -> bool {
        self.plugins_disabled
    }

    pub fn external_commands_disabled(&self) -> bool {
        self.external_commands_disabled
    }

    pub fn network_access_disabled(&self) -> bool {
        self.network_access_disabled
    }

    pub fn configuration_writes_disabled(&self) -> bool {
        self.configuration_writes_disabled
    }

    pub fn confirmation_required(&self) -> bool {
        self.confirmation_required
    }

    pub fn allows_plugins(&self) -> bool {
        !self.plugins_disabled
    }

    pub fn allows_external_commands(&self) -> bool {
        !self.external_commands_disabled
    }

    pub fn allows_network_access(&self) -> bool {
        !self.network_access_disabled
    }

    pub fn allows_configuration_writes(&self) -> bool {
        !self.configuration_writes_disabled
    }

    pub fn requires_confirmation(&self) -> bool {
        self.confirmation_required
    }

    pub fn is_active(&self) -> bool {
        self.state.mode() == UiMode::SafeMode
    }

    pub fn restricted_elements(&self) -> [UiElement; 3] {
        [
            UiElement::FlowView,
            UiElement::Toolbar,
            UiElement::CommandPalette,
        ]
    }
}
