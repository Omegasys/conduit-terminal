use super::{
    manifest::CustomProtocolManifest,
    protocol::CustomTerminalProtocol,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValidationSeverity {
    Warning,
    Error,
}

#[derive(Clone, Debug)]
pub struct ValidationIssue {
    pub severity: ValidationSeverity,
    pub field: String,
    pub message: String,
}

impl ValidationIssue {
    pub fn warning<S1, S2>(field: S1, message: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            severity: ValidationSeverity::Warning,
            field: field.into(),
            message: message.into(),
        }
    }

    pub fn error<S1, S2>(field: S1, message: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            severity: ValidationSeverity::Error,
            field: field.into(),
            message: message.into(),
        }
    }

    pub fn is_error(&self) -> bool {
        matches!(self.severity, ValidationSeverity::Error)
    }
}

#[derive(Clone, Debug, Default)]
pub struct ValidationResult {
    issues: Vec<ValidationIssue>,
}

impl ValidationResult {
    pub fn add(&mut self, issue: ValidationIssue) {
        self.issues.push(issue);
    }

    pub fn issues(&self) -> &[ValidationIssue] {
        &self.issues
    }

    pub fn errors(&self) -> Vec<&ValidationIssue> {
        self.issues.iter().filter(|i| i.is_error()).collect()
    }

    pub fn warnings(&self) -> Vec<&ValidationIssue> {
        self.issues
            .iter()
            .filter(|i| !i.is_error())
            .collect()
    }

    pub fn has_errors(&self) -> bool {
        self.issues.iter().any(ValidationIssue::is_error)
    }

    pub fn has_warnings(&self) -> bool {
        self.issues
            .iter()
            .any(|issue| !issue.is_error())
    }

    pub fn is_valid(&self) -> bool {
        !self.has_errors()
    }
}

pub struct CustomProtocolValidator;

impl CustomProtocolValidator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_manifest(
        &self,
        manifest: &CustomProtocolManifest,
    ) -> ValidationResult {
        let mut result = ValidationResult::default();

        if manifest.id().as_str().is_empty() {
            result.add(ValidationIssue::error(
                "id",
                "protocol ID cannot be empty",
            ));
        }

        if manifest.name().trim().is_empty() {
            result.add(ValidationIssue::error(
                "name",
                "protocol name cannot be empty",
            ));
        }

        if manifest.metadata().author.trim().is_empty() {
            result.add(ValidationIssue::warning(
                "author",
                "protocol author is not specified",
            ));
        }

        if manifest.metadata().description.trim().is_empty() {
            result.add(ValidationIssue::warning(
                "description",
                "protocol description is empty",
            ));
        }

        result
    }

    pub fn validate_protocol(
        &self,
        protocol: &dyn CustomTerminalProtocol,
    ) -> ValidationResult {
        self.validate_manifest(&protocol.manifest())
    }
}

impl Default for CustomProtocolValidator {
    fn default() -> Self {
        Self::new()
    }
}
