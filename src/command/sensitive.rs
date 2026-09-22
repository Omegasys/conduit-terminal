#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SensitiveCommandPattern {
    name: String,
    prefixes: Vec<String>,
    keywords: Vec<String>,
}

impl SensitiveCommandPattern {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            prefixes: Vec::new(),
            keywords: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn add_prefix(&mut self, prefix: impl Into<String>) {
        self.prefixes.push(prefix.into());
    }

    pub fn add_keyword(&mut self, keyword: impl Into<String>) {
        self.keywords.push(keyword.into());
    }

    pub fn matches(&self, command: &str) -> bool {
        let command = command.to_lowercase();

        self.prefixes
            .iter()
            .any(|prefix| command.starts_with(&prefix.to_lowercase()))
            || self
                .keywords
                .iter()
                .any(|keyword| command.contains(&keyword.to_lowercase()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SensitiveCommandResult {
    sensitive: bool,
    pattern: Option<String>,
    redacted_command: String,
}

impl SensitiveCommandResult {
    pub fn safe(command: impl Into<String>) -> Self {
        Self {
            sensitive: false,
            pattern: None,
            redacted_command: command.into(),
        }
    }

    pub fn sensitive(
        pattern: impl Into<String>,
        redacted_command: impl Into<String>,
    ) -> Self {
        Self {
            sensitive: true,
            pattern: Some(pattern.into()),
            redacted_command: redacted_command.into(),
        }
    }

    pub fn is_sensitive(&self) -> bool {
        self.sensitive
    }

    pub fn pattern(&self) -> Option<&str> {
        self.pattern.as_deref()
    }

    pub fn redacted_command(&self) -> &str {
        &self.redacted_command
    }
}

#[derive(Debug, Default)]
pub struct SensitiveCommandDetector {
    patterns: Vec<SensitiveCommandPattern>,
}

impl SensitiveCommandDetector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_defaults() -> Self {
        let mut detector = Self::new();

        let mut password = SensitiveCommandPattern::new("password");
        password.add_keyword("password=");
        password.add_keyword("--password");
        detector.add_pattern(password);

        let mut token = SensitiveCommandPattern::new("token");
        token.add_keyword("token=");
        token.add_keyword("--token");
        token.add_keyword("access_token");
        detector.add_pattern(token);

        let mut secret = SensitiveCommandPattern::new("secret");
        secret.add_keyword("secret=");
        secret.add_keyword("--secret");
        detector.add_pattern(secret);

        let mut private_key = SensitiveCommandPattern::new("private-key");
        private_key.add_keyword("-----begin private key-----");
        detector.add_pattern(private_key);

        detector
    }

    pub fn add_pattern(&mut self, pattern: SensitiveCommandPattern) {
        self.patterns.push(pattern);
    }

    pub fn patterns(&self) -> &[SensitiveCommandPattern] {
        &self.patterns
    }

    pub fn detect(&self, command: &str) -> SensitiveCommandResult {
        for pattern in &self.patterns {
            if pattern.matches(command) {
                return SensitiveCommandResult::sensitive(
                    pattern.name(),
                    Self::redact(command),
                );
            }
        }

        SensitiveCommandResult::safe(command)
    }

    fn redact(command: &str) -> String {
        let mut result = Vec::new();

        for token in command.split_whitespace() {
            let lower = token.to_lowercase();

            if lower.contains("password=")
                || lower.contains("token=")
                || lower.contains("secret=")
                || lower.contains("access_token=")
            {
                if let Some((key, _)) = token.split_once('=') {
                    result.push(format!("{key}=<redacted>"));
                    continue;
                }
            }

            result.push(token.to_string());
        }

        result.join(" ")
    }
}
