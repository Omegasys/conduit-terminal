/// Syntax element categories for TOML configuration files.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntaxElementKind {
    Plain,
    Comment,
    Section,
    Key,
    String,
    Number,
    Boolean,
    DateTime,
    Operator,
    Punctuation,
    Error,
}

/// A highlighted piece of a configuration line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxElement {
    pub kind: SyntaxElementKind,
    pub text: String,
    pub start: usize,
    pub end: usize,
}

impl SyntaxElement {
    pub fn new(
        kind: SyntaxElementKind,
        text: impl Into<String>,
        start: usize,
        end: usize,
    ) -> Self {
        Self {
            kind,
            text: text.into(),
            start,
            end,
        }
    }
}

/// Syntax information for one line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxLine {
    pub line: usize,
    pub elements: Vec<SyntaxElement>,
}

impl SyntaxLine {
    pub fn new(line: usize) -> Self {
        Self {
            line,
            elements: Vec::new(),
        }
    }

    pub fn add(&mut self, element: SyntaxElement) {
        self.elements.push(element);
    }

    pub fn elements(&self) -> &[SyntaxElement] {
        &self.elements
    }
}

/// Lightweight TOML syntax highlighter.
///
/// Parsing and semantic validation remain the responsibility of the
/// config engine. This component only determines how text should be
/// visually represented by the editor.
#[derive(Debug, Clone, Default)]
pub struct SyntaxHighlighter {
    enabled: bool,
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        Self { enabled: true }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn highlight(&self, text: &str) -> Vec<SyntaxLine> {
        if !self.enabled {
            return text
                .lines()
                .enumerate()
                .map(|(line, content)| {
                    let mut result = SyntaxLine::new(line);

                    result.add(SyntaxElement::new(
                        SyntaxElementKind::Plain,
                        content,
                        0,
                        content.len(),
                    ));

                    result
                })
                .collect();
        }

        text.lines()
            .enumerate()
            .map(|(line, content)| self.highlight_line(line, content))
            .collect()
    }

    fn highlight_line(&self, line_number: usize, line: &str) -> SyntaxLine {
        let mut result = SyntaxLine::new(line_number);

        let trimmed = line.trim_start();
        let leading = line.len() - trimmed.len();

        if trimmed.is_empty() {
            return result;
        }

        if trimmed.starts_with('#') {
            result.add(SyntaxElement::new(
                SyntaxElementKind::Comment,
                line,
                0,
                line.len(),
            ));

            return result;
        }

        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            result.add(SyntaxElement::new(
                SyntaxElementKind::Section,
                line,
                0,
                line.len(),
            ));

            return result;
        }

        if let Some(equal_offset) = line.find('=') {
            let key_part = &line[..equal_offset];

            let key_start = key_part
                .char_indices()
                .find(|(_, character)| !character.is_whitespace())
                .map(|(index, _)| index)
                .unwrap_or(leading);

            let key_end = key_part.trim_end().len();

            if key_start < key_end {
                result.add(SyntaxElement::new(
                    SyntaxElementKind::Key,
                    &line[key_start..key_end],
                    key_start,
                    key_end,
                ));
            }

            result.add(SyntaxElement::new(
                SyntaxElementKind::Operator,
                "=",
                equal_offset,
                equal_offset + 1,
            ));

            let value_start = equal_offset + 1;
            let value = &line[value_start..];

            self.highlight_value(&mut result, value, value_start);

            return result;
        }

        result.add(SyntaxElement::new(
            SyntaxElementKind::Plain,
            line,
            0,
            line.len(),
        ));

        result
    }

    fn highlight_value(
        &self,
        result: &mut SyntaxLine,
        value: &str,
        absolute_start: usize,
    ) {
        let trimmed = value.trim_start();
        let leading = value.len() - trimmed.len();
        let start = absolute_start + leading;

        if trimmed.starts_with('#') {
            result.add(SyntaxElement::new(
                SyntaxElementKind::Comment,
                trimmed,
                start,
                start + trimmed.len(),
            ));
            return;
        }

        let token = trimmed
            .split_once('#')
            .map(|(value, _)| value.trim_end())
            .unwrap_or(trimmed);

        let end = start + token.len();

        let kind = if (token.starts_with('"') && token.ends_with('"'))
            || (token.starts_with('\'') && token.ends_with('\''))
        {
            SyntaxElementKind::String
        } else if token == "true" || token == "false" {
            SyntaxElementKind::Boolean
        } else if token.parse::<f64>().is_ok() {
            SyntaxElementKind::Number
        } else if token.contains('T') && token.len() >= 10 {
            SyntaxElementKind::DateTime
        } else if token.starts_with('[') || token.starts_with('{') {
            SyntaxElementKind::Punctuation
        } else {
            SyntaxElementKind::Plain
        };

        result.add(SyntaxElement::new(kind, token, start, end));

        if let Some(comment_offset) = trimmed.find('#') {
            let comment_start = absolute_start + leading + comment_offset;

            result.add(SyntaxElement::new(
                SyntaxElementKind::Comment,
                &trimmed[comment_offset..],
                comment_start,
                absolute_start + leading + trimmed.len(),
            ));
        }
    }
}
