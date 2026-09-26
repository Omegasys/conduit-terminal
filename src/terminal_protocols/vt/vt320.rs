use super::{Vt220, VtCapabilities, VtLevel};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vt320;

impl Vt320 {
    pub fn level(&self) -> VtLevel {
        VtLevel::Vt320
    }

    pub fn capabilities(&self) -> VtCapabilities {
        VtCapabilities::for_level(self.level())
    }

    pub fn base(&self) -> Vt220 {
        Vt220
    }
}
