#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SensitiveDataKind {
    Password,
    ApiKey,
    AccessToken,
    PrivateKey,
    Secret,
    CreditCardLike,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct SensitiveDataResult {
    pub sensitive: bool,
    pub kinds: Vec<SensitiveDataKind>,
}

impl SensitiveDataResult {
    pub fn safe() -> Self {
        Self {
            sensitive: false,
            kinds: Vec::new(),
        }
    }

    pub fn is_sensitive(&self) -> bool {
        self.sensitive
    }

    pub fn contains(&self, kind: SensitiveDataKind) -> bool {
        self.kinds.contains(&kind)
    }
}

#[derive(Debug, Clone, Default)]
pub struct SensitiveDataDetector {
    detect_private_keys: bool,
    detect_tokens: bool,
    detect_password_patterns: bool,
}

impl SensitiveDataDetector {
    pub fn new() -> Self {
        Self {
            detect_private_keys: true,
            detect_tokens: true,
            detect_password_patterns: true,
        }
    }

    pub fn detect(&self, text: &str) -> SensitiveDataResult {
        let mut kinds = Vec::new();

        if self.detect_private_keys && looks_like_private_key(text) {
            kinds.push(SensitiveDataKind::PrivateKey);
        }

        if self.detect_tokens && looks_like_token(text) {
            kinds.push(SensitiveDataKind::AccessToken);
        }

        if self.detect_password_patterns && looks_like_password(text) {
            kinds.push(SensitiveDataKind::Password);
        }

        if looks_like_api_key(text) {
            kinds.push(SensitiveDataKind::ApiKey);
        }

        SensitiveDataResult {
            sensitive: !kinds.is_empty(),
            kinds,
        }
    }

    pub fn set_detect_private_keys(&mut self, enabled: bool) {
        self.detect_private_keys = enabled;
    }

    pub fn set_detect_tokens(&mut self, enabled: bool) {
        self.detect_tokens = enabled;
    }

    pub fn set_detect_password_patterns(&mut self, enabled: bool) {
        self.detect_password_patterns = enabled;
    }
}

fn looks_like_private_key(text: &str) -> bool {
    text.contains("-----BEGIN PRIVATE KEY-----")
        || text.contains("-----BEGIN RSA PRIVATE KEY-----")
        || text.contains("-----BEGIN OPENSSH PRIVATE KEY-----")
}

fn looks_like_token(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();

    lower.contains("bearer ")
        || lower.contains("access_token=")
        || lower.contains("refresh_token=")
        || lower.contains("authorization: bearer")
}

fn looks_like_password(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();

    lower.contains("password=")
        || lower.contains("passwd=")
        || lower.contains("passphrase=")
        || lower.contains("secret=")
}

fn looks_like_api_key(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();

    lower.contains("api_key=")
        || lower.contains("apikey=")
        || lower.contains("api-key=")
}
