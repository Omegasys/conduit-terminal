//! Custom shell command definitions.

use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct CustomCommand {
    name: String,
    executable: String,
    description: Option<String>,
    arguments: Vec<String>,
}

impl CustomCommand {
    pub fn new(
        name: impl Into<String>,
        executable: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            executable: executable.into(),
            description: None,
            arguments: Vec::new(),
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_argument(mut self, argument: impl Into<String>) -> Self {
        self.arguments.push(argument.into());
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn executable(&self) -> &str {
        &self.executable
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn arguments(&self) -> &[String] {
        &self.arguments
    }
}

#[derive(Debug, Default, Clone)]
pub struct CustomCommandRegistry {
    commands: BTreeMap<String, CustomCommand>,
}

impl CustomCommandRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, command: CustomCommand) -> Option<CustomCommand> {
        self.commands.insert(command.name.clone(), command)
    }

    pub fn get(&self, name: &str) -> Option<&CustomCommand> {
        self.commands.get(name)
    }

    pub fn remove(&mut self, name: &str) -> Option<CustomCommand> {
        self.commands.remove(name)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.commands.contains_key(name)
    }

    pub fn len(&self) -> usize {
        self.commands.len()
    }

    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &CustomCommand)> {
        self.commands.iter()
    }
}
