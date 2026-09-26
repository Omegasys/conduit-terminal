/// A parsed command token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandToken {
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

/// A shell-neutral parsed command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedCommand {
    pub source: String,
    pub tokens: Vec<CommandToken>,
}

impl ParsedCommand {
    pub fn new(source: impl Into<String>, tokens: Vec<CommandToken>) -> Self {
        Self {
            source: source.into(),
            tokens,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    pub fn command_name(&self) -> Option<&str> {
        self.tokens.iter().find_map(|token| match token {
            CommandToken::Word(value) => Some(value.as_str()),
            _ => None,
        })
    }
}

/// Lightweight shell parser.
///
/// This intentionally does not attempt to implement Bash, Zsh, Fish,
/// PowerShell, or Nushell grammar. Those shells should provide specialized
/// parsers/adapters. This parser is useful for generic command inspection.
#[derive(Debug, Default, Clone, Copy)]
pub struct ShellParser;

impl ShellParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, source: &str) -> ParsedCommand {
        let mut tokens = Vec::new();
        let chars: Vec<char> = source.chars().collect();
        let mut current = String::new();
        let mut index = 0;

        while index < chars.len() {
            let ch = chars[index];

            match ch {
                '\'' => {
                    Self::flush_word(&mut current, &mut tokens);

                    index += 1;
                    let mut value = String::new();

                    while index < chars.len() && chars[index] != '\'' {
                        value.push(chars[index]);
                        index += 1;
                    }

                    tokens.push(CommandToken::SingleQuoted(value));
                }

                '"' => {
                    Self::flush_word(&mut current, &mut tokens);

                    index += 1;
                    let mut value = String::new();

                    while index < chars.len() && chars[index] != '"' {
                        value.push(chars[index]);
                        index += 1;
                    }

                    tokens.push(CommandToken::DoubleQuoted(value));
                }

                ' ' | '\t' | '\n' | '\r' => {
                    Self::flush_word(&mut current, &mut tokens);
                }

                '|' => {
                    Self::flush_word(&mut current, &mut tokens);

                    if index + 1 < chars.len() && chars[index + 1] == '|' {
                        tokens.push(CommandToken::LogicalOr);
                        index += 1;
                    } else {
                        tokens.push(CommandToken::Pipe);
                    }
                }

                '&' => {
                    Self::flush_word(&mut current, &mut tokens);

                    if index + 1 < chars.len() && chars[index + 1] == '&' {
                        tokens.push(CommandToken::LogicalAnd);
                        index += 1;
                    } else {
                        tokens.push(CommandToken::Background);
                    }
                }

                ';' => {
                    Self::flush_word(&mut current, &mut tokens);
                    tokens.push(CommandToken::Separator);
                }

                '<' => {
                    Self::flush_word(&mut current, &mut tokens);
                    tokens.push(CommandToken::RedirectInput);
                }

                '>' => {
                    Self::flush_word(&mut current, &mut tokens);

                    if index + 1 < chars.len() && chars[index + 1] == '>' {
                        tokens.push(CommandToken::RedirectAppend);
                        index += 1;
                    } else {
                        tokens.push(CommandToken::RedirectOutput);
                    }
                }

                '(' => {
                    Self::flush_word(&mut current, &mut tokens);
                    tokens.push(CommandToken::OpenParen);
                }

                ')' => {
                    Self::flush_word(&mut current, &mut tokens);
                    tokens.push(CommandToken::CloseParen);
                }

                '\\' => {
                    if index + 1 < chars.len() {
                        index += 1;
                        current.push(chars[index]);
                    }
                }

                _ => current.push(ch),
            }

            index += 1;
        }

        Self::flush_word(&mut current, &mut tokens);

        ParsedCommand::new(source, tokens)
    }

    fn flush_word(current: &mut String, tokens: &mut Vec<CommandToken>) {
        if !current.is_empty() {
            tokens.push(CommandToken::Word(std::mem::take(current)));
        }
    }
}
