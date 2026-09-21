use std::path::PathBuf;

use super::permissions::SecurityPermission;
use super::sandbox::{SandboxPolicy, SandboxViolation};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HyperlinkTarget {
    Http(String),
    Https(String),
    File(PathBuf),
    Mailto(String),
    Custom(String),
}

impl HyperlinkTarget {
    pub fn parse(value: &str) -> Self {
        let lower = value.to_ascii_lowercase();

        if lower.starts_with("https://") {
            Self::Https(value.to_string())
        } else if lower.starts_with("http://") {
            Self::Http(value.to_string())
        } else if lower.starts_with("file://") {
            Self::File(PathBuf::from(
                value.trim_start_matches("file://"),
            ))
        } else if lower.starts_with("mailto:") {
            Self::Mailto(value.to_string())
        } else {
            Self::Custom(value.to_string())
        }
    }

    pub fn is_network(&self) -> bool {
        matches!(self, Self::Http(_) | Self::Https(_))
    }

    pub fn is_file(&self) -> bool {
        matches!(self, Self::File(_))
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Http(value)
            | Self::Https(value)
            | Self::Mailto(value)
            | Self::Custom(value) => value,
            Self::File(path) => path.to_str().unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HyperlinkAction {
    Allow,
    Ask,
    Block,
}

#[derive(Debug, Clone)]
pub struct HyperlinkSecurityPolicy {
    allow_http: bool,
    allow_https: bool,
    allow_file: bool,
    allow_mailto: bool,
    allow_custom: bool,
    require_confirmation: bool,
}

impl Default for HyperlinkSecurityPolicy {
    fn default() -> Self {
        Self {
            allow_http: true,
            allow_https: true,
            allow_file: false,
            allow_mailto: true,
            allow_custom: false,
            require_confirmation: true,
        }
    }
}

impl HyperlinkSecurityPolicy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_allow_http(&mut self, value: bool) {
        self.allow_http = value;
    }

    pub fn set_allow_https(&mut self, value: bool) {
        self.allow_https = value;
    }

    pub fn set_allow_file(&mut self, value: bool) {
        self.allow_file = value;
    }

    pub fn set_allow_mailto(&mut self, value: bool) {
        self.allow_mailto = value;
    }

    pub fn set_allow_custom(&mut self, value: bool) {
        self.allow_custom = value;
    }

    pub fn set_require_confirmation(&mut self, value: bool) {
        self.require_confirmation = value;
    }

    pub fn check(
        &self,
        target: &HyperlinkTarget,
        sandbox: &SandboxPolicy,
    ) -> Result<HyperlinkAction, SandboxViolation> {
        match target {
            HyperlinkTarget::Http(_) => {
                sandbox.can(SecurityPermission::OpenHyperlink)?;

                if !self.allow_http {
                    return Ok(HyperlinkAction::Block);
                }
            }

            HyperlinkTarget::Https(_) => {
                sandbox.can(SecurityPermission::OpenHyperlink)?;

                if !self.allow_https {
                    return Ok(HyperlinkAction::Block);
                }
            }

            HyperlinkTarget::File(path) => {
                sandbox.can(SecurityPermission::OpenFile)?;

                if !self.allow_file {
                    return Ok(HyperlinkAction::Block);
                }

                sandbox.can_read_path(path)?;
            }

            HyperlinkTarget::Mailto(_) => {
                sandbox.can(SecurityPermission::OpenHyperlink)?;

                if !self.allow_mailto {
                    return Ok(HyperlinkAction::Block);
                }
            }

            HyperlinkTarget::Custom(_) => {
                sandbox.can(SecurityPermission::OpenHyperlink)?;

                if !self.allow_custom {
                    return Ok(HyperlinkAction::Block);
                }
            }
        }

        if self.require_confirmation {
            Ok(HyperlinkAction::Ask)
        } else {
            Ok(HyperlinkAction::Allow)
        }
    }
}

#[derive(Debug, Clone)]
pub struct HyperlinkSecurity {
    policy: HyperlinkSecurityPolicy,
}

impl HyperlinkSecurity {
    pub fn new(policy: HyperlinkSecurityPolicy) -> Self {
        Self { policy }
    }

    pub fn policy(&self) -> &HyperlinkSecurityPolicy {
        &self.policy
    }

    pub fn policy_mut(&mut self) -> &mut HyperlinkSecurityPolicy {
        &mut self.policy
    }

    pub fn evaluate(
        &self,
        value: &str,
        sandbox: &SandboxPolicy,
    ) -> Result<HyperlinkAction, SandboxViolation> {
        let target = HyperlinkTarget::parse(value);
        self.policy.check(&target, sandbox)
    }
}

impl Default for HyperlinkSecurity {
    fn default() -> Self {
        Self::new(HyperlinkSecurityPolicy::default())
    }
}
