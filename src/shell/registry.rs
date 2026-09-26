//! Shell registry.

use std::collections::HashMap;

use super::{
    capabilities::ShellCapabilities,
    detection::{DetectedShell, ShellKind},
    manifest::{ShellManifest, ShellManifestError},
};

/// Errors produced by the shell registry.
#[derive(Debug)]
pub enum ShellRegistryError {
    AlreadyRegistered(String),
    NotFound(String),
    InvalidManifest(ShellManifestError),
}

impl std::fmt::Display for ShellRegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyRegistered(id) => {
                write!(f, "shell '{id}' is already registered")
            }
            Self::NotFound(id) => {
                write!(f, "shell '{id}' is not registered")
            }
            Self::InvalidManifest(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for ShellRegistryError {}

/// Registered shell information.
#[derive(Debug, Clone)]
pub struct RegisteredShell {
    pub manifest: ShellManifest,
    pub detected: Option<DetectedShell>,
}

/// Registry containing built-in and custom shells.
#[derive(Debug, Default)]
pub struct ShellRegistry {
    shells: HashMap<String, RegisteredShell>,
}

impl ShellRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(
        &mut self,
        manifest: ShellManifest,
    ) -> Result<(), ShellRegistryError> {
        manifest
            .validate()
            .map_err(ShellRegistryError::InvalidManifest)?;

        if self.shells.contains_key(&manifest.id) {
            return Err(ShellRegistryError::AlreadyRegistered(
                manifest.id,
            ));
        }

        let id = manifest.id.clone();

        self.shells.insert(
            id,
            RegisteredShell {
                manifest,
                detected: None,
            },
        );

        Ok(())
    }

    pub fn register_or_replace(
        &mut self,
        manifest: ShellManifest,
    ) -> Result<(), ShellRegistryError> {
        manifest
            .validate()
            .map_err(ShellRegistryError::InvalidManifest)?;

        let id = manifest.id.clone();

        let detected = self
            .shells
            .get(&id)
            .and_then(|shell| shell.detected.clone());

        self.shells.insert(
            id,
            RegisteredShell {
                manifest,
                detected,
            },
        );

        Ok(())
    }

    pub fn register_detected(
        &mut self,
        detected: DetectedShell,
    ) -> Result<(), ShellRegistryError> {
        let id = detected.kind.name().to_string();

        let capabilities = capabilities_for(&detected.kind);

        let manifest = ShellManifest {
            id: id.clone(),
            name: detected.kind.name().to_string(),
            executable: detected.executable.clone(),
            version: detected.version.clone(),
            description: Some(
                "Automatically detected shell".to_string(),
            ),
            capabilities,
            arguments: Vec::new(),
            startup_arguments: Vec::new(),
            rc_file: None,
            environment: Vec::new(),
        };

        self.shells.insert(
            id,
            RegisteredShell {
                manifest,
                detected: Some(detected),
            },
        );

        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&RegisteredShell> {
        self.shells.get(id)
    }

    pub fn get_mut(
        &mut self,
        id: &str,
    ) -> Option<&mut RegisteredShell> {
        self.shells.get_mut(id)
    }

    pub fn remove(
        &mut self,
        id: &str,
    ) -> Option<RegisteredShell> {
        self.shells.remove(id)
    }

    pub fn contains(&self, id: &str) -> bool {
        self.shells.contains_key(id)
    }

    pub fn len(&self) -> usize {
        self.shells.len()
    }

    pub fn is_empty(&self) -> bool {
        self.shells.is_empty()
    }

    pub fn ids(&self) -> impl Iterator<Item = &str> {
        self.shells.keys().map(String::as_str)
    }

    pub fn shells(&self) -> impl Iterator<Item = &RegisteredShell> {
        self.shells.values()
    }
}

fn capabilities_for(kind: &ShellKind) -> ShellCapabilities {
    match kind {
        ShellKind::Bash
        | ShellKind::Dash
        | ShellKind::Ash
        | ShellKind::Yash
        | ShellKind::HeirloomSh
        | ShellKind::Sh => ShellCapabilities::posix(),

        ShellKind::Nushell
        | ShellKind::Elvish
        | ShellKind::PowerShell => ShellCapabilities::structured(),

        ShellKind::Zsh
        | ShellKind::Fish
        | ShellKind::Ksh
        | ShellKind::Mksh
        | ShellKind::Csh
        | ShellKind::Tcsh
        | ShellKind::Xonsh
        | ShellKind::Oil
        | ShellKind::Ysh
        | ShellKind::Rc
        | ShellKind::Es
        | ShellKind::Unknown => ShellCapabilities::default(),
    }
}
