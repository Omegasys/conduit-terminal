use std::fs;
use std::path::{Path, PathBuf};

use crate::shell::custom::{
    CustomShellCapabilities,
    CustomShellId,
    CustomShellVersion,
    ShellColorSupport,
    ShellExecutionModel,
};

use super::manifest::{
    ShellResourceManifest,
    ShellResourceMetadata,
};

#[derive(Debug)]
pub enum ShellResourceLoadResult {
    Loaded(ShellResourceManifest),
    Skipped(String),
    Failed(String),
}

pub struct ShellResourceLoader;

impl ShellResourceLoader {
    pub fn new() -> Self {
        Self
    }

    pub fn load(
        &self,
        directory: impl AsRef<Path>,
    ) -> Result<ShellResourceManifest, String> {
        let directory = directory.as_ref();
        let manifest_path = directory.join("shell.toml");

        if !manifest_path.is_file() {
            return Err(format!(
                "Shell manifest not found: {}",
                manifest_path.display()
            ));
        }

        let contents = fs::read_to_string(&manifest_path)
            .map_err(|error| {
                format!(
                    "Failed to read {}: {error}",
                    manifest_path.display()
                )
            })?;

        self.parse(&contents, directory)
    }

    pub fn load_directory(
        &self,
        directory: impl AsRef<Path>,
    ) -> ShellResourceLoadResult {
        match self.load(directory.as_ref()) {
            Ok(manifest) => ShellResourceLoadResult::Loaded(manifest),
            Err(error) => ShellResourceLoadResult::Failed(error),
        }
    }

    fn parse(
        &self,
        contents: &str,
        root_path: &Path,
    ) -> Result<ShellResourceManifest, String> {
        let mut id: Option<String> = None;
        let mut name: Option<String> = None;
        let mut author = String::new();
        let mut description = String::new();
        let mut homepage = None;
        let mut license = None;

        let mut major = 1u32;
        let mut minor = 0u32;
        let mut patch = 0u32;

        let mut executable = None;
        let mut entrypoint = None;

        let mut execution_model = ShellExecutionModel::ExternalProcess;
        let mut color = ShellColorSupport::None;

        let mut unicode = true;
        let mut command_history = true;
        let mut command_completion = false;
        let mut command_aliases = false;
        let mut scripting = false;
        let mut environment_variables = true;
        let mut working_directory_tracking = true;
        let mut prompt_detection = true;
        let mut pipelines = true;
        let mut redirection = true;
        let mut job_control = false;
        let mut interactive_programs = true;
        let mut terminal_protocols = true;

        let mut dependencies = Vec::new();

        for raw_line in contents.lines() {
            let line = raw_line.trim();

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let Some((key, value)) = line.split_once('=') else {
                continue;
            };

            let key = key.trim();
            let value = value.trim().trim_matches('"');

            match key {
                "id" => id = Some(value.to_string()),
                "name" => name = Some(value.to_string()),
                "author" => author = value.to_string(),
                "description" => description = value.to_string(),
                "homepage" => homepage = Some(value.to_string()),
                "license" => license = Some(value.to_string()),

                "version.major" => {
                    major = value
                        .parse()
                        .map_err(|_| "Invalid version.major".to_string())?;
                }

                "version.minor" => {
                    minor = value
                        .parse()
                        .map_err(|_| "Invalid version.minor".to_string())?;
                }

                "version.patch" => {
                    patch = value
                        .parse()
                        .map_err(|_| "Invalid version.patch".to_string())?;
                }

                "executable" => {
                    executable = Some(value.to_string());
                }

                "entrypoint" => {
                    entrypoint = Some(value.to_string());
                }

                "execution_model" => {
                    execution_model = match value {
                        "external" | "external_process" => {
                            ShellExecutionModel::ExternalProcess
                        }
                        "embedded" => ShellExecutionModel::Embedded,
                        "hybrid" => ShellExecutionModel::Hybrid,
                        other => {
                            return Err(format!(
                                "Unknown execution model: {other}"
                            ));
                        }
                    };
                }

                "color" | "color_support" => {
                    color = match value {
                        "none" => ShellColorSupport::None,
                        "basic8" => ShellColorSupport::Basic8,
                        "standard16" => ShellColorSupport::Standard16,
                        "indexed256" => ShellColorSupport::Indexed256,
                        "truecolor" => ShellColorSupport::TrueColor,
                        other => {
                            return Err(format!(
                                "Unknown color support: {other}"
                            ));
                        }
                    };
                }

                "unicode" => unicode = parse_bool(value)?,
                "command_history" => command_history = parse_bool(value)?,
                "command_completion" => {
                    command_completion = parse_bool(value)?
                }
                "command_aliases" => command_aliases = parse_bool(value)?,
                "scripting" => scripting = parse_bool(value)?,
                "environment_variables" => {
                    environment_variables = parse_bool(value)?
                }
                "working_directory_tracking" => {
                    working_directory_tracking = parse_bool(value)?
                }
                "prompt_detection" => {
                    prompt_detection = parse_bool(value)?
                }
                "pipelines" => pipelines = parse_bool(value)?,
                "redirection" => redirection = parse_bool(value)?,
                "job_control" => job_control = parse_bool(value)?,
                "interactive_programs" => {
                    interactive_programs = parse_bool(value)?
                }
                "terminal_protocols" => {
                    terminal_protocols = parse_bool(value)?
                }

                "dependencies" => {
                    dependencies = value
                        .trim_matches(['[', ']'])
                        .split(',')
                        .map(str::trim)
                        .filter(|item| !item.is_empty())
                        .map(|item| item.trim_matches('"').to_string())
                        .collect();
                }

                _ => {}
            }
        }

        let id = id.ok_or_else(|| "Missing required field: id".to_string())?;
        let name =
            name.ok_or_else(|| "Missing required field: name".to_string())?;

        let metadata = ShellResourceMetadata::new(author, description);

        let mut metadata = metadata;

        if let Some(homepage) = homepage {
            metadata = metadata.homepage(homepage);
        }

        if let Some(license) = license {
            metadata = metadata.license(license);
        }

        let capabilities = CustomShellCapabilities {
            execution_model,
            color,
            unicode,
            command_history,
            command_completion,
            command_aliases,
            scripting,
            environment_variables,
            working_directory_tracking,
            prompt_detection,
            pipelines,
            redirection,
            job_control,
            interactive_programs,
            terminal_protocols,
        };

        let mut manifest = ShellResourceManifest::new(
            CustomShellId::new(id),
            name,
            metadata,
            PathBuf::from(root_path),
        );

        manifest.set_version(CustomShellVersion::new(
            major,
            minor,
            patch,
        ));

        manifest.set_capabilities(capabilities);

        if let Some(executable) = executable {
            manifest.set_executable(executable);
        }

        if let Some(entrypoint) = entrypoint {
            manifest.set_entrypoint(entrypoint);
        }

        for dependency in dependencies {
            manifest.add_dependency(dependency);
        }

        Ok(manifest)
    }
}

impl Default for ShellResourceLoader {
    fn default() -> Self {
        Self::new()
    }
}

fn parse_bool(value: &str) -> Result<bool, String> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        other => Err(format!("Expected boolean, got: {other}")),
    }
}
