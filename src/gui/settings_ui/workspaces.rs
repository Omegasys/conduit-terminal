use std::collections::BTreeMap;

use crate::config_engine::ConfigValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceCloseBehavior {
    Close,
    Confirm,
    SaveAndClose,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceSwitchBehavior {
    Immediate,
    Confirm,
    SaveCurrent,
}

#[derive(Debug, Clone)]
pub struct WorkspaceSettings {
    restore_workspaces: bool,
    remember_active_workspace: bool,
    auto_save: bool,
    auto_save_interval_seconds: u64,
    close_behavior: WorkspaceCloseBehavior,
    switch_behavior: WorkspaceSwitchBehavior,
    preserve_layout: bool,
    preserve_windows: bool,
    preserve_tabs: bool,
    preserve_panes: bool,
    allow_workspace_reordering: bool,
    show_workspace_indicator: bool,
}

impl Default for WorkspaceSettings {
    fn default() -> Self {
        Self {
            restore_workspaces: true,
            remember_active_workspace: true,
            auto_save: true,
            auto_save_interval_seconds: 60,
            close_behavior: WorkspaceCloseBehavior::Confirm,
            switch_behavior: WorkspaceSwitchBehavior::Immediate,
            preserve_layout: true,
            preserve_windows: true,
            preserve_tabs: true,
            preserve_panes: true,
            allow_workspace_reordering: true,
            show_workspace_indicator: true,
        }
    }
}

impl WorkspaceSettings {
    pub fn restore_workspaces(&self) -> bool {
        self.restore_workspaces
    }

    pub fn set_restore_workspaces(&mut self, value: bool) {
        self.restore_workspaces = value;
    }

    pub fn remember_active_workspace(&self) -> bool {
        self.remember_active_workspace
    }

    pub fn set_remember_active_workspace(&mut self, value: bool) {
        self.remember_active_workspace = value;
    }

    pub fn auto_save(&self) -> bool {
        self.auto_save
    }

    pub fn set_auto_save(&mut self, value: bool) {
        self.auto_save = value;
    }

    pub fn auto_save_interval_seconds(&self) -> u64 {
        self.auto_save_interval_seconds
    }

    pub fn set_auto_save_interval_seconds(&mut self, value: u64) {
        self.auto_save_interval_seconds = value.clamp(5, 86_400);
    }

    pub fn close_behavior(&self) -> WorkspaceCloseBehavior {
        self.close_behavior
    }

    pub fn set_close_behavior(&mut self, value: WorkspaceCloseBehavior) {
        self.close_behavior = value;
    }

    pub fn switch_behavior(&self) -> WorkspaceSwitchBehavior {
        self.switch_behavior
    }

    pub fn set_switch_behavior(&mut self, value: WorkspaceSwitchBehavior) {
        self.switch_behavior = value;
    }

    pub fn preserve_layout(&self) -> bool {
        self.preserve_layout
    }

    pub fn set_preserve_layout(&mut self, value: bool) {
        self.preserve_layout = value;
    }

    pub fn preserve_windows(&self) -> bool {
        self.preserve_windows
    }

    pub fn set_preserve_windows(&mut self, value: bool) {
        self.preserve_windows = value;
    }

    pub fn preserve_tabs(&self) -> bool {
        self.preserve_tabs
    }

    pub fn set_preserve_tabs(&mut self, value: bool) {
        self.preserve_tabs = value;
    }

    pub fn preserve_panes(&self) -> bool {
        self.preserve_panes
    }

    pub fn set_preserve_panes(&mut self, value: bool) {
        self.preserve_panes = value;
    }

    pub fn allow_workspace_reordering(&self) -> bool {
        self.allow_workspace_reordering
    }

    pub fn set_allow_workspace_reordering(&mut self, value: bool) {
        self.allow_workspace_reordering = value;
    }

    pub fn show_workspace_indicator(&self) -> bool {
        self.show_workspace_indicator
    }

    pub fn set_show_workspace_indicator(&mut self, value: bool) {
        self.show_workspace_indicator = value;
    }

    pub fn to_config(&self) -> BTreeMap<String, ConfigValue> {
        let mut values = BTreeMap::new();

        values.insert(
            "restore_workspaces".into(),
            ConfigValue::Boolean(self.restore_workspaces),
        );
        values.insert(
            "remember_active_workspace".into(),
            ConfigValue::Boolean(self.remember_active_workspace),
        );
        values.insert("auto_save".into(), ConfigValue::Boolean(self.auto_save));
        values.insert(
            "auto_save_interval_seconds".into(),
            ConfigValue::Integer(self.auto_save_interval_seconds as i64),
        );
        values.insert(
            "close_behavior".into(),
            ConfigValue::String(format!("{:?}", self.close_behavior).to_lowercase()),
        );
        values.insert(
            "switch_behavior".into(),
            ConfigValue::String(format!("{:?}", self.switch_behavior).to_lowercase()),
        );
        values.insert(
            "preserve_layout".into(),
            ConfigValue::Boolean(self.preserve_layout),
        );
        values.insert(
            "preserve_windows".into(),
            ConfigValue::Boolean(self.preserve_windows),
        );
        values.insert(
            "preserve_tabs".into(),
            ConfigValue::Boolean(self.preserve_tabs),
        );
        values.insert(
            "preserve_panes".into(),
            ConfigValue::Boolean(self.preserve_panes),
        );
        values.insert(
            "allow_workspace_reordering".into(),
            ConfigValue::Boolean(self.allow_workspace_reordering),
        );
        values.insert(
            "show_workspace_indicator".into(),
            ConfigValue::Boolean(self.show_workspace_indicator),
        );

        values
    }
}
