use super::terminal::{
    SearchDocument,
    SearchMatch,
    SearchOptions,
    SearchScope,
    TerminalSearcher,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct PaneSearcher {
    terminal: TerminalSearcher,
}

impl PaneSearcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn search(
        &self,
        panes: &[SearchDocument],
        query: &str,
        options: &SearchOptions,
    ) -> Vec<SearchMatch> {
        let mut scoped = Vec::new();

        for pane in panes {
            if pane.scope == SearchScope::Pane {
                scoped.push(pane.clone());
            }
        }

        self.terminal.search(&scoped, query, options)
    }
}
