//! Tab icon representation.

/// Source of a tab icon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TabIconSource {
    Default,
    Terminal,
    Shell,
    File,
    Remote,
    Custom,
}

/// Icon displayed by a tab.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TabIcon {
    source: TabIconSource,
    name: String,
    glyph: Option<String>,
}

impl TabIcon {
    pub fn new(
        source: TabIconSource,
        name: impl Into<String>,
    ) -> Self {
        Self {
            source,
            name: name.into(),
            glyph: None,
        }
    }

    pub fn source(&self) -> &TabIconSource {
        &self.source
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn glyph(&self) -> Option<&str> {
        self.glyph.as_deref()
    }

    pub fn set_glyph(
        &mut self,
        glyph: impl Into<String>,
    ) {
        self.glyph = Some(glyph.into());
    }

    pub fn clear_glyph(&mut self) {
        self.glyph = None;
    }

    pub fn custom(
        name: impl Into<String>,
    ) -> Self {
        Self::new(
            TabIconSource::Custom,
            name,
        )
    }

    pub fn terminal() -> Self {
        Self::new(
            TabIconSource::Terminal,
            "terminal",
        )
    }

    pub fn remote() -> Self {
        Self::new(
            TabIconSource::Remote,
            "remote",
        )
    }
}

impl Default for TabIcon {
    fn default() -> Self {
        Self::new(
            TabIconSource::Default,
            "terminal",
        )
    }
}
