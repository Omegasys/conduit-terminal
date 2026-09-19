//! Tab title management.
//!
//! Titles can originate from the shell, terminal protocol sequences,
//! the user, or Conduit's own session metadata.

/// Source of a tab title.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabTitleSource {
    Default,
    Shell,
    Terminal,
    User,
}

/// Tab title.
#[derive(Debug, Clone)]
pub struct TabTitle {
    default: String,
    current: String,
    source: TabTitleSource,

    custom: Option<String>,

    /// Whether terminal title changes are allowed to replace the
    /// current title.
    allow_terminal_updates: bool,
}

impl TabTitle {
    pub fn new(title: impl Into<String>) -> Self {
        let title = title.into();

        Self {
            default: title.clone(),
            current: title,
            source: TabTitleSource::Default,

            custom: None,

            allow_terminal_updates: true,
        }
    }

    pub fn current(&self) -> &str {
        &self.current
    }

    pub fn default_title(&self) -> &str {
        &self.default
    }

    pub fn source(&self) -> TabTitleSource {
        self.source
    }

    pub fn custom(&self) -> Option<&str> {
        self.custom.as_deref()
    }

    pub fn set_default(
        &mut self,
        title: impl Into<String>,
    ) {
        self.default = title.into();

        if self.custom.is_none() {
            self.current =
                self.default.clone();

            self.source =
                TabTitleSource::Default;
        }
    }

    pub fn set_shell_title(
        &mut self,
        title: impl Into<String>,
    ) {
        if self.custom.is_some() {
            return;
        }

        self.current = title.into();
        self.source = TabTitleSource::Shell;
    }

    pub fn set_terminal_title(
        &mut self,
        title: impl Into<String>,
    ) {
        if !self.allow_terminal_updates
            || self.custom.is_some()
        {
            return;
        }

        self.current = title.into();
        self.source =
            TabTitleSource::Terminal;
    }

    pub fn set_custom(
        &mut self,
        title: impl Into<String>,
    ) {
        let title = title.into();

        self.custom = Some(title.clone());
        self.current = title;
        self.source = TabTitleSource::User;
    }

    pub fn clear_custom(&mut self) {
        self.custom = None;
        self.current =
            self.default.clone();

        self.source =
            TabTitleSource::Default;
    }

    pub fn set_terminal_updates(
        &mut self,
        enabled: bool,
    ) {
        self.allow_terminal_updates =
            enabled;
    }

    pub fn terminal_updates_allowed(
        &self,
    ) -> bool {
        self.allow_terminal_updates
    }

    /// Resets the title back to the default.
    pub fn reset(&mut self) {
        self.custom = None;
        self.current =
            self.default.clone();

        self.source =
            TabTitleSource::Default;
    }
}

impl Default for TabTitle {
    fn default() -> Self {
        Self::new("Conduit")
    }
}
