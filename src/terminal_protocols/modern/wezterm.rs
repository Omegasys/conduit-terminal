use super::semantic_shell::ShellSemanticEvent;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WezTermEvent {
    Shell(ShellSemanticEvent),
    UserVariable {
        name: String,
        value: String,
    },
    Notification {
        title: Option<String>,
        body: String,
    },
    Unknown(Vec<u8>),
}

#[derive(Debug, Default)]
pub struct WezTermProtocol;

impl WezTermProtocol {
    pub fn new() -> Self {
        Self
    }

    pub fn working_directory(
        &self,
        path: impl Into<String>,
    ) -> WezTermEvent {
        WezTermEvent::Shell(
            ShellSemanticEvent::working_directory(path),
        )
    }

    pub fn user_variable(
        &self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> WezTermEvent {
        WezTermEvent::UserVariable {
            name: name.into(),
            value: value.into(),
        }
    }

    pub fn notification(
        &self,
        title: Option<String>,
        body: impl Into<String>,
    ) -> WezTermEvent {
        WezTermEvent::Notification {
            title,
            body: body.into(),
        }
    }
}
