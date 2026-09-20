use super::commands::CommandEntry;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchMatchKind {
    Exact,
    Prefix,
    Word,
    Substring,
    Keyword,
    Fuzzy,
}

#[derive(Debug, Clone)]
pub struct CommandSearchMatch {
    command_id: String,
    score: u32,
    kind: SearchMatchKind,
    matched_text: String,
}

impl CommandSearchMatch {
    pub fn new(
        command_id: impl Into<String>,
        score: u32,
        kind: SearchMatchKind,
        matched_text: impl Into<String>,
    ) -> Self {
        Self {
            command_id: command_id.into(),
            score,
            kind,
            matched_text: matched_text.into(),
        }
    }

    pub fn command_id(&self) -> &str {
        &self.command_id
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn kind(&self) -> SearchMatchKind {
        self.kind
    }

    pub fn matched_text(&self) -> &str {
        &self.matched_text
    }
}

#[derive(Debug, Default)]
pub struct CommandSearch {
    query: String,
    case_sensitive: bool,
    fuzzy: bool,
    max_results: usize,
}

impl CommandSearch {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            case_sensitive: false,
            fuzzy: true,
            max_results: 50,
        }
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn is_case_sensitive(&self) -> bool {
        self.case_sensitive
    }

    pub fn is_fuzzy_enabled(&self) -> bool {
        self.fuzzy
    }

    pub fn max_results(&self) -> usize {
        self.max_results
    }

    pub fn set_query(&mut self, query: impl Into<String>) {
        self.query = query.into();
    }

    pub fn clear_query(&mut self) {
        self.query.clear();
    }

    pub fn set_case_sensitive(&mut self, enabled: bool) {
        self.case_sensitive = enabled;
    }

    pub fn set_fuzzy(&mut self, enabled: bool) {
        self.fuzzy = enabled;
    }

    pub fn set_max_results(&mut self, max_results: usize) {
        self.max_results = max_results.max(1);
    }

    pub fn search<'a>(
        &self,
        commands: impl Iterator<Item = &'a CommandEntry>,
    ) -> Vec<CommandSearchMatch> {
        if self.query.trim().is_empty() {
            return commands
                .filter(|command| command.is_visible())
                .take(self.max_results)
                .map(|command| {
                    CommandSearchMatch::new(
                        command.id(),
                        0,
                        SearchMatchKind::Substring,
                        command.label(),
                    )
                })
                .collect();
        }

        let query = if self.case_sensitive {
            self.query.clone()
        } else {
            self.query.to_lowercase()
        };

        let mut matches = Vec::new();

        for command in commands {
            if !command.is_visible() {
                continue;
            }

            if let Some((score, kind, text)) = self.match_command(command, &query) {
                matches.push(CommandSearchMatch::new(
                    command.id(),
                    score,
                    kind,
                    text,
                ));
            }
        }

        matches.sort_by(|a, b| {
            b.score()
                .cmp(&a.score())
                .then_with(|| a.command_id().cmp(b.command_id()))
        });

        matches.truncate(self.max_results);
        matches
    }

    fn match_command(
        &self,
        command: &CommandEntry,
        query: &str,
    ) -> Option<(u32, SearchMatchKind, String)> {
        let label = if self.case_sensitive {
            command.label().to_string()
        } else {
            command.label().to_lowercase()
        };

        if label == query {
            return Some((1000, SearchMatchKind::Exact, command.label().to_string()));
        }

        if label.starts_with(query) {
            return Some((900, SearchMatchKind::Prefix, command.label().to_string()));
        }

        if label
            .split_whitespace()
            .any(|word| word.starts_with(query))
        {
            return Some((800, SearchMatchKind::Word, command.label().to_string()));
        }

        if label.contains(query) {
            return Some((700, SearchMatchKind::Substring, command.label().to_string()));
        }

        for keyword in command.keywords() {
            let keyword_cmp = if self.case_sensitive {
                keyword.clone()
            } else {
                keyword.to_lowercase()
            };

            if keyword_cmp == query {
                return Some((650, SearchMatchKind::Keyword, keyword.clone()));
            }

            if keyword_cmp.starts_with(query) {
                return Some((600, SearchMatchKind::Keyword, keyword.clone()));
            }

            if keyword_cmp.contains(query) {
                return Some((500, SearchMatchKind::Keyword, keyword.clone()));
            }
        }

        if self.fuzzy && Self::fuzzy_match(&label, query) {
            return Some((400, SearchMatchKind::Fuzzy, command.label().to_string()));
        }

        None
    }

    fn fuzzy_match(text: &str, query: &str) -> bool {
        if query.is_empty() {
            return true;
        }

        let mut query_chars = query.chars();

        let mut current = match query_chars.next() {
            Some(character) => character,
            None => return true,
        };

        for character in text.chars() {
            if character == current {
                match query_chars.next() {
                    Some(next) => current = next,
                    None => return true,
                }
            }
        }

        false
    }
}
