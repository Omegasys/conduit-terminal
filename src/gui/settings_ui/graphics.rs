#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GraphicsBackend {
    Auto,
    Software,
    OpenGL,
    Vulkan,
    Metal,
    DirectX,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImageScalingMode {
    Nearest,
    Linear,
    Cubic,
    Lanczos,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransparencyMode {
    Disabled,
    Simple,
    Composited,
    Blurred,
}

#[derive(Clone, Debug)]
pub struct GraphicsSettings {
    pub backend: GraphicsBackend,
    pub vsync: bool,
    pub damage_tracking: bool,
    pub hardware_cursor: bool,
    pub animations: bool,
    pub smooth_scrolling: bool,
    pub image_rendering: bool,
    pub sixel: bool,
    pub kitty_graphics: bool,
    pub iterm_graphics: bool,
    pub image_scaling: ImageScalingMode,
    pub transparency: TransparencyMode,
    pub opacity: f32,
    pub blur: bool,
    pub high_dpi: bool,
    pub fractional_scaling: bool,
    pub truecolor: bool,
    pub wide_gamut: bool,
}

impl Default for GraphicsSettings {
    fn default() -> Self {
        Self {
            backend: GraphicsBackend::Auto,
            vsync: true,
            damage_tracking: true,
            hardware_cursor: true,
            animations: true,
            smooth_scrolling: true,
            image_rendering: true,
            sixel: true,
            kitty_graphics: true,
            iterm_graphics: true,
            image_scaling: ImageScalingMode::Linear,
            transparency: TransparencyMode::Composited,
            opacity: 1.0,
            blur: false,
            high_dpi: true,
            fractional_scaling: true,
            truecolor: true,
            wide_gamut: false,
        }
    }
}

impl GraphicsSettings {
    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity.clamp(0.0, 1.0);
    }

    pub fn set_transparency(&mut self, mode: TransparencyMode) {
        self.transparency = mode;

        if matches!(mode, TransparencyMode::Disabled) {
            self.opacity = 1.0;
            self.blur = false;
        }
    }

    pub fn set_truecolor(&mut self, enabled: bool) {
        self.truecolor = enabled;
    }

    pub fn set_wide_gamut(&mut self, enabled: bool) {
        self.wide_gamut = enabled;
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
