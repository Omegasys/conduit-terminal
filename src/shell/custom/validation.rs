//! Custom shell validation.

use super::{
    errors::{CustomShellError, CustomShellResult},
    shell::CustomShell,
};

#[derive(Debug, Default)]
pub struct CustomShellValidator;

impl CustomShellValidator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate(&self, shell: &CustomShell) -> CustomShellResult<()> {
        shell.validate()?;

        if shell.id().trim().is_empty() {
            return Err(CustomShellError::Validation(
                "shell identifier is empty".into(),
            ));
        }

        if shell.name().trim().is_empty() {
            return Err(CustomShellError::Validation(
                "shell name is empty".into(),
            ));
        }

        if shell.executable().trim().is_empty() {
            return Err(CustomShellError::Validation(
                "shell executable is empty".into(),
            ));
        }

        Ok(())
    }

    pub fn validate_executable(
        &self,
        shell: &CustomShell,
    ) -> CustomShellResult<()> {
        let executable = shell.executable();

        if executable.contains('/') {
            let path = std::path::Path::new(executable);

            if !path.exists() {
                return Err(CustomShellError::ExecutableNotFound(
                    executable.to_string(),
                ));
            }
        }

        Ok(())
    }
}
