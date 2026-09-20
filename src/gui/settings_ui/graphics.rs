use std::collections::BTreeMap;

use crate::config_engine::ConfigValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphicsBackend {
    Automatic,
    OpenGl,
    Vulkan,
    Software,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageScalingMode {
    Nearest,
    Linear,
    Cubic,
    Auto,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransparencyMode {
    Disabled,
    Window,
    Terminal,
    Both,
}

#[derive(Debug, Clone)]
pub struct GraphicsSettings {
    backend: GraphicsBackend,
    image_scaling: ImageScalingMode,
    transparency: TransparencyMode,
    gpu_acceleration: bool,
    vsync: bool,
    damage_tracking: bool,
    hardware_cursor: bool,
    animations: bool,
    smooth_scrolling: bool,
    image_rendering: bool,
    sixel: bool,
    kitty_graphics: bool,
    i_term_images: bool,
    sixel_max_width: u32,
    sixel_max_height: u32,
    image_cache_mb: usize,
    transparency_opacity: f64,
    background_blur: bool,
    blur_radius: u32,
    high_dpi: bool,
    fractional_scaling: bool,
}

impl Default for GraphicsSettings {
    fn default() -> Self {
        Self {
            backend: GraphicsBackend::Automatic,
            image_scaling: ImageScalingMode::Auto,
            transparency: TransparencyMode::Disabled,
            gpu_acceleration: true,
            vsync: true,
            damage_tracking: true,
            hardware_cursor: true,
            animations: true,
            smooth_scrolling: true,
            image_rendering: true,
            sixel: true,
            kitty_graphics: true,
            i_term_images: true,
            sixel_max_width: 4096,
            sixel_max_height: 4096,
            image_cache_mb: 128,
            transparency_opacity: 1.0,
            background_blur: false,
            blur_radius: 8,
            high_dpi: true,
            fractional_scaling: true,
        }
    }
}

impl GraphicsSettings {
    pub fn backend(&self) -> GraphicsBackend {
        self.backend
    }

    pub fn set_backend(&mut self, value: GraphicsBackend) {
        self.backend = value;
    }

    pub fn image_scaling(&self) -> ImageScalingMode {
        self.image_scaling
    }

    pub fn set_image_scaling(&mut self, value: ImageScalingMode) {
        self.image_scaling = value;
    }

    pub fn transparency(&self) -> TransparencyMode {
        self.transparency
    }

    pub fn set_transparency(&mut self, value: TransparencyMode) {
        self.transparency = value;
    }

    pub fn gpu_acceleration(&self) -> bool {
        self.gpu_acceleration
    }

    pub fn set_gpu_acceleration(&mut self, value: bool) {
        self.gpu_acceleration = value;
    }

    pub fn vsync(&self) -> bool {
        self.vsync
    }

    pub fn set_vsync(&mut self, value: bool) {
        self.vsync = value;
    }

    pub fn damage_tracking(&self) -> bool {
        self.damage_tracking
    }

    pub fn set_damage_tracking(&mut self, value: bool) {
        self.damage_tracking = value;
    }

    pub fn hardware_cursor(&self) -> bool {
        self.hardware_cursor
    }

    pub fn set_hardware_cursor(&mut self, value: bool) {
        self.hardware_cursor = value;
    }

    pub fn animations(&self) -> bool {
        self.animations
    }

    pub fn set_animations(&mut self, value: bool) {
        self.animations = value;
    }

    pub fn smooth_scrolling(&self) -> bool {
        self.smooth_scrolling
    }

    pub fn set_smooth_scrolling(&mut self, value: bool) {
        self.smooth_scrolling = value;
    }

    pub fn image_rendering(&self) -> bool {
        self.image_rendering
    }

    pub fn set_image_rendering(&mut self, value: bool) {
        self.image_rendering = value;
    }

    pub fn sixel(&self) -> bool {
        self.sixel
    }

    pub fn set_sixel(&mut self, value: bool) {
        self.sixel = value;
    }

    pub fn kitty_graphics(&self) -> bool {
        self.kitty_graphics
    }

    pub fn set_kitty_graphics(&mut self, value: bool) {
        self.kitty_graphics = value;
    }

    pub fn i_term_images(&self) -> bool {
        self.i_term_images
    }

    pub fn set_i_term_images(&mut self, value: bool) {
        self.i_term_images = value;
    }

    pub fn sixel_max_width(&self) -> u32 {
        self.sixel_max_width
    }

    pub fn set_sixel_max_width(&mut self, value: u32) {
        self.sixel_max_width = value.clamp(64, 32_768);
    }

    pub fn sixel_max_height(&self) -> u32 {
        self.sixel_max_height
    }

    pub fn set_sixel_max_height(&mut self, value: u32) {
        self.sixel_max_height = value.clamp(64, 32_768);
    }

    pub fn image_cache_mb(&self) -> usize {
        self.image_cache_mb
    }

    pub fn set_image_cache_mb(&mut self, value: usize) {
        self.image_cache_mb = value.clamp(16, 4096);
    }

    pub fn transparency_opacity(&self) -> f64 {
        self.transparency_opacity
    }

    pub fn set_transparency_opacity(&mut self, value: f64) {
        self.transparency_opacity = value.clamp(0.1, 1.0);
    }

    pub fn background_blur(&self) -> bool {
        self.background_blur
    }

    pub fn set_background_blur(&mut self, value: bool) {
        self.background_blur = value;
    }

    pub fn blur_radius(&self) -> u32 {
        self.blur_radius
    }

    pub fn set_blur_radius(&mut self, value: u32) {
        self.blur_radius = value.clamp(1, 100);
    }

    pub fn high_dpi(&self) -> bool {
        self.high_dpi
    }

    pub fn set_high_dpi(&mut self, value: bool) {
        self.high_dpi = value;
    }

    pub fn fractional_scaling(&self) -> bool {
        self.fractional_scaling
    }

    pub fn set_fractional_scaling(&mut self, value: bool) {
        self.fractional_scaling = value;
    }

    pub fn to_config(&self) -> BTreeMap<String, ConfigValue> {
        let mut values = BTreeMap::new();

        values.insert(
            "backend".into(),
            ConfigValue::String(format!("{:?}", self.backend).to_lowercase()),
        );
        values.insert(
            "image_scaling".into(),
            ConfigValue::String(format!("{:?}", self.image_scaling).to_lowercase()),
        );
        values.insert(
            "transparency".into(),
            ConfigValue::String(format!("{:?}", self.transparency).to_lowercase()),
        );
        values.insert(
            "gpu_acceleration".into(),
            ConfigValue::Boolean(self.gpu_acceleration),
        );
        values.insert("vsync".into(), ConfigValue::Boolean(self.vsync));
        values.insert(
            "damage_tracking".into(),
            ConfigValue::Boolean(self.damage_tracking),
        );
        values.insert(
            "hardware_cursor".into(),
            ConfigValue::Boolean(self.hardware_cursor),
        );
        values.insert(
            "animations".into(),
            ConfigValue::Boolean(self.animations),
        );
        values.insert(
            "smooth_scrolling".into(),
            ConfigValue::Boolean(self.smooth_scrolling),
        );
        values.insert(
            "image_rendering".into(),
            ConfigValue::Boolean(self.image_rendering),
        );
        values.insert("sixel".into(), ConfigValue::Boolean(self.sixel));
        values.insert(
            "kitty_graphics".into(),
            ConfigValue::Boolean(self.kitty_graphics),
        );
        values.insert(
            "i_term_images".into(),
            ConfigValue::Boolean(self.i_term_images),
        );
        values.insert(
            "sixel_max_width".into(),
            ConfigValue::Integer(self.sixel_max_width as i64),
        );
        values.insert(
            "sixel_max_height".into(),
            ConfigValue::Integer(self.sixel_max_height as i64),
        );
        values.insert(
            "image_cache_mb".into(),
            ConfigValue::Integer(self.image_cache_mb as i64),
        );
        values.insert(
            "transparency_opacity".into(),
            ConfigValue::Float(self.transparency_opacity),
        );
        values.insert(
            "background_blur".into(),
            ConfigValue::Boolean(self.background_blur),
        );
        values.insert(
            "blur_radius".into(),
            ConfigValue::Integer(self.blur_radius as i64),
        );
        values.insert("high_dpi".into(), ConfigValue::Boolean(self.high_dpi));
        values.insert(
            "fractional_scaling".into(),
            ConfigValue::Boolean(self.fractional_scaling),
        );

        values
    }
}
