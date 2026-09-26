use super::database::HistoryDatabaseEntry;
use super::filtering::HistoryFilter;

#[derive(Debug, Clone)]
pub struct HistorySearchOptions {
    pub query: String,
    pub limit: usize,
    pub reverse: bool,
    pub filter: Option<HistoryFilter>,
}

impl Default for HistorySearchOptions {
    fn default() -> Self {
        Self {
            query: String::new(),
            limit: 100,
            reverse: true,
            filter: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HistorySearchResult {
    pub id: u64,
    pub command: String,
    pub score: usize,
}

#[derive(Debug, Default)]
pub struct HistorySearch;

impl HistorySearch {
    pub fn new() -> Self {
        Self
    }

    pub fn search(
        &self,
        entries: &[HistoryDatabaseEntry],
        options: &HistorySearchOptions,
    ) -> Vec<HistorySearchResult> {
        let query = options.query.to_lowercase();

        let iterator: Box<dyn Iterator<Item = &HistoryDatabaseEntry>> =
            if options.reverse {
                Box::new(entries.iter().rev())
            } else {
                Box::new(entries.iter())
            };

        let mut results = Vec::new();

        for entry in iterator {
            if let Some(filter) = &options.filter {
                if !filter.matches(
                    entry.command(),
                    entry.metadata(),
                ) {
                    continue;
                }
            }

            let score = score_command(entry.command(), &query);

            if !query.is_empty() && score == 0 {
                continue;
            }

            results.push(HistorySearchResult {
                id: entry.id(),
                command: entry.command().to_string(),
                score,
            });

            if results.len() >= options.limit {
                break;
            }
        }

        results
    }
}

fn score_command(command: &str, query: &str) -> usize {
    if query.is_empty() {
        return 1;
    }

    let command_lower = command.to_lowercase();

    if command_lower == query {
        return 100;
    }

    if command_lower.starts_with(query) {
        return 75;
    }

    if command_lower.contains(query) {
        return 50;
    }

    let mut score = 0;

    for token in query.split_whitespace() {
        if command_lower.contains(token) {
            score += 10;
        }
    }

    score
}
