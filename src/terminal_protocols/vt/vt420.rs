use super::{VtCapabilities, VtLevel, Vt340};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vt420;

impl Vt420 {
    pub fn level(&self) -> VtLevel {
        VtLevel::Vt420
    }

    pub fn capabilities(&self) -> VtCapabilities {
        VtCapabilities::for_level(self.level())
    }

    pub fn base(&self) -> Vt340 {
        Vt340
    }
}
