use std::collections::BTreeMap;

use crate::config_engine::ConfigValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellDetection {
    Automatic,
    Environment,
    LoginShell,
    Manual,
}

#[derive(Debug, Clone)]
pub struct ShellSettings {
    detection: ShellDetection,
    shell_path: Option<String>,
    login_shell: bool,
    interactive_shell: bool,
    inherit_environment: bool,
    clear_environment: bool,
    working_directory: Option<String>,
    start_directory_current: bool,
    preserve_shell_title: bool,
    allow_shell_title_changes: bool,
    shell_integration: bool,
    shell_integration_prompt: bool,
    shell_integration_directory: bool,
    shell_integration_command: bool,
    send_environment: bool,
}

impl Default for ShellSettings {
    fn default() -> Self {
        Self {
            detection: ShellDetection::Automatic,
            shell_path: None,
            login_shell: true,
            interactive_shell: true,
            inherit_environment: true,
            clear_environment: false,
            working_directory: None,
            start_directory_current: true,
            preserve_shell_title: false,
            allow_shell_title_changes: true,
            shell_integration: true,
            shell_integration_prompt: true,
            shell_integration_directory: true,
            shell_integration_command: true,
            send_environment: true,
        }
    }
}

impl ShellSettings {
    pub fn detection(&self) -> ShellDetection {
        self.detection
    }

    pub fn set_detection(&mut self, value: ShellDetection) {
        self.detection = value;
    }

    pub fn shell_path(&self) -> Option<&str> {
        self.shell_path.as_deref()
    }

    pub fn set_shell_path(&mut self, value: Option<String>) {
        self.shell_path = value;
    }

    pub fn login_shell(&self) -> bool {
        self.login_shell
    }

    pub fn set_login_shell(&mut self, value: bool) {
        self.login_shell = value;
    }

    pub fn interactive_shell(&self) -> bool {
        self.interactive_shell
    }

    pub fn set_interactive_shell(&mut self, value: bool) {
        self.interactive_shell = value;
    }

    pub fn inherit_environment(&self) -> bool {
        self.inherit_environment
    }

    pub fn set_inherit_environment(&mut self, value: bool) {
        self.inherit_environment = value;
    }

    pub fn clear_environment(&self) -> bool {
        self.clear_environment
    }

    pub fn set_clear_environment(&mut self, value: bool) {
        self.clear_environment = value;
    }

    pub fn working_directory(&self) -> Option<&str> {
        self.working_directory.as_deref()
    }

    pub fn set_working_directory(&mut self, value: Option<String>) {
        self.working_directory = value;
    }

    pub fn start_directory_current(&self) -> bool {
        self.start_directory_current
    }

    pub fn set_start_directory_current(&mut self, value: bool) {
        self.start_directory_current = value;
    }

    pub fn preserve_shell_title(&self) -> bool {
        self.preserve_shell_title
    }

    pub fn set_preserve_shell_title(&mut self, value: bool) {
        self.preserve_shell_title = value;
    }

    pub fn allow_shell_title_changes(&self) -> bool {
        self.allow_shell_title_changes
    }

    pub fn set_allow_shell_title_changes(&mut self, value: bool) {
        self.allow_shell_title_changes = value;
    }

    pub fn shell_integration(&self) -> bool {
        self.shell_integration
    }

    pub fn set_shell_integration(&mut self, value: bool) {
        self.shell_integration = value;
    }

    pub fn shell_integration_prompt(&self) -> bool {
        self.shell_integration_prompt
    }

    pub fn set_shell_integration_prompt(&mut self, value: bool) {
        self.shell_integration_prompt = value;
    }

    pub fn shell_integration_directory(&self) -> bool {
        self.shell_integration_directory
    }

    pub fn set_shell_integration_directory(&mut self, value: bool) {
        self.shell_integration_directory = value;
    }

    pub fn shell_integration_command(&self) -> bool {
        self.shell_integration_command
    }

    pub fn set_shell_integration_command(&mut self, value: bool) {
        self.shell_integration_command = value;
    }

    pub fn send_environment(&self) -> bool {
        self.send_environment
    }

    pub fn set_send_environment(&mut self, value: bool) {
        self.send_environment = value;
    }

    pub fn to_config(&self) -> BTreeMap<String, ConfigValue> {
        let mut values = BTreeMap::new();

        values.insert(
            "detection".into(),
            ConfigValue::String(format!("{:?}", self.detection).to_lowercase()),
        );

        if let Some(path) = &self.shell_path {
            values.insert("shell_path".into(), ConfigValue::String(path.clone()));
        }

        if let Some(directory) = &self.working_directory {
            values.insert(
                "working_directory".into(),
                ConfigValue::String(directory.clone()),
            );
        }

        values.insert("login_shell".into(), ConfigValue::Boolean(self.login_shell));
        values.insert(
            "interactive_shell".into(),
            ConfigValue::Boolean(self.interactive_shell),
        );
        values.insert(
            "inherit_environment".into(),
            ConfigValue::Boolean(self.inherit_environment),
        );
        values.insert(
            "clear_environment".into(),
            ConfigValue::Boolean(self.clear_environment),
        );
        values.insert(
            "start_directory_current".into(),
            ConfigValue::Boolean(self.start_directory_current),
        );
        values.insert(
            "preserve_shell_title".into(),
            ConfigValue::Boolean(self.preserve_shell_title),
        );
        values.insert(
            "allow_shell_title_changes".into(),
            ConfigValue::Boolean(self.allow_shell_title_changes),
        );
        values.insert(
            "shell_integration".into(),
            ConfigValue::Boolean(self.shell_integration),
        );
        values.insert(
            "shell_integration_prompt".into(),
            ConfigValue::Boolean(self.shell_integration_prompt),
        );
        values.insert(
            "shell_integration_directory".into(),
            ConfigValue::Boolean(self.shell_integration_directory),
        );
        values.insert(
            "shell_integration_command".into(),
            ConfigValue::Boolean(self.shell_integration_command),
        );
        values.insert(
            "send_environment".into(),
            ConfigValue::Boolean(self.send_environment),
        );

        values
    }
}
