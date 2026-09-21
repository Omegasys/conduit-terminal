#[derive(Debug, Clone, Default)]
pub struct PrimarySelection {
    text: String,
}

impl PrimarySelection {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    pub fn clear(&mut self) {
        self.text.clear();
    }
}

#[derive(Debug, Clone, Default)]
pub struct PrimarySelectionManager {
    selection: Option<PrimarySelection>,
}

impl PrimarySelectionManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, text: impl Into<String>) {
        self.selection = Some(PrimarySelection::new(text));
    }

    pub fn get(&self) -> Option<&PrimarySelection> {
        self.selection.as_ref()
    }

    pub fn text(&self) -> Option<&str> {
        self.selection.as_ref().map(PrimarySelection::text)
    }

    pub fn clear(&mut self) {
        self.selection = None;
    }

    pub fn is_empty(&self) -> bool {
        self.selection
            .as_ref()
            .map(PrimarySelection::is_empty)
            .unwrap_or(true)
    }
}
