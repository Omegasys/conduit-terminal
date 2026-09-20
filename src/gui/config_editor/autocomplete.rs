use std::collections::BTreeMap;

/// Completion item categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionItemKind {
    Key,
    Section,
    Value,
    Boolean,
    Number,
    String,
    Command,
    Profile,
    Theme,
    Workspace,
    Plugin,
}

/// A single autocomplete suggestion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionItem {
    pub label: String,
    pub insert_text: String,
    pub description: Option<String>,
    pub kind: CompletionItemKind,
    pub priority: i32,
}

impl CompletionItem {
    pub fn new(
        label: impl Into<String>,
        insert_text: impl Into<String>,
        kind: CompletionItemKind,
    ) -> Self {
        Self {
            label: label.into(),
            insert_text: insert_text.into(),
            description: None,
            kind,
            priority: 0,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }
}

/// Context surrounding the cursor when autocomplete is requested.
#[derive(Debug, Clone)]
pub struct AutoCompleteContext {
    pub line: String,
    pub line_number: usize,
    pub cursor_column: usize,
    pub current_section: Option<String>,
    pub current_word: String,
}

impl AutoCompleteContext {
    pub fn new(
        line: impl Into<String>,
        line_number: usize,
        cursor_column: usize,
        current_section: Option<String>,
    ) -> Self {
        let line = line.into();

        let current_word = Self::extract_current_word(&line, cursor_column);

        Self {
            line,
            line_number,
            cursor_column,
            current_section,
            current_word,
        }
    }

    fn extract_current_word(line: &str, cursor: usize) -> String {
        let cursor = cursor.min(line.len());

        let prefix = &line[..cursor];

        prefix
            .rsplit(|character: char| {
                character.is_whitespace()
                    || character == '='
                    || character == '.'
                    || character == '['
                    || character == ']'
            })
            .next()
            .unwrap_or("")
            .trim_matches('"')
            .trim_matches('\'')
            .to_string()
    }
}

/// Autocomplete engine for the configuration editor.
#[derive(Debug, Clone, Default)]
pub struct AutoCompleteEngine {
    keys: BTreeMap<String, String>,
    values: BTreeMap<String, Vec<String>>,
    sections: Vec<String>,
    profiles: Vec<String>,
    themes: Vec<String>,
    workspaces: Vec<String>,
    plugins: Vec<String>,
}

impl AutoCompleteEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_key(
        &mut self,
        key: impl Into<String>,
        description: impl Into<String>,
    ) {
        self.keys.insert(key.into(), description.into());
    }

    pub fn add_value<S: Into<String>>(
        &mut self,
        key: impl Into<String>,
        value: S,
    ) {
        self.values
            .entry(key.into())
            .or_default()
            .push(value.into());
    }

    pub fn add_section<S: Into<String>>(&mut self, section: S) {
        let section = section.into();

        if !self.sections.contains(&section) {
            self.sections.push(section);
        }
    }

    pub fn add_profile<S: Into<String>>(&mut self, profile: S) {
        let profile = profile.into();

        if !self.profiles.contains(&profile) {
            self.profiles.push(profile);
        }
    }

    pub fn add_theme<S: Into<String>>(&mut self, theme: S) {
        let theme = theme.into();

        if !self.themes.contains(&theme) {
            self.themes.push(theme);
        }
    }

    pub fn add_workspace<S: Into<String>>(&mut self, workspace: S) {
        let workspace = workspace.into();

        if !self.workspaces.contains(&workspace) {
            self.workspaces.push(workspace);
        }
    }

    pub fn add_plugin<S: Into<String>>(&mut self, plugin: S) {
        let plugin = plugin.into();

        if !self.plugins.contains(&plugin) {
            self.plugins.push(plugin);
        }
    }

    pub fn complete(&self, context: &AutoCompleteContext) -> Vec<CompletionItem> {
        let mut results = Vec::new();

        let prefix = context.current_word.to_lowercase();
        let is_value_context = context.line.contains('=');

        if is_value_context {
            if let Some(values) = self.values_for_line(context) {
                for value in values {
                    if value.to_lowercase().starts_with(&prefix) {
                        results.push(
                            CompletionItem::new(
                                value.clone(),
                                value.clone(),
                                CompletionItemKind::Value,
                            )
                            .with_priority(100),
                        );
                    }
                }
            }

            for value in ["true", "false"] {
                if value.starts_with(&prefix) {
                    results.push(
                        CompletionItem::new(
                            value,
                            value,
                            CompletionItemKind::Boolean,
                        )
                        .with_priority(90),
                    );
                }
            }
        } else {
            for (key, description) in &self.keys {
                if key.to_lowercase().starts_with(&prefix) {
                    results.push(
                        CompletionItem::new(
                            key,
                            key,
                            CompletionItemKind::Key,
                        )
                        .with_description(description)
                        .with_priority(100),
                    );
                }
            }

            for section in &self.sections {
                if section.to_lowercase().starts_with(&prefix) {
                    results.push(
                        CompletionItem::new(
                            section,
                            format!("[{section}]"),
                            CompletionItemKind::Section,
                        )
                        .with_priority(80),
                    );
                }
            }
        }

        self.add_named_results(
            &mut results,
            &self.profiles,
            &prefix,
            CompletionItemKind::Profile,
            60,
        );

        self.add_named_results(
            &mut results,
            &self.themes,
            &prefix,
            CompletionItemKind::Theme,
            60,
        );

        self.add_named_results(
            &mut results,
            &self.workspaces,
            &prefix,
            CompletionItemKind::Workspace,
            50,
        );

        self.add_named_results(
            &mut results,
            &self.plugins,
            &prefix,
            CompletionItemKind::Plugin,
            40,
        );

        results.sort_by(|a, b| {
            b.priority
                .cmp(&a.priority)
                .then_with(|| a.label.cmp(&b.label))
        });

        results
    }

    fn values_for_line(
        &self,
        context: &AutoCompleteContext,
    ) -> Option<&Vec<String>> {
        let key = context
            .line
            .split_once('=')
            .map(|(key, _)| key.trim())?;

        self.values.get(key)
    }

    fn add_named_results(
        &self,
        results: &mut Vec<CompletionItem>,
        values: &[String],
        prefix: &str,
        kind: CompletionItemKind,
        priority: i32,
    ) {
        for value in values {
            if value.to_lowercase().starts_with(prefix) {
                results.push(
                    CompletionItem::new(value, value, kind)
                        .with_priority(priority),
                );
            }
        }
    }

    pub fn clear(&mut self) {
        self.keys.clear();
        self.values.clear();
        self.sections.clear();
        self.profiles.clear();
        self.themes.clear();
        self.workspaces.clear();
        self.plugins.clear();
    }
}
