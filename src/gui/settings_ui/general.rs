#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartupBehavior {
    NewSession,
    RestoreLastSession,
    RestoreWorkspace,
    ShowStartPage,
}

#[derive(Debug, Clone)]
pub struct GeneralSettings {
    startup_behavior: StartupBehavior,
    confirm_close_window: bool,
    confirm_close_tab: bool,
    confirm_close_session: bool,
    restore_open_tabs: bool,
    restore_open_windows: bool,
    restore_workspaces: bool,
    check_for_updates: bool,
    telemetry_enabled: bool,
    notifications_enabled: bool,
}

impl GeneralSettings {
    pub fn new() -> Self {
        Self {
            startup_behavior: StartupBehavior::RestoreLastSession,
            confirm_close_window: true,
            confirm_close_tab: false,
            confirm_close_session: true,
            restore_open_tabs: true,
            restore_open_windows: true,
            restore_workspaces: true,
            check_for_updates: true,
            telemetry_enabled: false,
            notifications_enabled: true,
        }
    }

    pub fn startup_behavior(&self) -> StartupBehavior {
        self.startup_behavior
    }

    pub fn confirm_close_window(&self) -> bool {
        self.confirm_close_window
    }

    pub fn confirm_close_tab(&self) -> bool {
        self.confirm_close_tab
    }

    pub fn confirm_close_session(&self) -> bool {
        self.confirm_close_session
    }

    pub fn restore_open_tabs(&self) -> bool {
        self.restore_open_tabs
    }

    pub fn restore_open_windows(&self) -> bool {
        self.restore_open_windows
    }

    pub fn restore_workspaces(&self) -> bool {
        self.restore_workspaces
    }

    pub fn check_for_updates(&self) -> bool {
        self.check_for_updates
    }

    pub fn telemetry_enabled(&self) -> bool {
        self.telemetry_enabled
    }

    pub fn notifications_enabled(&self) -> bool {
        self.notifications_enabled
    }

    pub fn set_startup_behavior(&mut self, value: StartupBehavior) {
        self.startup_behavior = value;
    }

    pub fn set_confirm_close_window(&mut self, value: bool) {
        self.confirm_close_window = value;
    }

    pub fn set_confirm_close_tab(&mut self, value: bool) {
        self.confirm_close_tab = value;
    }

    pub fn set_confirm_close_session(&mut self, value: bool) {
        self.confirm_close_session = value;
    }

    pub fn set_restore_open_tabs(&mut self, value: bool) {
        self.restore_open_tabs = value;
    }

    pub fn set_restore_open_windows(&mut self, value: bool) {
        self.restore_open_windows = value;
    }

    pub fn set_restore_workspaces(&mut self, value: bool) {
        self.restore_workspaces = value;
    }

    pub fn set_check_for_updates(&mut self, value: bool) {
        self.check_for_updates = value;
    }

    pub fn set_telemetry_enabled(&mut self, value: bool) {
        self.telemetry_enabled = value;
    }

    pub fn set_notifications_enabled(&mut self, value: bool) {
        self.notifications_enabled = value;
    }
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self::new()
    }
}
