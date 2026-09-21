use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct CustomShellOption {
    pub name: String,
    pub description: String,
    pub default_value: String,
    pub value: Option<String>,
}

impl CustomShellOption {
    pub fn new<S1, S2, S3>(
        name: S1,
        description: S2,
        default_value: S3,
    ) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
    {
        Self {
            name: name.into(),
            description: description.into(),
            default_value: default_value.into(),
            value: None,
        }
    }

    pub fn effective_value(&self) -> &str {
        self.value
            .as_deref()
            .unwrap_or(&self.default_value)
    }
}

#[derive(Clone, Debug, Default)]
pub struct CustomShellConfiguration {
    options: BTreeMap<String, CustomShellOption>,
}

impl CustomShellConfiguration {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_option(
        &mut self,
        option: CustomShellOption,
    ) {
        self.options.insert(
            option.name.clone(),
            option,
        );
    }

    pub fn get(
        &self,
        name: &str,
    ) -> Option<&CustomShellOption> {
        self.options.get(name)
    }

    pub fn get_mut(
        &mut self,
        name: &str,
    ) -> Option<&mut CustomShellOption> {
        self.options.get_mut(name)
    }

    pub fn set<S: Into<String>>(
        &mut self,
        name: &str,
        value: S,
    ) -> bool {
        if let Some(option) = self.options.get_mut(name) {
            option.value = Some(value.into());
            true
        } else {
            false
        }
    }

    pub fn remove(&mut self, name: &str) {
        self.options.remove(name);
    }

    pub fn options(
        &self,
    ) -> &BTreeMap<String, CustomShellOption> {
        &self.options
    }

    pub fn clear(&mut self) {
        self.options.clear();
    }
}
