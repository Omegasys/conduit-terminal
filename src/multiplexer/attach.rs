use super::{
    Multiplexer,
    MultiplexerKind,
    MultiplexerSession,
    SessionId,
};

/// How Conduit should handle an attachment request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttachMode {
    Attach,
    Reattach,
    CreateIfMissing,
}

/// Request to attach Conduit to a multiplexer session.
#[derive(Debug, Clone)]
pub struct AttachRequest {
    pub multiplexer: MultiplexerKind,
    pub session: SessionId,
    pub mode: AttachMode,
}

impl AttachRequest {
    pub fn new(
        multiplexer: MultiplexerKind,
        session: SessionId,
    ) -> Self {
        Self {
            multiplexer,
            session,
            mode: AttachMode::Attach,
        }
    }

    pub fn with_mode(mut self, mode: AttachMode) -> Self {
        self.mode = mode;
        self
    }
}

/// Result of an attachment operation.
#[derive(Debug, Clone)]
pub enum AttachResult {
    Attached {
        session: MultiplexerSession,
    },
    Created {
        session: SessionId,
    },
    AlreadyAttached {
        session: SessionId,
    },
}

/// Coordinates session attachment independently of the GUI/TUI.
pub struct MultiplexerAttacher {
    multiplexer: Box<dyn Multiplexer>,
}

impl MultiplexerAttacher {
    pub fn new(multiplexer: Box<dyn Multiplexer>) -> Self {
        Self { multiplexer }
    }

    pub fn multiplexer(&self) -> &dyn Multiplexer {
        self.multiplexer.as_ref()
    }

    pub fn multiplexer_mut(&mut self) -> &mut dyn Multiplexer {
        self.multiplexer.as_mut()
    }

    pub fn attach(
        &mut self,
        request: &AttachRequest,
    ) -> Result<AttachResult, String> {
        if self.multiplexer.kind() != request.multiplexer {
            return Err(format!(
                "requested multiplexer '{}' but attached multiplexer is '{}'",
                request.multiplexer.name(),
                self.multiplexer.kind().name()
            ));
        }

        if !self.multiplexer.is_available() {
            return Err(format!(
                "multiplexer '{}' is not available",
                self.multiplexer.kind().name()
            ));
        }

        let sessions = self.multiplexer.list_sessions()?;

        if let Some(session) = sessions
            .iter()
            .find(|session| session.id() == &request.session)
        {
            if session.is_attached() {
                return Ok(AttachResult::AlreadyAttached {
                    session: session.clone(),
                });
            }

            self.multiplexer.attach(&request.session)?;

            return Ok(AttachResult::Attached {
                session: session.clone(),
            });
        }

        match request.mode {
            AttachMode::CreateIfMissing => {
                let id = self
                    .multiplexer
                    .create_session(Some(request.session.as_str()))?;

                Ok(AttachResult::Created { session: id })
            }

            AttachMode::Attach | AttachMode::Reattach => Err(format!(
                "session '{}' was not found",
                request.session.as_str()
            )),
        }
    }
}
