use std::{
    collections::BTreeMap,
    env,
    path::PathBuf,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShellCommand {
    pub command: String,
    pub arguments: Vec<String>,
}

impl ShellCommand {
    pub fn new<S: Into<String>>(command: S) -> Self {
        Self {
            command: command.into(),
            arguments: Vec::new(),
        }
    }

    pub fn argument<S: Into<String>>(mut self, argument: S) -> Self {
        self.arguments.push(argument.into());
        self
    }

    pub fn arguments(&self) -> &[String] {
        &self.arguments
    }

    pub fn command(&self) -> &str {
        &self.command
    }
}

#[derive(Clone, Debug)]
pub struct ShellEnvironment {
    variables: BTreeMap<String, String>,
    working_directory: Option<PathBuf>,
}

impl ShellEnvironment {
    pub fn new() -> Self {
        Self {
            variables: BTreeMap::new(),
            working_directory: None,
        }
    }

    pub fn from_current_process() -> Self {
        let mut environment = Self::new();

        for (key, value) in env::vars() {
            environment.set(key, value);
        }

        if let Ok(directory) = env::current_dir() {
            environment.set_working_directory(directory);
        }

        environment
    }

    pub fn set<S1, S2>(&mut self, key: S1, value: S2)
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        self.variables.insert(key.into(), value.into());
    }

    pub fn remove(&mut self, key: &str) {
        self.variables.remove(key);
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.variables.get(key).map(String::as_str)
    }

    pub fn variables(&self) -> &BTreeMap<String, String> {
        &self.variables
    }

    pub fn set_working_directory(&mut self, directory: PathBuf) {
        self.working_directory = Some(directory);
    }

    pub fn working_directory(&self) -> Option<&PathBuf> {
        self.working_directory.as_ref()
    }
}

impl Default for ShellEnvironment {
    fn default() -> Self {
        Self::from_current_process()
    }
}

#[derive(Clone, Debug, Default)]
pub struct ShellIntegrationResult {
    pub output: Vec<u8>,
    pub commands: Vec<ShellCommand>,
    pub environment_changes: BTreeMap<String, Option<String>>,
    pub working_directory: Option<PathBuf>,
}

impl ShellIntegrationResult {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_output(&mut self, output: &[u8]) {
        self.output.extend_from_slice(output);
    }

    pub fn add_command(&mut self, command: ShellCommand) {
        self.commands.push(command);
    }

    pub fn set_environment<S: Into<String>>(
        &mut self,
        key: S,
        value: Option<String>,
    ) {
        self.environment_changes.insert(key.into(), value);
    }

    pub fn set_working_directory(&mut self, directory: PathBuf) {
        self.working_directory = Some(directory);
    }
}

pub trait ShellIntegration: Send {
    fn name(&self) -> &str;

    fn executable(&self) -> &str;

    fn detect(&self) -> bool;

    fn startup_command(&self) -> ShellCommand;

    fn initialization_script(&self) -> String;

    fn prompt_markers(&self) -> &[String];

    fn parse_output(
        &mut self,
        output: &[u8],
    ) -> ShellIntegrationResult;

    fn reset(&mut self);

    fn environment(&self) -> &ShellEnvironment;

    fn environment_mut(&mut self) -> &mut ShellEnvironment;
}

pub struct ShellIntegrationManager {
    integrations: Vec<Box<dyn ShellIntegration>>,
}

impl ShellIntegrationManager {
    pub fn new() -> Self {
        Self {
            integrations: Vec::new(),
        }
    }

    pub fn register(
        &mut self,
        integration: Box<dyn ShellIntegration>,
    ) {
        self.integrations.push(integration);
    }

    pub fn detect(&self) -> Option<usize> {
        self.integrations
            .iter()
            .position(|integration| integration.detect())
    }

    pub fn get(
        &self,
        index: usize,
    ) -> Option<&dyn ShellIntegration> {
        self.integrations
            .get(index)
            .map(|integration| integration.as_ref())
    }

    pub fn get_mut(
        &mut self,
        index: usize,
    ) -> Option<&mut dyn ShellIntegration> {
        match self.integrations.get_mut(index) {
            Some(integration) => Some(integration.as_mut()),
            None => None,
        }
    }

    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &dyn ShellIntegration> {
        self.integrations
            .iter()
            .map(|integration| integration.as_ref())
    }

    pub fn len(&self) -> usize {
        self.integrations.len()
    }

    pub fn is_empty(&self) -> bool {
        self.integrations.is_empty()
    }

    pub fn clear(&mut self) {
        self.integrations.clear();
    }

    pub fn register_defaults(&mut self) {
        self.register(Box::new(crate::shell::bash::BashIntegration::new()));
        self.register(Box::new(crate::shell::zsh::ZshIntegration::new()));
        self.register(Box::new(crate::shell::fish::FishIntegration::new()));
    }
}

impl Default for ShellIntegrationManager {
    fn default() -> Self {
        let mut manager = Self::new();
        manager.register_defaults();
        manager
    }
}
