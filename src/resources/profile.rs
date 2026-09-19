use std::collections::BTreeMap;

use super::resource::{Resource, ResourceError, ResourceId};

#[derive(Debug, Clone, PartialEq)]
pub enum ProfileValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    StringList(Vec<String>),
}

impl ProfileValue {
    pub fn as_string(&self) -> Option<&str> {
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
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Self::Boolean(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_string_list(&self) -> Option<&[String]> {
        match self {
            Self::StringList(value) => Some(value),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Profile {
    id: ResourceId,
    name: String,
    description: String,
    parent: Option<String>,
    values: BTreeMap<String, ProfileValue>,
    enabled: bool,
}

impl Profile {
    pub fn new(
        id: ResourceId,
        name: impl Into<String>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            description: String::new(),
            parent: None,
            values: BTreeMap::new(),
            enabled: true,
        }
    }

    pub fn from_resource(resource: &Resource) -> Result<Self, ResourceError> {
        if resource.kind() != super::resource::ResourceKind::Profile {
            return Err(ResourceError::InvalidResource(
                "resource is not a profile".to_owned(),
            ));
        }

        Ok(Self::new(resource.id(), resource.name()))
    }

    pub fn id(&self) -> ResourceId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn parent(&self) -> Option<&str> {
        self.parent.as_deref()
    }

    pub fn values(&self) -> &BTreeMap<String, ProfileValue> {
        &self.values
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_description(&mut self, value: impl Into<String>) {
        self.description = value.into();
    }

    pub fn set_parent(&mut self, value: impl Into<String>) {
        self.parent = Some(value.into());
    }

    pub fn clear_parent(&mut self) {
        self.parent = None;
    }

    pub fn set(
        &mut self,
        key: impl Into<String>,
        value: ProfileValue,
    ) {
        self.values.insert(key.into(), value);
    }

    pub fn get(&self, key: &str) -> Option<&ProfileValue> {
        self.values.get(key)
    }

    pub fn remove(&mut self, key: &str) -> Option<ProfileValue> {
        self.values.remove(key)
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }
}
