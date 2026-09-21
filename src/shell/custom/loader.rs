use std::{
    fs,
    path::{
        Path,
        PathBuf,
    },
};

use super::{
    errors::{
        CustomShellError,
        CustomShellResult,
    },
    manifest::{
        CustomShellManifest,
        CustomShellMetadata,
    },
    shell::{
        CustomShellId,
        CustomShellVersion,
    },
    validation::CustomShellValidator,
};

pub struct LoadedCustomShell {
    pub manifest: CustomShellManifest,
    pub source_path: PathBuf,
}

pub struct CustomShellLoader {
    validator: CustomShellValidator,
}

impl CustomShellLoader {
    pub fn new() -> Self {
        Self {
            validator:
                CustomShellValidator::new(),
        }
    }

    pub fn load_manifest(
        &self,
        path: impl AsRef<Path>,
    ) -> CustomShellResult<LoadedCustomShell> {
        let path = path.as_ref();

        let contents =
            fs::read_to_string(path)?;

        let manifest =
            self.parse_manifest(&contents)?;

        let validation =
            self.validator
                .validate_manifest(&manifest);

        if validation.has_errors() {
            return Err(
                CustomShellError::InvalidManifest(
                    validation
                        .errors()
                        .iter()
                        .map(|issue| {
                            issue.message.clone()
                        })
                        .collect::<Vec<_>>()
                        .join("; "),
                ),
            );
        }

        Ok(LoadedCustomShell {
            manifest,
            source_path:
                path.to_path_buf(),
        })
    }

    fn parse_manifest(
        &self,
        contents: &str,
    ) -> CustomShellResult<CustomShellManifest> {
        let mut id = None;
        let mut name = None;
        let mut author = None;
        let mut description = None;
        let mut executable = None;

        let mut version =
            CustomShellVersion::initial();

        for line in contents.lines() {
            let line = line.trim();

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
            let value =
                value.trim().trim_matches('"');

            match key {
                "id" => {
                    id = Some(value.to_string())
                }

                "name" => {
                    name = Some(value.to_string())
                }

                "author" => {
                    author = Some(value.to_string())
                }

                "description" => {
                    description =
                        Some(value.to_string())
                }

                "executable" => {
                    executable =
                        Some(value.to_string())
                }

                "version.major" => {
                    version.major =
                        value.parse().unwrap_or(0);
                }

                "version.minor" => {
                    version.minor =
                        value.parse().unwrap_or(0);
                }

                "version.patch" => {
                    version.patch =
                        value.parse().unwrap_or(0);
                }

                _ => {}
            }
        }

        let id = id.ok_or_else(|| {
            CustomShellError::InvalidManifest(
                "missing shell id".to_string(),
            )
        })?;

        let name =
            name.unwrap_or_else(|| id.clone());

        let metadata =
            CustomShellMetadata::new(
                author.unwrap_or_else(|| {
                    "Unknown".to_string()
                }),
                description
                    .unwrap_or_default(),
            );

        let mut manifest =
            CustomShellManifest::new(
                CustomShellId::new(id),
                name,
                metadata,
            );

        manifest.set_version(version);

        if let Some(executable) =
            executable
        {
            manifest
                .set_executable(executable);
        }

        Ok(manifest)
    }
}

impl Default for CustomShellLoader {
    fn default() -> Self {
        Self::new()
    }
}
