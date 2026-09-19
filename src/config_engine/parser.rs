//! Configuration parser.
//!
//! TOML is currently the primary on-disk configuration format.

use std::collections::BTreeMap;

use super::schema::ConfigValue;

/// Parsed configuration tree.
#[derive(Debug, Clone)]
pub struct ParsedConfig {
    root: ConfigValue,
}

impl ParsedConfig {
    pub fn new(
        root: ConfigValue,
    ) -> Self {
        Self { root }
    }

    pub fn root(
        &self,
    ) -> &ConfigValue {
        &self.root
    }

    pub fn get(
        &self,
        path: &str,
    ) -> Option<&ConfigValue> {
        let mut current =
            &self.root;

        for component in path.split('.') {
            let ConfigValue::Table(table) =
                current
            else {
                return None;
            };

            current =
                table.get(component)?;
        }

        Some(current)
    }
}

/// Errors produced while parsing configuration.
#[derive(Debug)]
pub enum ConfigParseError {
    InvalidToml(String),
    RootMustBeTable,
}

impl std::fmt::Display
    for ConfigParseError
{
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::InvalidToml(error) => {
                write!(
                    formatter,
                    "invalid TOML: {}",
                    error
                )
            }

            Self::RootMustBeTable => {
                write!(
                    formatter,
                    "configuration root must be a table"
                )
            }
        }
    }
}

impl std::error::Error
    for ConfigParseError {}

/// Parses configuration files.
#[derive(Debug, Default)]
pub struct ConfigParser;

impl ConfigParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(
        &self,
        source: &str,
    ) -> Result<ParsedConfig, ConfigParseError> {
        let value: toml::Value =
            source
                .parse()
                .map_err(
                    |error: toml::de::Error| {
                        ConfigParseError::InvalidToml(
                            error.to_string()
                        )
                    }
                )?;

        let converted =
            convert_toml_value(
                value
            );

        if !converted.is_table() {
            return Err(
                ConfigParseError::RootMustBeTable
            );
        }

        Ok(
            ParsedConfig::new(
                converted
            )
        )
    }
}

fn convert_toml_value(
    value: toml::Value,
) -> ConfigValue {
    match value {
        toml::Value::String(value) =>
            ConfigValue::String(value),

        toml::Value::Integer(value) =>
            ConfigValue::Integer(value),

        toml::Value::Float(value) =>
            ConfigValue::Float(value),

        toml::Value::Boolean(value) =>
            ConfigValue::Boolean(value),

        toml::Value::Datetime(value) =>
            ConfigValue::String(
                value.to_string()
            ),

        toml::Value::Array(values) =>
            ConfigValue::Array(
                values
                    .into_iter()
                    .map(
                        convert_toml_value
                    )
                    .collect()
            ),

        toml::Value::Table(table) =>
            ConfigValue::Table(
                table
                    .into_iter()
                    .map(
                        |(key, value)| {
                            (
                                key,
                                convert_toml_value(
                                    value
                                ),
                            )
                        }
                    )
                    .collect::<BTreeMap<_, _>>()
            ),
    }
}
