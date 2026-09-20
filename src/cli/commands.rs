use std::fmt;

use super::{
    panes::PaneCommand,
    tabs::TabCommand,
    windows::WindowCommand,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandAction {
    NewWindow,
    CloseWindow,
    ListWindows,

    NewTab,
    CloseTab,
    ListTabs,

    NewPane,
    ClosePane,
    SplitHorizontal,
    SplitVertical,
    ListPanes,

    NewWorkspace,
    CloseWorkspace,
    SwitchWorkspace,

    NewSession,
    CloseSession,
    RestoreSession,

    OpenSettings,
    ReloadConfiguration,
    ReloadResources,

    ListPlugins,
    SecurityStatus,

    Version,
    Help,
    Quit,

    Custom(String),
}

impl CommandAction {
    pub fn name(&self) -> &str {
        match self {
            Self::NewWindow => "new-window",
            Self::CloseWindow => "close-window",
            Self::ListWindows => "list-windows",

            Self::NewTab => "new-tab",
            Self::CloseTab => "close-tab",
            Self::ListTabs => "list-tabs",

            Self::NewPane => "new-pane",
            Self::ClosePane => "close-pane",
            Self::SplitHorizontal => "split-horizontal",
            Self::SplitVertical => "split-vertical",
            Self::ListPanes => "list-panes",

            Self::NewWorkspace => "new-workspace",
            Self::CloseWorkspace => "close-workspace",
            Self::SwitchWorkspace => "switch-workspace",

            Self::NewSession => "new-session",
            Self::CloseSession => "close-session",
            Self::RestoreSession => "restore-session",

            Self::OpenSettings => "settings",
            Self::ReloadConfiguration => "reload-config",
            Self::ReloadResources => "reload-resources",

            Self::ListPlugins => "list-plugins",
            Self::SecurityStatus => "security-status",

            Self::Version => "version",
            Self::Help => "help",
            Self::Quit => "quit",

            Self::Custom(value) => value,
        }
    }

    pub fn from_name(name: &str) -> Self {
        match name {
            "new-window" | "window" => Self::NewWindow,
            "close-window" => Self::CloseWindow,
            "list-windows" | "windows" => Self::ListWindows,

            "new-tab" | "tab" => Self::NewTab,
            "close-tab" => Self::CloseTab,
            "list-tabs" | "tabs" => Self::ListTabs,

            "new-pane" | "pane" => Self::NewPane,
            "close-pane" => Self::ClosePane,
            "split-horizontal" => Self::SplitHorizontal,
            "split-vertical" => Self::SplitVertical,
            "list-panes" | "panes" => Self::ListPanes,

            "new-workspace" => Self::NewWorkspace,
            "close-workspace" => Self::CloseWorkspace,
            "switch-workspace" => Self::SwitchWorkspace,

            "new-session" => Self::NewSession,
            "close-session" => Self::CloseSession,
            "restore-session" => Self::RestoreSession,

            "settings" => Self::OpenSettings,
            "reload-config" => Self::ReloadConfiguration,
            "reload-resources" => Self::ReloadResources,

            "list-plugins" | "plugins" => Self::ListPlugins,
            "security-status" => Self::SecurityStatus,

            "version" | "--version" => Self::Version,
            "help" | "--help" | "-h" => Self::Help,
            "quit" | "exit" => Self::Quit,

            other => Self::Custom(other.to_string()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CliCommand {
    action: CommandAction,
    arguments: Vec<String>,
    options: Vec<(String, String)>,
}

impl CliCommand {
    pub fn new(action: CommandAction) -> Self {
        Self {
            action,
            arguments: Vec::new(),
            options: Vec::new(),
        }
    }

    pub fn action(&self) -> &CommandAction {
        &self.action
    }

    pub fn arguments(&self) -> &[String] {
        &self.arguments
    }

    pub fn options(&self) -> &[(String, String)] {
        &self.options
    }

    pub fn add_argument(&mut self, value: impl Into<String>) {
        self.arguments.push(value.into());
    }

    pub fn set_option(
        &mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) {
        self.options.push((name.into(), value.into()));
    }

    pub fn option(&self, name: &str) -> Option<&str> {
        self.options
            .iter()
            .find(|(key, _)| key == name)
            .map(|(_, value)| value.as_str())
    }

    pub fn from_line(line: &super::parser::CliCommandLine) -> Self {
        let mut command = Self::new(CommandAction::from_name(line.command()));

        for argument in line.arguments() {
            command.add_argument(argument.clone());
        }

        for (name, value) in line.options() {
            let value = match value {
                super::parser::CliValue::String(value) => value.clone(),
                super::parser::CliValue::Integer(value) => value.to_string(),
                super::parser::CliValue::Boolean(value) => value.to_string(),
            };

            command.set_option(name.clone(), value);
        }

        command
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CliCommandResult {
    Success,
    Message(String),
    Error(String),
}

impl CliCommandResult {
    pub fn success() -> Self {
        Self::Success
    }

    pub fn message(value: impl Into<String>) -> Self {
        Self::Message(value.into())
    }

    pub fn error(value: impl Into<String>) -> Self {
        Self::Error(value.into())
    }

    pub fn is_success(&self) -> bool {
        !matches!(self, Self::Error(_))
    }
}

impl fmt::Display for CliCommandResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Success => write!(formatter, "ok"),
            Self::Message(value) => write!(formatter, "{}", value),
            Self::Error(value) => write!(formatter, "error: {}", value),
        }
    }
}

pub fn parse_window_command(
    command: &CliCommand,
) -> Option<WindowCommand> {
    match command.action() {
        CommandAction::NewWindow => Some(WindowCommand::new(
            super::windows::WindowCommandAction::Create,
        )),

        CommandAction::CloseWindow => Some(WindowCommand::new(
            super::windows::WindowCommandAction::Close,
        )),

        CommandAction::ListWindows => Some(WindowCommand::new(
            super::windows::WindowCommandAction::List,
        )),

        _ => None,
    }
}

pub fn parse_tab_command(command: &CliCommand) -> Option<TabCommand> {
    match command.action() {
        CommandAction::NewTab => Some(TabCommand::new(
            super::tabs::TabCommandAction::Create,
        )),

        CommandAction::CloseTab => Some(TabCommand::new(
            super::tabs::TabCommandAction::Close,
        )),

        CommandAction::ListTabs => Some(TabCommand::new(
            super::tabs::TabCommandAction::List,
        )),

        _ => None,
    }
}

pub fn parse_pane_command(command: &CliCommand) -> Option<PaneCommand> {
    match command.action() {
        CommandAction::NewPane => Some(PaneCommand::new(
            super::panes::PaneCommandAction::Create,
        )),

        CommandAction::ClosePane => Some(PaneCommand::new(
            super::panes::PaneCommandAction::Close,
        )),

        CommandAction::SplitHorizontal => Some(PaneCommand::new(
            super::panes::PaneCommandAction::SplitHorizontal,
        )),

        CommandAction::SplitVertical => Some(PaneCommand::new(
            super::panes::PaneCommandAction::SplitVertical,
        )),

        CommandAction::ListPanes => Some(PaneCommand::new(
            super::panes::PaneCommandAction::List,
        )),

        _ => None,
    }
}
