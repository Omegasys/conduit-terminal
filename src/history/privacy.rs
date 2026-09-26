#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoryPrivacyMode {
    Normal,
    Private,
    Ephemeral,
    Disabled,
}

#[derive(Debug, Clone)]
pub struct HistoryPrivacyPolicy {
    mode: HistoryPrivacyMode,
    store_commands: bool,
    store_metadata: bool,
    store_working_directory: bool,
    store_shell: bool,
}

impl Default for HistoryPrivacyPolicy {
    fn default() -> Self {
        Self {
            mode: HistoryPrivacyMode::Normal,
            store_commands: true,
            store_metadata: true,
            store_working_directory: true,
            store_shell: true,
        }
    }
}

impl HistoryPrivacyPolicy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mode(&self) -> HistoryPrivacyMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: HistoryPrivacyMode) {
        self.mode = mode;

        match mode {
            HistoryPrivacyMode::Normal => {
                self.store_commands = true;
                self.store_metadata = true;
            }

            HistoryPrivacyMode::Private => {
                self.store_commands = false;
                self.store_metadata = true;
            }

            HistoryPrivacyMode::Ephemeral => {
                self.store_commands = true;
                self.store_metadata = true;
            }

            HistoryPrivacyMode::Disabled => {
                self.store_commands = false;
                self.store_metadata = false;
            }
        }
    }

    pub fn should_store_commands(&self) -> bool {
        self.store_commands
            && self.mode != HistoryPrivacyMode::Disabled
    }

    pub fn should_store_metadata(&self) -> bool {
        self.store_metadata
            && self.mode != HistoryPrivacyMode::Disabled
    }

    pub fn store_working_directory(&self) -> bool {
        self.store_working_directory
    }

    pub fn set_store_working_directory(&mut self, value: bool) {
        self.store_working_directory = value;
    }

    pub fn store_shell(&self) -> bool {
        self.store_shell
    }

    pub fn set_store_shell(&mut self, value: bool) {
        self.store_shell = value;
    }
}
