use crate::colors::{
    Color,
    ColorPalette,
    Rgba,
    SemanticColor,
    SemanticColorMap,
};
use crate::resources::{
    Resource,
    ResourceError,
    ResourceKind,
};

#[derive(Clone, Debug)]
pub struct ThemeColors {
    pub foreground: Color,
    pub background: Color,
    pub cursor: Color,
    pub selection: Color,
    pub border: Color,
    pub accent: Color,
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            foreground: Color::rgb(216, 222, 233),
            background: Color::rgb(46, 52, 64),
            cursor: Color::rgb(136, 192, 208),
            selection: Color::rgba(67, 76, 94, 180),
            border: Color::rgb(76, 86, 106),
            accent: Color::rgb(136, 192, 208),
        }
    }
}

impl ThemeColors {
    pub fn set(&mut self, semantic: SemanticColor, color: Color) {
        match semantic {
            SemanticColor::TerminalForeground => self.foreground = color,
            SemanticColor::TerminalBackground => self.background = color,
            SemanticColor::Cursor => self.cursor = color,
            SemanticColor::Selection => self.selection = color,
            SemanticColor::PaneBorder | SemanticColor::ActivePaneBorder => {
                self.border = color
            }
            SemanticColor::Accent => self.accent = color,
            _ => {}
        }
    }
}

#[derive(Clone, Debug)]
pub struct ThemePalette {
    colors: [Color; 16],
}

impl Default for ThemePalette {
    fn default() -> Self {
        let colors = [
            Rgba::rgb(46, 52, 64),
            Rgba::rgb(191, 97, 106),
            Rgba::rgb(163, 190, 140),
            Rgba::rgb(235, 203, 139),
            Rgba::rgb(129, 161, 193),
            Rgba::rgb(180, 142, 173),
            Rgba::rgb(136, 192, 208),
            Rgba::rgb(216, 222, 233),
            Rgba::rgb(76, 86, 106),
            Rgba::rgb(191, 97, 106),
            Rgba::rgb(163, 190, 140),
            Rgba::rgb(235, 203, 139),
            Rgba::rgb(129, 161, 193),
            Rgba::rgb(180, 142, 173),
            Rgba::rgb(143, 188, 187),
            Rgba::rgb(236, 239, 244),
        ];

        Self {
            colors: colors.map(Color::Rgba),
        }
    }
}

impl ThemePalette {
    pub fn get(&self, index: usize) -> Option<Color> {
        self.colors.get(index).copied()
    }

    pub fn set(&mut self, index: usize, color: Color) {
        if let Some(entry) = self.colors.get_mut(index) {
            *entry = color;
        }
    }

    pub fn colors(&self) -> &[Color; 16] {
        &self.colors
    }
}

#[derive(Clone, Debug)]
pub struct Theme {
    id: u64,
    name: String,
    description: String,
    author: String,
    version: String,
    colors: ThemeColors,
    palette: ThemePalette,
    semantic: SemanticColorMap,
    extended_palette: ColorPalette,
    attributes: std::collections::BTreeMap<String, String>,
}

impl Theme {
    pub fn new(id: u64, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            description: String::new(),
            author: String::new(),
            version: String::from("1.0.0"),
            colors: ThemeColors::default(),
            palette: ThemePalette::default(),
            semantic: SemanticColorMap::new(),
            extended_palette: ColorPalette::new(),
            attributes: std::collections::BTreeMap::new(),
        }
    }

    pub fn from_resource(resource: &Resource) -> Result<Self, ResourceError> {
        if resource.kind() != ResourceKind::Theme {
            return Err(ResourceError::InvalidResource(
                "resource is not a theme".to_string(),
            ));
        }

        Ok(Self::new(resource.id().value(), resource.name()))
    }

    pub fn id(&self) -> u64 {
        self.id
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

    pub fn colors_mut(&mut self) -> &mut ThemeColors {
        &mut self.colors
    }

    pub fn palette(&self) -> &ThemePalette {
        &self.palette
    }

    pub fn palette_mut(&mut self) -> &mut ThemePalette {
        &mut self.palette
    }

    pub fn semantic(&self) -> &SemanticColorMap {
        &self.semantic
    }

    pub fn semantic_mut(&mut self) -> &mut SemanticColorMap {
        &mut self.semantic
    }

    pub fn extended_palette(&self) -> &ColorPalette {
        &self.extended_palette
    }

    pub fn extended_palette_mut(&mut self) -> &mut ColorPalette {
        &mut self.extended_palette
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

    pub fn set_color(&mut self, semantic: SemanticColor, color: Color) {
        self.semantic.set(semantic, color);
        self.colors.set(semantic, color);
    }

    pub fn set_attribute(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) {
        self.attributes.insert(key.into(), value.into());
    }

    pub fn attribute(&self, key: &str) -> Option<&str> {
        self.attributes.get(key).map(String::as_str)
    }

    pub fn attributes(
        &self,
    ) -> &std::collections::BTreeMap<String, String> {
        &self.attributes
    }
}
