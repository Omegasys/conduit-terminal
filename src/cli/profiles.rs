use crate::resources::profile::Profile;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProfileCommandAction {
    Create,
    Delete,
    Activate,
    Deactivate,
    Show,
    List,
    Set,
    Reset,
}

impl ProfileCommandAction {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Create => "profile-create",
            Self::Delete => "profile-delete",
            Self::Activate => "profile-activate",
            Self::Deactivate => "profile-deactivate",
            Self::Show => "profile-show",
            Self::List => "profile-list",
            Self::Set => "profile-set",
            Self::Reset => "profile-reset",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProfileCommand {
    action: ProfileCommandAction,
    profile: Option<String>,
    key: Option<String>,
    value: Option<String>,
}

impl ProfileCommand {
    pub fn new(action: ProfileCommandAction) -> Self {
        Self {
            action,
            profile: None,
            key: None,
            value: None,
        }
    }

    pub fn with_profile(
        action: ProfileCommandAction,
        profile: impl Into<String>,
    ) -> Self {
        Self {
            action,
            profile: Some(profile.into()),
            key: None,
            value: None,
        }
    }

    pub fn with_setting(
        action: ProfileCommandAction,
        profile: impl Into<String>,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            action,
            profile: Some(profile.into()),
            key: Some(key.into()),
            value: Some(value.into()),
        }
    }

    pub fn action(&self) -> ProfileCommandAction {
        self.action
    }

    pub fn profile(&self) -> Option<&str> {
        self.profile.as_deref()
    }

    pub fn key(&self) -> Option<&str> {
        self.key.as_deref()
    }

    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    pub fn set_profile(&mut self, profile: impl Into<String>) {
        self.profile = Some(profile.into());
    }

    pub fn set_key(&mut self, key: impl Into<String>) {
        self.key = Some(key.into());
    }

    pub fn set_value(&mut self, value: impl Into<String>) {
        self.value = Some(value.into());
    }

    pub fn is_targeted(&self) -> bool {
        self.profile.is_some()
    }

    pub fn applies_to(&self, profile: &Profile) -> bool {
        self.profile
            .as_deref()
            .map(|name| name == profile.name())
            .unwrap_or(true)
    }
}
