use super::{VtCapabilities, VtLevel};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vt220;

impl Vt220 {
    pub fn level(&self) -> VtLevel {
        VtLevel::Vt220
    }

    pub fn capabilities(&self) -> VtCapabilities {
        VtCapabilities::for_level(self.level())
    }

    pub fn identify(&self) -> &'static [u8] {
        b"\x1b[c"
    }

    pub fn soft_reset(&self) -> &'static [u8] {
        b"\x1b[!p"
    }
}
