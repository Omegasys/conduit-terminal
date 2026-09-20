use crate::config_engine::{
    ConfigDiff,
    ConfigError,
    ConfigSerializer,
    ConfigTransaction,
    ConfigValue,
};

/// Current state of the configuration editor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigEditorState {
    Clean,
    Modified,
    Parsing,
    Validating,
    Valid,
    Invalid,
    Saving,
    Error,
}

/// Zero-based editor cursor position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct EditorCursor {
    pub line: usize,
    pub column: usize,
}

impl EditorCursor {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    pub fn move_to(&mut self, line: usize, column: usize) {
        self.line = line;
        self.column = column;
    }
}

/// Text selection between two cursor positions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EditorSelection {
    pub start: EditorCursor,
    pub end: EditorCursor,
}

impl EditorSelection {
    pub fn new(start: EditorCursor, end: EditorCursor) -> Self {
        Self { start, end }
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

/// Main GUI configuration editor.
///
/// The editor owns editable text and a configuration transaction.
/// Actual persistence remains the responsibility of the config engine.
#[derive(Debug)]
pub struct ConfigEditor {
    text: String,
    original_text: String,
    cursor: EditorCursor,
    selection: Option<EditorSelection>,
    state: ConfigEditorState,
    transaction: Option<ConfigTransaction>,
    last_error: Option<String>,
    revision: u64,
}

impl Default for ConfigEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigEditor {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            original_text: String::new(),
            cursor: EditorCursor::default(),
            selection: None,
            state: ConfigEditorState::Clean,
            transaction: None,
            last_error: None,
            revision: 0,
        }
    }

    pub fn from_text(text: impl Into<String>) -> Self {
        let text = text.into();

        Self {
            original_text: text.clone(),
            text,
            ..Self::new()
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn original_text(&self) -> &str {
        &self.original_text
    }

    pub fn cursor(&self) -> EditorCursor {
        self.cursor
    }

    pub fn selection(&self) -> Option<EditorSelection> {
        self.selection
    }

    pub fn state(&self) -> ConfigEditorState {
        self.state
    }

    pub fn last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }

    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn is_modified(&self) -> bool {
        self.text != self.original_text
    }

    pub fn set_cursor(&mut self, cursor: EditorCursor) {
        self.cursor = cursor;
    }

    pub fn set_selection(&mut self, selection: Option<EditorSelection>) {
        self.selection = selection;
    }

    pub fn clear_selection(&mut self) {
        self.selection = None;
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
        self.mark_modified();
    }

    pub fn replace_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
        self.original_text = self.text.clone();
        self.selection = None;
        self.state = ConfigEditorState::Clean;
        self.last_error = None;
        self.revision = self.revision.wrapping_add(1);
    }

    pub fn insert_text(&mut self, text: &str) {
        let offset = self.cursor_to_offset(self.cursor);

        self.text.insert_str(offset, text);

        let inserted_lines = text.matches('\n').count();

        if inserted_lines == 0 {
            self.cursor.column += text.chars().count();
        } else {
            let last_line_length = text
                .rsplit('\n')
                .next()
                .map(|line| line.chars().count())
                .unwrap_or(0);

            self.cursor.line += inserted_lines;
            self.cursor.column = last_line_length;
        }

        self.mark_modified();
    }

    pub fn delete_selection(&mut self) -> bool {
        let selection = match self.selection.take() {
            Some(selection) if !selection.is_empty() => selection,
            _ => return false,
        };

        let start = self.cursor_to_offset(selection.start);
        let end = self.cursor_to_offset(selection.end);

        if start >= end || end > self.text.len() {
            return false;
        }

        self.text.replace_range(start..end, "");
        self.cursor = selection.start;

        self.mark_modified();
        true
    }

    pub fn undo_transaction(&mut self) {
        if let Some(transaction) = &mut self.transaction {
            transaction.rollback();
            self.state = ConfigEditorState::Clean;
        }
    }

    pub fn begin_transaction(&mut self) -> Result<(), ConfigError> {
        let values = ConfigSerializer::from_toml(&self.text)?;
        self.transaction = Some(ConfigTransaction::new(values));
        self.state = ConfigEditorState::Clean;
        self.last_error = None;

        Ok(())
    }

    pub fn transaction(&self) -> Option<&ConfigTransaction> {
        self.transaction.as_ref()
    }

    pub fn transaction_mut(&mut self) -> Option<&mut ConfigTransaction> {
        self.transaction.as_mut()
    }

    pub fn diff(&self) -> Result<ConfigDiff, ConfigError> {
        let values = ConfigSerializer::from_toml(&self.text)?;

        if let Some(transaction) = &self.transaction {
            Ok(transaction.diff())
        } else {
            let transaction = ConfigTransaction::new(values);
            Ok(transaction.diff())
        }
    }

    pub fn parse(&mut self) -> Result<ConfigValueMap, ConfigError> {
        self.state = ConfigEditorState::Parsing;
        self.last_error = None;

        match ConfigSerializer::from_toml(&self.text) {
            Ok(values) => {
                self.state = ConfigEditorState::Valid;
                Ok(ConfigValueMap(values))
            }
            Err(error) => {
                self.state = ConfigEditorState::Invalid;
                self.last_error = Some(error.to_string());
                Err(error)
            }
        }
    }

    pub fn mark_valid(&mut self) {
        self.state = ConfigEditorState::Valid;
        self.last_error = None;
    }

    pub fn mark_invalid(&mut self, error: impl Into<String>) {
        self.state = ConfigEditorState::Invalid;
        self.last_error = Some(error.into());
    }

    pub fn mark_saving(&mut self) {
        self.state = ConfigEditorState::Saving;
    }

    pub fn mark_saved(&mut self) {
        self.original_text = self.text.clone();
        self.state = ConfigEditorState::Clean;
        self.last_error = None;
    }

    pub fn mark_error(&mut self, error: impl Into<String>) {
        self.state = ConfigEditorState::Error;
        self.last_error = Some(error.into());
    }

    pub fn reset(&mut self) {
        self.text = self.original_text.clone();
        self.cursor = EditorCursor::default();
        self.selection = None;
        self.state = ConfigEditorState::Clean;
        self.last_error = None;
        self.transaction = None;
        self.revision = self.revision.wrapping_add(1);
    }

    fn mark_modified(&mut self) {
        self.state = ConfigEditorState::Modified;
        self.last_error = None;
        self.revision = self.revision.wrapping_add(1);
    }

    fn cursor_to_offset(&self, cursor: EditorCursor) -> usize {
        let mut current_line = 0usize;
        let mut offset = 0usize;

        for line in self.text.split('\n') {
            if current_line == cursor.line {
                let byte_offset = line
                    .char_indices()
                    .nth(cursor.column)
                    .map(|(index, _)| index)
                    .unwrap_or(line.len());

                return offset + byte_offset;
            }

            offset += line.len() + 1;
            current_line += 1;
        }

        self.text.len()
    }
}

/// Small wrapper used when exposing parsed configuration values.
#[derive(Debug, Clone)]
pub struct ConfigValueMap(pub std::collections::BTreeMap<String, ConfigValue>);

impl ConfigValueMap {
    pub fn values(&self) -> &std::collections::BTreeMap<String, ConfigValue> {
        &self.0
    }

    pub fn into_values(self) -> std::collections::BTreeMap<String, ConfigValue> {
        self.0
    }
}
