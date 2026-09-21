use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnvironmentChange {
    pub key: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
}

impl EnvironmentChange {
    pub fn new<S: Into<String>>(
        key: S,
        old_value: Option<String>,
        new_value: Option<String>,
    ) -> Self {
        Self {
            key: key.into(),
            old_value,
            new_value,
        }
    }

    pub fn was_added(&self) -> bool {
        self.old_value.is_none()
            && self.new_value.is_some()
    }

    pub fn was_removed(&self) -> bool {
        self.old_value.is_some()
            && self.new_value.is_none()
    }

    pub fn was_modified(&self) -> bool {
        self.old_value.is_some()
            && self.new_value.is_some()
            && self.old_value != self.new_value
    }
}

#[derive(Clone, Debug)]
pub struct EnvironmentState {
    variables: BTreeMap<String, String>,
}

impl EnvironmentState {
    pub fn new() -> Self {
        Self {
            variables: BTreeMap::new(),
        }
    }

    pub fn from_current_process() -> Self {
        Self {
            variables: std::env::vars().collect(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.variables.get(key).map(String::as_str)
    }

    pub fn set<S1, S2>(
        &mut self,
        key: S1,
        value: S2,
    ) -> EnvironmentChange
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        let key = key.into();
        let value = value.into();

        let old_value =
            self.variables.insert(key.clone(), value.clone());

        EnvironmentChange::new(
            key,
            old_value,
            Some(value),
        )
    }

    pub fn remove(
        &mut self,
        key: &str,
    ) -> Option<EnvironmentChange> {
        let old_value = self.variables.remove(key)?;

        Some(EnvironmentChange::new(
            key.to_string(),
            Some(old_value),
            None,
        ))
    }

    pub fn contains(&self, key: &str) -> bool {
        self.variables.contains_key(key)
    }

    pub fn variables(
        &self,
    ) -> &BTreeMap<String, String> {
        &self.variables
    }

    pub fn replace(
        &mut self,
        variables: BTreeMap<String, String>,
    ) -> Vec<EnvironmentChange> {
        let mut changes = Vec::new();

        for (key, value) in &variables {
            if self.get(key) != Some(value.as_str()) {
                changes.push(self.set(key.clone(), value.clone()));
            }
        }

        let existing_keys: Vec<String> =
            self.variables.keys().cloned().collect();

        for key in existing_keys {
            if !variables.contains_key(&key) {
                if let Some(change) = self.remove(&key) {
                    changes.push(change);
                }
            }
        }

        changes
    }

    pub fn clear(&mut self) {
        self.variables.clear();
    }

    pub fn len(&self) -> usize {
        self.variables.len()
    }

    pub fn is_empty(&self) -> bool {
        self.variables.is_empty()
    }
}

impl Default for EnvironmentState {
    fn default() -> Self {
        Self::from_current_process()
    }
}
