#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfigCommandAction {
    Show,
    Get,
    Set,
    Remove,
    Reset,
    Validate,
    Reload,
    Save,
    Edit,
    Diff,
    Rollback,
}

impl ConfigCommandAction {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Show => "config-show",
            Self::Get => "config-get",
            Self::Set => "config-set",
            Self::Remove => "config-remove",
            Self::Reset => "config-reset",
            Self::Validate => "config-validate",
            Self::Reload => "reload-config",
            Self::Save => "config-save",
            Self::Edit => "config-edit",
            Self::Diff => "config-diff",
            Self::Rollback => "config-rollback",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConfigCommand {
    action: ConfigCommandAction,
    key: Option<String>,
    value: Option<String>,
}

impl ConfigCommand {
    pub fn new(action: ConfigCommandAction) -> Self {
        Self {
            action,
            key: None,
            value: None,
        }
    }

    pub fn with_key(
        action: ConfigCommandAction,
        key: impl Into<String>,
    ) -> Self {
        Self {
            action,
            key: Some(key.into()),
            value: None,
        }
    }

    pub fn with_value(
        action: ConfigCommandAction,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            action,
            key: Some(key.into()),
            value: Some(value.into()),
        }
    }

    pub fn action(&self) -> ConfigCommandAction {
        self.action
    }

    pub fn key(&self) -> Option<&str> {
        self.key.as_deref()
    }

    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    pub fn set_key(&mut self, key: impl Into<String>) {
        self.key = Some(key.into());
    }

    pub fn set_value(&mut self, value: impl Into<String>) {
        self.value = Some(value.into());
    }

    pub fn requires_key(&self) -> bool {
        matches!(
            self.action,
            ConfigCommandAction::Get
                | ConfigCommandAction::Set
                | ConfigCommandAction::Remove
        )
    }

    pub fn requires_value(&self) -> bool {
        matches!(self.action, ConfigCommandAction::Set)
    }
}
