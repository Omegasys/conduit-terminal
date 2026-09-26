use super::terminal::{
    SearchDocument,
    SearchMatch,
    SearchOptions,
    SearchScope,
    TerminalSearcher,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct ScrollbackSearcher {
    terminal: TerminalSearcher,
}

impl ScrollbackSearcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn search(
        &self,
        scrollback: &[String],
        query: &str,
        options: &SearchOptions,
        document_id: impl Into<String>,
    ) -> Vec<SearchMatch> {
        let text = scrollback.join("\n");

        let document = SearchDocument::new(
            document_id,
            SearchScope::Scrollback,
            text,
        );

        self.terminal
            .search(&[document], query, options)
    }
}
