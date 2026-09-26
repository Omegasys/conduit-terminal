use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryIndexEntry {
    pub id: u64,
    pub position: usize,
}

#[derive(Debug, Default)]
pub struct HistoryIndex {
    tokens: HashMap<String, Vec<HistoryIndexEntry>>,
}

impl HistoryIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.tokens.clear();
    }

    pub fn index(
        &mut self,
        id: u64,
        position: usize,
        command: &str,
    ) {
        for token in tokenize(command) {
            let entries = self.tokens.entry(token).or_default();

            if !entries.iter().any(|entry| entry.id == id) {
                entries.push(HistoryIndexEntry { id, position });
            }
        }
    }

    pub fn remove(&mut self, id: u64) {
        for entries in self.tokens.values_mut() {
            entries.retain(|entry| entry.id != id);
        }

        self.tokens.retain(|_, entries| !entries.is_empty());
    }

    pub fn lookup(&self, token: &str) -> &[HistoryIndexEntry] {
        static EMPTY: [HistoryIndexEntry; 0] = [];

        self.tokens
            .get(&token.to_lowercase())
            .map(Vec::as_slice)
            .unwrap_or(&EMPTY)
    }

    pub fn rebuild<I>(&mut self, entries: I)
    where
        I: IntoIterator<Item = (u64, String)>,
    {
        self.clear();

        for (position, (id, command)) in entries.into_iter().enumerate() {
            self.index(id, position, &command);
        }
    }
}

fn tokenize(command: &str) -> Vec<String> {
    command
        .split_whitespace()
        .map(str::to_lowercase)
        .filter(|token| !token.is_empty())
        .collect()
}
