use std::collections::BTreeMap;
use std::fmt;

use super::{
    distraction_free::DistractionFreeMode,
    experimental::ExperimentalMode,
    fullscreen::FullscreenMode,
    full::FullMode,
    minimal::MinimalMode,
    safe::SafeMode,
    terminal_only::TerminalOnlyMode,
    Mode, ModePolicy,
};

/// Stable identifier for an application mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ModeId {
    Full,
    Minimal,
    TerminalOnly,
    Fullscreen,
    DistractionFree,
    Safe,
    Experimental,
}

impl ModeId {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Full => "full",
            Self::Minimal => "minimal",
            Self::TerminalOnly => "terminal-only",
            Self::Fullscreen => "fullscreen",
            Self::DistractionFree => "distraction-free",
            Self::Safe => "safe",
            Self::Experimental => "experimental",
        }
    }
}

impl fmt::Display for ModeId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Runtime state of the mode manager.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModeState {
    /// No mode has been activated yet.
    Uninitialized,

    /// A mode is currently active.
    Active(ModeId),

    /// Mode switching has temporarily been suspended.
    Suspended(ModeId),
}

/// Errors produced by mode management.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModeError {
    NotFound(ModeId),
    AlreadyRegistered(ModeId),
    CannotSwitchWhileSuspended,
}

impl fmt::Display for ModeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(id) => write!(formatter, "mode '{}' was not found", id),
            Self::AlreadyRegistered(id) => {
                write!(formatter, "mode '{}' is already registered", id)
            }
            Self::CannotSwitchWhileSuspended => {
                formatter.write_str("cannot switch modes while mode manager is suspended")
            }
        }
    }
}

impl std::error::Error for ModeError {}

/// Manages available modes and the currently active mode.
pub struct ModeManager {
    modes: BTreeMap<ModeId, Box<dyn Mode>>,
    state: ModeState,
}

impl Default for ModeManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ModeManager {
    /// Creates a manager containing all built-in modes.
    pub fn new() -> Self {
        let mut manager = Self {
            modes: BTreeMap::new(),
            state: ModeState::Uninitialized,
        };

        manager.register(Box::new(FullMode)).ok();
        manager.register(Box::new(MinimalMode)).ok();
        manager.register(Box::new(TerminalOnlyMode)).ok();
        manager.register(Box::new(FullscreenMode)).ok();
        manager.register(Box::new(DistractionFreeMode)).ok();
        manager.register(Box::new(SafeMode)).ok();
        manager.register(Box::new(ExperimentalMode)).ok();

        manager
    }

    /// Registers a mode.
    pub fn register(&mut self, mode: Box<dyn Mode>) -> Result<(), ModeError> {
        let id = mode.id();

        if self.modes.contains_key(&id) {
            return Err(ModeError::AlreadyRegistered(id));
        }

        self.modes.insert(id, mode);
        Ok(())
    }

    /// Registers a mode, replacing an existing mode with the same ID.
    pub fn register_or_replace(&mut self, mode: Box<dyn Mode>) {
        self.modes.insert(mode.id(), mode);
    }

    /// Returns the currently active mode.
    pub fn current(&self) -> Option<ModeId> {
        match self.state {
            ModeState::Active(id) | ModeState::Suspended(id) => Some(id),
            ModeState::Uninitialized => None,
        }
    }

    /// Returns the current manager state.
    pub const fn state(&self) -> ModeState {
        self.state
    }

    /// Returns a registered mode.
    pub fn get(&self, id: ModeId) -> Option<&dyn Mode> {
        self.modes.get(&id).map(|mode| mode.as_ref())
    }

    /// Returns the policy of the active mode.
    pub fn policy(&self) -> Option<ModePolicy> {
        self.current()
            .and_then(|id| self.modes.get(&id))
            .map(|mode| mode.policy())
    }

    /// Activates a mode.
    pub fn activate(&mut self, id: ModeId) -> Result<(), ModeError> {
        if matches!(self.state, ModeState::Suspended(_)) {
            return Err(ModeError::CannotSwitchWhileSuspended);
        }

        let new_mode = self
            .modes
            .get(&id)
            .ok_or(ModeError::NotFound(id))?;

        if let ModeState::Active(current) = self.state {
            if current == id {
                return Ok(());
            }

            if let Some(old_mode) = self.modes.get(&current) {
                old_mode.exit();
            }
        }

        new_mode.enter();
        self.state = ModeState::Active(id);

        Ok(())
    }

    /// Activates the full interface mode.
    pub fn activate_full(&mut self) -> Result<(), ModeError> {
        self.activate(ModeId::Full)
    }

    /// Activates minimal mode.
    pub fn activate_minimal(&mut self) -> Result<(), ModeError> {
        self.activate(ModeId::Minimal)
    }

    /// Activates terminal-only mode.
    pub fn activate_terminal_only(&mut self) -> Result<(), ModeError> {
        self.activate(ModeId::TerminalOnly)
    }

    /// Activates fullscreen mode.
    pub fn activate_fullscreen(&mut self) -> Result<(), ModeError> {
        self.activate(ModeId::Fullscreen)
    }

    /// Activates distraction-free mode.
    pub fn activate_distraction_free(&mut self) -> Result<(), ModeError> {
        self.activate(ModeId::DistractionFree)
    }

    /// Activates safe mode.
    pub fn activate_safe(&mut self) -> Result<(), ModeError> {
        self.activate(ModeId::Safe)
    }

    /// Activates experimental mode.
    pub fn activate_experimental(&mut self) -> Result<(), ModeError> {
        self.activate(ModeId::Experimental)
    }

    /// Suspends mode switching while retaining the current mode.
    pub fn suspend(&mut self) {
        if let ModeState::Active(id) = self.state {
            self.state = ModeState::Suspended(id);
        }
    }

    /// Resumes mode switching.
    pub fn resume(&mut self) {
        if let ModeState::Suspended(id) = self.state {
            self.state = ModeState::Active(id);
        }
    }

    /// Returns all registered mode IDs.
    pub fn ids(&self) -> impl Iterator<Item = ModeId> + '_ {
        self.modes.keys().copied()
    }

    /// Returns the number of registered modes.
    pub fn len(&self) -> usize {
        self.modes.len()
    }

    /// Returns whether no modes are registered.
    pub fn is_empty(&self) -> bool {
        self.modes.is_empty()
    }
}
