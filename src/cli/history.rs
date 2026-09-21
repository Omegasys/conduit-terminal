use std::fmt;

use crate::tui::history::{
    HistoryEntry,
    HistoryEntryKind,
};

/// Actions supported by the `conduit history` command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HistoryCommandAction {
    List,
    Show,
    Search,
    Clear,
    ClearCommands,
    ClearSessions,
    ClearWorkspaces,
    Pin,
    Unpin,
    Remove,
    Export,
}

impl HistoryCommandAction {
    pub fn name(&self) -> &'static str {
        match self {
            Self::List => "list",
            Self::Show => "show",
            Self::Search => "search",
            Self::Clear => "clear",
            Self::ClearCommands => "clear-commands",
            Self::ClearSessions => "clear-sessions",
            Self::ClearWorkspaces => "clear-workspaces",
            Self::Pin => "pin",
            Self::Unpin => "unpin",
            Self::Remove => "remove",
            Self::Export => "export",
        }
    }

    pub fn from_name(value: &str) -> Option<Self> {
        match value {
            "list" => Some(Self::List),
            "show" => Some(Self::Show),
            "search" => Some(Self::Search),
            "clear" => Some(Self::Clear),
            "clear-commands" => Some(Self::ClearCommands),
            "clear-sessions" => Some(Self::ClearSessions),
            "clear-workspaces" => Some(Self::ClearWorkspaces),
            "pin" => Some(Self::Pin),
            "unpin" => Some(Self::Unpin),
            "remove" => Some(Self::Remove),
            "export" => Some(Self::Export),
            _ => None,
        }
    }
}

impl fmt::Display for HistoryCommandAction {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

/// A parsed history-management command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryCommand {
    action: HistoryCommandAction,
    entry_id: Option<u64>,
    search: Option<String>,
    kind: Option<HistoryEntryKind>,
    pinned_only: bool,
}

impl HistoryCommand {
    pub fn new(action: HistoryCommandAction) -> Self {
        Self {
            action,
            entry_id: None,
            search: None,
            kind: None,
            pinned_only: false,
        }
    }

    pub fn action(&self) -> &HistoryCommandAction {
        &self.action
    }

    pub fn entry_id(&self) -> Option<u64> {
        self.entry_id
    }

    pub fn search(&self) -> Option<&str> {
        self.search.as_deref()
    }

    pub fn kind(&self) -> Option<&HistoryEntryKind> {
        self.kind.as_ref()
    }

    pub fn pinned_only(&self) -> bool {
        self.pinned_only
    }

    pub fn set_entry_id(&mut self, entry_id: u64) {
        self.entry_id = Some(entry_id);
    }

    pub fn with_entry_id(mut self, entry_id: u64) -> Self {
        self.set_entry_id(entry_id);
        self
    }

    pub fn set_search<S>(&mut self, search: S)
    where
        S: Into<String>,
    {
        self.search = Some(search.into());
    }

    pub fn with_search<S>(mut self, search: S) -> Self
    where
        S: Into<String>,
    {
        self.set_search(search);
        self
    }

    pub fn set_kind(&mut self, kind: HistoryEntryKind) {
        self.kind = Some(kind);
    }

    pub fn with_kind(mut self, kind: HistoryEntryKind) -> Self {
        self.set_kind(kind);
        self
    }

    pub fn set_pinned_only(&mut self, pinned_only: bool) {
        self.pinned_only = pinned_only;
    }

    pub fn with_pinned_only(mut self, pinned_only: bool) -> Self {
        self.set_pinned_only(pinned_only);
        self
    }

    pub fn requires_entry_id(&self) -> bool {
        matches!(
            self.action,
            HistoryCommandAction::Show
                | HistoryCommandAction::Pin
                | HistoryCommandAction::Unpin
                | HistoryCommandAction::Remove
        )
    }

    pub fn requires_search(&self) -> bool {
        matches!(self.action, HistoryCommandAction::Search)
    }

    pub fn applies_to(&self, entry: &HistoryEntry) -> bool {
        if let Some(entry_id) = self.entry_id {
            if entry.id() != entry_id {
                return false;
            }
        }

        if let Some(kind) = self.kind() {
            if entry.kind() != kind {
                return false;
            }
        }

        if self.pinned_only && !entry.pinned() {
            return false;
        }

        if let Some(search) = self.search() {
            let search = search.to_lowercase();
            let text = entry.text().to_lowercase();

            if !text.contains(&search) {
                return false;
            }
        }

        true
    }
}
