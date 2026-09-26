use super::terminal::{
    SearchDocument,
    SearchMatch,
    SearchOptions,
    SearchScope,
    TerminalSearcher,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct SessionSearcher {
    terminal: TerminalSearcher,
}

impl SessionSearcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn search(
        &self,
        sessions: &[SearchDocument],
        query: &str,
        options: &SearchOptions,
    ) -> Vec<SearchMatch> {
        let documents: Vec<SearchDocument> = sessions
            .iter()
            .filter(|document| document.scope == SearchScope::Session)
            .cloned()
            .collect();

        self.terminal.search(&documents, query, options)
    }
}
