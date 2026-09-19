//! Configuration validation.

use super::{
    parser::ParsedConfig,
    schema::{
        ConfigSchema,
        ConfigValue,
        SchemaType,
    },
};

/// Severity of a validation issue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationSeverity {
    Warning,
    Error,
}

/// One configuration validation issue.
#[derive(Debug, Clone)]
pub struct ValidationIssue {
    pub path: String,
    pub message: String,
    pub severity: ValidationSeverity,
}

impl ValidationIssue {
    pub fn error(
        path: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            path: path.into(),
            message: message.into(),
            severity:
                ValidationSeverity::Error,
        }
    }

    pub fn warning(
        path: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            path: path.into(),
            message: message.into(),
            severity:
                ValidationSeverity::Warning,
        }
    }
}

/// Validation result containing all discovered problems.
#[derive(Debug, Clone, Default)]
pub struct ConfigValidationError {
    issues: Vec<ValidationIssue>,
}

impl ConfigValidationError {
    pub fn new(
        issues: Vec<ValidationIssue>,
    ) -> Self {
        Self { issues }
    }

    pub fn issues(
        &self,
    ) -> &[ValidationIssue] {
        &self.issues
    }

    pub fn has_errors(
        &self,
    ) -> bool {
        self.issues.iter().any(
            |issue| {
                issue.severity
                    == ValidationSeverity::Error
            }
        )
    }

    pub fn is_empty(
        &self,
    ) -> bool {
        self.issues.is_empty()
    }
}

impl std::fmt::Display
    for ConfigValidationError
{
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        for issue in &self.issues {
            writeln!(
                formatter,
                "{}: {}",
                issue.path,
                issue.message
            )?;
        }

        Ok(())
    }
}

impl std::error::Error
    for ConfigValidationError {}

/// Validates parsed configuration against a schema.
#[derive(Debug, Clone)]
pub struct ConfigValidator {
    schema: ConfigSchema,
}

impl ConfigValidator {
    pub fn new(
        schema: ConfigSchema,
    ) -> Self {
        Self { schema }
    }

    pub fn schema(
        &self,
    ) -> &ConfigSchema {
        &self.schema
    }

    pub fn validate(
        &self,
        config: &ParsedConfig,
    ) -> Result<(), ConfigValidationError> {
        let mut issues =
            Vec::new();

        for (
            path,
            field,
        ) in self.schema.fields()
        {
            let Some(value) =
                config.get(path)
            else {
                if field.is_required() {
                    issues.push(
                        ValidationIssue::error(
                            path,
                            "required configuration value is missing",
                        )
                    );
                }

                continue;
            };

            if !matches_type(
                value,
                field.field_type(),
            ) {
                issues.push(
                    ValidationIssue::error(
                        path,
                        format!(
                            "expected {:?}, got {:?}",
                            field.field_type(),
                            value_type(value)
                        ),
                    )
                );

                continue;
            }

            if let Some(number) =
                numeric_value(value)
            {
                if let Some(minimum) =
                    field.minimum_value()
                {
                    if number < minimum {
                        issues.push(
                            ValidationIssue::error(
                                path,
                                format!(
                                    "value must be at least {}",
                                    minimum
                                ),
                            )
                        );
                    }
                }

                if let Some(maximum) =
                    field.maximum_value()
                {
                    if number > maximum {
                        issues.push(
                            ValidationIssue::error(
                                path,
                                format!(
                                    "value must be at most {}",
                                    maximum
                                ),
                            )
                        );
                    }
                }
            }
        }

        if issues.is_empty() {
            Ok(())
        } else {
            Err(
                ConfigValidationError::new(
                    issues
                )
            )
        }
    }
}

impl Default for ConfigValidator {
    fn default() -> Self {
        Self::new(
            ConfigSchema::default()
        )
    }
}

fn matches_type(
    value: &ConfigValue,
    expected: SchemaType,
) -> bool {
    matches!(
        (value, expected),
        (
            ConfigValue::String(_),
            SchemaType::String
        )
        | (
            ConfigValue::Integer(_),
            SchemaType::Integer
        )
        | (
            ConfigValue::Float(_),
            SchemaType::Float
        )
        | (
            ConfigValue::Boolean(_),
            SchemaType::Boolean
        )
        | (
            ConfigValue::Array(_),
            SchemaType::Array
        )
        | (
            ConfigValue::Table(_),
            SchemaType::Table
        )
    )
}

fn numeric_value(
    value: &ConfigValue,
) -> Option<f64> {
    match value {
        ConfigValue::Integer(value) =>
            Some(*value as f64),

        ConfigValue::Float(value) =>
            Some(*value),

        _ => None,
    }
}

fn value_type(
    value: &ConfigValue,
) -> SchemaType {
    match value {
        ConfigValue::String(_) =>
            SchemaType::String,

        ConfigValue::Integer(_) =>
            SchemaType::Integer,

        ConfigValue::Float(_) =>
            SchemaType::Float,

        ConfigValue::Boolean(_) =>
            SchemaType::Boolean,

        ConfigValue::Array(_) =>
            SchemaType::Array,

        ConfigValue::Table(_) =>
            SchemaType::Table,

        ConfigValue::Null =>
            SchemaType::String,
    }
}
