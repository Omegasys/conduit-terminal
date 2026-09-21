use super::{
    manifest::CustomProtocolManifest,
    protocol::CustomProtocol,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolValidationSeverity {
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub struct ProtocolValidationIssue {
    pub severity: ProtocolValidationSeverity,
    pub field: String,
    pub message: String,
}

impl ProtocolValidationIssue {
    pub fn warning(
        field: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity: ProtocolValidationSeverity::Warning,
            field: field.into(),
            message: message.into(),
        }
    }

    pub fn error(
        field: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity: ProtocolValidationSeverity::Error,
            field: field.into(),
            message: message.into(),
        }
    }

    pub fn is_error(&self) -> bool {
        self.severity == ProtocolValidationSeverity::Error
    }
}

#[derive(Debug, Default)]
pub struct ProtocolValidationResult {
    issues: Vec<ProtocolValidationIssue>,
}

impl ProtocolValidationResult {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(
        &mut self,
        issue: ProtocolValidationIssue,
    ) {
        self.issues.push(issue);
    }

    pub fn issues(
        &self,
    ) -> &[ProtocolValidationIssue] {
        &self.issues
    }

    pub fn errors(
        &self,
    ) -> impl Iterator<Item = &ProtocolValidationIssue> {
        self.issues
            .iter()
            .filter(|issue| issue.is_error())
    }

    pub fn warnings(
        &self,
    ) -> impl Iterator<Item = &ProtocolValidationIssue> {
        self.issues
            .iter()
            .filter(|issue| !issue.is_error())
    }

    pub fn has_errors(&self) -> bool {
        self.issues.iter().any(|issue| issue.is_error())
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
    ) -> ProtocolValidationResult {
        let mut result = ProtocolValidationResult::new();

        if manifest.id().as_str().trim().is_empty() {
            result.add(
                ProtocolValidationIssue::error(
                    "id",
                    "Protocol ID cannot be empty.",
                ),
            );
        }

        if manifest.name().trim().is_empty() {
            result.add(
                ProtocolValidationIssue::error(
                    "name",
                    "Protocol name cannot be empty.",
                ),
            );
        }

        if manifest.metadata().author.trim().is_empty() {
            result.add(
                ProtocolValidationIssue::warning(
                    "author",
                    "Protocol author is empty.",
                ),
            );
        }

        if manifest.metadata().description.trim().is_empty() {
            result.add(
                ProtocolValidationIssue::warning(
                    "description",
                    "Protocol description is empty.",
                ),
            );
        }

        if manifest.entrypoint().is_none() {
            result.add(
                ProtocolValidationIssue::warning(
                    "entrypoint",
                    "No protocol entrypoint was declared.",
                ),
            );
        }

        result
    }

    pub fn validate_protocol(
        &self,
        protocol: &dyn CustomProtocol,
    ) -> ProtocolValidationResult {
        self.validate_manifest(&protocol.manifest())
    }
}

impl Default for CustomProtocolValidator {
    fn default() -> Self {
        Self::new()
    }
}
