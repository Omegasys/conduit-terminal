use super::{Mode, ModeId, ModePolicy};

/// Experimental mode.
///
/// Enables features explicitly marked as experimental. Individual
/// experimental features should still perform their own capability and
/// security checks.
#[derive(Debug, Clone, Copy, Default)]
pub struct ExperimentalMode;

impl Mode for ExperimentalMode {
    fn id(&self) -> ModeId {
        ModeId::Experimental
    }

    fn name(&self) -> &'static str {
        "Experimental"
    }

    fn description(&self) -> &'static str {
        "Enables Conduit features that are explicitly marked experimental."
    }

    fn policy(&self) -> ModePolicy {
        ModePolicy::experimental()
    }
}
