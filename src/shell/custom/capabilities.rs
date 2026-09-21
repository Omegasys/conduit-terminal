#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShellExecutionModel {
    ExternalProcess,
    Embedded,
    Hybrid,
}

impl Default for ShellExecutionModel {
    fn default() -> Self {
        Self::ExternalProcess
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShellColorSupport {
    None,
    Basic8,
    Standard16,
    Indexed256,
    TrueColor,
}

impl Default for ShellColorSupport {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Debug)]
pub struct CustomShellCapabilities {
    pub execution_model: ShellExecutionModel,
    pub color: ShellColorSupport,

    pub unicode: bool,
    pub command_history: bool,
    pub command_completion: bool,
    pub command_aliases: bool,
    pub scripting: bool,
    pub environment_variables: bool,
    pub working_directory_tracking: bool,
    pub prompt_detection: bool,

    pub pipelines: bool,
    pub redirection: bool,
    pub job_control: bool,

    pub interactive_programs: bool,
    pub terminal_protocols: bool,
}

impl Default for CustomShellCapabilities {
    fn default() -> Self {
        Self {
            execution_model:
                ShellExecutionModel::ExternalProcess,

            color: ShellColorSupport::None,

            unicode: true,
            command_history: true,
            command_completion: false,
            command_aliases: false,
            scripting: false,
            environment_variables: true,
            working_directory_tracking: true,
            prompt_detection: true,

            pipelines: false,
            redirection: false,
            job_control: false,

            interactive_programs: true,
            terminal_protocols: true,
        }
    }
}

impl CustomShellCapabilities {
    pub fn basic() -> Self {
        Self::default()
    }

    pub fn modern() -> Self {
        Self {
            color: ShellColorSupport::TrueColor,
            command_completion: true,
            command_aliases: true,
            scripting: true,
            pipelines: true,
            redirection: true,
            job_control: true,
            ..Default::default()
        }
    }

    pub fn embedded() -> Self {
        Self {
            execution_model:
                ShellExecutionModel::Embedded,
            ..Default::default()
        }
    }

    pub fn supports_truecolor(&self) -> bool {
        matches!(
            self.color,
            ShellColorSupport::TrueColor
        )
    }
}
