use super::terminal::{
    SearchDocument,
    SearchMatch,
    SearchOptions,
    SearchScope,
    TerminalSearcher,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct HistorySearcher {
    terminal: TerminalSearcher,
}

impl HistorySearcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn search(
        &self,
        history: &[String],
        query: &str,
        options: &SearchOptions,
    ) -> Vec<SearchMatch> {
        let documents: Vec<SearchDocument> = history
            .iter()
            .enumerate()
            .map(|(index, command)| {
                SearchDocument::new(
                    format!("history-{index}"),
                    SearchScope::History,
                    command,
                )
            })
            .collect();

        self.terminal.search(&documents, query, options)
    }
}
