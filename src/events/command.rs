use std::collections::HashMap;

/// Unique command identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommandId(String);

impl CommandId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A command that can be dispatched through the event system.
#[derive(Debug, Clone)]
pub struct Command {
    id: CommandId,
    arguments: Vec<String>,
    metadata: HashMap<String, String>,
}

impl Command {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: CommandId::new(id),
            arguments: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn id(&self) -> &CommandId {
        &self.id
    }

    pub fn name(&self) -> &str {
        self.id.as_str()
    }

    pub fn arguments(&self) -> &[String] {
        &self.arguments
    }

    pub fn add_argument(&mut self, argument: impl Into<String>) {
        self.arguments.push(argument.into());
    }

    pub fn with_argument(mut self, argument: impl Into<String>) -> Self {
        self.add_argument(argument);
        self
    }

    pub fn set_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) {
        self.metadata.insert(key.into(), value.into());
    }

    pub fn metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    pub fn metadata_value(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(String::as_str)
    }
}

/// Result returned from command execution.
#[derive(Debug, Clone)]
pub enum CommandResult {
    Success,
    Text(String),
    Boolean(bool),
    Error(String),
}

impl CommandResult {
    pub fn is_success(&self) -> bool {
        !matches!(self, Self::Error(_))
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self::Error(message.into())
    }
}

/// Trait implemented by command handlers.
pub trait CommandHandler: Send + Sync {
    fn execute(&self, command: &Command) -> CommandResult;
}
