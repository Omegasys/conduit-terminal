use super::{Vt520, VtCapabilities, VtLevel};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vt525;

impl Vt525 {
    pub fn level(&self) -> VtLevel {
        VtLevel::Vt525
    }

    pub fn capabilities(&self) -> VtCapabilities {
        VtCapabilities::for_level(self.level())
    }

    pub fn base(&self) -> Vt520 {
        Vt520
    }
}
