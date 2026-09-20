use std::collections::BTreeMap;

/// Documentation for a configuration key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigDocumentation {
    key: String,
    title: String,
    description: String,
    default_value: Option<String>,
    value_type: Option<String>,
    example: Option<String>,
    notes: Vec<String>,
    related_keys: Vec<String>,
}

impl ConfigDocumentation {
    pub fn new(
        key: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            key: key.into(),
            title: title.into(),
            description: description.into(),
            default_value: None,
            value_type: None,
            example: None,
            notes: Vec::new(),
            related_keys: Vec::new(),
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn default_value(&self) -> Option<&str> {
        self.default_value.as_deref()
    }

    pub fn value_type(&self) -> Option<&str> {
        self.value_type.as_deref()
    }

    pub fn example(&self) -> Option<&str> {
        self.example.as_deref()
    }

    pub fn notes(&self) -> &[String] {
        &self.notes
    }

    pub fn related_keys(&self) -> &[String] {
        &self.related_keys
    }

    pub fn set_default_value<S: Into<String>>(&mut self, value: S) {
        self.default_value = Some(value.into());
    }

    pub fn set_value_type<S: Into<String>>(&mut self, value: S) {
        self.value_type = Some(value.into());
    }

    pub fn set_example<S: Into<String>>(&mut self, value: S) {
        self.example = Some(value.into());
    }

    pub fn add_note<S: Into<String>>(&mut self, note: S) {
        self.notes.push(note.into());
    }

    pub fn add_related_key<S: Into<String>>(&mut self, key: S) {
        let key = key.into();

        if !self.related_keys.contains(&key) {
            self.related_keys.push(key);
        }
    }
}

/// Documentation registry used by the configuration editor.
#[derive(Debug, Clone, Default)]
pub struct DocumentationRegistry {
    entries: BTreeMap<String, ConfigDocumentation>,
}

impl DocumentationRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, documentation: ConfigDocumentation) {
        self.entries
            .insert(documentation.key().to_string(), documentation);
    }

    pub fn get(&self, key: &str) -> Option<&ConfigDocumentation> {
        self.entries.get(key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut ConfigDocumentation> {
        self.entries.get_mut(key)
    }

    pub fn remove(&mut self, key: &str) -> Option<ConfigDocumentation> {
        self.entries.remove(key)
    }

    pub fn contains(&self, key: &str) -> bool {
        self.entries.contains_key(key)
    }

    pub fn search(&self, query: &str) -> Vec<&ConfigDocumentation> {
        let query = query.to_lowercase();

        self.entries
            .values()
            .filter(|entry| {
                entry.key().to_lowercase().contains(&query)
                    || entry.title().to_lowercase().contains(&query)
                    || entry.description().to_lowercase().contains(&query)
            })
            .collect()
    }

    pub fn entries(&self) -> impl Iterator<Item = &ConfigDocumentation> {
        self.entries.values()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
