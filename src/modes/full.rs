use super::{Mode, ModeId, ModePolicy};

/// Full Conduit interface.
///
/// This is the normal desktop experience with the complete application
/// interface available.
#[derive(Debug, Clone, Copy, Default)]
pub struct FullMode;

impl Mode for FullMode {
    fn id(&self) -> ModeId {
        ModeId::Full
    }

    fn name(&self) -> &'static str {
        "Full"
    }

    fn description(&self) -> &'static str {
        "The complete Conduit interface with all normal UI components enabled."
    }

    fn policy(&self) -> ModePolicy {
        ModePolicy::full()
    }
}
