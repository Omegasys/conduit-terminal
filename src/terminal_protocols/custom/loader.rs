use std::fs;
use std::path::{Path, PathBuf};

use super::{
    capabilities::{
        CustomProtocolCapabilities,
        ProtocolColorSupport,
        ProtocolExecutionModel,
    },
    manifest::{
        CustomProtocolManifest,
        CustomProtocolMetadata,
    },
    protocol::{
        CustomProtocolId,
        CustomProtocolVersion,
    },
    validation::CustomProtocolValidator,
};

pub struct LoadedCustomProtocol {
    pub manifest: CustomProtocolManifest,
    pub source_path: PathBuf,
}

pub struct CustomProtocolLoader {
    validator: CustomProtocolValidator,
}

impl CustomProtocolLoader {
    pub fn new() -> Self {
        Self {
            validator: CustomProtocolValidator::new(),
        }
    }

    pub fn load_manifest(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<LoadedCustomProtocol, String> {
        let path = path.as_ref();

        let contents = fs::read_to_string(path)
            .map_err(|error| {
                format!(
                    "Failed to read {}: {error}",
                    path.display()
                )
            })?;

        let root = path
            .parent()
            .unwrap_or_else(|| Path::new("."));

        let manifest = self.parse_manifest(
            &contents,
            root,
        )?;

        let validation =
            self.validator.validate_manifest(&manifest);

        if validation.has_errors() {
            let errors = validation
                .errors()
                .map(|issue| {
                    format!(
                        "{}: {}",
                        issue.field,
                        issue.message
                    )
                })
                .collect::<Vec<_>>()
                .join("; ");

            return Err(format!(
                "Protocol manifest validation failed: {errors}"
            ));
        }

        Ok(LoadedCustomProtocol {
            manifest,
            source_path: path.to_path_buf(),
        })
    }

    fn parse_manifest(
        &self,
        contents: &str,
        root: &Path,
    ) -> Result<CustomProtocolManifest, String> {
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

            if line.is_empty() || line.starts_with('#') {
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
                "author" => author = value.to_string(),
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
                    major = value
                        .parse()
                        .map_err(|_| {
                            "Invalid version.major"
                                .to_string()
                        })?;
                }

                "version.minor" => {
                    minor = value
                        .parse()
                        .map_err(|_| {
                            "Invalid version.minor"
                                .to_string()
                        })?;
                }

                "version.patch" => {
                    patch = value
                        .parse()
                        .map_err(|_| {
                            "Invalid version.patch"
                                .to_string()
                        })?;
                }

                "entrypoint" => {
                    entrypoint = Some(value.to_string())
                }

                "execution_model" => {
                    execution_model = match value {
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
            CustomProtocolMetadata::new(
                author,
                description,
            );

        if let Some(homepage) = homepage {
            metadata = metadata.homepage(homepage);
        }

        if let Some(license) = license {
            metadata = metadata.license(license);
        }

        let mut capabilities =
            CustomProtocolCapabilities::default();

        capabilities.execution_model =
            execution_model;

        capabilities.color = color;

        let mut manifest =
            CustomProtocolManifest::new(
                CustomProtocolId::new(id),
                name,
                metadata,
                root,
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

impl Default for CustomProtocolLoader {
    fn default() -> Self {
        Self::new()
    }
}
