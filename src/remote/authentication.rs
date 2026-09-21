use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthenticationMethod {
    Agent,
    Password,
    PrivateKey,
    KeyboardInteractive,
    Certificate,
    None,
}

#[derive(Debug, Clone)]
pub enum Credential {
    Password(String),
    PrivateKey {
        path: PathBuf,
        passphrase: Option<String>,
    },
    Certificate {
        path: PathBuf,
    },
    Agent,
}

impl Credential {
    pub fn method(&self) -> AuthenticationMethod {
        match self {
            Self::Password(_) => AuthenticationMethod::Password,
            Self::PrivateKey { .. } => AuthenticationMethod::PrivateKey,
            Self::Certificate { .. } => AuthenticationMethod::Certificate,
            Self::Agent => AuthenticationMethod::Agent,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AuthenticationRequest {
    pub username: String,
    pub method: AuthenticationMethod,
    pub credential: Option<Credential>,
    pub allow_interactive: bool,
}

impl AuthenticationRequest {
    pub fn new(
        username: impl Into<String>,
        method: AuthenticationMethod,
    ) -> Self {
        Self {
            username: username.into(),
            method,
            credential: None,
            allow_interactive: true,
        }
    }

    pub fn with_credential(
        mut self,
        credential: Credential,
    ) -> Self {
        self.credential = Some(credential);
        self
    }
}

#[derive(Debug, Clone)]
pub struct AuthenticationResult {
    pub success: bool,
    pub method: AuthenticationMethod,
    pub message: Option<String>,
}

impl AuthenticationResult {
    pub fn success(method: AuthenticationMethod) -> Self {
        Self {
            success: true,
            method,
            message: None,
        }
    }

    pub fn failure(
        method: AuthenticationMethod,
        message: impl Into<String>,
    ) -> Self {
        Self {
            success: false,
            method,
            message: Some(message.into()),
        }
    }
}
