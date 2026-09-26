#[derive(Debug, Clone)]
pub struct RedactionRule {
    name: String,
    pattern: String,
    replacement: String,
}

impl RedactionRule {
    pub fn new(
        name: impl Into<String>,
        pattern: impl Into<String>,
        replacement: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            pattern: pattern.into(),
            replacement: replacement.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn pattern(&self) -> &str {
        &self.pattern
    }

    pub fn replacement(&self) -> &str {
        &self.replacement
    }
}

#[derive(Debug, Clone)]
pub struct HistoryRedactionPolicy {
    enabled: bool,
    rules: Vec<RedactionRule>,
    redact_sensitive_commands: bool,
}

impl Default for HistoryRedactionPolicy {
    fn default() -> Self {
        Self {
            enabled: true,
            rules: Vec::new(),
            redact_sensitive_commands: true,
        }
    }
}

impl HistoryRedactionPolicy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn add_rule(&mut self, rule: RedactionRule) {
        self.rules.push(rule);
    }

    pub fn remove_rule(&mut self, name: &str) {
        self.rules.retain(|rule| rule.name() != name);
    }

    pub fn rules(&self) -> &[RedactionRule] {
        &self.rules
    }

    pub fn redact_sensitive_commands(&self) -> bool {
        self.redact_sensitive_commands
    }

    pub fn set_redact_sensitive_commands(&mut self, enabled: bool) {
        self.redact_sensitive_commands = enabled;
    }

    pub fn redact(&self, command: &str) -> String {
        if !self.enabled {
            return command.to_string();
        }

        let mut result = command.to_string();

        for rule in &self.rules {
            result = result.replace(
                rule.pattern(),
                rule.replacement(),
            );
        }

        if self.redact_sensitive_commands
            && looks_sensitive(&result)
        {
            return "[REDACTED]".to_string();
        }

        result
    }
}

fn looks_sensitive(command: &str) -> bool {
    let lower = command.to_lowercase();

    let indicators = [
        "password=",
        "passwd=",
        "token=",
        "api_key=",
        "apikey=",
        "secret=",
        "authorization=",
        "bearer ",
    ];

    indicators
        .iter()
        .any(|indicator| lower.contains(indicator))
}
