use std::{
    env,
    path::PathBuf,
};

use super::integration::{
    ShellCommand,
    ShellEnvironment,
    ShellIntegration,
    ShellIntegrationResult,
};

pub struct PowerShellIntegration {
    environment: ShellEnvironment,
    prompt_markers: Vec<String>,
}

impl PowerShellIntegration {
    pub fn new() -> Self {
        Self {
            environment: ShellEnvironment::from_current_process(),
            prompt_markers: vec![
                "PS ".to_string(),
                "PS>".to_string(),
            ],
        }
    }

    fn parse_directory_marker(
        &self,
        output: &[u8],
    ) -> Option<PathBuf> {
        let text = String::from_utf8_lossy(output);

        for line in text.lines() {
            if let Some(directory) = line.strip_prefix("CONDUIT_PWD=") {
                if !directory.is_empty() {
                    return Some(PathBuf::from(directory));
                }
            }
        }

        None
    }

    fn parse_environment_marker(
        &self,
        output: &[u8],
        result: &mut ShellIntegrationResult,
    ) {
        let text = String::from_utf8_lossy(output);

        for line in text.lines() {
            if let Some(value) = line.strip_prefix("CONDUIT_ENV=") {
                if let Some((key, value)) = value.split_once('=') {
                    result.set_environment(
                        key.to_string(),
                        Some(value.to_string()),
                    );
                }
            }

            if let Some(key) = line.strip_prefix("CONDUIT_UNSET=") {
                if !key.is_empty() {
                    result.set_environment(
                        key.to_string(),
                        None,
                    );
                }
            }
        }
    }
}

impl Default for PowerShellIntegration {
    fn default() -> Self {
        Self::new()
    }
}

impl ShellIntegration for PowerShellIntegration {
    fn name(&self) -> &str {
        "powershell"
    }

    fn executable(&self) -> &str {
        "pwsh"
    }

    fn detect(&self) -> bool {
        env::var("PSModulePath").is_ok()
            || env::var("POWERSHELL_DISTRIBUTION_CHANNEL").is_ok()
            || env::var("SHELL")
                .map(|shell| {
                    shell.ends_with("/pwsh")
                        || shell.ends_with("/powershell")
                })
                .unwrap_or(false)
    }

    fn startup_command(&self) -> ShellCommand {
        ShellCommand::new("pwsh")
            .argument("-NoLogo")
            .argument("-NoProfile")
            .argument("-NoExit")
    }

    fn initialization_script(&self) -> String {
        r#"
# Conduit PowerShell integration

function global:__conduit_report_state {
    $path = (Get-Location).Path
    [Console]::Write(
        "`e]1337;ConduitPWD=$path`a"
    )
}

function global:prompt {
    __conduit_report_state
    "PS $((Get-Location).Path)> "
}
"#
        .trim_start()
        .to_string()
    }

    fn prompt_markers(&self) -> &[String] {
        &self.prompt_markers
    }

    fn parse_output(
        &mut self,
        output: &[u8],
    ) -> ShellIntegrationResult {
        let mut result = ShellIntegrationResult::new();

        if let Some(directory) = self.parse_directory_marker(output) {
            self.environment
                .set_working_directory(directory.clone());

            result.set_working_directory(directory);
        }

        self.parse_environment_marker(output, &mut result);

        result.add_output(output);

        result
    }

    fn reset(&mut self) {
        self.environment = ShellEnvironment::from_current_process();
    }

    fn environment(&self) -> &ShellEnvironment {
        &self.environment
    }

    fn environment_mut(&mut self) -> &mut ShellEnvironment {
        &mut self.environment
    }
}
