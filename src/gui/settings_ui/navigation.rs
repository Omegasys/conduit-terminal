#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SettingsPage {
    General,
    Appearance,
    Fonts,
    Colors,
    Terminal,
    Keybindings,
    Profiles,
    Workspaces,
    Security,
    Accessibility,
    Plugins,
    Advanced,
}

#[derive(Debug, Clone)]
pub struct SettingsUiNavigation {
    current_page: SettingsPage,
    history: Vec<SettingsPage>,
    history_index: usize,
}

impl SettingsUiNavigation {
    pub fn new() -> Self {
        Self {
            current_page: SettingsPage::General,
            history: vec![SettingsPage::General],
            history_index: 0,
        }
    }

    pub fn current(&self) -> SettingsPage {
        self.current_page
    }

    pub fn can_go_back(&self) -> bool {
        self.history_index > 0
    }

    pub fn can_go_forward(&self) -> bool {
        self.history_index + 1 < self.history.len()
    }

    pub fn navigate(&mut self, page: SettingsPage) {
        if page == self.current_page {
            return;
        }

        if self.history_index + 1 < self.history.len() {
            self.history.truncate(self.history_index + 1);
        }

        self.history.push(page);
        self.history_index = self.history.len() - 1;
        self.current_page = page;
    }

    pub fn back(&mut self) -> Option<SettingsPage> {
        if !self.can_go_back() {
            return None;
        }

        self.history_index -= 1;
        self.current_page = self.history[self.history_index];

        Some(self.current_page)
    }

    pub fn forward(&mut self) -> Option<SettingsPage> {
        if !self.can_go_forward() {
            return None;
        }

        self.history_index += 1;
        self.current_page = self.history[self.history_index];

        Some(self.current_page)
    }

    pub fn reset(&mut self) {
        self.current_page = SettingsPage::General;
        self.history.clear();
        self.history.push(SettingsPage::General);
        self.history_index = 0;
    }

    pub fn history(&self) -> &[SettingsPage] {
        &self.history
    }
}

impl Default for SettingsUiNavigation {
    fn default() -> Self {
        Self::new()
    }
}
