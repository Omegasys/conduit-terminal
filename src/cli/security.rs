use std::fmt;

use crate::gui::security_center::{
    PermissionDecision,
    PermissionKind,
    RestrictionKind,
    SecurityProfileId,
};

/// Actions supported by the `conduit security` command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecurityCommandAction {
    Status,
    Dashboard,
    Permissions,
    Allow,
    Deny,
    Ask,
    Restrictions,
    EnableRestriction,
    DisableRestriction,
    Profiles,
    ActivateProfile,
    DeactivateProfile,
    Audit,
    Warnings,
    SafeMode,
    EnterSafeMode,
    ExitSafeMode,
}

impl SecurityCommandAction {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Status => "status",
            Self::Dashboard => "dashboard",
            Self::Permissions => "permissions",
            Self::Allow => "allow",
            Self::Deny => "deny",
            Self::Ask => "ask",
            Self::Restrictions => "restrictions",
            Self::EnableRestriction => "enable-restriction",
            Self::DisableRestriction => "disable-restriction",
            Self::Profiles => "profiles",
            Self::ActivateProfile => "activate-profile",
            Self::DeactivateProfile => "deactivate-profile",
            Self::Audit => "audit",
            Self::Warnings => "warnings",
            Self::SafeMode => "safe-mode",
            Self::EnterSafeMode => "enter-safe-mode",
            Self::ExitSafeMode => "exit-safe-mode",
        }
    }

    pub fn from_name(value: &str) -> Option<Self> {
        match value {
            "status" => Some(Self::Status),
            "dashboard" => Some(Self::Dashboard),
            "permissions" => Some(Self::Permissions),
            "allow" => Some(Self::Allow),
            "deny" => Some(Self::Deny),
            "ask" => Some(Self::Ask),
            "restrictions" => Some(Self::Restrictions),
            "enable-restriction" => Some(Self::EnableRestriction),
            "disable-restriction" => Some(Self::DisableRestriction),
            "profiles" => Some(Self::Profiles),
            "activate-profile" => Some(Self::ActivateProfile),
            "deactivate-profile" => Some(Self::DeactivateProfile),
            "audit" => Some(Self::Audit),
            "warnings" => Some(Self::Warnings),
            "safe-mode" => Some(Self::SafeMode),
            "enter-safe-mode" => Some(Self::EnterSafeMode),
            "exit-safe-mode" => Some(Self::ExitSafeMode),
            _ => None,
        }
    }
}

impl fmt::Display for SecurityCommandAction {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

/// A security permission decision request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityDecision {
    permission: PermissionKind,
    decision: PermissionDecision,
    reason: Option<String>,
}

impl SecurityDecision {
    pub fn new(
        permission: PermissionKind,
        decision: PermissionDecision,
    ) -> Self {
        Self {
            permission,
            decision,
            reason: None,
        }
    }

    pub fn permission(&self) -> &PermissionKind {
        &self.permission
    }

    pub fn decision(&self) -> &PermissionDecision {
        &self.decision
    }

    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    pub fn set_reason<S>(&mut self, reason: S)
    where
        S: Into<String>,
    {
        self.reason = Some(reason.into());
    }
}

/// A parsed security command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityCommand {
    action: SecurityCommandAction,
    permission: Option<PermissionKind>,
    restriction: Option<RestrictionKind>,
    profile: Option<SecurityProfileId>,
    decision: Option<PermissionDecision>,
    reason: Option<String>,
}

impl SecurityCommand {
    pub fn new(action: SecurityCommandAction) -> Self {
        Self {
            action,
            permission: None,
            restriction: None,
            profile: None,
            decision: None,
            reason: None,
        }
    }

    pub fn action(&self) -> &SecurityCommandAction {
        &self.action
    }

    pub fn permission(&self) -> Option<&PermissionKind> {
        self.permission.as_ref()
    }

    pub fn restriction(&self) -> Option<&RestrictionKind> {
        self.restriction.as_ref()
    }

    pub fn profile(&self) -> Option<&SecurityProfileId> {
        self.profile.as_ref()
    }

    pub fn decision(&self) -> Option<&PermissionDecision> {
        self.decision.as_ref()
    }

    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    pub fn set_permission(&mut self, permission: PermissionKind) {
        self.permission = Some(permission);
    }

    pub fn set_restriction(&mut self, restriction: RestrictionKind) {
        self.restriction = Some(restriction);
    }

    pub fn set_profile(&mut self, profile: SecurityProfileId) {
        self.profile = Some(profile);
    }

    pub fn set_decision(&mut self, decision: PermissionDecision) {
        self.decision = Some(decision);
    }

    pub fn set_reason<S>(&mut self, reason: S)
    where
        S: Into<String>,
    {
        self.reason = Some(reason.into());
    }

    pub fn requires_permission(&self) -> bool {
        matches!(
            self.action,
            SecurityCommandAction::Allow
                | SecurityCommandAction::Deny
                | SecurityCommandAction::Ask
        )
    }

    pub fn requires_restriction(&self) -> bool {
        matches!(
            self.action,
            SecurityCommandAction::EnableRestriction
                | SecurityCommandAction::DisableRestriction
        )
    }

    pub fn requires_profile(&self) -> bool {
        matches!(
            self.action,
            SecurityCommandAction::ActivateProfile
                | SecurityCommandAction::DeactivateProfile
        )
    }
}
