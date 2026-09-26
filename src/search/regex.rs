use regex::Regex;

use super::case::CaseSensitivity;
use super::terminal::{
    SearchDocument,
    SearchMatch,
    SearchOptions,
    SearchScope,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct RegexSearcher;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegexSearchError {
    InvalidPattern(String),
}

impl RegexSearcher {
    pub fn new() -> Self {
        Self
    }

    pub fn compile(
        &self,
        pattern: &str,
        sensitivity: CaseSensitivity,
    ) -> Result<Regex, RegexSearchError> {
        let pattern = match sensitivity {
            CaseSensitivity::Sensitive => pattern.to_owned(),
            CaseSensitivity::Insensitive => {
                format!("(?i:{pattern})")
            }
        };

        Regex::new(&pattern)
            .map_err(|error| RegexSearchError::InvalidPattern(error.to_string()))
    }

    pub fn search(
        &self,
        documents: &[SearchDocument],
        pattern: &str,
        options: &SearchOptions,
    ) -> Vec<SearchMatch> {
        let regex = match self.compile(pattern, options.case_sensitivity) {
            Ok(regex) => regex,
            Err(_) => return Vec::new(),
        };

        let mut results = Vec::new();

        for document in documents {
            if !Self::scope_allowed(document.scope) {
                continue;
            }

            for found in regex.find_iter(&document.text) {
                if options.whole_word
                    && !super::whole_word::WholeWordMatcher::new()
                        .matches(
                            &document.text,
                            found.start(),
                            found.end(),
                        )
                {
                    continue;
                }

                results.push(SearchMatch::new(
                    document,
                    found.start(),
                    found.end(),
                ));

                if let Some(limit) = options.max_results {
                    if results.len() >= limit {
                        if options.reverse {
                            results.reverse();
                        }
                        return results;
                    }
                }
            }
        }

        if options.reverse {
            results.reverse();
        }

        results
    }

    fn scope_allowed(scope: SearchScope) -> bool {
        matches!(
            scope,
            SearchScope::Terminal
                | SearchScope::Scrollback
                | SearchScope::Pane
                | SearchScope::Tab
                | SearchScope::Session
                | SearchScope::Recording
                | SearchScope::History
        )
    }
}
