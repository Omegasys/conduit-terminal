use super::{Mode, ModeId, ModePolicy};

/// Terminal-only mode.
///
/// Hides Conduit's surrounding application interface and presents the
/// terminal workspace as the primary surface.
#[derive(Debug, Clone, Copy, Default)]
pub struct TerminalOnlyMode;

impl Mode for TerminalOnlyMode {
    fn id(&self) -> ModeId {
        ModeId::TerminalOnly
    }

    fn name(&self) -> &'static str {
        "Terminal Only"
    }

    fn description(&self) -> &'static str {
        "Displays only the terminal workspace and removes application chrome."
    }

    fn policy(&self) -> ModePolicy {
        ModePolicy::terminal_only()
    }
}
