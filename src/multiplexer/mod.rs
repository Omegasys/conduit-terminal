pub mod attach;
pub mod screen;
pub mod sessions;
pub mod tmux;
pub mod zellij;

pub use attach::{
    AttachMode,
    AttachRequest,
    AttachResult,
    MultiplexerAttacher,
};

pub use screen::{
    ScreenConfig,
    ScreenMultiplexer,
};

pub use sessions::{
    MultiplexerSession,
    SessionId,
    SessionManager,
    SessionState,
};

pub use tmux::{
    TmuxConfig,
    TmuxMultiplexer,
};

pub use zellij::{
    ZellijConfig,
    ZellijMultiplexer,
};

/// Supported terminal multiplexer implementations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MultiplexerKind {
    Tmux,
    Screen,
    Zellij,
}

impl MultiplexerKind {
    pub fn name(self) -> &'static str {
        match self {
            Self::Tmux => "tmux",
            Self::Screen => "screen",
            Self::Zellij => "zellij",
        }
    }

    pub fn executable(self) -> &'static str {
        match self {
            Self::Tmux => "tmux",
            Self::Screen => "screen",
            Self::Zellij => "zellij",
        }
    }
}

/// Common interface implemented by supported multiplexers.
pub trait Multiplexer: Send {
    fn kind(&self) -> MultiplexerKind;

    fn is_available(&self) -> bool;

    fn list_sessions(&self) -> Result<Vec<MultiplexerSession>, String>;

    fn create_session(
        &mut self,
        name: Option<&str>,
    ) -> Result<SessionId, String>;

    fn attach(
        &mut self,
        session: &SessionId,
    ) -> Result<(), String>;

    fn detach(&mut self) -> Result<(), String>;

    fn kill_session(
        &mut self,
        session: &SessionId,
    ) -> Result<(), String>;
}
