use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PermissionKind {
    ClipboardRead,
    ClipboardWrite,
    FileRead,
    FileWrite,
    Network,
    ExternalCommand,
    ProcessSpawn,
    PluginLoad,
    TerminalControl,
    ConfigurationRead,
    ConfigurationWrite,
    UiExtension,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionDecision {
    Allow,
    Deny,
    Ask,
}

#[derive(Debug, Clone)]
pub struct PermissionRequest {
    id: u64,
    source: String,
    permission: PermissionKind,
    reason: String,
    decision: PermissionDecision,
}

impl PermissionRequest {
    pub fn new(
        id: u64,
        source: impl Into<String>,
        permission: PermissionKind,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            id,
            source: source.into(),
            permission,
            reason: reason.into(),
            decision: PermissionDecision::Ask,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn permission(&self) -> PermissionKind {
        self.permission
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn decision(&self) -> PermissionDecision {
        self.decision
    }

    pub fn set_decision(&mut self, decision: PermissionDecision) {
        self.decision = decision;
    }
}

#[derive(Debug, Default)]
pub struct SecurityPermissionManager {
    requests: BTreeMap<u64, PermissionRequest>,
    decisions: BTreeMap<(String, PermissionKind), PermissionDecision>,
    next_id: u64,
}

impl SecurityPermissionManager {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            ..Default::default()
        }
    }

    pub fn request(
        &mut self,
        source: impl Into<String>,
        permission: PermissionKind,
        reason: impl Into<String>,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let request = PermissionRequest::new(id, source, permission, reason);
        self.requests.insert(id, request);

        id
    }

    pub fn get(&self, id: u64) -> Option<&PermissionRequest> {
        self.requests.get(&id)
    }

    pub fn get_mut(&mut self, id: u64) -> Option<&mut PermissionRequest> {
        self.requests.get_mut(&id)
    }

    pub fn decide(&mut self, id: u64, decision: PermissionDecision) -> bool {
        let Some(request) = self.requests.get_mut(&id) else {
            return false;
        };

        request.set_decision(decision);

        if decision != PermissionDecision::Ask {
            self.decisions.insert(
                (request.source().to_string(), request.permission()),
                decision,
            );
        }

        true
    }

    pub fn resolve(&mut self, id: u64) -> Option<PermissionRequest> {
        self.requests.remove(&id)
    }

    pub fn decision(
        &self,
        source: &str,
        permission: PermissionKind,
    ) -> PermissionDecision {
        self.decisions
            .get(&(source.to_string(), permission))
            .copied()
            .unwrap_or(PermissionDecision::Ask)
    }

    pub fn set_decision(
        &mut self,
        source: impl Into<String>,
        permission: PermissionKind,
        decision: PermissionDecision,
    ) {
        self.decisions
            .insert((source.into(), permission), decision);
    }

    pub fn requests(&self) -> impl Iterator<Item = &PermissionRequest> {
        self.requests.values()
    }

    pub fn decisions(
        &self,
    ) -> impl Iterator<
        Item = (&(String, PermissionKind), &PermissionDecision),
    > {
        self.decisions.iter()
    }

    pub fn pending_count(&self) -> usize {
        self.requests
            .values()
            .filter(|request| request.decision() == PermissionDecision::Ask)
            .count()
    }

    pub fn clear_requests(&mut self) {
        self.requests.clear();
    }

    pub fn clear_decisions(&mut self) {
        self.decisions.clear();
    }

    pub fn clear(&mut self) {
        self.clear_requests();
        self.clear_decisions();
    }
}
