use super::semantic_shell::ShellSemanticEvent;

#[derive(Debug, Default)]
pub struct VsCodeShellIntegration;

impl VsCodeShellIntegration {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(
        &self,
        command: &str,
        data: &str,
    ) -> Option<ShellSemanticEvent> {
        match command {
            "A" => Some(ShellSemanticEvent::prompt_start()),

            "B" => Some(ShellSemanticEvent::prompt_end()),

            "C" => Some(ShellSemanticEvent::command_start()),

            "D" => Some(ShellSemanticEvent::command_end()),

            "E" => Some(ShellSemanticEvent::exit_status(
                data.parse().unwrap_or(0),
            )),

            "P" => {
                let mut parts = data.splitn(2, '=');

                let key = parts.next()?;
                let value = parts.next().unwrap_or_default();

                if key == "Cwd" {
                    Some(ShellSemanticEvent::working_directory(value))
                } else {
                    None
                }
            }

            _ => None,
        }
    }
}
