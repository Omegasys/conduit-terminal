use super::{
    manifest::CustomShellManifest,
    shell::CustomShell,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShellValidationSeverity {
    Warning,
    Error,
}

#[derive(Clone, Debug)]
pub struct ShellValidationIssue {
    pub severity: ShellValidationSeverity,
    pub field: String,
    pub message: String,
}

impl ShellValidationIssue {
    pub fn warning<S1, S2>(
        field: S1,
        message: S2,
    ) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            severity:
                ShellValidationSeverity::Warning,
            field: field.into(),
            message: message.into(),
        }
    }

    pub fn error<S1, S2>(
        field: S1,
        message: S2,
    ) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            severity:
                ShellValidationSeverity::Error,
            field: field.into(),
            message: message.into(),
        }
    }

    pub fn is_error(&self) -> bool {
        matches!(
            self.severity,
            ShellValidationSeverity::Error
        )
    }
}

#[derive(Clone, Debug, Default)]
pub struct ShellValidationResult {
    issues: Vec<ShellValidationIssue>,
}

impl ShellValidationResult {
    pub fn add(
        &mut self,
        issue: ShellValidationIssue,
    ) {
        self.issues.push(issue);
    }

    pub fn issues(
        &self,
    ) -> &[ShellValidationIssue] {
        &self.issues
    }

    pub fn errors(
        &self,
    ) -> Vec<&ShellValidationIssue> {
        self.issues
            .iter()
            .filter(|issue| issue.is_error())
            .collect()
    }

    pub fn warnings(
        &self,
    ) -> Vec<&ShellValidationIssue> {
        self.issues
            .iter()
            .filter(|issue| !issue.is_error())
            .collect()
    }

    pub fn has_errors(&self) -> bool {
        self.issues
            .iter()
            .any(ShellValidationIssue::is_error)
    }

    pub fn is_valid(&self) -> bool {
        !self.has_errors()
    }
}

pub struct CustomShellValidator;

impl CustomShellValidator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_manifest(
        &self,
        manifest: &CustomShellManifest,
    ) -> ShellValidationResult {
        let mut result =
            ShellValidationResult::default();

        if manifest.id().as_str().is_empty() {
            result.add(
                ShellValidationIssue::error(
                    "id",
                    "shell ID cannot be empty",
                ),
            );
        }

        if manifest.name().trim().is_empty() {
            result.add(
                ShellValidationIssue::error(
                    "name",
                    "shell name cannot be empty",
                ),
            );
        }

        if manifest
            .metadata()
            .author
            .trim()
            .is_empty()
        {
            result.add(
                ShellValidationIssue::warning(
                    "author",
                    "shell author is not specified",
                ),
            );
        }

        if manifest.executable().is_none()
            && manifest.entrypoint().is_none()
        {
            result.add(
                ShellValidationIssue::warning(
                    "executable",
                    "shell has no executable or embedded entrypoint",
                ),
            );
        }

        result
    }

    pub fn validate_shell(
        &self,
        shell: &dyn CustomShell,
    ) -> ShellValidationResult {
        self.validate_manifest(
            &shell.manifest(),
        )
    }
}

impl Default for CustomShellValidator {
    fn default() -> Self {
        Self::new()
    }
}
