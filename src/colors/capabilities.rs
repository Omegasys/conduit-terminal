use super::ansi::AnsiColorDepth;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorCapability {
    Basic8,
    Standard16,
    Indexed256,
    TrueColor,
    Alpha,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorCapabilities {
    depth: AnsiColorDepth,
    alpha: bool,
}

impl ColorCapabilities {
    pub fn new(depth: AnsiColorDepth, alpha: bool) -> Self {
        Self { depth, alpha }
    }

    pub fn basic() -> Self {
        Self::new(AnsiColorDepth::Basic8, false)
    }

    pub fn standard() -> Self {
        Self::new(AnsiColorDepth::Standard16, false)
    }

    pub fn indexed() -> Self {
        Self::new(AnsiColorDepth::Indexed256, false)
    }

    pub fn truecolor() -> Self {
        Self::new(AnsiColorDepth::TrueColor, false)
    }

    pub fn truecolor_alpha() -> Self {
        Self::new(AnsiColorDepth::TrueColor, true)
    }

    pub fn depth(&self) -> AnsiColorDepth {
        self.depth
    }

    pub fn supports(&self, capability: ColorCapability) -> bool {
        match capability {
            ColorCapability::Basic8 => true,
            ColorCapability::Standard16 => {
                AnsiColorDepth::depth_supports(self.depth, AnsiColorDepth::Standard16)
            }
            ColorCapability::Indexed256 => {
                AnsiColorDepth::depth_supports(self.depth, AnsiColorDepth::Indexed256)
            }
            ColorCapability::TrueColor => {
                AnsiColorDepth::depth_supports(self.depth, AnsiColorDepth::TrueColor)
            }
            ColorCapability::Alpha => self.alpha,
        }
    }

    pub fn alpha(&self) -> bool {
        self.alpha
    }

    pub fn set_depth(&mut self, depth: AnsiColorDepth) {
        self.depth = depth;
    }

    pub fn set_alpha(&mut self, enabled: bool) {
        self.alpha = enabled;
    }
}

impl Default for ColorCapabilities {
    fn default() -> Self {
        Self::truecolor()
    }
}
