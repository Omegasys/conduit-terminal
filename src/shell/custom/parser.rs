//! Lightweight parser for custom shell command integration.
//!
//! Custom shells can provide their own parser through plugins or future
//! parser implementations. This parser intentionally remains shell-neutral.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CustomToken {
    Word(String),
    SingleQuoted(String),
    DoubleQuoted(String),
    Pipe,
    LogicalAnd,
    LogicalOr,
    RedirectInput,
    RedirectOutput,
    RedirectAppend,
    Background,
    Separator,
    OpenParen,
    CloseParen,
}

#[derive(Debug, Clone)]
pub struct CustomParseResult {
    pub source: String,
    pub tokens: Vec<CustomToken>,
}

impl CustomParseResult {
    pub fn command_name(&self) -> Option<&str> {
        self.tokens.iter().find_map(|token| match token {
            CustomToken::Word(value)
            | CustomToken::SingleQuoted(value)
            | CustomToken::DoubleQuoted(value) => Some(value.as_str()),
            _ => None,
        })
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }
}

pub trait CustomCommandParser: Send + Sync {
    fn parse(&self, input: &str) -> CustomParseResult;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct BasicCustomParser;

impl CustomCommandParser for BasicCustomParser {
    fn parse(&self, input: &str) -> CustomParseResult {
        let mut tokens = Vec::new();
        let mut current = String::new();
        let mut chars = input.chars().peekable();

        while let Some(ch) = chars.next() {
            match ch {
                '\'' => {
                    if !current.is_empty() {
                        tokens.push(CustomToken::Word(std::mem::take(&mut current)));
                    }

                    let mut value = String::new();

                    while let Some(next) = chars.next() {
                        if next == '\'' {
                            break;
                        }

                        value.push(next);
                    }

                    tokens.push(CustomToken::SingleQuoted(value));
                }

                '"' => {
                    if !current.is_empty() {
                        tokens.push(CustomToken::Word(std::mem::take(&mut current)));
                    }

                    let mut value = String::new();

                    while let Some(next) = chars.next() {
                        if next == '"' {
                            break;
                        }

                        value.push(next);
                    }

                    tokens.push(CustomToken::DoubleQuoted(value));
                }

                ' ' | '\t' | '\n' => {
                    if !current.is_empty() {
                        tokens.push(CustomToken::Word(std::mem::take(&mut current)));
                    }
                }

                '|' => {
                    if !current.is_empty() {
                        tokens.push(CustomToken::Word(std::mem::take(&mut current)));
                    }

                    if chars.peek() == Some(&'|') {
                        chars.next();
                        tokens.push(CustomToken::LogicalOr);
                    } else {
                        tokens.push(CustomToken::Pipe);
                    }
                }

                '&' => {
                    if !current.is_empty() {
                        tokens.push(CustomToken::Word(std::mem::take(&mut current)));
                    }

                    if chars.peek() == Some(&'&') {
                        chars.next();
                        tokens.push(CustomToken::LogicalAnd);
                    } else {
                        tokens.push(CustomToken::Background);
                    }
                }

                ';' => {
                    if !current.is_empty() {
                        tokens.push(CustomToken::Word(std::mem::take(&mut current)));
                    }

                    tokens.push(CustomToken::Separator);
                }

                '<' => {
                    if !current.is_empty() {
                        tokens.push(CustomToken::Word(std::mem::take(&mut current)));
                    }

                    tokens.push(CustomToken::RedirectInput);
                }

                '>' => {
                    if !current.is_empty() {
                        tokens.push(CustomToken::Word(std::mem::take(&mut current)));
                    }

                    if chars.peek() == Some(&'>') {
                        chars.next();
                        tokens.push(CustomToken::RedirectAppend);
                    } else {
                        tokens.push(CustomToken::RedirectOutput);
                    }
                }

                '(' => {
                    if !current.is_empty() {
                        tokens.push(CustomToken::Word(std::mem::take(&mut current)));
                    }

                    tokens.push(CustomToken::OpenParen);
                }

                ')' => {
                    if !current.is_empty() {
                        tokens.push(CustomToken::Word(std::mem::take(&mut current)));
                    }

                    tokens.push(CustomToken::CloseParen);
                }

                '\\' => {
                    if let Some(next) = chars.next() {
                        current.push(next);
                    }
                }

                _ => current.push(ch),
            }
        }

        if !current.is_empty() {
            tokens.push(CustomToken::Word(current));
        }

        CustomParseResult {
            source: input.to_string(),
            tokens,
        }
    }
}
