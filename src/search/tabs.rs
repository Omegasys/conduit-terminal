use super::terminal::{
    SearchDocument,
    SearchMatch,
    SearchOptions,
    SearchScope,
    TerminalSearcher,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct TabSearcher {
    terminal: TerminalSearcher,
}

impl TabSearcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn search(
        &self,
        tabs: &[SearchDocument],
        query: &str,
        options: &SearchOptions,
    ) -> Vec<SearchMatch> {
        let documents: Vec<SearchDocument> = tabs
            .iter()
            .filter(|document| document.scope == SearchScope::Tab)
            .cloned()
            .collect();

        self.terminal.search(&documents, query, options)
    }
}
