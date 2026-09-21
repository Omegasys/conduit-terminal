#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProtocolColorSupport {
    None,
    Basic8,
    Standard16,
    Indexed256,
    TrueColor,
}

impl Default for ProtocolColorSupport {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProtocolImageSupport {
    None,
    Sixel,
    Iterm2,
    Kitty,
    Custom,
}

impl Default for ProtocolImageSupport {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProtocolMouseSupport {
    None,
    X10,
    Utf8,
    Sgr,
    Custom,
}

impl Default for ProtocolMouseSupport {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Debug)]
pub struct CustomProtocolCapabilities {
    pub color: ProtocolColorSupport,
    pub images: ProtocolImageSupport,
    pub mouse: ProtocolMouseSupport,

    pub clipboard: bool,
    pub bracketed_paste: bool,
    pub hyperlinks: bool,
    pub alternate_screen: bool,
    pub synchronized_updates: bool,
    pub focus_reporting: bool,
    pub unicode: bool,
    pub truecolor_alpha: bool,
}

impl Default for CustomProtocolCapabilities {
    fn default() -> Self {
        Self {
            color: ProtocolColorSupport::None,
            images: ProtocolImageSupport::None,
            mouse: ProtocolMouseSupport::None,
            clipboard: false,
            bracketed_paste: false,
            hyperlinks: false,
            alternate_screen: false,
            synchronized_updates: false,
            focus_reporting: false,
            unicode: true,
            truecolor_alpha: false,
        }
    }
}

impl CustomProtocolCapabilities {
    pub fn basic_terminal() -> Self {
        Self {
            color: ProtocolColorSupport::Basic8,
            ..Default::default()
        }
    }

    pub fn modern_terminal() -> Self {
        Self {
            color: ProtocolColorSupport::TrueColor,
            clipboard: true,
            bracketed_paste: true,
            hyperlinks: true,
            alternate_screen: true,
            synchronized_updates: true,
            focus_reporting: true,
            unicode: true,
            ..Default::default()
        }
    }

    pub fn supports_color(&self) -> bool {
        !matches!(self.color, ProtocolColorSupport::None)
    }

    pub fn supports_images(&self) -> bool {
        !matches!(self.images, ProtocolImageSupport::None)
    }

    pub fn supports_mouse(&self) -> bool {
        !matches!(self.mouse, ProtocolMouseSupport::None)
    }
}
