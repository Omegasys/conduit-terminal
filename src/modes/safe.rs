use super::{Mode, ModeId, ModePolicy};

/// Safe mode.
///
/// Designed for opening or operating Conduit in a more restricted
/// environment. UI remains available, while plugins and external
/// integrations are disabled by policy.
#[derive(Debug, Clone, Copy, Default)]
pub struct SafeMode;

impl Mode for SafeMode {
    fn id(&self) -> ModeId {
        ModeId::Safe
    }

    fn name(&self) -> &'static str {
        "Safe Mode"
    }

    fn description(&self) -> &'static str {
        "Restricts plugins and external integrations while retaining the main interface."
    }

    fn policy(&self) -> ModePolicy {
        ModePolicy::safe()
    }
}
