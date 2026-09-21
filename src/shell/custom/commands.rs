use std::time::Duration;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CustomCommand {
    pub command: String,
    pub arguments: Vec<String>,
}

impl CustomCommand {
    pub fn new<S: Into<String>>(
        command: S,
    ) -> Self {
        Self {
            command: command.into(),
            arguments: Vec::new(),
        }
    }

    pub fn argument<S: Into<String>>(
        mut self,
        argument: S,
    ) -> Self {
        self.arguments.push(argument.into());
        self
    }
}

#[derive(Clone, Debug)]
pub struct CustomCommandResult {
    pub exit_code: Option<i32>,
    pub output: Vec<u8>,
    pub error_output: Vec<u8>,
    pub duration: Option<Duration>,
}

impl CustomCommandResult {
    pub fn success() -> Self {
        Self {
            exit_code: Some(0),
            output: Vec::new(),
            error_output: Vec::new(),
            duration: None,
        }
    }

    pub fn failed(code: i32) -> Self {
        Self {
            exit_code: Some(code),
            output: Vec::new(),
            error_output: Vec::new(),
            duration: None,
        }
    }

    pub fn is_success(&self) -> bool {
        self.exit_code == Some(0)
    }

    pub fn is_failure(&self) -> bool {
        matches!(
            self.exit_code,
            Some(code) if code != 0
        )
    }
}
