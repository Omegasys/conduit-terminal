//! Window lifecycle management.

/// Window-specific lifecycle phases.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowLifecyclePhase {
    Created,
    Initializing,
    CreatingNativeWindow,
    Ready,
    Showing,
    Visible,
    Hiding,
    Hidden,
    Closing,
    Closed,
    Failed,
}

/// Window lifecycle errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowLifecycleError {
    InvalidTransition {
        from: WindowLifecyclePhase,
        to: WindowLifecyclePhase,
    },

    NativeWindowCreationFailed,
}

impl std::fmt::Display for WindowLifecycleError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::InvalidTransition { from, to } => {
                write!(
                    formatter,
                    "invalid window lifecycle transition: {:?} -> {:?}",
                    from,
                    to
                )
            }

            Self::NativeWindowCreationFailed => {
                write!(
                    formatter,
                    "native window creation failed"
                )
            }
        }
    }
}

impl std::error::Error for WindowLifecycleError {}

/// Window lifecycle state machine.
#[derive(Debug, Clone)]
pub struct WindowLifecycle {
    phase: WindowLifecyclePhase,
    last_error: Option<String>,
}

impl WindowLifecycle {
    pub fn new() -> Self {
        Self {
            phase: WindowLifecyclePhase::Created,
            last_error: None,
        }
    }

    pub fn phase(&self) -> WindowLifecyclePhase {
        self.phase
    }

    pub fn last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }

    pub fn transition(
        &mut self,
        next: WindowLifecyclePhase,
    ) -> Result<(), WindowLifecycleError> {
        if !valid_transition(self.phase, next) {
            return Err(
                WindowLifecycleError::InvalidTransition {
                    from: self.phase,
                    to: next,
                },
            );
        }

        self.phase = next;

        if next != WindowLifecyclePhase::Failed {
            self.last_error = None;
        }

        Ok(())
    }

    pub fn fail<S: Into<String>>(
        &mut self,
        error: S,
    ) {
        self.phase = WindowLifecyclePhase::Failed;
        self.last_error = Some(error.into());
    }

    pub fn is_open(&self) -> bool {
        !matches!(
            self.phase,
            WindowLifecyclePhase::Closed
        )
    }

    pub fn is_visible(&self) -> bool {
        self.phase == WindowLifecyclePhase::Visible
    }
}

impl Default for WindowLifecycle {
    fn default() -> Self {
        Self::new()
    }
}

fn valid_transition(
    from: WindowLifecyclePhase,
    to: WindowLifecyclePhase,
) -> bool {
    use WindowLifecyclePhase::*;

    match (from, to) {
        (Created, Initializing) => true,
        (Initializing, CreatingNativeWindow) => true,
        (Initializing, Failed) => true,

        (CreatingNativeWindow, Ready) => true,
        (CreatingNativeWindow, Failed) => true,

        (Ready, Showing) => true,
        (Ready, Closing) => true,

        (Showing, Visible) => true,
        (Showing, Failed) => true,

        (Visible, Hiding) => true,
        (Visible, Closing) => true,

        (Hiding, Hidden) => true,
        (Hiding, Failed) => true,

        (Hidden, Showing) => true,
        (Hidden, Closing) => true,

        (Closing, Closed) => true,
        (Closing, Failed) => true,

        (Failed, Initializing) => true,
        (Failed, Closing) => true,

        (a, b) if a == b => true,

        _ => false,
    }
}
