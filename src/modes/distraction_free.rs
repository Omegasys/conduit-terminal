use super::{Mode, ModeId, ModePolicy};

/// Distraction-free mode.
///
/// Removes non-essential visual elements while retaining enough navigation
/// to work with multiple terminals.
#[derive(Debug, Clone, Copy, Default)]
pub struct DistractionFreeMode;

impl Mode for DistractionFreeMode {
    fn id(&self) -> ModeId {
        ModeId::DistractionFree
    }

    fn name(&self) -> &'static str {
        "Distraction Free"
    }

    fn description(&self) -> &'static str {
        "Hides non-essential interface elements and emphasizes terminal work."
    }

    fn policy(&self) -> ModePolicy {
        ModePolicy::distraction_free()
    }
}
