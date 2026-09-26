use super::{VtCapabilities, VtLevel};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vt100;

impl Vt100 {
    pub fn level(&self) -> VtLevel {
        VtLevel::Vt100
    }

    pub fn capabilities(&self) -> VtCapabilities {
        VtCapabilities::for_level(self.level())
    }

    pub fn reset(&self) -> &'static [u8] {
        b"\x1b[c"
    }

    pub fn identify(&self) -> &'static [u8] {
        b"\x1b[c"
    }
}
