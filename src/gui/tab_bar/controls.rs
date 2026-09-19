#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabBarControl {
    NewTab,
    TabList,
    ScrollLeft,
    ScrollRight,
    Overflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabBarControlAction {
    NewTab,
    OpenTabList,
    ScrollLeft,
    ScrollRight,
    OpenOverflow,
}

impl TabBarControl {
    pub fn action(self) -> TabBarControlAction {
        match self {
            Self::NewTab => TabBarControlAction::NewTab,
            Self::TabList => TabBarControlAction::OpenTabList,
            Self::ScrollLeft => TabBarControlAction::ScrollLeft,
            Self::ScrollRight => TabBarControlAction::ScrollRight,
            Self::Overflow => TabBarControlAction::OpenOverflow,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TabBarControlState {
    control: TabBarControl,
    enabled: bool,
    visible: bool,
    tooltip: String,
}

impl TabBarControlState {
    pub fn new(control: TabBarControl) -> Self {
        let tooltip = match control {
            TabBarControl::NewTab => "New Tab",
            TabBarControl::TabList => "Tab List",
            TabBarControl::ScrollLeft => "Scroll Tabs Left",
            TabBarControl::ScrollRight => "Scroll Tabs Right",
            TabBarControl::Overflow => "More Tabs",
        };

        Self {
            control,
            enabled: true,
            visible: true,
            tooltip: tooltip.to_string(),
        }
    }

    pub fn control(&self) -> TabBarControl {
        self.control
    }

    pub fn action(&self) -> TabBarControlAction {
        self.control.action()
    }

    pub fn tooltip(&self) -> &str {
        &self.tooltip
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

#[derive(Debug, Clone)]
pub struct TabBarControls {
    controls: Vec<TabBarControlState>,
}

impl Default for TabBarControls {
    fn default() -> Self {
        Self::new()
    }
}

impl TabBarControls {
    pub fn new() -> Self {
        Self {
            controls: vec![
                TabBarControlState::new(TabBarControl::NewTab),
                TabBarControlState::new(TabBarControl::TabList),
                TabBarControlState::new(TabBarControl::ScrollLeft),
                TabBarControlState::new(TabBarControl::ScrollRight),
                TabBarControlState::new(TabBarControl::Overflow),
            ],
        }
    }

    pub fn controls(&self) -> &[TabBarControlState] {
        &self.controls
    }

    pub fn controls_mut(&mut self) -> &mut [TabBarControlState] {
        &mut self.controls
    }

    pub fn get(&self, control: TabBarControl) -> Option<&TabBarControlState> {
        self.controls
            .iter()
            .find(|item| item.control() == control)
    }

    pub fn get_mut(
        &mut self,
        control: TabBarControl,
    ) -> Option<&mut TabBarControlState> {
        self.controls
            .iter_mut()
            .find(|item| item.control() == control)
    }

    pub fn set_enabled(&mut self, control: TabBarControl, enabled: bool) {
        if let Some(item) = self.get_mut(control) {
            item.set_enabled(enabled);
        }
    }

    pub fn set_visible(&mut self, control: TabBarControl, visible: bool) {
        if let Some(item) = self.get_mut(control) {
            item.set_visible(visible);
        }
    }

    pub fn enable_all(&mut self) {
        for control in &mut self.controls {
            control.set_enabled(true);
        }
    }

    pub fn disable_all(&mut self) {
        for control in &mut self.controls {
            control.set_enabled(false);
        }
    }
}
