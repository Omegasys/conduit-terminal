#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TuiCommandCategory {
    File,
    Edit,
    View,
    Terminal,
    Tabs,
    Panes,
    Sessions,
    Workspaces,
    Plugins,
    Security,
    Configuration,
    Tools,
    Help,
    Custom,
}

impl TuiCommandCategory {
    pub fn name(&self) -> &'static str {
        match self {
            Self::File => "File",
            Self::Edit => "Edit",
            Self::View => "View",
            Self::Terminal => "Terminal",
            Self::Tabs => "Tabs",
            Self::Panes => "Panes",
            Self::Sessions => "Sessions",
            Self::Workspaces => "Workspaces",
            Self::Plugins => "Plugins",
            Self::Security => "Security",
            Self::Configuration => "Configuration",
            Self::Tools => "Tools",
            Self::Help => "Help",
            Self::Custom => "Custom",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TuiCommandState {
    Enabled,
    Disabled,
    Hidden,
}

pub struct TuiCommand {
    id: String,
    name: String,
    description: String,
    category: TuiCommandCategory,
    shortcut: Option<String>,
    state: TuiCommandState,
}

impl TuiCommand {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        category: TuiCommandCategory,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: String::new(),
            category,
            shortcut: None,
            state: TuiCommandState::Enabled,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn set_description(&mut self, description: impl Into<String>) {
        self.description = description.into();
    }

    pub fn category(&self) -> TuiCommandCategory {
        self.category
    }

    pub fn shortcut(&self) -> Option<&str> {
        self.shortcut.as_deref()
    }

    pub fn set_shortcut(&mut self, shortcut: Option<String>) {
        self.shortcut = shortcut;
    }

    pub fn state(&self) -> TuiCommandState {
        self.state
    }

    pub fn set_state(&mut self, state: TuiCommandState) {
        self.state = state;
    }

    pub fn enabled(&self) -> bool {
        self.state == TuiCommandState::Enabled
    }
}

#[derive(Debug, Clone)]
pub struct TuiCommandMatch {
    pub command_id: String,
    pub score: usize,
    pub matched_name: bool,
    pub matched_description: bool,
}

pub struct TuiCommandPalette {
    commands: Vec<TuiCommand>,
    query: String,
    results: Vec<TuiCommandMatch>,
    selected_index: usize,
    visible: bool,
    max_results: usize,
}

impl Default for TuiCommandPalette {
    fn default() -> Self {
        Self::new()
    }
}

impl TuiCommandPalette {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            query: String::new(),
            results: Vec::new(),
            selected_index: 0,
            visible: false,
            max_results: 50,
        }
    }

    pub fn register(&mut self, command: TuiCommand) {
        if let Some(existing) = self
            .commands
            .iter_mut()
            .find(|existing| existing.id() == command.id())
        {
            *existing = command;
        } else {
            self.commands.push(command);
        }

        self.search();
    }

    pub fn unregister(&mut self, id: &str) -> Option<TuiCommand> {
        let index = self.commands.iter().position(|command| command.id() == id)?;
        let removed = self.commands.remove(index);

        self.search();
        Some(removed)
    }

    pub fn command(&self, id: &str) -> Option<&TuiCommand> {
        self.commands.iter().find(|command| command.id() == id)
    }

    pub fn commands(&self) -> &[TuiCommand] {
        &self.commands
    }

    pub fn open(&mut self) {
        self.visible = true;
        self.selected_index = 0;
        self.search();
    }

    pub fn close(&mut self) {
        self.visible = false;
    }

    pub fn toggle(&mut self) {
        if self.visible {
            self.close();
        } else {
            self.open();
        }
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn set_query(&mut self, query: impl Into<String>) {
        self.query = query.into();
        self.selected_index = 0;
        self.search();
    }

    pub fn clear_query(&mut self) {
        self.query.clear();
        self.selected_index = 0;
        self.search();
    }

    pub fn results(&self) -> &[TuiCommandMatch] {
        &self.results
    }

    pub fn selected(&self) -> Option<&TuiCommandMatch> {
        self.results.get(self.selected_index)
    }

    pub fn move_down(&mut self) {
        if self.results.is_empty() {
            return;
        }

        self.selected_index =
            (self.selected_index + 1) % self.results.len();
    }

    pub fn move_up(&mut self) {
        if self.results.is_empty() {
            return;
        }

        if self.selected_index == 0 {
            self.selected_index = self.results.len() - 1;
        } else {
            self.selected_index -= 1;
        }
    }

    pub fn select_index(&mut self, index: usize) -> bool {
        if index >= self.results.len() {
            return false;
        }

        self.selected_index = index;
        true
    }

    pub fn execute_selected(&mut self) -> Option<String> {
        let result = self.selected()?.command_id.clone();

        if self
            .command(&result)
            .map(|command| command.enabled())
            .unwrap_or(false)
        {
            self.close();
            Some(result)
        } else {
            None
        }
    }

    pub fn search(&mut self) {
        let query = self.query.trim().to_lowercase();

        self.results.clear();

        for command in &self.commands {
            if !command.enabled() {
                continue;
            }

            let name = command.name().to_lowercase();
            let description = command.description().to_lowercase();
            let id = command.id().to_lowercase();

            if query.is_empty() {
                self.results.push(TuiCommandMatch {
                    command_id: command.id().to_owned(),
                    score: 0,
                    matched_name: true,
                    matched_description: false,
                });

                continue;
            }

            let name_match = name.contains(&query);
            let description_match = description.contains(&query);
            let id_match = id.contains(&query);

            if !(name_match || description_match || id_match) {
                continue;
            }

            let mut score = 100;

            if name_match {
                score = score.saturating_sub(30);
            }

            if id_match {
                score = score.saturating_sub(20);
            }

            if description_match {
                score = score.saturating_sub(10);
            }

            self.results.push(TuiCommandMatch {
                command_id: command.id().to_owned(),
                score,
                matched_name: name_match,
                matched_description: description_match,
            });
        }

        self.results.sort_by(|a, b| {
            a.score
                .cmp(&b.score)
                .then_with(|| a.command_id.cmp(&b.command_id))
        });

        self.results.truncate(self.max_results);

        if self.results.is_empty() {
            self.selected_index = 0;
        } else {
            self.selected_index =
                self.selected_index.min(self.results.len() - 1);
        }
    }

    pub fn set_max_results(&mut self, max_results: usize) {
        self.max_results = max_results.max(1);
        self.search();
    }

    pub fn max_results(&self) -> usize {
        self.max_results
    }

    pub fn clear(&mut self) {
        self.commands.clear();
        self.results.clear();
        self.query.clear();
        self.selected_index = 0;
    }
}
