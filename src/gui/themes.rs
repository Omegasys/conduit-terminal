use std::collections::HashMap;

use crate::resources::{Theme, ThemeColors, ThemePalette};

/// GUI representation of a Conduit theme.
#[derive(Debug, Clone)]
pub struct GuiTheme {
    name: String,
    description: String,
    author: String,
    version: String,
    colors: ThemeColors,
    palette: ThemePalette,
    selected: bool,
    enabled: bool,
}

impl GuiTheme {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: String::new(),
            author: String::new(),
            version: String::from("1.0.0"),
            colors: ThemeColors::default(),
            palette: ThemePalette::default(),
            selected: false,
            enabled: true,
        }
    }

    pub fn from_resource(theme: &Theme) -> Self {
        Self {
            name: theme.name().to_string(),
            description: theme.description().to_string(),
            author: theme.author().to_string(),
            version: theme.version().to_string(),
            colors: theme.colors().clone(),
            palette: theme.palette().clone(),
            selected: false,
            enabled: true,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn author(&self) -> &str {
        &self.author
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn colors(&self) -> &ThemeColors {
        &self.colors
    }

    pub fn palette(&self) -> &ThemePalette {
        &self.palette
    }

    pub fn is_selected(&self) -> bool {
        self.selected
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_description(&mut self, value: impl Into<String>) {
        self.description = value.into();
    }

    pub fn set_author(&mut self, value: impl Into<String>) {
        self.author = value.into();
    }

    pub fn set_version(&mut self, value: impl Into<String>) {
        self.version = value.into();
    }

    pub fn set_colors(&mut self, colors: ThemeColors) {
        self.colors = colors;
    }

    pub fn set_palette(&mut self, palette: ThemePalette) {
        self.palette = palette;
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

/// Manages themes visible to the GUI.
#[derive(Debug, Default)]
pub struct GuiThemeManager {
    themes: HashMap<String, GuiTheme>,
    active: Option<String>,
}

impl GuiThemeManager {
    pub fn new() -> Self {
        Self {
            themes: HashMap::new(),
            active: None,
        }
    }

    pub fn add(&mut self, theme: GuiTheme) -> bool {
        let name = theme.name().to_string();

        if self.themes.contains_key(&name) {
            return false;
        }

        self.themes.insert(name, theme);
        true
    }

    pub fn add_from_resource(&mut self, theme: &Theme) -> bool {
        self.add(GuiTheme::from_resource(theme))
    }

    pub fn remove(&mut self, name: &str) -> Option<GuiTheme> {
        let removed = self.themes.remove(name)?;

        if self.active.as_deref() == Some(name) {
            self.active = None;
        }

        Some(removed)
    }

    pub fn get(&self, name: &str) -> Option<&GuiTheme> {
        self.themes.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut GuiTheme> {
        self.themes.get_mut(name)
    }

    pub fn activate(&mut self, name: &str) -> bool {
        if !self.themes.contains_key(name) {
            return false;
        }

        for theme in self.themes.values_mut() {
            theme.set_selected(false);
        }

        if let Some(theme) = self.themes.get_mut(name) {
            theme.set_selected(true);
        }

        self.active = Some(name.to_string());
        true
    }

    pub fn active(&self) -> Option<&str> {
        self.active.as_deref()
    }

    pub fn iter(&self) -> impl Iterator<Item = &GuiTheme> {
        self.themes.values()
    }

    pub fn len(&self) -> usize {
        self.themes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.themes.is_empty()
    }

    pub fn clear(&mut self) {
        self.themes.clear();
        self.active = None;
    }
}
