use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecurityLevel {
    Normal,
    Elevated,
    Restricted,
    SafeMode,
}

impl SecurityLevel {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Normal => "Normal",
            Self::Elevated => "Elevated",
            Self::Restricted => "Restricted",
            Self::SafeMode => "Safe Mode",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityIndicator {
    Secure,
    Notice,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityPanel {
    Overview,
    Permissions,
    Restrictions,
    Warnings,
    Profiles,
    Audit,
}

impl SecurityPanel {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Overview => "Overview",
            Self::Permissions => "Permissions",
            Self::Restrictions => "Restrictions",
            Self::Warnings => "Warnings",
            Self::Profiles => "Profiles",
            Self::Audit => "Audit",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionDisplayState {
    Allowed,
    Denied,
    Pending,
}

pub struct PermissionDisplay {
    id: String,
    name: String,
    state: PermissionDisplayState,
    source: String,
    details: Option<String>,
}

impl PermissionDisplay {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        source: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            state: PermissionDisplayState::Pending,
            source: source.into(),
            details: None,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn state(&self) -> PermissionDisplayState {
        self.state
    }

    pub fn set_state(&mut self, state: PermissionDisplayState) {
        self.state = state;
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn details(&self) -> Option<&str> {
        self.details.as_deref()
    }

    pub fn set_details(&mut self, details: Option<String>) {
        self.details = details;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestrictionDisplayState {
    Enabled,
    Disabled,
}

pub struct RestrictionDisplay {
    id: String,
    name: String,
    state: RestrictionDisplayState,
    reason: Option<String>,
}

impl RestrictionDisplay {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            state: RestrictionDisplayState::Disabled,
            reason: None,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn state(&self) -> RestrictionDisplayState {
        self.state
    }

    pub fn enable(&mut self) {
        self.state = RestrictionDisplayState::Enabled;
    }

    pub fn disable(&mut self) {
        self.state = RestrictionDisplayState::Disabled;
    }

    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    pub fn set_reason(&mut self, reason: Option<String>) {
        self.reason = reason;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityWarningSeverity {
    Information,
    Warning,
    High,
    Critical,
}

pub struct SecurityWarningDisplay {
    id: u64,
    severity: SecurityWarningSeverity,
    message: String,
    dismissed: bool,
}

impl SecurityWarningDisplay {
    pub fn new(
        id: u64,
        severity: SecurityWarningSeverity,
        message: impl Into<String>,
    ) -> Self {
        Self {
            id,
            severity,
            message: message.into(),
            dismissed: false,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn severity(&self) -> SecurityWarningSeverity {
        self.severity
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn dismissed(&self) -> bool {
        self.dismissed
    }

    pub fn dismiss(&mut self) {
        self.dismissed = true;
    }

    pub fn restore(&mut self) {
        self.dismissed = false;
    }
}

pub struct TuiSecurityCenter {
    visible: bool,
    active_panel: SecurityPanel,
    security_level: SecurityLevel,
    indicator: SecurityIndicator,

    permissions: BTreeMap<String, PermissionDisplay>,
    restrictions: BTreeMap<String, RestrictionDisplay>,
    warnings: Vec<SecurityWarningDisplay>,

    safe_mode: bool,
    confirmation_required: bool,
    plugin_access_restricted: bool,
    network_access_restricted: bool,
    external_commands_restricted: bool,
    configuration_writes_restricted: bool,

    next_warning_id: u64,
}

impl Default for TuiSecurityCenter {
    fn default() -> Self {
        Self::new()
    }
}

impl TuiSecurityCenter {
    pub fn new() -> Self {
        Self {
            visible: false,
            active_panel: SecurityPanel::Overview,
            security_level: SecurityLevel::Normal,
            indicator: SecurityIndicator::Secure,

            permissions: BTreeMap::new(),
            restrictions: BTreeMap::new(),
            warnings: Vec::new(),

            safe_mode: false,
            confirmation_required: false,
            plugin_access_restricted: false,
            network_access_restricted: false,
            external_commands_restricted: false,
            configuration_writes_restricted: false,

            next_warning_id: 1,
        }
    }

    pub fn open(&mut self) {
        self.visible = true;
    }

    pub fn close(&mut self) {
        self.visible = false;
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn active_panel(&self) -> SecurityPanel {
        self.active_panel
    }

    pub fn set_active_panel(&mut self, panel: SecurityPanel) {
        self.active_panel = panel;
    }

    pub fn security_level(&self) -> SecurityLevel {
        self.security_level
    }

    pub fn set_security_level(&mut self, level: SecurityLevel) {
        self.security_level = level;

        if level == SecurityLevel::SafeMode {
            self.safe_mode = true;
        }
    }

    pub fn indicator(&self) -> SecurityIndicator {
        self.indicator
    }

    pub fn set_indicator(&mut self, indicator: SecurityIndicator) {
        self.indicator = indicator;
    }

    pub fn register_permission(&mut self, permission: PermissionDisplay) {
        self.permissions
            .insert(permission.id().to_owned(), permission);
    }

    pub fn permission(&self, id: &str) -> Option<&PermissionDisplay> {
        self.permissions.get(id)
    }

    pub fn permission_mut(&mut self, id: &str) -> Option<&mut PermissionDisplay> {
        self.permissions.get_mut(id)
    }

    pub fn permissions(&self) -> impl Iterator<Item = &PermissionDisplay> {
        self.permissions.values()
    }

    pub fn pending_permissions(&self) -> usize {
        self.permissions
            .values()
            .filter(|permission| {
                permission.state() == PermissionDisplayState::Pending
            })
            .count()
    }

    pub fn register_restriction(&mut self, restriction: RestrictionDisplay) {
        self.restrictions
            .insert(restriction.id().to_owned(), restriction);
    }

    pub fn restriction(&self, id: &str) -> Option<&RestrictionDisplay> {
        self.restrictions.get(id)
    }

    pub fn restriction_mut(
        &mut self,
        id: &str,
    ) -> Option<&mut RestrictionDisplay> {
        self.restrictions.get_mut(id)
    }

    pub fn restrictions(&self) -> impl Iterator<Item = &RestrictionDisplay> {
        self.restrictions.values()
    }

    pub fn enabled_restrictions(&self) -> usize {
        self.restrictions
            .values()
            .filter(|restriction| {
                restriction.state() == RestrictionDisplayState::Enabled
            })
            .count()
    }

    pub fn add_warning(
        &mut self,
        severity: SecurityWarningSeverity,
        message: impl Into<String>,
    ) -> u64 {
        let id = self.next_warning_id;
        self.next_warning_id = self.next_warning_id.saturating_add(1);

        self.warnings
            .push(SecurityWarningDisplay::new(id, severity, message));

        self.recalculate_indicator();

        id
    }

    pub fn warning(&self, id: u64) -> Option<&SecurityWarningDisplay> {
        self.warnings.iter().find(|warning| warning.id() == id)
    }

    pub fn warning_mut(
        &mut self,
        id: u64,
    ) -> Option<&mut SecurityWarningDisplay> {
        self.warnings.iter_mut().find(|warning| warning.id() == id)
    }

    pub fn warnings(&self) -> &[SecurityWarningDisplay] {
        &self.warnings
    }

    pub fn pending_warnings(&self) -> usize {
        self.warnings
            .iter()
            .filter(|warning| !warning.dismissed())
            .count()
    }

    pub fn dismiss_warning(&mut self, id: u64) -> bool {
        if let Some(warning) = self.warning_mut(id) {
            warning.dismiss();
            self.recalculate_indicator();
            true
        } else {
            false
        }
    }

    pub fn safe_mode(&self) -> bool {
        self.safe_mode
    }

    pub fn enable_safe_mode(&mut self) {
        self.safe_mode = true;
        self.security_level = SecurityLevel::SafeMode;
        self.confirmation_required = true;
        self.plugin_access_restricted = true;
        self.network_access_restricted = true;
        self.external_commands_restricted = true;
        self.configuration_writes_restricted = true;
        self.indicator = SecurityIndicator::Warning;
    }

    pub fn disable_safe_mode(&mut self) {
        self.safe_mode = false;

        if self.security_level == SecurityLevel::SafeMode {
            self.security_level = SecurityLevel::Normal;
        }

        self.recalculate_indicator();
    }

    pub fn confirmation_required(&self) -> bool {
        self.confirmation_required
    }

    pub fn set_confirmation_required(&mut self, required: bool) {
        self.confirmation_required = required;
    }

    pub fn plugin_access_restricted(&self) -> bool {
        self.plugin_access_restricted
    }

    pub fn set_plugin_access_restricted(&mut self, restricted: bool) {
        self.plugin_access_restricted = restricted;
    }

    pub fn network_access_restricted(&self) -> bool {
        self.network_access_restricted
    }

    pub fn set_network_access_restricted(&mut self, restricted: bool) {
        self.network_access_restricted = restricted;
    }

    pub fn external_commands_restricted(&self) -> bool {
        self.external_commands_restricted
    }

    pub fn set_external_commands_restricted(&mut self, restricted: bool) {
        self.external_commands_restricted = restricted;
    }

    pub fn configuration_writes_restricted(&self) -> bool {
        self.configuration_writes_restricted
    }

    pub fn set_configuration_writes_restricted(&mut self, restricted: bool) {
        self.configuration_writes_restricted = restricted;
    }

    fn recalculate_indicator(&mut self) {
        if self.safe_mode {
            self.indicator = SecurityIndicator::Warning;
            return;
        }

        let has_critical = self.warnings.iter().any(|warning| {
            !warning.dismissed()
                && warning.severity() == SecurityWarningSeverity::Critical
        });

        let has_high = self.warnings.iter().any(|warning| {
            !warning.dismissed()
                && warning.severity() == SecurityWarningSeverity::High
        });

        if has_critical {
            self.indicator = SecurityIndicator::Critical;
        } else if has_high {
            self.indicator = SecurityIndicator::Warning;
        } else if self.pending_warnings() > 0 {
            self.indicator = SecurityIndicator::Notice;
        } else {
            self.indicator = SecurityIndicator::Secure;
        }
    }

    pub fn clear_warnings(&mut self) {
        self.warnings.clear();
        self.recalculate_indicator();
    }

    pub fn clear(&mut self) {
        self.permissions.clear();
        self.restrictions.clear();
        self.warnings.clear();
        self.safe_mode = false;
        self.security_level = SecurityLevel::Normal;
        self.indicator = SecurityIndicator::Secure;
    }
}
