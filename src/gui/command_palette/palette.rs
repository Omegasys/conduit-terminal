use super::categories::CommandCategoryManager;
use super::commands::{CommandEntry, CommandManager};
use super::history::CommandHistory;
use super::search::{CommandSearch, CommandSearchMatch};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandPaletteMode {
    Commands,
    Search,
    Recent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandPaletteState {
    Closed,
    Opening,
    Open,
    Searching,
    Executing,
    Closing,
}

#[derive(Debug)]
pub struct CommandPalette {
    id: String,
    title: String,
    state: CommandPaletteState,
    mode: CommandPaletteMode,
    visible: bool,
    enabled: bool,
    query: String,
    selected_index: usize,
    selected_command: Option<String>,
    commands: CommandManager,
    categories: CommandCategoryManager,
    search: CommandSearch,
    history: CommandHistory,
}

impl CommandPalette {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: "Command Palette".to_string(),
            state: CommandPaletteState::Closed,
            mode: CommandPaletteMode::Commands,
            visible: false,
            enabled: true,
            query: String::new(),
            selected_index: 0,
            selected_command: None,
            commands: CommandManager::new(),
            categories: CommandCategoryManager::new(),
            search: CommandSearch::new(),
            history: CommandHistory::new(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn state(&self) -> CommandPaletteState {
        self.state
    }

    pub fn mode(&self) -> CommandPaletteMode {
        self.mode
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    pub fn selected_command(&self) -> Option<&str> {
        self.selected_command.as_deref()
    }

    pub fn commands(&self) -> &CommandManager {
        &self.commands
    }

    pub fn commands_mut(&mut self) -> &mut CommandManager {
        &mut self.commands
    }

    pub fn categories(&self) -> &CommandCategoryManager {
        &self.categories
    }

    pub fn categories_mut(&mut self) -> &mut CommandCategoryManager {
        &mut self.categories
    }

    pub fn search(&self) -> &CommandSearch {
        &self.search
    }

    pub fn search_mut(&mut self) -> &mut CommandSearch {
        &mut self.search
    }

    pub fn history(&self) -> &CommandHistory {
        &self.history
    }

    pub fn history_mut(&mut self) -> &mut CommandHistory {
        &mut self.history
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;

        if !enabled {
            self.close();
        }
    }

    pub fn set_mode(&mut self, mode: CommandPaletteMode) {
        self.mode = mode;
        self.selected_index = 0;
        self.selected_command = None;
    }

    pub fn open(&mut self) -> bool {
        if !self.enabled {
            return false;
        }

        self.state = CommandPaletteState::Opening;
        self.visible = true;
        self.query.clear();
        self.search.clear_query();
        self.selected_index = 0;
        self.selected_command = None;
        self.state = CommandPaletteState::Open;

        true
    }

    pub fn close(&mut self) {
        self.state = CommandPaletteState::Closing;
        self.visible = false;
        self.query.clear();
        self.search.clear_query();
        self.selected_index = 0;
        self.selected_command = None;
        self.state = CommandPaletteState::Closed;
    }

    pub fn toggle(&mut self) {
        if self.visible {
            self.close();
        } else {
            self.open();
        }
    }

    pub fn set_query(&mut self, query: impl Into<String>) {
        let query = query.into();

        self.query = query.clone();
        self.search.set_query(query);

        if self.query.is_empty() {
            self.state = CommandPaletteState::Open;
        } else {
            self.state = CommandPaletteState::Searching;
        }

        self.selected_index = 0;
        self.selected_command = None;
    }

    pub fn append_query(&mut self, text: &str) {
        self.query.push_str(text);
        self.set_query(self.query.clone());
    }

    pub fn backspace_query(&mut self) {
        self.query.pop();
        self.set_query(self.query.clone());
    }

    pub fn results(&self) -> Vec<CommandSearchMatch> {
        self.search.search(self.commands.visible())
    }

    pub fn select_index(&mut self, index: usize) -> bool {
        let results = self.results();

        if index >= results.len() {
            return false;
        }

        self.selected_index = index;
        self.selected_command = Some(results[index].command_id().to_string());

        true
    }

    pub fn selected_entry(&self) -> Option<&CommandEntry> {
        self.selected_command
            .as_deref()
            .and_then(|id| self.commands.get(id))
    }

    pub fn select_next(&mut self) -> Option<&CommandEntry> {
        let results = self.results();

        if results.is_empty() {
            return None;
        }

        self.selected_index =
            (self.selected_index + 1) % results.len();

        self.selected_command =
            Some(results[self.selected_index].command_id().to_string());

        self.selected_entry()
    }

    pub fn select_previous(&mut self) -> Option<&CommandEntry> {
        let results = self.results();

        if results.is_empty() {
            return None;
        }

        if self.selected_index == 0 {
            self.selected_index = results.len() - 1;
        } else {
            self.selected_index -= 1;
        }

        self.selected_command =
            Some(results[self.selected_index].command_id().to_string());

        self.selected_entry()
    }

    pub fn execute_selected(&mut self) -> Option<String> {
        let command_id = self.selected_command.clone()?;

        let command = self.commands.get(&command_id)?;

        if !command.can_execute() {
            return None;
        }

        let label = command.label().to_string();

        self.state = CommandPaletteState::Executing;
        self.history.record(command_id.clone(), label);
        self.close();

        Some(command_id)
    }

    pub fn execute(&mut self, command_id: &str) -> bool {
        let command = match self.commands.get(command_id) {
            Some(command) if command.can_execute() => command,
            _ => return false,
        };

        self.history
            .record(command.id().to_string(), command.label().to_string());

        self.close();

        true
    }

    pub fn register_command(&mut self, command: CommandEntry) {
        self.commands.add(command);
    }

    pub fn clear_commands(&mut self) {
        self.commands.clear();
        self.selected_command = None;
    }

    pub fn reset(&mut self) {
        self.close();
        self.mode = CommandPaletteMode::Commands;
        self.history.clear();
        self.selected_index = 0;
        self.selected_command = None;
    }
}
