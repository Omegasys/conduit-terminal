use std::collections::BTreeMap;

use super::audit::SecurityAuditLog;
use super::permissions::SecurityPermissionManager;
use super::profiles::SecurityProfileManager;
use super::restrictions::RestrictionManager;
use super::warnings::SecurityWarningManager;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecurityHealthLevel {
    Secure,
    Caution,
    Restricted,
    AtRisk,
}

#[derive(Debug, Clone)]
pub struct SecurityHealth {
    level: SecurityHealthLevel,
    score: u8,
    reasons: Vec<String>,
}

impl SecurityHealth {
    pub fn new(level: SecurityHealthLevel, score: u8) -> Self {
        Self {
            level,
            score: score.min(100),
            reasons: Vec::new(),
        }
    }

    pub fn level(&self) -> SecurityHealthLevel {
        self.level
    }

    pub fn score(&self) -> u8 {
        self.score
    }

    pub fn reasons(&self) -> &[String] {
        &self.reasons
    }

    pub fn add_reason(&mut self, reason: impl Into<String>) {
        self.reasons.push(reason.into());
    }
}

impl Default for SecurityHealth {
    fn default() -> Self {
        Self::new(SecurityHealthLevel::Secure, 100)
    }
}

#[derive(Debug, Clone)]
pub struct SecuritySummary {
    pub health: SecurityHealth,
    pub active_profile: Option<String>,
    pub active_restrictions: usize,
    pub pending_warnings: usize,
    pub permission_requests: usize,
    pub audit_entries: usize,
}

impl Default for SecuritySummary {
    fn default() -> Self {
        Self {
            health: SecurityHealth::default(),
            active_profile: None,
            active_restrictions: 0,
            pending_warnings: 0,
            permission_requests: 0,
            audit_entries: 0,
        }
    }
}

#[derive(Debug, Default)]
pub struct SecurityDashboard {
    summary: SecuritySummary,
    metrics: BTreeMap<String, u64>,
}

impl SecurityDashboard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn summary(&self) -> &SecuritySummary {
        &self.summary
    }

    pub fn health(&self) -> &SecurityHealth {
        &self.summary.health
    }

    pub fn metrics(&self) -> &BTreeMap<String, u64> {
        &self.metrics
    }

    pub fn set_metric(&mut self, name: impl Into<String>, value: u64) {
        self.metrics.insert(name.into(), value);
    }

    pub fn metric(&self, name: &str) -> Option<u64> {
        self.metrics.get(name).copied()
    }

    pub fn refresh(
        &mut self,
        permissions: &SecurityPermissionManager,
        restrictions: &RestrictionManager,
        warnings: &SecurityWarningManager,
        profiles: &SecurityProfileManager,
        audit: &SecurityAuditLog,
    ) {
        self.summary.active_profile = profiles
            .active()
            .map(|profile| profile.name().to_string());

        self.summary.active_restrictions = restrictions.active_count();
        self.summary.pending_warnings = warnings.pending_count();
        self.summary.permission_requests = permissions.pending_count();
        self.summary.audit_entries = audit.len();

        self.summary.health = self.calculate_health(
            restrictions,
            warnings,
            permissions,
        );

        self.metrics
            .insert("active_restrictions".into(), restrictions.active_count() as u64);
        self.metrics
            .insert("pending_warnings".into(), warnings.pending_count() as u64);
        self.metrics
            .insert("permission_requests".into(), permissions.pending_count() as u64);
        self.metrics
            .insert("audit_entries".into(), audit.len() as u64);
    }

    fn calculate_health(
        &self,
        restrictions: &RestrictionManager,
        warnings: &SecurityWarningManager,
        permissions: &SecurityPermissionManager,
    ) -> SecurityHealth {
        let mut score = 100u8;
        let mut reasons = Vec::new();

        let restrictions_count = restrictions.active_count();
        let warnings_count = warnings.pending_count();
        let permission_count = permissions.pending_count();

        score = score.saturating_sub((restrictions_count.min(5) * 5) as u8);
        score = score.saturating_sub((warnings_count.min(5) * 8) as u8);
        score = score.saturating_sub((permission_count.min(5) * 3) as u8);

        if restrictions_count > 0 {
            reasons.push(format!(
                "{} security restriction(s) are active",
                restrictions_count
            ));
        }

        if warnings_count > 0 {
            reasons.push(format!(
                "{} security warning(s) require attention",
                warnings_count
            ));
        }

        if permission_count > 0 {
            reasons.push(format!(
                "{} permission request(s) are pending",
                permission_count
            ));
        }

        let level = if score >= 85 {
            SecurityHealthLevel::Secure
        } else if score >= 65 {
            SecurityHealthLevel::Caution
        } else if score >= 40 {
            SecurityHealthLevel::Restricted
        } else {
            SecurityHealthLevel::AtRisk
        };

        let mut health = SecurityHealth::new(level, score);

        for reason in reasons {
            health.add_reason(reason);
        }

        health
    }
}
