use std::path::{Path, PathBuf};

/// Supported native shell history formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HistoryFormat {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    PlainText,
    Unknown,
}

/// A shell-owned history entry as observed by Conduit.
///
/// Conduit does not become the authoritative owner of shell history.
/// This type provides a common representation for integration and search.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellHistoryEntry {
    pub command: String,
    pub timestamp: Option<i64>,
    pub working_directory: Option<PathBuf>,
}

impl ShellHistoryEntry {
    pub fn new(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            timestamp: None,
            working_directory: None,
        }
    }

    pub fn with_timestamp(mut self, timestamp: i64) -> Self {
        self.timestamp = Some(timestamp);
        self
    }

    pub fn with_working_directory(mut self, path: impl Into<PathBuf>) -> Self {
        self.working_directory = Some(path.into());
        self
    }
}

/// Locates native history files without taking ownership of them.
#[derive(Debug, Clone)]
pub struct HistoryLocator {
    home: Option<PathBuf>,
}

impl HistoryLocator {
    pub fn new(home: Option<PathBuf>) -> Self {
        Self { home }
    }

    pub fn from_environment() -> Self {
        Self {
            home: std::env::var_os("HOME").map(PathBuf::from),
        }
    }

    pub fn home(&self) -> Option<&Path> {
        self.home.as_deref()
    }

    pub fn bash(&self) -> Option<PathBuf> {
        self.home.as_ref().map(|p| p.join(".bash_history"))
    }

    pub fn zsh(&self) -> Option<PathBuf> {
        self.home.as_ref().map(|p| p.join(".zsh_history"))
    }

    pub fn fish(&self) -> Option<PathBuf> {
        self.home
            .as_ref()
            .map(|p| p.join(".local/share/fish/fish_history"))
    }

    pub fn powershell(&self) -> Option<PathBuf> {
        self.home.as_ref().map(|p| {
            p.join(".local/share/powershell/PSReadLine/ConsoleHost_history.txt")
        })
    }

    pub fn locate(&self, format: HistoryFormat) -> Option<PathBuf> {
        match format {
            HistoryFormat::Bash => self.bash(),
            HistoryFormat::Zsh => self.zsh(),
            HistoryFormat::Fish => self.fish(),
            HistoryFormat::PowerShell => self.powershell(),
            HistoryFormat::PlainText | HistoryFormat::Unknown => None,
        }
    }
}

/// Common history adapter interface.
pub trait ShellHistoryAdapter: Send + Sync {
    fn format(&self) -> HistoryFormat;

    fn parse(&self, contents: &str) -> Vec<ShellHistoryEntry>;

    fn serialize(&self, entries: &[ShellHistoryEntry]) -> String;
}

/// Basic line-oriented history implementation.
///
/// Shell-specific adapters should override this behavior when timestamps,
/// multiline commands, or shell metadata require special handling.
#[derive(Debug, Default, Clone, Copy)]
pub struct LineHistoryAdapter;

impl ShellHistoryAdapter for LineHistoryAdapter {
    fn format(&self) -> HistoryFormat {
        HistoryFormat::PlainText
    }

    fn parse(&self, contents: &str) -> Vec<ShellHistoryEntry> {
        contents
            .lines()
            .filter_map(|line| {
                let command = line.trim();

                if command.is_empty() {
                    None
                } else {
                    Some(ShellHistoryEntry::new(command))
                }
            })
            .collect()
    }

    fn serialize(&self, entries: &[ShellHistoryEntry]) -> String {
        entries
            .iter()
            .map(|entry| entry.command.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    }
}
