#[derive(Debug, Clone, Default)]
pub struct ConfigurationDiagnostics {
    loaded: bool,
    valid: bool,
    source: Option<String>,
    profile: Option<String>,
    workspace: Option<String>,
    warnings: Vec<String>,
    errors: Vec<String>,
}

impl ConfigurationDiagnostics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_loaded(&mut self, loaded: bool) {
        self.loaded = loaded;
    }

    pub fn set_valid(&mut self, valid: bool) {
        self.valid = valid;
    }

    pub fn set_source(&mut self, source: impl Into<String>) {
        self.source = Some(source.into());
    }

    pub fn set_profile(&mut self, profile: impl Into<String>) {
        self.profile = Some(profile.into());
    }

    pub fn set_workspace(&mut self, workspace: impl Into<String>) {
        self.workspace = Some(workspace.into());
    }

    pub fn add_warning(&mut self, warning: impl Into<String>) {
        self.warnings.push(warning.into());
    }

    pub fn add_error(&mut self, error: impl Into<String>) {
        self.errors.push(error.into());
        self.valid = false;
    }

    pub fn loaded(&self) -> bool {
        self.loaded
    }

    pub fn valid(&self) -> bool {
        self.valid
    }

    pub fn source(&self) -> Option<&str> {
        self.source.as_deref()
    }

    pub fn profile(&self) -> Option<&str> {
        self.profile.as_deref()
    }

    pub fn workspace(&self) -> Option<&str> {
        self.workspace.as_deref()
    }

    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }

    pub fn errors(&self) -> &[String] {
        &self.errors
    }
}
