//! Configuration schema.
//!
//! The schema provides the common typed representation used by Conduit's
//! configuration engine. Individual subsystems may extend this structure
//! later without changing how configuration is loaded or validated.

use std::collections::BTreeMap;

/// A generic configuration value.
///
/// This is intentionally independent from TOML so the GUI, CLI, TUI,
/// plugins, and future configuration sources can use the same model.
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<ConfigValue>),
    Table(BTreeMap<String, ConfigValue>),
    Null,
}

impl ConfigValue {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_integer(&self) -> Option<i64> {
        match self {
            Self::Integer(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            Self::Float(value) => Some(*value),
            Self::Integer(value) => Some(*value as f64),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Boolean(value) => Some(*value),
            _ => None,
        }
    }

    pub fn is_table(&self) -> bool {
        matches!(self, Self::Table(_))
    }

    pub fn is_scalar(&self) -> bool {
        matches!(
            self,
            Self::String(_)
                | Self::Integer(_)
                | Self::Float(_)
                | Self::Boolean(_)
                | Self::Null
        )
    }
}

/// Describes the expected configuration structure.
#[derive(Debug, Clone)]
pub struct ConfigSchema {
    version: u32,
    fields: BTreeMap<String, SchemaField>,
}

impl ConfigSchema {
    pub fn new(version: u32) -> Self {
        Self {
            version,
            fields: BTreeMap::new(),
        }
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn add_field(
        &mut self,
        path: impl Into<String>,
        field: SchemaField,
    ) {
        self.fields.insert(
            path.into(),
            field,
        );
    }

    pub fn field(
        &self,
        path: &str,
    ) -> Option<&SchemaField> {
        self.fields.get(path)
    }

    pub fn fields(
        &self,
    ) -> &BTreeMap<String, SchemaField> {
        &self.fields
    }
}

impl Default for ConfigSchema {
    fn default() -> Self {
        let mut schema =
            Self::new(1);

        schema.add_field(
            "terminal.shell",
            SchemaField::string(),
        );

        schema.add_field(
            "terminal.scrollback",
            SchemaField::integer()
                .minimum(0),
        );

        schema.add_field(
            "terminal.cursor_style",
            SchemaField::string(),
        );

        schema.add_field(
            "window.width",
            SchemaField::integer()
                .minimum(1),
        );

        schema.add_field(
            "window.height",
            SchemaField::integer()
                .minimum(1),
        );

        schema.add_field(
            "appearance.theme",
            SchemaField::string(),
        );

        schema.add_field(
            "appearance.font_size",
            SchemaField::float()
                .minimum(1.0),
        );

        schema.add_field(
            "security.safe_mode",
            SchemaField::boolean(),
        );

        schema
    }
}

/// Expected type of a configuration field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemaType {
    String,
    Integer,
    Float,
    Boolean,
    Array,
    Table,
}

/// Schema information for one field.
#[derive(Debug, Clone)]
pub struct SchemaField {
    field_type: SchemaType,
    required: bool,
    minimum: Option<f64>,
    maximum: Option<f64>,
}

impl SchemaField {
    pub fn new(
        field_type: SchemaType,
    ) -> Self {
        Self {
            field_type,
            required: false,
            minimum: None,
            maximum: None,
        }
    }

    pub fn string() -> Self {
        Self::new(
            SchemaType::String
        )
    }

    pub fn integer() -> Self {
        Self::new(
            SchemaType::Integer
        )
    }

    pub fn float() -> Self {
        Self::new(
            SchemaType::Float
        )
    }

    pub fn boolean() -> Self {
        Self::new(
            SchemaType::Boolean
        )
    }

    pub fn array() -> Self {
        Self::new(
            SchemaType::Array
        )
    }

    pub fn table() -> Self {
        Self::new(
            SchemaType::Table
        )
    }

    pub fn required(
        mut self,
        required: bool,
    ) -> Self {
        self.required = required;
        self
    }

    pub fn minimum(
        mut self,
        value: f64,
    ) -> Self {
        self.minimum = Some(value);
        self
    }

    pub fn maximum(
        mut self,
        value: f64,
    ) -> Self {
        self.maximum = Some(value);
        self
    }

    pub fn field_type(
        &self,
    ) -> SchemaType {
        self.field_type
    }

    pub fn is_required(
        &self,
    ) -> bool {
        self.required
    }

    pub fn minimum_value(
        &self,
    ) -> Option<f64> {
        self.minimum
    }

    pub fn maximum_value(
        &self,
    ) -> Option<f64> {
        self.maximum
    }
}
