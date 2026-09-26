use super::semantic_shell::ShellSemanticEvent;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FinalTermEvent {
    Shell(ShellSemanticEvent),
    Unknown(Vec<u8>),
}

#[derive(Debug, Default)]
pub struct FinalTermProtocol;

impl FinalTermProtocol {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, command: u8, data: &str) -> FinalTermEvent {
        match command {
            b'A' => FinalTermEvent::Shell(
                ShellSemanticEvent::prompt_start(),
            ),

            b'B' => FinalTermEvent::Shell(
                ShellSemanticEvent::command_start(),
            ),

            b'C' => FinalTermEvent::Shell(
                ShellSemanticEvent::command_end(),
            ),

            b'D' => {
                let status = data.parse().unwrap_or(0);

                FinalTermEvent::Shell(
                    ShellSemanticEvent::exit_status(status),
                )
            }

            _ => FinalTermEvent::Unknown(data.as_bytes().to_vec()),
        }
    }
}
