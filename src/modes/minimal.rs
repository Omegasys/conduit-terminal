use super::{Mode, ModeId, ModePolicy};

/// Minimal interface mode.
///
/// Removes most application chrome while retaining the essential terminal
/// and tab interfaces.
#[derive(Debug, Clone, Copy, Default)]
pub struct MinimalMode;

impl Mode for MinimalMode {
    fn id(&self) -> ModeId {
        ModeId::Minimal
    }

    fn name(&self) -> &'static str {
        "Minimal"
    }

    fn description(&self) -> &'static str {
        "A reduced interface focused on terminals and essential navigation."
    }

    fn policy(&self) -> ModePolicy {
        ModePolicy::minimal()
    }
}
