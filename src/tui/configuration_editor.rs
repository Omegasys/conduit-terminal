use std::collections::BTreeMap;

use crate::config_engine::ConfigValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigEditorMode {
    Normal,
    Editing,
    Searching,
    Preview,
    Applying,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigEditorLineKind {
    Section,
    Key,
    Comment,
    Blank,
    Diagnostic,
}

#[derive(Debug, Clone)]
pub struct ConfigEditorLine {
    key: Option<String>,
    text: String,
    kind: ConfigEditorLineKind,
    modified: bool,
}

impl ConfigEditorLine {
    pub fn new(
        text: impl Into<String>,
        kind: ConfigEditorLineKind,
    ) -> Self {
        Self {
            key: None,
            text: text.into(),
            kind,
            modified: false,
        }
    }

    pub fn with_key(
        key: impl Into<String>,
        text: impl Into<String>,
        kind: ConfigEditorLineKind,
    ) -> Self {
        Self {
            key: Some(key.into()),
            text: text.into(),
            kind,
            modified: false,
        }
    }

    pub fn key(&self) -> Option<&str> {
        self.key.as_deref()
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
        self.modified = true;
    }

    pub fn kind(&self) -> ConfigEditorLineKind {
        self.kind
    }

    pub fn modified(&self) -> bool {
        self.modified
    }

    pub fn mark_modified(&mut self) {
        self.modified = true;
    }

    pub fn mark_clean(&mut self) {
        self.modified = false;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EditorCursor {
    pub line: usize,
    pub column: usize,
}

impl EditorCursor {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EditorSelection {
    pub start: EditorCursor,
    pub end: EditorCursor,
}

pub struct TuiConfigurationEditor {
    values: BTreeMap<String, ConfigValue>,
    original: BTreeMap<String, ConfigValue>,
    lines: Vec<ConfigEditorLine>,

    cursor: EditorCursor,
    selection: Option<EditorSelection>,

    mode: ConfigEditorMode,
    modified: bool,
    diagnostics: Vec<String>,
    search: String,

    read_only: bool,
}

impl Default for TuiConfigurationEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl TuiConfigurationEditor {
    pub fn new() -> Self {
        Self {
            values: BTreeMap::new(),
            original: BTreeMap::new(),
            lines: Vec::new(),
            cursor: EditorCursor::new(0, 0),
            selection: None,
            mode: ConfigEditorMode::Normal,
            modified: false,
            diagnostics: Vec::new(),
            search: String::new(),
            read_only: false,
        }
    }

    pub fn load(&mut self, values: BTreeMap<String, ConfigValue>) {
        self.values = values.clone();
        self.original = values;
        self.modified = false;
        self.mode = ConfigEditorMode::Normal;
        self.rebuild_lines();
    }

    pub fn values(&self) -> &BTreeMap<String, ConfigValue> {
        &self.values
    }

    pub fn value(&self, key: &str) -> Option<&ConfigValue> {
        self.values.get(key)
    }

    pub fn set_value(&mut self, key: impl Into<String>, value: ConfigValue) -> bool {
        if self.read_only {
            return false;
        }

        self.values.insert(key.into(), value);
        self.modified = true;
        self.mode = ConfigEditorMode::Editing;
        self.rebuild_lines();

        true
    }

    pub fn remove_value(&mut self, key: &str) -> Option<ConfigValue> {
        if self.read_only {
            return None;
        }

        let removed = self.values.remove(key);

        if removed.is_some() {
            self.modified = true;
            self.mode = ConfigEditorMode::Editing;
            self.rebuild_lines();
        }

        removed
    }

    pub fn lines(&self) -> &[ConfigEditorLine] {
        &self.lines
    }

    pub fn cursor(&self) -> EditorCursor {
        self.cursor
    }

    pub fn set_cursor(&mut self, cursor: EditorCursor) {
        let line = cursor.line.min(self.lines.len().saturating_sub(1));

        let column = self
            .lines
            .get(line)
            .map(|line| cursor.column.min(line.text().len()))
            .unwrap_or(0);

        self.cursor = EditorCursor::new(line, column);
    }

    pub fn move_cursor_up(&mut self) {
        if self.cursor.line > 0 {
            self.cursor.line -= 1;
            self.clamp_column();
        }
    }

    pub fn move_cursor_down(&mut self) {
        if self.cursor.line + 1 < self.lines.len() {
            self.cursor.line += 1;
            self.clamp_column();
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor.column > 0 {
            self.cursor.column -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        let max = self
            .lines
            .get(self.cursor.line)
            .map(|line| line.text().len())
            .unwrap_or(0);

        self.cursor.column = (self.cursor.column + 1).min(max);
    }

    pub fn selection(&self) -> Option<EditorSelection> {
        self.selection
    }

    pub fn set_selection(&mut self, selection: Option<EditorSelection>) {
        self.selection = selection;
    }

    pub fn mode(&self) -> ConfigEditorMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: ConfigEditorMode) {
        self.mode = mode;
    }

    pub fn modified(&self) -> bool {
        self.modified
    }

    pub fn mark_clean(&mut self) {
        self.original = self.values.clone();
        self.modified = false;

        for line in &mut self.lines {
            line.mark_clean();
        }

        self.mode = ConfigEditorMode::Normal;
    }

    pub fn revert(&mut self) {
        self.values = self.original.clone();
        self.modified = false;
        self.mode = ConfigEditorMode::Normal;
        self.rebuild_lines();
    }

    pub fn read_only(&self) -> bool {
        self.read_only
    }

    pub fn set_read_only(&mut self, read_only: bool) {
        self.read_only = read_only;
    }

    pub fn diagnostics(&self) -> &[String] {
        &self.diagnostics
    }

    pub fn add_diagnostic(&mut self, message: impl Into<String>) {
        self.diagnostics.push(message.into());
        self.mode = ConfigEditorMode::Error;
    }

    pub fn clear_diagnostics(&mut self) {
        self.diagnostics.clear();

        if self.mode == ConfigEditorMode::Error {
            self.mode = if self.modified {
                ConfigEditorMode::Editing
            } else {
                ConfigEditorMode::Normal
            };
        }
    }

    pub fn search(&self) -> &str {
        &self.search
    }

    pub fn set_search(&mut self, search: impl Into<String>) {
        self.search = search.into();
        self.mode = ConfigEditorMode::Searching;
    }

    pub fn clear_search(&mut self) {
        self.search.clear();
        self.mode = ConfigEditorMode::Normal;
    }

    pub fn search_lines(&self) -> Vec<usize> {
        let query = self.search.trim().to_lowercase();

        if query.is_empty() {
            return Vec::new();
        }

        self.lines
            .iter()
            .enumerate()
            .filter_map(|(index, line)| {
                if line.text().to_lowercase().contains(&query) {
                    Some(index)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn begin_preview(&mut self) {
        self.mode = ConfigEditorMode::Preview;
    }

    pub fn begin_apply(&mut self) -> bool {
        if self.read_only || !self.modified {
            return false;
        }

        self.mode = ConfigEditorMode::Applying;
        true
    }

    pub fn cancel_apply(&mut self) {
        self.mode = if self.modified {
            ConfigEditorMode::Editing
        } else {
            ConfigEditorMode::Normal
        };
    }

    pub fn apply_succeeded(&mut self) {
        self.mark_clean();
    }

    fn rebuild_lines(&mut self) {
        self.lines.clear();

        for (key, value) in &self.values {
            let text = format!("{key} = {}", format_config_value(value));

            self.lines.push(ConfigEditorLine::with_key(
                key.clone(),
                text,
                ConfigEditorLineKind::Key,
            ));
        }

        if self.lines.is_empty() {
            self.lines.push(ConfigEditorLine::new(
                "",
                ConfigEditorLineKind::Blank,
            ));
        }

        self.set_cursor(self.cursor);
    }

    fn clamp_column(&mut self) {
        let max = self
            .lines
            .get(self.cursor.line)
            .map(|line| line.text().len())
            .unwrap_or(0);

        self.cursor.column = self.cursor.column.min(max);
    }

    pub fn clear(&mut self) {
        self.values.clear();
        self.original.clear();
        self.lines.clear();
        self.cursor = EditorCursor::new(0, 0);
        self.selection = None;
        self.modified = false;
        self.diagnostics.clear();
        self.search.clear();
        self.mode = ConfigEditorMode::Normal;
    }
}

fn format_config_value(value: &ConfigValue) -> String {
    match value {
        ConfigValue::String(value) => format!("{value:?}"),
        ConfigValue::Integer(value) => value.to_string(),
        ConfigValue::Float(value) => value.to_string(),
        ConfigValue::Boolean(value) => value.to_string(),
        ConfigValue::Null => "null".to_owned(),
        ConfigValue::Array(values) => {
            let values = values
                .iter()
                .map(format_config_value)
                .collect::<Vec<_>>();

            format!("[{}]", values.join(", "))
        }
        ConfigValue::Table(values) => {
            let values = values
                .iter()
                .map(|(key, value)| {
                    format!("{key} = {}", format_config_value(value))
                })
                .collect::<Vec<_>>();

            format!("{{ {} }}", values.join(", "))
        }
    }
}
