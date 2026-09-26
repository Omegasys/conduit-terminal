pub mod case;
pub mod history;
pub mod panes;
pub mod recordings;
pub mod regex;
pub mod scrollback;
pub mod sessions;
pub mod tabs;
pub mod terminal;
pub mod whole_word;

pub use case::CaseSensitivity;
pub use history::HistorySearcher;
pub use panes::PaneSearcher;
pub use recordings::RecordingSearcher;
pub use regex::RegexSearcher;
pub use scrollback::ScrollbackSearcher;
pub use sessions::SessionSearcher;
pub use tabs::TabSearcher;
pub use terminal::{
    SearchDocument,
    SearchMatch,
    SearchOptions,
    SearchQuery,
    SearchScope,
    TerminalSearcher,
};
pub use whole_word::WholeWordMatcher;

/// A shared search service used by GUI, TUI, CLI, and command palette code.
#[derive(Debug, Default)]
pub struct SearchEngine {
    terminal: TerminalSearcher,
    regex: RegexSearcher,
    scrollback: ScrollbackSearcher,
    panes: PaneSearcher,
    tabs: TabSearcher,
    sessions: SessionSearcher,
    recordings: RecordingSearcher,
    history: HistorySearcher,
}

impl SearchEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn search(
        &self,
        documents: &[SearchDocument],
        query: &SearchQuery,
        options: &SearchOptions,
    ) -> Vec<SearchMatch> {
        match query {
            SearchQuery::Literal(value) => {
                self.terminal.search(documents, value, options)
            }
            SearchQuery::Regex(value) => {
                self.regex.search(documents, value, options)
            }
        }
    }

    pub fn terminal(&self) -> &TerminalSearcher {
        &self.terminal
    }

    pub fn regex(&self) -> &RegexSearcher {
        &self.regex
    }

    pub fn scrollback(&self) -> &ScrollbackSearcher {
        &self.scrollback
    }

    pub fn panes(&self) -> &PaneSearcher {
        &self.panes
    }

    pub fn tabs(&self) -> &TabSearcher {
        &self.tabs
    }

    pub fn sessions(&self) -> &SessionSearcher {
        &self.sessions
    }

    pub fn recordings(&self) -> &RecordingSearcher {
        &self.recordings
    }

    pub fn history(&self) -> &HistorySearcher {
        &self.history
    }
}
