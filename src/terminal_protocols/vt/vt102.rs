use super::{Vt100, VtCapabilities, VtLevel};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vt102;

impl Vt102 {
    pub fn level(&self) -> VtLevel {
        VtLevel::Vt102
    }

    pub fn capabilities(&self) -> VtCapabilities {
        VtCapabilities::for_level(self.level())
    }

    pub fn base(&self) -> Vt100 {
        Vt100
    }
}
