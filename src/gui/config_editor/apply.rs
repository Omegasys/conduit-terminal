use crate::config_engine::{
    ConfigError,
    ConfigSerializer,
    ConfigState,
    ConfigTransaction,
};

use super::validation::{
    ConfigDiagnostic,
    ConfigDiagnosticLevel,
};

/// Result of attempting to apply editor changes.
#[derive(Debug, Clone)]
pub enum ApplyResult {
    Applied,
    ValidationFailed(Vec<ConfigDiagnostic>),
    Error(String),
}

/// Controls whether applying configuration should require confirmation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplyConfirmation {
    Never,
    OnRiskyChanges,
    Always,
}

/// Configuration apply controller.
///
/// This component coordinates editor text, parsing, validation and
/// the configuration state. Persistent file I/O remains outside this
/// UI layer.
#[derive(Debug)]
pub struct ConfigApplyController {
    confirmation: ApplyConfirmation,
    automatic_validation: bool,
    last_result: Option<ApplyResult>,
}

impl Default for ConfigApplyController {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigApplyController {
    pub fn new() -> Self {
        Self {
            confirmation: ApplyConfirmation::OnRiskyChanges,
            automatic_validation: true,
            last_result: None,
        }
    }

    pub fn confirmation(&self) -> ApplyConfirmation {
        self.confirmation
    }

    pub fn set_confirmation(&mut self, confirmation: ApplyConfirmation) {
        self.confirmation = confirmation;
    }

    pub fn automatic_validation(&self) -> bool {
        self.automatic_validation
    }

    pub fn set_automatic_validation(&mut self, enabled: bool) {
        self.automatic_validation = enabled;
    }

    pub fn last_result(&self) -> Option<&ApplyResult> {
        self.last_result.as_ref()
    }

    pub fn validate(
        &self,
        text: &str,
    ) -> Result<
        std::collections::BTreeMap<String, crate::config_engine::ConfigValue>,
        ConfigError,
    > {
        ConfigSerializer::from_toml(text)
    }

    pub fn apply(
        &mut self,
        text: &str,
        state: &mut ConfigState,
    ) -> ApplyResult {
        match self.validate(text) {
            Ok(values) => {
                state.replace(values);
                state.mark_saved();

                let result = ApplyResult::Applied;
                self.last_result = Some(result.clone());

                result
            }

            Err(error) => {
                let diagnostic = ConfigDiagnostic::new(
                    ConfigDiagnosticLevel::Error,
                    error.to_string(),
                )
                .with_code("config.apply");

                let result = ApplyResult::ValidationFailed(vec![diagnostic]);

                self.last_result = Some(result.clone());

                result
            }
        }
    }

    pub fn apply_transaction(
        &mut self,
        transaction: &mut ConfigTransaction,
        state: &mut ConfigState,
    ) -> ApplyResult {
        if transaction.is_committed() {
            let result = ApplyResult::Error(
                "configuration transaction has already been committed"
                    .to_string(),
            );

            self.last_result = Some(result.clone());

            return result;
        }

        let values = transaction.values().clone();

        state.replace(values);
        transaction.commit();

        let result = ApplyResult::Applied;
        self.last_result = Some(result.clone());

        result
    }

    pub fn reset_result(&mut self) {
        self.last_result = None;
    }
}
