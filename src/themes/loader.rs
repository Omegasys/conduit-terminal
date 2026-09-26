//! Theme file loading.

use std::{
    fs,
    path::{Path, PathBuf},
};

use super::{
    colors::Color,
    theme::{Theme, ThemeId, ThemeMetadata},
};

/// Errors produced while loading a theme.
#[derive(Debug)]
pub enum ThemeLoadError {
    Io(std::io::Error),
    InvalidFormat(String),
    InvalidColor(String),
    InvalidTheme(String),
}

impl std::fmt::Display for ThemeLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(f, "theme I/O error: {error}"),
            Self::InvalidFormat(error) => write!(f, "invalid theme format: {error}"),
            Self::InvalidColor(error) => write!(f, "invalid theme color: {error}"),
            Self::InvalidTheme(error) => write!(f, "invalid theme: {error}"),
        }
    }
}

impl std::error::Error for ThemeLoadError {}

impl From<std::io::Error> for ThemeLoadError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

/// Loads themes from disk.
///
/// The current implementation intentionally uses a small, dependency-free
/// parser for a simple `key = value` format. It can later be replaced by
/// TOML/JSON deserialization without changing the Theme API.
#[derive(Debug, Clone, Default)]
pub struct ThemeLoader;

impl ThemeLoader {
    pub fn new() -> Self {
        Self
    }

    pub fn load(&self, path: impl AsRef<Path>) -> Result<Theme, ThemeLoadError> {
        let path = path.as_ref();
        let contents = fs::read_to_string(path)?;

        let id = path
            .file_stem()
            .and_then(|value| value.to_str())
            .unwrap_or("custom")
            .to_string();

        self.parse(&contents, ThemeId::new(id))
    }

    pub fn parse(
        &self,
        contents: &str,
        id: ThemeId,
    ) -> Result<Theme, ThemeLoadError> {
        let mut name = id.as_str().to_string();
        let mut theme = Theme::new(id, ThemeMetadata::new(name.clone()));

        for (line_number, raw_line) in contents.lines().enumerate() {
            let line = raw_line.trim();

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let Some((key, value)) = line.split_once('=') else {
                return Err(ThemeLoadError::InvalidFormat(format!(
                    "line {} is missing '='",
                    line_number + 1
                )));
            };

            let key = key.trim();
            let value = value.trim().trim_matches('"');

            match key {
                "name" => {
                    name = value.to_string();
                    theme.metadata.name = name.clone();
                }
                "author" => theme.metadata.author = Some(value.to_string()),
                "version" => theme.metadata.version = Some(value.to_string()),
                "description" => {
                    theme.metadata.description = Some(value.to_string())
                }
                "colors.background" => {
                    theme.colors.background = parse_color(key, value)?
                }
                "colors.foreground" => {
                    theme.colors.foreground = parse_color(key, value)?
                }
                "colors.surface" => {
                    theme.colors.surface = parse_color(key, value)?
                }
                "colors.primary" => {
                    theme.colors.primary = parse_color(key, value)?
                }
                "colors.accent" => {
                    theme.colors.accent = parse_color(key, value)?
                }
                "colors.success" => {
                    theme.colors.success = parse_color(key, value)?
                }
                "colors.warning" => {
                    theme.colors.warning = parse_color(key, value)?
                }
                "colors.error" => {
                    theme.colors.error = parse_color(key, value)?
                }
                "colors.link" => {
                    theme.colors.link = parse_color(key, value)?
                }
                "appearance.opacity" => {
                    theme.appearance.opacity = value.parse::<f32>().map_err(|_| {
                        ThemeLoadError::InvalidFormat(format!(
                            "invalid opacity: {value}"
                        ))
                    })?;
                }
                _ => {
                    // Unknown keys are ignored for forward compatibility.
                }
            }
        }

        theme
            .validate()
            .map_err(ThemeLoadError::InvalidTheme)?;

        Ok(theme)
    }

    pub fn load_directory(
        &self,
        directory: impl AsRef<Path>,
    ) -> Result<Vec<(PathBuf, Theme)>, ThemeLoadError> {
        let mut themes = Vec::new();

        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|value| value.to_str()) != Some("theme") {
                continue;
            }

            let theme = self.load(&path)?;
            themes.push((path, theme));
        }

        Ok(themes)
    }
}

fn parse_color(key: &str, value: &str) -> Result<Color, ThemeLoadError> {
    Color::from_hex(value).ok_or_else(|| {
        ThemeLoadError::InvalidColor(format!("{key} = {value}"))
    })
}
