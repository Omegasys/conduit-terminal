use super::{Mode, ModeId, ModePolicy};

/// Fullscreen mode.
///
/// Keeps the normal Conduit interface but expands the application to the
/// entire display.
#[derive(Debug, Clone, Copy, Default)]
pub struct FullscreenMode;

impl Mode for FullscreenMode {
    fn id(&self) -> ModeId {
        ModeId::Fullscreen
    }

    fn name(&self) -> &'static str {
        "Fullscreen"
    }

    fn description(&self) -> &'static str {
        "Uses the entire display while retaining the normal Conduit interface."
    }

    fn policy(&self) -> ModePolicy {
        ModePolicy::fullscreen()
    }
}
