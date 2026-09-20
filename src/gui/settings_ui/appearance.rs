#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    System,
    Light,
    Dark,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorStyle {
    Block,
    Underline,
    Bar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabBarStyle {
    Standard,
    Compact,
    Expanded,
    Minimal,
}

#[derive(Debug, Clone)]
pub struct AppearanceSettings {
    theme_mode: ThemeMode,
    theme_name: String,
    cursor_style: CursorStyle,
    cursor_blink: bool,
    tab_bar_style: TabBarStyle,
    show_toolbar: bool,
    show_status_bar: bool,
    show_sidebar: bool,
    show_window_decorations: bool,
    animations_enabled: bool,
    transparency_enabled: bool,
    opacity: f32,
}

impl AppearanceSettings {
    pub fn new() -> Self {
        Self {
            theme_mode: ThemeMode::System,
            theme_name: "default".to_string(),
            cursor_style: CursorStyle::Block,
            cursor_blink: true,
            tab_bar_style: TabBarStyle::Standard,
            show_toolbar: true,
            show_status_bar: true,
            show_sidebar: true,
            show_window_decorations: true,
            animations_enabled: true,
            transparency_enabled: false,
            opacity: 1.0,
        }
    }

    pub fn theme_mode(&self) -> ThemeMode {
        self.theme_mode
    }

    pub fn theme_name(&self) -> &str {
        &self.theme_name
    }

    pub fn cursor_style(&self) -> CursorStyle {
        self.cursor_style
    }

    pub fn cursor_blink(&self) -> bool {
        self.cursor_blink
    }

    pub fn tab_bar_style(&self) -> TabBarStyle {
        self.tab_bar_style
    }

    pub fn show_toolbar(&self) -> bool {
        self.show_toolbar
    }

    pub fn show_status_bar(&self) -> bool {
        self.show_status_bar
    }

    pub fn show_sidebar(&self) -> bool {
        self.show_sidebar
    }

    pub fn show_window_decorations(&self) -> bool {
        self.show_window_decorations
    }

    pub fn animations_enabled(&self) -> bool {
        self.animations_enabled
    }

    pub fn transparency_enabled(&self) -> bool {
        self.transparency_enabled
    }

    pub fn opacity(&self) -> f32 {
        self.opacity
    }

    pub fn set_theme_mode(&mut self, value: ThemeMode) {
        self.theme_mode = value;
    }

    pub fn set_theme_name(&mut self, value: impl Into<String>) {
        self.theme_name = value.into();
    }

    pub fn set_cursor_style(&mut self, value: CursorStyle) {
        self.cursor_style = value;
    }

    pub fn set_cursor_blink(&mut self, value: bool) {
        self.cursor_blink = value;
    }

    pub fn set_tab_bar_style(&mut self, value: TabBarStyle) {
        self.tab_bar_style = value;
    }

    pub fn set_show_toolbar(&mut self, value: bool) {
        self.show_toolbar = value;
    }

    pub fn set_show_status_bar(&mut self, value: bool) {
        self.show_status_bar = value;
    }

    pub fn set_show_sidebar(&mut self, value: bool) {
        self.show_sidebar = value;
    }

    pub fn set_show_window_decorations(&mut self, value: bool) {
        self.show_window_decorations = value;
    }

    pub fn set_animations_enabled(&mut self, value: bool) {
        self.animations_enabled = value;
    }

    pub fn set_transparency_enabled(&mut self, value: bool) {
        self.transparency_enabled = value;
    }

    pub fn set_opacity(&mut self, value: f32) {
        self.opacity = value.clamp(0.1, 1.0);
    }
}

impl Default for AppearanceSettings {
    fn default() -> Self {
        Self::new()
    }
}
