use super::validation::{
    ConfigDiagnostic,
    ConfigDiagnosticLevel,
};

/// Collection of diagnostics displayed by the configuration editor.
#[derive(Debug, Clone, Default)]
pub struct DiagnosticCollection {
    diagnostics: Vec<ConfigDiagnostic>,
}

impl DiagnosticCollection {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    pub fn add(&mut self, diagnostic: ConfigDiagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn extend<I>(&mut self, diagnostics: I)
    where
        I: IntoIterator<Item = ConfigDiagnostic>,
    {
        self.diagnostics.extend(diagnostics);
    }

    pub fn remove(&mut self, index: usize) -> Option<ConfigDiagnostic> {
        if index < self.diagnostics.len() {
            Some(self.diagnostics.remove(index))
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.diagnostics.clear();
    }

    pub fn all(&self) -> &[ConfigDiagnostic] {
        &self.diagnostics
    }

    pub fn errors(&self) -> impl Iterator<Item = &ConfigDiagnostic> {
        self.diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.level == ConfigDiagnosticLevel::Error)
    }

    pub fn warnings(&self) -> impl Iterator<Item = &ConfigDiagnostic> {
        self.diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.level == ConfigDiagnosticLevel::Warning)
    }

    pub fn hints(&self) -> impl Iterator<Item = &ConfigDiagnostic> {
        self.diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.level == ConfigDiagnosticLevel::Hint)
    }

    pub fn information(&self) -> impl Iterator<Item = &ConfigDiagnostic> {
        self.diagnostics
            .iter()
            .filter(|diagnostic| {
                diagnostic.level == ConfigDiagnosticLevel::Information
            })
    }

    pub fn error_count(&self) -> usize {
        self.errors().count()
    }

    pub fn warning_count(&self) -> usize {
        self.warnings().count()
    }

    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }

    pub fn has_errors(&self) -> bool {
        self.error_count() > 0
    }

    pub fn has_warnings(&self) -> bool {
        self.warning_count() > 0
    }
}

/// Controls how diagnostics are displayed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticDisplayMode {
    Inline,
    Panel,
    Both,
    Hidden,
}

/// Diagnostics UI state.
#[derive(Debug, Clone)]
pub struct DiagnosticsSettings {
    display_mode: DiagnosticDisplayMode,
    show_hints: bool,
    show_information: bool,
    show_warnings: bool,
    show_errors: bool,
    underline_errors: bool,
    underline_warnings: bool,
    auto_scroll_to_errors: bool,
    maximum_visible: usize,
}

impl Default for DiagnosticsSettings {
    fn default() -> Self {
        Self {
            display_mode: DiagnosticDisplayMode::Both,
            show_hints: true,
            show_information: true,
            show_warnings: true,
            show_errors: true,
            underline_errors: true,
            underline_warnings: true,
            auto_scroll_to_errors: true,
            maximum_visible: 1000,
        }
    }
}

impl DiagnosticsSettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn display_mode(&self) -> DiagnosticDisplayMode {
        self.display_mode
    }

    pub fn show_hints(&self) -> bool {
        self.show_hints
    }

    pub fn show_information(&self) -> bool {
        self.show_information
    }

    pub fn show_warnings(&self) -> bool {
        self.show_warnings
    }

    pub fn show_errors(&self) -> bool {
        self.show_errors
    }

    pub fn underline_errors(&self) -> bool {
        self.underline_errors
    }

    pub fn underline_warnings(&self) -> bool {
        self.underline_warnings
    }

    pub fn auto_scroll_to_errors(&self) -> bool {
        self.auto_scroll_to_errors
    }

    pub fn maximum_visible(&self) -> usize {
        self.maximum_visible
    }

    pub fn set_display_mode(&mut self, mode: DiagnosticDisplayMode) {
        self.display_mode = mode;
    }

    pub fn set_show_hints(&mut self, value: bool) {
        self.show_hints = value;
    }

    pub fn set_show_information(&mut self, value: bool) {
        self.show_information = value;
    }

    pub fn set_show_warnings(&mut self, value: bool) {
        self.show_warnings = value;
    }

    pub fn set_show_errors(&mut self, value: bool) {
        self.show_errors = value;
    }

    pub fn set_underline_errors(&mut self, value: bool) {
        self.underline_errors = value;
    }

    pub fn set_underline_warnings(&mut self, value: bool) {
        self.underline_warnings = value;
    }

    pub fn set_auto_scroll_to_errors(&mut self, value: bool) {
        self.auto_scroll_to_errors = value;
    }

    pub fn set_maximum_visible(&mut self, value: usize) {
        self.maximum_visible = value.clamp(10, 10_000);
    }
}
