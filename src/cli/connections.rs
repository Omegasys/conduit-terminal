use std::fmt;

use crate::tui::connections::{
    ConnectionKind,
    ConnectionState,
};

/// Actions supported by the `conduit connections` command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionCommandAction {
    List,
    Show,
    Connect,
    Disconnect,
    Reconnect,
    Open,
    Close,
    Test,
    Add,
    Remove,
}

impl ConnectionCommandAction {
    pub fn name(&self) -> &'static str {
        match self {
            Self::List => "list",
            Self::Show => "show",
            Self::Connect => "connect",
            Self::Disconnect => "disconnect",
            Self::Reconnect => "reconnect",
            Self::Open => "open",
            Self::Close => "close",
            Self::Test => "test",
            Self::Add => "add",
            Self::Remove => "remove",
        }
    }

    pub fn from_name(value: &str) -> Option<Self> {
        match value {
            "list" => Some(Self::List),
            "show" => Some(Self::Show),
            "connect" => Some(Self::Connect),
            "disconnect" => Some(Self::Disconnect),
            "reconnect" => Some(Self::Reconnect),
            "open" => Some(Self::Open),
            "close" => Some(Self::Close),
            "test" => Some(Self::Test),
            "add" => Some(Self::Add),
            "remove" => Some(Self::Remove),
            _ => None,
        }
    }
}

impl fmt::Display for ConnectionCommandAction {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

/// A parsed connection command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionCommand {
    action: ConnectionCommandAction,
    connection_id: Option<String>,
    name: Option<String>,
    kind: Option<ConnectionKind>,
    state: Option<ConnectionState>,
    target: Option<String>,
    user: Option<String>,
    host: Option<String>,
    port: Option<u16>,
}

impl ConnectionCommand {
    pub fn new(action: ConnectionCommandAction) -> Self {
        Self {
            action,
            connection_id: None,
            name: None,
            kind: None,
            state: None,
            target: None,
            user: None,
            host: None,
            port: None,
        }
    }

    pub fn action(&self) -> &ConnectionCommandAction {
        &self.action
    }

    pub fn connection_id(&self) -> Option<&str> {
        self.connection_id.as_deref()
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn kind(&self) -> Option<&ConnectionKind> {
        self.kind.as_ref()
    }

    pub fn state(&self) -> Option<&ConnectionState> {
        self.state.as_ref()
    }

    pub fn target(&self) -> Option<&str> {
        self.target.as_deref()
    }

    pub fn user(&self) -> Option<&str> {
        self.user.as_deref()
    }

    pub fn host(&self) -> Option<&str> {
        self.host.as_deref()
    }

    pub fn port(&self) -> Option<u16> {
        self.port
    }

    pub fn set_connection_id<S>(&mut self, connection_id: S)
    where
        S: Into<String>,
    {
        self.connection_id = Some(connection_id.into());
    }

    pub fn set_name<S>(&mut self, name: S)
    where
        S: Into<String>,
    {
        self.name = Some(name.into());
    }

    pub fn set_kind(&mut self, kind: ConnectionKind) {
        self.kind = Some(kind);
    }

    pub fn set_state(&mut self, state: ConnectionState) {
        self.state = Some(state);
    }

    pub fn set_target<S>(&mut self, target: S)
    where
        S: Into<String>,
    {
        self.target = Some(target.into());
    }

    pub fn set_user<S>(&mut self, user: S)
    where
        S: Into<String>,
    {
        self.user = Some(user.into());
    }

    pub fn set_host<S>(&mut self, host: S)
    where
        S: Into<String>,
    {
        self.host = Some(host.into());
    }

    pub fn set_port(&mut self, port: u16) {
        self.port = Some(port);
    }

    pub fn requires_connection_id(&self) -> bool {
        matches!(
            self.action,
            ConnectionCommandAction::Show
                | ConnectionCommandAction::Connect
                | ConnectionCommandAction::Disconnect
                | ConnectionCommandAction::Reconnect
                | ConnectionCommandAction::Open
                | ConnectionCommandAction::Close
                | ConnectionCommandAction::Test
                | ConnectionCommandAction::Remove
        )
    }

    pub fn requires_target(&self) -> bool {
        matches!(self.action, ConnectionCommandAction::Add)
    }
}
