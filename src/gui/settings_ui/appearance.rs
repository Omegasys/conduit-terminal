#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeMode {
    System,
    Light,
    Dark,
    Custom,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CursorStyle {
    Block,
    Underline,
    Bar,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabBarStyle {
    Standard,
    Compact,
    Expanded,
    Minimal,
}

#[derive(Clone, Debug)]
pub struct AppearanceSettings {
    pub theme_mode: ThemeMode,
    pub custom_theme: Option<String>,
    pub cursor_style: CursorStyle,
    pub tab_bar_style: TabBarStyle,
    pub toolbar_visible: bool,
    pub status_bar_visible: bool,
    pub sidebar_visible: bool,
    pub window_decorations: bool,
    pub animations: bool,
    pub transparency: bool,
    pub opacity: f32,
    pub gradients: bool,
    pub truecolor: bool,
}

impl Default for AppearanceSettings {
    fn default() -> Self {
        Self {
            theme_mode: ThemeMode::System,
            custom_theme: None,
            cursor_style: CursorStyle::Block,
            tab_bar_style: TabBarStyle::Standard,
            toolbar_visible: true,
            status_bar_visible: true,
            sidebar_visible: true,
            window_decorations: true,
            animations: true,
            transparency: true,
            opacity: 1.0,
            gradients: false,
            truecolor: true,
        }
    }
}

impl AppearanceSettings {
    pub fn set_theme_mode(&mut self, mode: ThemeMode) {
        self.theme_mode = mode;
    }

    pub fn set_custom_theme(&mut self, theme: Option<String>) {
        self.custom_theme = theme;
        self.theme_mode = ThemeMode::Custom;
    }

    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity.clamp(0.0, 1.0);
    }

    pub fn set_transparency(&mut self, enabled: bool) {
        self.transparency = enabled;

        if !enabled {
            self.opacity = 1.0;
        }
    }

    pub fn set_gradients(&mut self, enabled: bool) {
        self.gradients = enabled;
    }

    pub fn set_truecolor(&mut self, enabled: bool) {
        self.truecolor = enabled;
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
