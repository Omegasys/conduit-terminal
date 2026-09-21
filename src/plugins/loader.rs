use std::fs;
use std::path::{Path, PathBuf};

use super::api::Plugin;
use super::manifest::PluginManifest;

#[derive(Debug, Clone)]
pub enum PluginSource {
    Directory(PathBuf),
    Manifest(PathBuf),
    BuiltIn,
}

#[derive(Debug)]
pub struct LoadedPlugin {
    pub manifest: PluginManifest,
    pub source: PluginSource,
    pub instance: Option<Box<dyn Plugin>>,
}

impl LoadedPlugin {
    pub fn new(
        manifest: PluginManifest,
        source: PluginSource,
    ) -> Self {
        Self {
            manifest,
            source,
            instance: None,
        }
    }

    pub fn with_instance(
        mut self,
        plugin: Box<dyn Plugin>,
    ) -> Self {
        self.instance = Some(plugin);
        self
    }

    pub fn is_instantiated(&self) -> bool {
        self.instance.is_some()
    }

    pub fn take_instance(&mut self) -> Option<Box<dyn Plugin>> {
        self.instance.take()
    }
}

#[derive(Debug, Default)]
pub struct PluginLoader;

impl PluginLoader {
    pub fn new() -> Self {
        Self
    }

    pub fn load_manifest<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<PluginManifest, String> {
        let path = path.as_ref();

        let contents = fs::read_to_string(path)
            .map_err(|error| {
                format!(
                    "failed to read plugin manifest '{}': {}",
                    path.display(),
                    error
                )
            })?;

        let manifest: PluginManifest =
            toml::from_str(&contents)
                .map_err(|error| {
                    format!(
                        "failed to parse plugin manifest '{}': {}",
                        path.display(),
                        error
                    )
                })?;

        manifest.validate()?;

        Ok(manifest)
    }

    pub fn discover<P: AsRef<Path>>(
        &self,
        directory: P,
    ) -> Result<Vec<PathBuf>, String> {
        let directory = directory.as_ref();

        if !directory.exists() {
            return Ok(Vec::new());
        }

        let entries = fs::read_dir(directory)
            .map_err(|error| error.to_string())?;

        let mut manifests = Vec::new();

        for entry in entries {
            let entry = entry.map_err(|error| error.to_string())?;
            let path = entry.path();

            if path.is_dir() {
                let manifest = path.join("plugin.toml");

                if manifest.is_file() {
                    manifests.push(manifest);
                }
            } else if path
                .file_name()
                .and_then(|name| name.to_str())
                == Some("plugin.toml")
            {
                manifests.push(path);
            }
        }

        manifests.sort();

        Ok(manifests)
    }

    pub fn load_from_manifest<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<LoadedPlugin, String> {
        let path = path.as_ref();
        let manifest = self.load_manifest(path)?;

        let source = PluginSource::Manifest(path.to_path_buf());

        Ok(LoadedPlugin::new(manifest, source))
    }
}
