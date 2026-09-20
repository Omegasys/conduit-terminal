#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SessionCommandAction {
    Create,
    Close,
    Attach,
    Detach,
    Restore,
    Save,
    List,
}

impl SessionCommandAction {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Create => "new-session",
            Self::Close => "close-session",
            Self::Attach => "attach-session",
            Self::Detach => "detach-session",
            Self::Restore => "restore-session",
            Self::Save => "save-session",
            Self::List => "list-sessions",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SessionCommand {
    action: SessionCommandAction,
    session_id: Option<String>,
    name: Option<String>,
}

impl SessionCommand {
    pub fn new(action: SessionCommandAction) -> Self {
        Self {
            action,
            session_id: None,
            name: None,
        }
    }

    pub fn with_id(
        action: SessionCommandAction,
        session_id: impl Into<String>,
    ) -> Self {
        Self {
            action,
            session_id: Some(session_id.into()),
            name: None,
        }
    }

    pub fn with_name(
        action: SessionCommandAction,
        name: impl Into<String>,
    ) -> Self {
        Self {
            action,
            session_id: None,
            name: Some(name.into()),
        }
    }

    pub fn action(&self) -> SessionCommandAction {
        self.action
    }

    pub fn session_id(&self) -> Option<&str> {
        self.session_id.as_deref()
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_session_id(&mut self, id: impl Into<String>) {
        self.session_id = Some(id.into());
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = Some(name.into());
    }

    pub fn is_targeted(&self) -> bool {
        self.session_id.is_some() || self.name.is_some()
    }
}
