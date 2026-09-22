use super::history::CommandHistory;

#[derive(Debug, Clone, Copy)]
pub struct CommandSearchOptions {
    pub case_sensitive: bool,
    pub include_sensitive: bool,
    pub maximum_results: usize,
}

impl Default for CommandSearchOptions {
    fn default() -> Self {
        Self {
            case_sensitive: false,
            include_sensitive: false,
            maximum_results: 100,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CommandSearchMatch {
    pub history_id: u64,
    pub command: String,
    pub score: usize,
}

impl CommandSearchMatch {
    pub fn new(history_id: u64, command: String, score: usize) -> Self {
        Self {
            history_id,
            command,
            score,
        }
    }
}

#[derive(Debug, Default)]
pub struct CommandSearch;

impl CommandSearch {
    pub fn search(
        history: &CommandHistory,
        query: &str,
        options: CommandSearchOptions,
    ) -> Vec<CommandSearchMatch> {
        if query.is_empty() {
            return Vec::new();
        }

        let normalized_query = if options.case_sensitive {
            query.to_string()
        } else {
            query.to_lowercase()
        };

        let mut results = Vec::new();

        for entry in history.entries().rev() {
            if entry.sensitive() && !options.include_sensitive {
                continue;
            }

            let command = entry.command();

            let haystack = if options.case_sensitive {
                command.to_string()
            } else {
                command.to_lowercase()
            };

            if !haystack.contains(&normalized_query) {
                continue;
            }

            let score = Self::score(&haystack, &normalized_query);

            results.push(CommandSearchMatch::new(
                entry.id(),
                command.to_string(),
                score,
            ));

            if results.len() >= options.maximum_results {
                break;
            }
        }

        results.sort_by(|a, b| b.score.cmp(&a.score));
        results
    }

    fn score(command: &str, query: &str) -> usize {
        if command == query {
            return 100;
        }

        if command.starts_with(query) {
            return 90;
        }

        if command.contains(&format!(" {query}")) {
            return 80;
        }

        50
    }
}
