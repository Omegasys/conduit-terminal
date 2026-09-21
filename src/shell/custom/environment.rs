use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EnvironmentOperation {
    Set {
        key: String,
        value: String,
    },
    Remove {
        key: String,
    },
    Clear,
}

#[derive(Clone, Debug, Default)]
pub struct CustomShellEnvironment {
    variables: BTreeMap<String, String>,
}

impl CustomShellEnvironment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_current_process() -> Self {
        Self {
            variables:
                std::env::vars().collect(),
        }
    }

    pub fn get(
        &self,
        key: &str,
    ) -> Option<&str> {
        self.variables
            .get(key)
            .map(String::as_str)
    }

    pub fn set<S1, S2>(
        &mut self,
        key: S1,
        value: S2,
    )
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        self.variables.insert(
            key.into(),
            value.into(),
        );
    }

    pub fn remove(
        &mut self,
        key: &str,
    ) {
        self.variables.remove(key);
    }

    pub fn contains(
        &self,
        key: &str,
    ) -> bool {
        self.variables.contains_key(key)
    }

    pub fn variables(
        &self,
    ) -> &BTreeMap<String, String> {
        &self.variables
    }

    pub fn apply(
        &mut self,
        operation: EnvironmentOperation,
    ) {
        match operation {
            EnvironmentOperation::Set {
                key,
                value,
            } => self.set(key, value),

            EnvironmentOperation::Remove {
                key,
            } => self.remove(&key),

            EnvironmentOperation::Clear => {
                self.variables.clear();
            }
        }
    }

    pub fn clear(&mut self) {
        self.variables.clear();
    }
}
