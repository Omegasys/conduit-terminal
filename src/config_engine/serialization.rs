use std::collections::BTreeMap;

use super::errors::ConfigError;
use super::ConfigValue;

pub struct ConfigSerializer;

impl ConfigSerializer {
    pub fn to_toml(
        values: &BTreeMap<String, ConfigValue>,
    ) -> Result<String, ConfigError> {
        let mut table = toml::map::Map::new();

        for (key, value) in values {
            table.insert(key.clone(), Self::to_toml_value(value)?);
        }

        Ok(toml::Value::Table(table).to_string())
    }

    pub fn from_toml(
        input: &str,
    ) -> Result<BTreeMap<String, ConfigValue>, ConfigError> {
        let document: toml::Value =
            input.parse::<toml::Value>()
                .map_err(|error| ConfigError::Serialization(error.to_string()))?;

        let table = document
            .as_table()
            .ok_or_else(|| {
                ConfigError::Serialization(
                    "configuration root must be a TOML table".to_owned(),
                )
            })?;

        let mut values = BTreeMap::new();

        for (key, value) in table {
            values.insert(key.clone(), Self::from_toml_value(value));
        }

        Ok(values)
    }

    fn to_toml_value(
        value: &ConfigValue,
    ) -> Result<toml::Value, ConfigError> {
        match value {
            ConfigValue::String(value) => {
                Ok(toml::Value::String(value.clone()))
            }

            ConfigValue::Integer(value) => {
                Ok(toml::Value::Integer(*value))
            }

            ConfigValue::Float(value) => {
                Ok(toml::Value::Float(*value))
            }

            ConfigValue::Boolean(value) => {
                Ok(toml::Value::Boolean(*value))
            }

            ConfigValue::Null => {
                Err(ConfigError::Serialization(
                    "TOML does not support null values".to_owned(),
                ))
            }

            ConfigValue::Array(values) => {
                let mut result = Vec::with_capacity(values.len());

                for value in values {
                    result.push(Self::to_toml_value(value)?);
                }

                Ok(toml::Value::Array(result))
            }

            ConfigValue::Table(values) => {
                let mut table = toml::map::Map::new();

                for (key, value) in values {
                    table.insert(key.clone(), Self::to_toml_value(value)?);
                }

                Ok(toml::Value::Table(table))
            }
        }
    }

    fn from_toml_value(value: &toml::Value) -> ConfigValue {
        match value {
            toml::Value::String(value) => {
                ConfigValue::String(value.clone())
            }

            toml::Value::Integer(value) => {
                ConfigValue::Integer(*value)
            }

            toml::Value::Float(value) => {
                ConfigValue::Float(*value)
            }

            toml::Value::Boolean(value) => {
                ConfigValue::Boolean(*value)
            }

            toml::Value::Datetime(value) => {
                ConfigValue::String(value.to_string())
            }

            toml::Value::Array(values) => {
                ConfigValue::Array(
                    values
                        .iter()
                        .map(Self::from_toml_value)
                        .collect(),
                )
            }

            toml::Value::Table(values) => {
                let mut result = BTreeMap::new();

                for (key, value) in values {
                    result.insert(
                        key.clone(),
                        Self::from_toml_value(value),
                    );
                }

                ConfigValue::Table(result)
            }
        }
    }
}
