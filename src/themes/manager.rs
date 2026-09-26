//! Central theme manager.

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use super::{
    loader::{ThemeLoadError, ThemeLoader},
    theme::{Theme, ThemeId},
};

/// Errors from theme management.
#[derive(Debug)]
pub enum ThemeError {
    Load(ThemeLoadError),
    NotFound(ThemeId),
    AlreadyExists(ThemeId),
    InvalidTheme(String),
}

impl std::fmt::Display for ThemeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Load(error) => write!(f, "{error}"),
            Self::NotFound(id) => write!(f, "theme '{}' not found", id.as_str()),
            Self::AlreadyExists(id) => {
                write!(f, "theme '{}' already exists", id.as_str())
            }
            Self::InvalidTheme(error) => write!(f, "invalid theme: {error}"),
        }
    }
}

impl std::error::Error for ThemeError {}

impl From<ThemeLoadError> for ThemeError {
    fn from(error: ThemeLoadError) -> Self {
        Self::Load(error)
    }
}

/// Central registry and active-theme controller.
#[derive(Debug)]
pub struct ThemeManager {
    themes: HashMap<ThemeId, Theme>,
    paths: HashMap<ThemeId, PathBuf>,
    active: ThemeId,
    loader: ThemeLoader,
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ThemeManager {
    pub fn new() -> Self {
        let dark = Theme::builtin_dark();
        let light = Theme::builtin_light();

        let active = dark.id.clone();

        let mut themes = HashMap::new();
        themes.insert(dark.id.clone(), dark);
        themes.insert(light.id.clone(), light);

        Self {
            themes,
            paths: HashMap::new(),
            active,
            loader: ThemeLoader::new(),
        }
    }

    pub fn active(&self) -> Option<&Theme> {
        self.themes.get(&self.active)
    }

    pub fn active_id(&self) -> &ThemeId {
        &self.active
    }

    pub fn set_active(
        &mut self,
        id: impl Into<ThemeId>,
    ) -> Result<(), ThemeError> {
        let id = id.into();

        if !self.themes.contains_key(&id) {
            return Err(ThemeError::NotFound(id));
        }

        self.active = id;
        Ok(())
    }

    pub fn register(&mut self, theme: Theme) -> Result<(), ThemeError> {
        if self.themes.contains_key(&theme.id) {
            return Err(ThemeError::AlreadyExists(theme.id));
        }

        theme
            .validate()
            .map_err(ThemeError::InvalidTheme)?;

        self.themes.insert(theme.id.clone(), theme);
        Ok(())
    }

    pub fn register_or_replace(&mut self, theme: Theme) -> Result<(), ThemeError> {
        theme
            .validate()
            .map_err(ThemeError::InvalidTheme)?;

        self.themes.insert(theme.id.clone(), theme);
        Ok(())
    }

    pub fn load_file(
        &mut self,
        path: impl AsRef<Path>,
    ) -> Result<ThemeId, ThemeError> {
        let path = path.as_ref();
        let theme = self.loader.load(path)?;
        let id = theme.id.clone();

        self.register_or_replace(theme)?;
        self.paths.insert(id.clone(), path.to_path_buf());

        Ok(id)
    }

    pub fn remove(&mut self, id: &ThemeId) -> Option<Theme> {
        if *id == self.active {
            return None;
        }

        self.paths.remove(id);
        self.themes.remove(id)
    }

    pub fn get(&self, id: &ThemeId) -> Option<&Theme> {
        self.themes.get(id)
    }

    pub fn get_mut(&mut self, id: &ThemeId) -> Option<&mut Theme> {
        self.themes.get_mut(id)
    }

    pub fn path(&self, id: &ThemeId) -> Option<&Path> {
        self.paths.get(id).map(PathBuf::as_path)
    }

    pub fn ids(&self) -> impl Iterator<Item = &ThemeId> {
        self.themes.keys()
    }

    pub fn themes(&self) -> impl Iterator<Item = &Theme> {
        self.themes.values()
    }

    pub fn len(&self) -> usize {
        self.themes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.themes.is_empty()
    }

    pub fn loader(&self) -> &ThemeLoader {
        &self.loader
    }
}
