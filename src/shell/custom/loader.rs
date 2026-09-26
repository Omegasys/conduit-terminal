//! Custom shell loader.
//!
//! The loader currently provides an in-memory loading path. A higher-level
//! configuration layer can deserialize TOML/JSON/YAML and construct the
//! manifest before passing it here.

use super::{
    errors::{CustomShellError, CustomShellResult},
    manifest::CustomShellManifest,
    shell::CustomShell,
};

#[derive(Debug, Default)]
pub struct CustomShellLoader;

impl CustomShellLoader {
    pub fn new() -> Self {
        Self
    }

    pub fn load(
        &self,
        manifest: CustomShellManifest,
    ) -> CustomShellResult<CustomShell> {
        manifest.validate()?;
        CustomShell::new(manifest)
    }

    pub fn validate(
        &self,
        manifest: &CustomShellManifest,
    ) -> CustomShellResult<()> {
        manifest.validate()
    }

    pub fn executable_available(
        &self,
        manifest: &CustomShellManifest,
    ) -> bool {
        if manifest.executable().contains('/') {
            std::path::Path::new(manifest.executable()).is_file()
        } else {
            find_in_path(manifest.executable()).is_some()
        }
    }

    pub fn load_if_available(
        &self,
        manifest: CustomShellManifest,
    ) -> CustomShellResult<CustomShell> {
        if !self.executable_available(&manifest) {
            return Err(CustomShellError::ExecutableNotFound(
                manifest.executable().to_string(),
            ));
        }

        self.load(manifest)
    }
}

fn find_in_path(name: &str) -> Option<std::path::PathBuf> {
    let path = std::env::var_os("PATH")?;

    for directory in std::env::split_paths(&path) {
        let candidate = directory.join(name);

        if candidate.is_file() {
            return Some(candidate);
        }
    }

    None
}
