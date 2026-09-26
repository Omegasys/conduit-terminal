use super::case::CaseSensitivity;
use super::whole_word::WholeWordMatcher;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchQuery {
    Literal(String),
    Regex(String),
}

impl SearchQuery {
    pub fn literal(value: impl Into<String>) -> Self {
        Self::Literal(value.into())
    }

    pub fn regex(value: impl Into<String>) -> Self {
        Self::Regex(value.into())
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Literal(value) | Self::Regex(value) => value,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchScope {
    Terminal,
    Scrollback,
    Pane,
    Tab,
    Session,
    Recording,
    History,
    All,
}

impl Default for SearchScope {
    fn default() -> Self {
        Self::Terminal
    }
}

#[derive(Debug, Clone)]
pub struct SearchDocument {
    pub id: String,
    pub scope: SearchScope,
    pub text: String,
}

impl SearchDocument {
    pub fn new(
        id: impl Into<String>,
        scope: SearchScope,
        text: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            scope,
            text: text.into(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SearchOptions {
    pub case_sensitivity: CaseSensitivity,
    pub whole_word: bool,
    pub reverse: bool,
    pub max_results: Option<usize>,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            case_sensitivity: CaseSensitivity::Insensitive,
            whole_word: false,
            reverse: false,
            max_results: Some(100),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchMatch {
    pub document_id: String,
    pub scope: SearchScope,
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
    pub matched_text: String,
}

impl SearchMatch {
    pub fn new(
        document: &SearchDocument,
        start: usize,
        end: usize,
    ) -> Self {
        let line = document.text[..start]
            .bytes()
            .filter(|byte| *byte == b'\n')
            .count();

        let line_start = document.text[..start]
            .rfind('\n')
            .map(|index| index + 1)
            .unwrap_or(0);

        let column = document.text[line_start..start].chars().count();

        Self {
            document_id: document.id.clone(),
            scope: document.scope,
            start,
            end,
            line,
            column,
            matched_text: document.text[start..end].to_owned(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct TerminalSearcher;

impl TerminalSearcher {
    pub fn new() -> Self {
        Self
    }

    pub fn search(
        &self,
        documents: &[SearchDocument],
        query: &str,
        options: &SearchOptions,
    ) -> Vec<SearchMatch> {
        if query.is_empty() {
            return Vec::new();
        }

        let matcher = WholeWordMatcher::new();
        let normalized_query =
            options.case_sensitivity.normalize(query);

        let mut results = Vec::new();

        for document in documents {
            if !Self::scope_matches(document.scope, options_scope(options)) {
                continue;
            }

            let search_text =
                options.case_sensitivity.normalize(&document.text);

            let mut offset = 0;

            while let Some(relative) =
                search_text[offset..].find(&normalized_query)
            {
                let start = offset + relative;
                let end = start + normalized_query.len();

                if !options.whole_word
                    || matcher.matches(&document.text, start, end)
                {
                    results.push(SearchMatch::new(document, start, end));

                    if let Some(limit) = options.max_results {
                        if results.len() >= limit {
                            return Self::finish(results, options);
                        }
                    }
                }

                offset = end;

                if offset >= search_text.len() {
                    break;
                }
            }
        }

        Self::finish(results, options)
    }

    fn scope_matches(
        document_scope: SearchScope,
        requested_scope: SearchScope,
    ) -> bool {
        requested_scope == SearchScope::All
            || document_scope == requested_scope
    }

    fn finish(
        mut results: Vec<SearchMatch>,
        options: &SearchOptions,
    ) -> Vec<SearchMatch> {
        if options.reverse {
            results.reverse();
        }

        results
    }
}

fn options_scope(_options: &SearchOptions) -> SearchScope {
    // Scope is intentionally kept outside SearchOptions so the same
    // options can be reused by the specialized searchers.
    SearchScope::All
}
