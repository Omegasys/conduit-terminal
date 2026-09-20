use crate::config_engine::{ConfigError, ConfigSerializer};

/// Severity of an editor diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConfigDiagnosticLevel {
    Hint,
    Information,
    Warning,
    Error,
}

/// A single validation diagnostic shown by the editor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigDiagnostic {
    pub level: ConfigDiagnosticLevel,
    pub message: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub length: Option<usize>,
    pub code: Option<String>,
}

impl ConfigDiagnostic {
    pub fn new(
        level: ConfigDiagnosticLevel,
        message: impl Into<String>,
    ) -> Self {
        Self {
            level,
            message: message.into(),
            line: None,
            column: None,
            length: None,
            code: None,
        }
    }

    pub fn at(mut self, line: usize, column: usize) -> Self {
        self.line = Some(line);
        self.column = Some(column);
        self
    }

    pub fn with_length(mut self, length: usize) -> Self {
        self.length = Some(length);
        self
    }

    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    pub fn is_error(&self) -> bool {
        self.level == ConfigDiagnosticLevel::Error
    }

    pub fn is_warning(&self) -> bool {
        self.level == ConfigDiagnosticLevel::Warning
    }
}

/// Current validation state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigValidationState {
    NotValidated,
    Validating,
    Valid,
    Warnings,
    Invalid,
}

/// GUI-side configuration validation manager.
///
/// This provides editor-friendly diagnostics while the config engine
/// remains responsible for authoritative parsing and schema validation.
#[derive(Debug, Clone, Default)]
pub struct ConfigValidatorUi {
    state: ConfigValidationState,
    diagnostics: Vec<ConfigDiagnostic>,
    validation_revision: u64,
}

impl ConfigValidatorUi {
    pub fn new() -> Self {
        Self {
            state: ConfigValidationState::NotValidated,
            diagnostics: Vec::new(),
            validation_revision: 0,
        }
    }

    pub fn state(&self) -> ConfigValidationState {
        self.state
    }

    pub fn diagnostics(&self) -> &[ConfigDiagnostic] {
        &self.diagnostics
    }

    pub fn validation_revision(&self) -> u64 {
        self.validation_revision
    }

    pub fn errors(&self) -> impl Iterator<Item = &ConfigDiagnostic> {
        self.diagnostics.iter().filter(|item| item.is_error())
    }

    pub fn warnings(&self) -> impl Iterator<Item = &ConfigDiagnostic> {
        self.diagnostics.iter().filter(|item| item.is_warning())
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(ConfigDiagnostic::is_error)
    }

    pub fn has_warnings(&self) -> bool {
        self.diagnostics.iter().any(ConfigDiagnostic::is_warning)
    }

    pub fn is_valid(&self) -> bool {
        self.state == ConfigValidationState::Valid
            || self.state == ConfigValidationState::Warnings
    }

    pub fn set_validating(&mut self) {
        self.state = ConfigValidationState::Validating;
    }

    pub fn clear(&mut self) {
        self.diagnostics.clear();
        self.state = ConfigValidationState::NotValidated;
        self.validation_revision = self.validation_revision.wrapping_add(1);
    }

    pub fn add(&mut self, diagnostic: ConfigDiagnostic) {
        self.diagnostics.push(diagnostic);
        self.update_state();
    }

    pub fn validate(&mut self, text: &str) -> bool {
        self.set_validating();
        self.diagnostics.clear();

        match ConfigSerializer::from_toml(text) {
            Ok(_) => {
                self.state = ConfigValidationState::Valid;
                self.validation_revision =
                    self.validation_revision.wrapping_add(1);

                true
            }
            Err(error) => {
                self.add_config_error(error);

                self.validation_revision =
                    self.validation_revision.wrapping_add(1);

                false
            }
        }
    }

    pub fn validate_with<F>(
        &mut self,
        text: &str,
        validator: F,
    ) -> bool
    where
        F: FnOnce(
            &std::collections::BTreeMap<
                String,
                crate::config_engine::ConfigValue,
            >,
        ) -> Vec<ConfigDiagnostic>,
    {
        self.set_validating();
        self.diagnostics.clear();

        match ConfigSerializer::from_toml(text) {
            Ok(values) => {
                self.diagnostics = validator(&values);
                self.update_state();

                self.validation_revision =
                    self.validation_revision.wrapping_add(1);

                !self.has_errors()
            }
            Err(error) => {
                self.add_config_error(error);

                self.validation_revision =
                    self.validation_revision.wrapping_add(1);

                false
            }
        }
    }

    fn add_config_error(&mut self, error: ConfigError) {
        let message = error.to_string();

        let diagnostic = ConfigDiagnostic::new(
            ConfigDiagnosticLevel::Error,
            message,
        )
        .with_code("config.parse");

        self.diagnostics.push(diagnostic);
        self.state = ConfigValidationState::Invalid;
    }

    fn update_state(&mut self) {
        if self.has_errors() {
            self.state = ConfigValidationState::Invalid;
        } else if self.has_warnings() {
            self.state = ConfigValidationState::Warnings;
        } else {
            self.state = ConfigValidationState::Valid;
        }
    }
}
