use std::collections::BTreeMap;

use super::resource::{Resource, ResourceError, ResourceId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThemeColors {
    pub foreground: String,
    pub background: String,
    pub cursor: String,
    pub selection: String,
    pub border: String,
    pub accent: String,
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            foreground: "#FFFFFF".to_owned(),
            background: "#000000".to_owned(),
            cursor: "#FFFFFF".to_owned(),
            selection: "#444444".to_owned(),
            border: "#666666".to_owned(),
            accent: "#66CCFF".to_owned(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThemePalette {
    pub black: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub magenta: String,
    pub cyan: String,
    pub white: String,
    pub bright_black: String,
    pub bright_red: String,
    pub bright_green: String,
    pub bright_yellow: String,
    pub bright_blue: String,
    pub bright_magenta: String,
    pub bright_cyan: String,
    pub bright_white: String,
}

impl Default for ThemePalette {
    fn default() -> Self {
        Self {
            black: "#000000".into(),
            red: "#CC0000".into(),
            green: "#00CC00".into(),
            yellow: "#CCCC00".into(),
            blue: "#0000CC".into(),
            magenta: "#CC00CC".into(),
            cyan: "#00CCCC".into(),
            white: "#CCCCCC".into(),
            bright_black: "#555555".into(),
            bright_red: "#FF5555".into(),
            bright_green: "#55FF55".into(),
            bright_yellow: "#FFFF55".into(),
            bright_blue: "#5555FF".into(),
            bright_magenta: "#FF55FF".into(),
            bright_cyan: "#55FFFF".into(),
            bright_white: "#FFFFFF".into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Theme {
    id: ResourceId,
    name: String,
    description: String,
    author: Option<String>,
    version: Option<String>,
    colors: ThemeColors,
    palette: ThemePalette,
    attributes: BTreeMap<String, String>,
}

impl Theme {
    pub fn new(
        id: ResourceId,
        name: impl Into<String>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            description: String::new(),
            author: None,
            version: None,
            colors: ThemeColors::default(),
            palette: ThemePalette::default(),
            attributes: BTreeMap::new(),
        }
    }

    pub fn from_resource(resource: &Resource) -> Result<Self, ResourceError> {
        if resource.kind() != super::resource::ResourceKind::Theme {
            return Err(ResourceError::InvalidResource(
                "resource is not a theme".to_owned(),
            ));
        }

        Ok(Self::new(resource.id(), resource.name()))
    }

    pub fn id(&self) -> ResourceId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn author(&self) -> Option<&str> {
        self.author.as_deref()
    }

    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    pub fn colors(&self) -> &ThemeColors {
        &self.colors
    }

    pub fn palette(&self) -> &ThemePalette {
        &self.palette
    }

    pub fn attributes(&self) -> &BTreeMap<String, String> {
        &self.attributes
    }

    pub fn set_description(&mut self, value: impl Into<String>) {
        self.description = value.into();
    }

    pub fn set_author(&mut self, value: impl Into<String>) {
        self.author = Some(value.into());
    }

    pub fn set_version(&mut self, value: impl Into<String>) {
        self.version = Some(value.into());
    }

    pub fn set_colors(&mut self, colors: ThemeColors) {
        self.colors = colors;
    }

    pub fn set_palette(&mut self, palette: ThemePalette) {
        self.palette = palette;
    }

    pub fn set_attribute(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) {
        self.attributes.insert(key.into(), value.into());
    }
}
