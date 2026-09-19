use std::path::Path;

use super::manifest::ResourceManifest;
use super::resource::{
    Resource,
    ResourceKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationSeverity {
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub struct ValidationIssue {
    pub severity: ValidationSeverity,
    pub field: String,
    pub message: String,
}

impl ValidationIssue {
    pub fn error(
        field: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity: ValidationSeverity::Error,
            field: field.into(),
            message: message.into(),
        }
    }

    pub fn warning(
        field: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity: ValidationSeverity::Warning,
            field: field.into(),
            message: message.into(),
        }
    }

    pub fn is_error(&self) -> bool {
        self.severity == ValidationSeverity::Error
    }
}

#[derive(Debug, Default, Clone)]
pub struct ValidationResult {
    issues: Vec<ValidationIssue>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, issue: ValidationIssue) {
        self.issues.push(issue);
    }

    pub fn error(
        &mut self,
        field: impl Into<String>,
        message: impl Into<String>,
    ) {
        self.add(ValidationIssue::error(field, message));
    }

    pub fn warning(
        &mut self,
        field: impl Into<String>,
        message: impl Into<String>,
    ) {
        self.add(ValidationIssue::warning(field, message));
    }

    pub fn issues(&self) -> &[ValidationIssue] {
        &self.issues
    }

    pub fn errors(&self) -> impl Iterator<Item = &ValidationIssue> {
        self.issues.iter().filter(|issue| issue.is_error())
    }

    pub fn warnings(
        &self,
    ) -> impl Iterator<Item = &ValidationIssue> {
        self.issues
            .iter()
            .filter(|issue| !issue.is_error())
    }

    pub fn is_valid(&self) -> bool {
        !self.issues.iter().any(ValidationIssue::is_error)
    }

    pub fn has_warnings(&self) -> bool {
        self.issues
            .iter()
            .any(|issue| !issue.is_error())
    }

    pub fn len(&self) -> usize {
        self.issues.len()
    }

    pub fn is_empty(&self) -> bool {
        self.issues.is_empty()
    }
}

pub struct ResourceValidator;

impl ResourceValidator {
    pub fn validate_resource(
        resource: &Resource,
    ) -> ValidationResult {
        let mut result = ValidationResult::new();

        if resource.name().trim().is_empty() {
            result.error(
                "name",
                "resource name cannot be empty",
            );
        }

        if resource.path().as_os_str().is_empty() {
            result.error(
                "path",
                "resource path cannot be empty",
            );
        }

        if !resource.path().exists() {
            result.warning(
                "path",
                format!(
                    "resource path does not currently exist: {}",
                    resource.path().display()
                ),
            );
        }

        if let Some(extension) = resource.kind().extension() {
            Self::validate_extension(
                resource.path(),
                extension,
                &mut result,
            );
        }

        result
    }

    pub fn validate_manifest(
        manifest: &ResourceManifest,
    ) -> ValidationResult {
        let mut result = ValidationResult::new();

        if manifest.name().trim().is_empty() {
            result.error(
                "name",
                "manifest name cannot be empty",
            );
        }

        if manifest.version().trim().is_empty() {
            result.error(
                "version",
                "manifest version cannot be empty",
            );
        }

        if manifest.name().contains('/') {
            result.error(
                "name",
                "manifest name cannot contain path separators",
            );
        }

        if manifest.version().contains('/') {
            result.error(
                "version",
                "manifest version cannot contain path separators",
            );
        }

        if matches!(
            manifest.kind(),
            ResourceKind::Plugin | ResourceKind::Extension
        ) && manifest.entrypoint().is_none()
        {
            result.warning(
                "entrypoint",
                "this resource type normally specifies an entrypoint",
            );
        }

        result
    }

    pub fn validate_name(name: &str) -> ValidationResult {
        let mut result = ValidationResult::new();

        if name.trim().is_empty() {
            result.error(
                "name",
                "name cannot be empty",
            );
            return result;
        }

        if name == "." || name == ".." {
            result.error(
                "name",
                "name cannot be a path traversal component",
            );
        }

        if name.contains('/') || name.contains('\\') {
            result.error(
                "name",
                "name cannot contain path separators",
            );
        }

        if name.contains('\0') {
            result.error(
                "name",
                "name cannot contain NUL characters",
            );
        }

        result
    }

    fn validate_extension(
        path: &Path,
        expected: &str,
        result: &mut ValidationResult,
    ) {
        match path.extension().and_then(|extension| extension.to_str()) {
            Some(actual) if actual.eq_ignore_ascii_case(expected) => {}

            Some(actual) => {
                result.warning(
                    "extension",
                    format!(
                        "expected '.{expected}' but found '.{actual}'"
                    ),
                );
            }

            None => {
                result.warning(
                    "extension",
                    format!(
                        "resource has no file extension; expected '.{expected}'"
                    ),
                );
            }
        }
    }
}
