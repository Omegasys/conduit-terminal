#[derive(Debug, Clone)]
pub struct ColorScheme {
    pub foreground: String,
    pub background: String,
    pub cursor: String,
    pub selection: String,
    pub border: String,
    pub accent: String,
}

impl ColorScheme {
    pub fn new() -> Self {
        Self {
            foreground: "#D8DEE9".to_string(),
            background: "#2E3440".to_string(),
            cursor: "#D8DEE9".to_string(),
            selection: "#4C566A".to_string(),
            border: "#434C5E".to_string(),
            accent: "#88C0D0".to_string(),
        }
    }

    pub fn set_foreground(&mut self, value: impl Into<String>) {
        self.foreground = value.into();
    }

    pub fn set_background(&mut self, value: impl Into<String>) {
        self.background = value.into();
    }

    pub fn set_cursor(&mut self, value: impl Into<String>) {
        self.cursor = value.into();
    }

    pub fn set_selection(&mut self, value: impl Into<String>) {
        self.selection = value.into();
    }

    pub fn set_border(&mut self, value: impl Into<String>) {
        self.border = value.into();
    }

    pub fn set_accent(&mut self, value: impl Into<String>) {
        self.accent = value.into();
    }
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct ColorSettings {
    scheme: ColorScheme,
    use_theme_palette: bool,
    dim_inactive_panes: bool,
    inactive_pane_opacity: f32,
    bold_is_bright: bool,
}

impl ColorSettings {
    pub fn new() -> Self {
        Self {
            scheme: ColorScheme::new(),
            use_theme_palette: true,
            dim_inactive_panes: false,
            inactive_pane_opacity: 0.75,
            bold_is_bright: true,
        }
    }

    pub fn scheme(&self) -> &ColorScheme {
        &self.scheme
    }

    pub fn scheme_mut(&mut self) -> &mut ColorScheme {
        &mut self.scheme
    }

    pub fn use_theme_palette(&self) -> bool {
        self.use_theme_palette
    }

    pub fn dim_inactive_panes(&self) -> bool {
        self.dim_inactive_panes
    }

    pub fn inactive_pane_opacity(&self) -> f32 {
        self.inactive_pane_opacity
    }

    pub fn bold_is_bright(&self) -> bool {
        self.bold_is_bright
    }

    pub fn set_use_theme_palette(&mut self, value: bool) {
        self.use_theme_palette = value;
    }

    pub fn set_dim_inactive_panes(&mut self, value: bool) {
        self.dim_inactive_panes = value;
    }

    pub fn set_inactive_pane_opacity(&mut self, value: f32) {
        self.inactive_pane_opacity = value.clamp(0.0, 1.0);
    }

    pub fn set_bold_is_bright(&mut self, value: bool) {
        self.bold_is_bright = value;
    }
}

impl Default for ColorSettings {
    fn default() -> Self {
        Self::new()
    }
}
