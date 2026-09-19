//! Session and terminal lifecycle management.
//!
//! Lifecycle is intentionally separate from session state so future
//! features such as crash recovery, safe-mode startup, live component
//! replacement, and workspace restoration can observe lifecycle
//! transitions without owning the terminal itself.

/// Lifecycle phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecyclePhase {
    Created,
    Initializing,
    Starting,
    Running,
    Pausing,
    Resuming,
    Stopping,
    Stopped,
    Failed,
    Recovering,
    Destroyed,
}

/// A lifecycle transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LifecycleTransition {
    pub from: LifecyclePhase,
    pub to: LifecyclePhase,
}

/// Tracks the lifecycle of a Conduit component or session.
#[derive(Debug, Clone)]
pub struct Lifecycle {
    phase: LifecyclePhase,
    previous: Option<LifecyclePhase>,
    last_error: Option<String>,
}

impl Lifecycle {
    pub fn new() -> Self {
        Self {
            phase: LifecyclePhase::Created,
            previous: None,
            last_error: None,
        }
    }

    pub fn phase(&self) -> LifecyclePhase {
        self.phase
    }

    pub fn previous_phase(&self) -> Option<LifecyclePhase> {
        self.previous
    }

    pub fn last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }

    /// Attempts a lifecycle transition.
    pub fn transition(
        &mut self,
        next: LifecyclePhase,
    ) -> Result<LifecycleTransition, LifecycleError> {
        if !is_valid_transition(self.phase, next) {
            return Err(LifecycleError::InvalidTransition {
                from: self.phase,
                to: next,
            });
        }

        let transition = LifecycleTransition {
            from: self.phase,
            to: next,
        };

        self.previous = Some(self.phase);
        self.phase = next;

        if next != LifecyclePhase::Failed {
            self.last_error = None;
        }

        Ok(transition)
    }

    /// Records a failure and moves the lifecycle into the failed state.
    pub fn fail<S: Into<String>>(
        &mut self,
        error: S,
    ) -> LifecycleTransition {
        let transition = LifecycleTransition {
            from: self.phase,
            to: LifecyclePhase::Failed,
        };

        self.previous = Some(self.phase);
        self.phase = LifecyclePhase::Failed;
        self.last_error = Some(error.into());

        transition
    }

    /// Returns the lifecycle to recovery mode.
    pub fn recover(&mut self) -> Result<LifecycleTransition, LifecycleError> {
        self.transition(LifecyclePhase::Recovering)
    }

    pub fn is_running(&self) -> bool {
        self.phase == LifecyclePhase::Running
    }

    pub fn is_terminal(&self) -> bool {
        matches!(
            self.phase,
            LifecyclePhase::Stopped
                | LifecyclePhase::Destroyed
        )
    }
}

impl Default for Lifecycle {
    fn default() -> Self {
        Self::new()
    }
}

/// Lifecycle transition failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LifecycleError {
    InvalidTransition {
        from: LifecyclePhase,
        to: LifecyclePhase,
    },
}

impl std::fmt::Display for LifecycleError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::InvalidTransition { from, to } => {
                write!(
                    formatter,
                    "invalid lifecycle transition: {:?} -> {:?}",
                    from,
                    to
                )
            }
        }
    }
}

impl std::error::Error for LifecycleError {}

fn is_valid_transition(
    from: LifecyclePhase,
    to: LifecyclePhase,
) -> bool {
    use LifecyclePhase::*;

    match (from, to) {
        (Created, Initializing) => true,

        (Initializing, Starting) => true,
        (Initializing, Failed) => true,

        (Starting, Running) => true,
        (Starting, Failed) => true,

        (Running, Pausing) => true,
        (Running, Stopping) => true,
        (Running, Failed) => true,

        (Pausing, Pausing) => false,
        (Pausing, Running) => true,
        (Pausing, Stopping) => true,
        (Pausing, Failed) => true,

        (Resuming, Running) => true,
        (Resuming, Failed) => true,

        (Stopping, Stopped) => true,
        (Stopping, Failed) => true,

        (Stopped, Recovering) => true,
        (Stopped, Destroyed) => true,

        (Failed, Recovering) => true,
        (Failed, Stopped) => true,
        (Failed, Destroyed) => true,

        (Recovering, Initializing) => true,
        (Recovering, Starting) => true,
        (Recovering, Failed) => true,

        (Destroyed, _) => false,

        (a, b) if a == b => true,

        _ => false,
    }
}
