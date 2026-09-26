use super::{Vt420, VtCapabilities, VtLevel};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vt520;

impl Vt520 {
    pub fn level(&self) -> VtLevel {
        VtLevel::Vt520
    }

    pub fn capabilities(&self) -> VtCapabilities {
        VtCapabilities::for_level(self.level())
    }

    pub fn base(&self) -> Vt420 {
        Vt420
    }
}
