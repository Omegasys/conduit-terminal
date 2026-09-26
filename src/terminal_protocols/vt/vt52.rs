use super::{VtCapabilities, VtLevel};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vt52;

impl Vt52 {
    pub fn level(&self) -> VtLevel {
        VtLevel::Vt52
    }

    pub fn capabilities(&self) -> VtCapabilities {
        VtCapabilities::for_level(self.level())
    }

    pub fn enter(&self) -> &'static [u8] {
        b"\x1b[?2l"
    }

    pub fn exit(&self) -> &'static [u8] {
        b"\x1b[?2h"
    }

    pub fn identify(&self) -> &'static [u8] {
        b"\x1b/Z"
    }
}
