use std::collections::BTreeMap;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum CliValue {
    String(String),
    Integer(i64),
    Boolean(bool),
}

impl CliValue {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Self::Integer(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Boolean(value) => Some(*value),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CliArgument {
    name: String,
    value: CliValue,
}

impl CliArgument {
    pub fn new(name: impl Into<String>, value: CliValue) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &CliValue {
        &self.value
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CliCommandLine {
    command: String,
    arguments: Vec<String>,
    options: BTreeMap<String, CliValue>,
}

impl CliCommandLine {
    pub fn new(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            arguments: Vec::new(),
            options: BTreeMap::new(),
        }
    }

    pub fn command(&self) -> &str {
        &self.command
    }

    pub fn arguments(&self) -> &[String] {
        &self.arguments
    }

    pub fn options(&self) -> &BTreeMap<String, CliValue> {
        &self.options
    }

    pub fn option(&self, name: &str) -> Option<&CliValue> {
        self.options.get(name)
    }

    pub fn add_argument(&mut self, argument: impl Into<String>) {
        self.arguments.push(argument.into());
    }

    pub fn set_option(&mut self, name: impl Into<String>, value: CliValue) {
        self.options.insert(name.into(), value);
    }

    pub fn has_option(&self, name: &str) -> bool {
        self.options.contains_key(name)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CliParseError {
    EmptyCommand,
    MissingOptionValue(String),
    InvalidInteger(String),
    InvalidBoolean(String),
    UnexpectedArgument(String),
}

impl fmt::Display for CliParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyCommand => write!(formatter, "no command was provided"),
            Self::MissingOptionValue(option) => {
                write!(formatter, "missing value for option '--{}'", option)
            }
            Self::InvalidInteger(value) => {
                write!(formatter, "invalid integer '{}'", value)
            }
            Self::InvalidBoolean(value) => {
                write!(formatter, "invalid boolean '{}'", value)
            }
            Self::UnexpectedArgument(value) => {
                write!(formatter, "unexpected argument '{}'", value)
            }
        }
    }
}

impl std::error::Error for CliParseError {}

pub struct CliParser;

impl CliParser {
    pub fn parse(input: &str) -> Result<CliCommandLine, CliParseError> {
        let tokens = Self::tokenize(input);

        if tokens.is_empty() {
            return Err(CliParseError::EmptyCommand);
        }

        Self::parse_tokens(&tokens)
    }

    pub fn parse_args<I, S>(
        arguments: I,
    ) -> Result<CliCommandLine, CliParseError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let tokens: Vec<String> = arguments.into_iter().map(Into::into).collect();

        if tokens.is_empty() {
            return Err(CliParseError::EmptyCommand);
        }

        Self::parse_tokens(&tokens)
    }

    fn parse_tokens(tokens: &[String]) -> Result<CliCommandLine, CliParseError> {
        let mut result = CliCommandLine::new(&tokens[0]);

        let mut index = 1;

        while index < tokens.len() {
            let token = &tokens[index];

            if let Some(option) = token.strip_prefix("--") {
                if option.is_empty() {
                    index += 1;
                    continue;
                }

                if let Some((name, value)) = option.split_once('=') {
                    result.set_option(
                        name.to_string(),
                        Self::parse_value(value),
                    );

                    index += 1;
                    continue;
                }

                if index + 1 < tokens.len()
                    && !tokens[index + 1].starts_with('-')
                {
                    let value = &tokens[index + 1];

                    result.set_option(
                        option.to_string(),
                        Self::parse_value(value),
                    );

                    index += 2;
                } else {
                    result.set_option(
                        option.to_string(),
                        CliValue::Boolean(true),
                    );

                    index += 1;
                }

                continue;
            }

            if let Some(option) = token.strip_prefix('-') {
                if option.is_empty() {
                    result.add_argument(token.clone());
                    index += 1;
                    continue;
                }

                if option.len() > 1 && !option.contains('=') {
                    for character in option.chars() {
                        result.set_option(
                            character.to_string(),
                            CliValue::Boolean(true),
                        );
                    }

                    index += 1;
                    continue;
                }

                let option = option.trim_start_matches('-');

                if let Some((name, value)) = option.split_once('=') {
                    result.set_option(
                        name.to_string(),
                        Self::parse_value(value),
                    );

                    index += 1;
                    continue;
                }

                if index + 1 < tokens.len()
                    && !tokens[index + 1].starts_with('-')
                {
                    result.set_option(
                        option.to_string(),
                        Self::parse_value(&tokens[index + 1]),
                    );

                    index += 2;
                } else {
                    result.set_option(
                        option.to_string(),
                        CliValue::Boolean(true),
                    );

                    index += 1;
                }

                continue;
            }

            result.add_argument(token.clone());
            index += 1;
        }

        Ok(result)
    }

    fn parse_value(value: &str) -> CliValue {
        if let Ok(integer) = value.parse::<i64>() {
            return CliValue::Integer(integer);
        }

        match value.to_ascii_lowercase().as_str() {
            "true" | "yes" | "on" => CliValue::Boolean(true),
            "false" | "no" | "off" => CliValue::Boolean(false),
            _ => CliValue::String(value.to_string()),
        }
    }

    fn tokenize(input: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut current = String::new();
        let mut quote = None;
        let mut escaped = false;

        for character in input.chars() {
            if escaped {
                current.push(character);
                escaped = false;
                continue;
            }

            if character == '\\' {
                escaped = true;
                continue;
            }

            if let Some(active_quote) = quote {
                if character == active_quote {
                    quote = None;
                } else {
                    current.push(character);
                }

                continue;
            }

            match character {
                '\'' | '"' => {
                    quote = Some(character);
                }
                character if character.is_whitespace() => {
                    if !current.is_empty() {
                        tokens.push(std::mem::take(&mut current));
                    }
                }
                _ => current.push(character),
            }
        }

        if !current.is_empty() {
            tokens.push(current);
        }

        tokens
    }
}
