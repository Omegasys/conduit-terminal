use super::terminal::{
    SearchDocument,
    SearchMatch,
    SearchOptions,
    SearchScope,
    TerminalSearcher,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct RecordingSearcher {
    terminal: TerminalSearcher,
}

impl RecordingSearcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn search(
        &self,
        recordings: &[SearchDocument],
        query: &str,
        options: &SearchOptions,
    ) -> Vec<SearchMatch> {
        let documents: Vec<SearchDocument> = recordings
            .iter()
            .filter(|document| {
                document.scope == SearchScope::Recording
            })
            .cloned()
            .collect();

        self.terminal.search(&documents, query, options)
    }
}
