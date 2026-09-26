use std::path::{Path, PathBuf};

/// Type of completion candidate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompletionKind {
    Command,
    Argument,
    File,
    Directory,
    Variable,
    Option,
    Alias,
    Custom,
}

/// A single completion candidate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionCandidate {
    pub value: String,
    pub display: Option<String>,
    pub kind: CompletionKind,
    pub description: Option<String>,
}

impl CompletionCandidate {
    pub fn new(value: impl Into<String>, kind: CompletionKind) -> Self {
        Self {
            value: value.into(),
            display: None,
            kind,
            description: None,
        }
    }

    pub fn with_display(mut self, display: impl Into<String>) -> Self {
        self.display = Some(display.into());
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn display_value(&self) -> &str {
        self.display.as_deref().unwrap_or(&self.value)
    }
}

/// Completion request from the terminal UI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionRequest {
    pub input: String,
    pub cursor: usize,
    pub working_directory: PathBuf,
}

impl CompletionRequest {
    pub fn new(
        input: impl Into<String>,
        cursor: usize,
        working_directory: impl Into<PathBuf>,
    ) -> Self {
        Self {
            input: input.into(),
            cursor,
            working_directory: working_directory.into(),
        }
    }

    pub fn current_word(&self) -> &str {
        let cursor = self.cursor.min(self.input.len());
        let prefix = &self.input[..cursor];

        prefix
            .split_whitespace()
            .last()
            .unwrap_or("")
    }
}

/// Generic completion provider.
pub trait CompletionProvider: Send + Sync {
    fn complete(&self, request: &CompletionRequest) -> Vec<CompletionCandidate>;
}

/// Simple filesystem completion provider.
#[derive(Debug, Default, Clone, Copy)]
pub struct FileCompletionProvider;

impl FileCompletionProvider {
    pub fn new() -> Self {
        Self
    }

    fn directory_for(&self, input: &str, cwd: &Path) -> (PathBuf, String) {
        let path = Path::new(input);

        if path.is_absolute() {
            let directory = path.parent().unwrap_or(Path::new("/"));
            let prefix = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("");

            return (directory.to_path_buf(), prefix.to_string());
        }

        let directory = path.parent().map_or_else(
            || cwd.to_path_buf(),
            |parent| cwd.join(parent),
        );

        let prefix = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");

        (directory, prefix.to_string())
    }
}

impl CompletionProvider for FileCompletionProvider {
    fn complete(&self, request: &CompletionRequest) -> Vec<CompletionCandidate> {
        let word = request.current_word();

        if word.is_empty() {
            return Vec::new();
        }

        let (directory, prefix) = self.directory_for(word, &request.working_directory);

        let entries = match std::fs::read_dir(&directory) {
            Ok(entries) => entries,
            Err(_) => return Vec::new(),
        };

        let mut candidates = Vec::new();

        for entry in entries.flatten() {
            let name = entry.file_name();
            let name = match name.to_str() {
                Some(name) => name,
                None => continue,
            };

            if !name.starts_with(&prefix) {
                continue;
            }

            let kind = match entry.file_type() {
                Ok(file_type) if file_type.is_dir() => CompletionKind::Directory,
                _ => CompletionKind::File,
            };

            candidates.push(CompletionCandidate::new(name, kind));
        }

        candidates.sort_by(|a, b| a.value.cmp(&b.value));
        candidates
    }
}

/// Aggregates multiple completion providers.
#[derive(Default)]
pub struct CompletionManager {
    providers: Vec<Box<dyn CompletionProvider>>,
}

impl CompletionManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_provider<P>(&mut self, provider: P)
    where
        P: CompletionProvider + 'static,
    {
        self.providers.push(Box::new(provider));
    }

    pub fn complete(&self, request: &CompletionRequest) -> Vec<CompletionCandidate> {
        let mut candidates = Vec::new();

        for provider in &self.providers {
            candidates.extend(provider.complete(request));
        }

        candidates.sort_by(|a, b| a.value.cmp(&b.value));
        candidates.dedup_by(|a, b| a.value == b.value);

        candidates
    }

    pub fn clear(&mut self) {
        self.providers.clear();
    }

    pub fn provider_count(&self) -> usize {
        self.providers.len()
    }
}
