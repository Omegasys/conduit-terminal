use std::collections::BTreeMap;

use crate::config_engine::ConfigValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderingBackend {
    Automatic,
    OpenGl,
    Vulkan,
    Software,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VsyncMode {
    Automatic,
    Enabled,
    Disabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AntialiasingMode {
    None,
    Grayscale,
    Subpixel,
}

#[derive(Debug, Clone)]
pub struct RenderingSettings {
    backend: RenderingBackend,
    vsync: VsyncMode,
    antialiasing: AntialiasingMode,
    gpu_acceleration: bool,
    damage_tracking: bool,
    frame_rate_limit: u32,
    scroll_animation: bool,
    cursor_animation: bool,
    smooth_scrolling: bool,
    ligatures: bool,
    font_hinting: bool,
    fractional_scaling: bool,
    scale_factor: f64,
    use_system_scale: bool,
    render_bold_as_bright: bool,
    sixel_enabled: bool,
    kitty_graphics_enabled: bool,
    image_cache_size_mb: usize,
}

impl Default for RenderingSettings {
    fn default() -> Self {
        Self {
            backend: RenderingBackend::Automatic,
            vsync: VsyncMode::Automatic,
            antialiasing: AntialiasingMode::Grayscale,
            gpu_acceleration: true,
            damage_tracking: true,
            frame_rate_limit: 120,
            scroll_animation: true,
            cursor_animation: true,
            smooth_scrolling: true,
            ligatures: true,
            font_hinting: true,
            fractional_scaling: true,
            scale_factor: 1.0,
            use_system_scale: true,
            render_bold_as_bright: true,
            sixel_enabled: true,
            kitty_graphics_enabled: true,
            image_cache_size_mb: 128,
        }
    }
}

impl RenderingSettings {
    pub fn backend(&self) -> RenderingBackend {
        self.backend
    }

    pub fn set_backend(&mut self, value: RenderingBackend) {
        self.backend = value;
    }

    pub fn vsync(&self) -> VsyncMode {
        self.vsync
    }

    pub fn set_vsync(&mut self, value: VsyncMode) {
        self.vsync = value;
    }

    pub fn antialiasing(&self) -> AntialiasingMode {
        self.antialiasing
    }

    pub fn set_antialiasing(&mut self, value: AntialiasingMode) {
        self.antialiasing = value;
    }

    pub fn gpu_acceleration(&self) -> bool {
        self.gpu_acceleration
    }

    pub fn set_gpu_acceleration(&mut self, value: bool) {
        self.gpu_acceleration = value;
    }

    pub fn damage_tracking(&self) -> bool {
        self.damage_tracking
    }

    pub fn set_damage_tracking(&mut self, value: bool) {
        self.damage_tracking = value;
    }

    pub fn frame_rate_limit(&self) -> u32 {
        self.frame_rate_limit
    }

    pub fn set_frame_rate_limit(&mut self, value: u32) {
        self.frame_rate_limit = value.clamp(30, 1000);
    }

    pub fn scroll_animation(&self) -> bool {
        self.scroll_animation
    }

    pub fn set_scroll_animation(&mut self, value: bool) {
        self.scroll_animation = value;
    }

    pub fn cursor_animation(&self) -> bool {
        self.cursor_animation
    }

    pub fn set_cursor_animation(&mut self, value: bool) {
        self.cursor_animation = value;
    }

    pub fn smooth_scrolling(&self) -> bool {
        self.smooth_scrolling
    }

    pub fn set_smooth_scrolling(&mut self, value: bool) {
        self.smooth_scrolling = value;
    }

    pub fn ligatures(&self) -> bool {
        self.ligatures
    }

    pub fn set_ligatures(&mut self, value: bool) {
        self.ligatures = value;
    }

    pub fn font_hinting(&self) -> bool {
        self.font_hinting
    }

    pub fn set_font_hinting(&mut self, value: bool) {
        self.font_hinting = value;
    }

    pub fn fractional_scaling(&self) -> bool {
        self.fractional_scaling
    }

    pub fn set_fractional_scaling(&mut self, value: bool) {
        self.fractional_scaling = value;
    }

    pub fn scale_factor(&self) -> f64 {
        self.scale_factor
    }

    pub fn set_scale_factor(&mut self, value: f64) {
        self.scale_factor = value.clamp(0.5, 4.0);
        self.use_system_scale = false;
    }

    pub fn use_system_scale(&self) -> bool {
        self.use_system_scale
    }

    pub fn set_use_system_scale(&mut self, value: bool) {
        self.use_system_scale = value;
    }

    pub fn render_bold_as_bright(&self) -> bool {
        self.render_bold_as_bright
    }

    pub fn set_render_bold_as_bright(&mut self, value: bool) {
        self.render_bold_as_bright = value;
    }

    pub fn sixel_enabled(&self) -> bool {
        self.sixel_enabled
    }

    pub fn set_sixel_enabled(&mut self, value: bool) {
        self.sixel_enabled = value;
    }

    pub fn kitty_graphics_enabled(&self) -> bool {
        self.kitty_graphics_enabled
    }

    pub fn set_kitty_graphics_enabled(&mut self, value: bool) {
        self.kitty_graphics_enabled = value;
    }

    pub fn image_cache_size_mb(&self) -> usize {
        self.image_cache_size_mb
    }

    pub fn set_image_cache_size_mb(&mut self, value: usize) {
        self.image_cache_size_mb = value.clamp(16, 4096);
    }

    pub fn to_config(&self) -> BTreeMap<String, ConfigValue> {
        let mut values = BTreeMap::new();

        values.insert(
            "backend".into(),
            ConfigValue::String(format!("{:?}", self.backend).to_lowercase()),
        );
        values.insert(
            "vsync".into(),
            ConfigValue::String(format!("{:?}", self.vsync).to_lowercase()),
        );
        values.insert(
            "antialiasing".into(),
            ConfigValue::String(format!("{:?}", self.antialiasing).to_lowercase()),
        );
        values.insert(
            "gpu_acceleration".into(),
            ConfigValue::Boolean(self.gpu_acceleration),
        );
        values.insert(
            "damage_tracking".into(),
            ConfigValue::Boolean(self.damage_tracking),
        );
        values.insert(
            "frame_rate_limit".into(),
            ConfigValue::Integer(self.frame_rate_limit as i64),
        );
        values.insert(
            "scroll_animation".into(),
            ConfigValue::Boolean(self.scroll_animation),
        );
        values.insert(
            "cursor_animation".into(),
            ConfigValue::Boolean(self.cursor_animation),
        );
        values.insert(
            "smooth_scrolling".into(),
            ConfigValue::Boolean(self.smooth_scrolling),
        );
        values.insert("ligatures".into(), ConfigValue::Boolean(self.ligatures));
        values.insert(
            "font_hinting".into(),
            ConfigValue::Boolean(self.font_hinting),
        );
        values.insert(
            "fractional_scaling".into(),
            ConfigValue::Boolean(self.fractional_scaling),
        );
        values.insert(
            "scale_factor".into(),
            ConfigValue::Float(self.scale_factor),
        );
        values.insert(
            "use_system_scale".into(),
            ConfigValue::Boolean(self.use_system_scale),
        );
        values.insert(
            "render_bold_as_bright".into(),
            ConfigValue::Boolean(self.render_bold_as_bright),
        );
        values.insert(
            "sixel_enabled".into(),
            ConfigValue::Boolean(self.sixel_enabled),
        );
        values.insert(
            "kitty_graphics_enabled".into(),
            ConfigValue::Boolean(self.kitty_graphics_enabled),
        );
        values.insert(
            "image_cache_size_mb".into(),
            ConfigValue::Integer(self.image_cache_size_mb as i64),
        );

        values
    }
}
