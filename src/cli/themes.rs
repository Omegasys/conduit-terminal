use crate::resources::theme::Theme;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeCommandAction {
    List,
    Show,
    Use,
    Preview,
    Reload,
    Install,
    Remove,
    Enable,
    Disable,
}

impl ThemeCommandAction {
    pub fn name(&self) -> &'static str {
        match self {
            Self::List => "theme-list",
            Self::Show => "theme-show",
            Self::Use => "theme-use",
            Self::Preview => "theme-preview",
            Self::Reload => "theme-reload",
            Self::Install => "theme-install",
            Self::Remove => "theme-remove",
            Self::Enable => "theme-enable",
            Self::Disable => "theme-disable",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ThemeCommand {
    action: ThemeCommandAction,
    theme: Option<String>,
    source: Option<String>,
}

impl ThemeCommand {
    pub fn new(action: ThemeCommandAction) -> Self {
        Self {
            action,
            theme: None,
            source: None,
        }
    }

    pub fn with_theme(
        action: ThemeCommandAction,
        theme: impl Into<String>,
    ) -> Self {
        Self {
            action,
            theme: Some(theme.into()),
            source: None,
        }
    }

    pub fn with_source(
        action: ThemeCommandAction,
        source: impl Into<String>,
    ) -> Self {
        Self {
            action,
            theme: None,
            source: Some(source.into()),
        }
    }

    pub fn action(&self) -> ThemeCommandAction {
        self.action
    }

    pub fn theme(&self) -> Option<&str> {
        self.theme.as_deref()
    }

    pub fn source(&self) -> Option<&str> {
        self.source.as_deref()
    }

    pub fn set_theme(&mut self, theme: impl Into<String>) {
        self.theme = Some(theme.into());
    }

    pub fn set_source(&mut self, source: impl Into<String>) {
        self.source = Some(source.into());
    }

    pub fn is_targeted(&self) -> bool {
        self.theme.is_some()
    }

    pub fn applies_to(&self, theme: &Theme) -> bool {
        self.theme
            .as_deref()
            .map(|name| name == theme.name())
            .unwrap_or(true)
    }

    pub fn requires_theme(&self) -> bool {
        matches!(
            self.action,
            ThemeCommandAction::Show
                | ThemeCommandAction::Use
                | ThemeCommandAction::Preview
                | ThemeCommandAction::Reload
                | ThemeCommandAction::Remove
                | ThemeCommandAction::Enable
                | ThemeCommandAction::Disable
        )
    }
}
