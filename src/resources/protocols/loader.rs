use std::fs;
use std::path::{Path, PathBuf};

use crate::terminal_protocols::custom::{
    CustomProtocolCapabilities,
    CustomProtocolId,
    CustomProtocolMetadata,
    CustomProtocolVersion,
    ProtocolColorSupport,
    ProtocolExecutionModel,
};

use super::manifest::{
    ProtocolResourceManifest,
    ProtocolResourceMetadata,
};

pub enum ProtocolResourceLoadResult {
    Loaded(ProtocolResourceManifest),
    Skipped(String),
    Failed(String),
}

pub struct ProtocolResourceLoader;

impl ProtocolResourceLoader {
    pub fn new() -> Self {
        Self
    }

    pub fn load(
        &self,
        directory: impl AsRef<Path>,
    ) -> Result<ProtocolResourceManifest, String> {
        let directory = directory.as_ref();
        let manifest_path =
            directory.join("protocol.toml");

        if !manifest_path.is_file() {
            return Err(format!(
                "Protocol manifest not found: {}",
                manifest_path.display()
            ));
        }

        let contents =
            fs::read_to_string(&manifest_path)
                .map_err(|error| {
                    format!(
                        "Failed to read {}: {error}",
                        manifest_path.display()
                    )
                })?;

        self.parse(
            &contents,
            directory,
        )
    }

    pub fn load_directory(
        &self,
        directory: impl AsRef<Path>,
    ) -> ProtocolResourceLoadResult {
        match self.load(directory.as_ref()) {
            Ok(manifest) => {
                ProtocolResourceLoadResult::Loaded(manifest)
            }

            Err(error) => {
                ProtocolResourceLoadResult::Failed(error)
            }
        }
    }

    fn parse(
        &self,
        contents: &str,
        root: &Path,
    ) -> Result<ProtocolResourceManifest, String> {
        let mut id = None;
        let mut name = None;

        let mut author = String::new();
        let mut description = String::new();

        let mut homepage = None;
        let mut license = None;

        let mut major = 1;
        let mut minor = 0;
        let mut patch = 0;

        let mut entrypoint = None;

        let mut execution_model =
            ProtocolExecutionModel::Embedded;

        let mut color =
            ProtocolColorSupport::None;

        for raw_line in contents.lines() {
            let line = raw_line.trim();

            if line.is_empty()
                || line.starts_with('#')
            {
                continue;
            }

            let Some((key, value)) =
                line.split_once('=')
            else {
                continue;
            };

            let key = key.trim();
            let value = value.trim().trim_matches('"');

            match key {
                "id" => id = Some(value.to_string()),
                "name" => name = Some(value.to_string()),

                "author" => {
                    author = value.to_string()
                }

                "description" => {
                    description = value.to_string()
                }

                "homepage" => {
                    homepage = Some(value.to_string())
                }

                "license" => {
                    license = Some(value.to_string())
                }

                "version.major" => {
                    major = value.parse().map_err(|_| {
                        "Invalid version.major".to_string()
                    })?;
                }

                "version.minor" => {
                    minor = value.parse().map_err(|_| {
                        "Invalid version.minor".to_string()
                    })?;
                }

                "version.patch" => {
                    patch = value.parse().map_err(|_| {
                        "Invalid version.patch".to_string()
                    })?;
                }

                "entrypoint" => {
                    entrypoint = Some(value.to_string())
                }

                "execution_model" => {
                    execution_model =
                        match value {
                            "external" |
                            "external_process" => {
                                ProtocolExecutionModel::ExternalProcess
                            }

                            "embedded" => {
                                ProtocolExecutionModel::Embedded
                            }

                            "hybrid" => {
                                ProtocolExecutionModel::Hybrid
                            }

                            other => {
                                return Err(format!(
                                    "Unknown execution model: {other}"
                                ));
                            }
                        };
                }

                "color" |
                "color_support" => {
                    color = match value {
                        "none" => {
                            ProtocolColorSupport::None
                        }

                        "basic8" => {
                            ProtocolColorSupport::Basic8
                        }

                        "standard16" => {
                            ProtocolColorSupport::Standard16
                        }

                        "indexed256" => {
                            ProtocolColorSupport::Indexed256
                        }

                        "truecolor" => {
                            ProtocolColorSupport::TrueColor
                        }

                        other => {
                            return Err(format!(
                                "Unknown color support: {other}"
                            ));
                        }
                    };
                }

                _ => {}
            }
        }

        let id = id.ok_or_else(|| {
            "Missing required field: id".to_string()
        })?;

        let name = name.ok_or_else(|| {
            "Missing required field: name".to_string()
        })?;

        let mut metadata =
            ProtocolResourceMetadata::new(
                author,
                description,
            );

        if let Some(homepage) = homepage {
            metadata =
                metadata.homepage(homepage);
        }

        if let Some(license) = license {
            metadata =
                metadata.license(license);
        }

        let mut capabilities =
            CustomProtocolCapabilities::default();

        capabilities.execution_model =
            execution_model;

        capabilities.color = color;

        let mut manifest =
            ProtocolResourceManifest::new(
                CustomProtocolId::new(id),
                name,
                metadata,
                PathBuf::from(root),
            );

        manifest.set_version(
            CustomProtocolVersion::new(
                major,
                minor,
                patch,
            ),
        );

        manifest.set_capabilities(
            capabilities,
        );

        if let Some(entrypoint) = entrypoint {
            manifest.set_entrypoint(entrypoint);
        }

        Ok(manifest)
    }
}

impl Default for ProtocolResourceLoader {
    fn default() -> Self {
        Self::new()
    }
}
