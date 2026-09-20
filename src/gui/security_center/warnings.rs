use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WarningSeverity {
    Information,
    Warning,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WarningAction {
    Dismiss,
    Allow,
    Deny,
    Review,
}

#[derive(Debug, Clone)]
pub struct SecurityWarning {
    id: u64,
    severity: WarningSeverity,
    title: String,
    message: String,
    source: Option<String>,
    action: WarningAction,
    acknowledged: bool,
}

impl SecurityWarning {
    pub fn new(
        id: u64,
        severity: WarningSeverity,
        title: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            id,
            severity,
            title: title.into(),
            message: message.into(),
            source: None,
            action: WarningAction::Review,
            acknowledged: false,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn severity(&self) -> WarningSeverity {
        self.severity
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn source(&self) -> Option<&str> {
        self.source.as_deref()
    }

    pub fn action(&self) -> WarningAction {
        self.action
    }

    pub fn acknowledged(&self) -> bool {
        self.acknowledged
    }

    pub fn set_source(&mut self, source: impl Into<String>) {
        self.source = Some(source.into());
    }

    pub fn set_action(&mut self, action: WarningAction) {
        self.action = action;
    }

    pub fn acknowledge(&mut self) {
        self.acknowledged = true;
    }

    pub fn dismiss(&mut self) {
        self.acknowledged = true;
        self.action = WarningAction::Dismiss;
    }
}

#[derive(Debug)]
pub struct SecurityWarningManager {
    warnings: VecDeque<SecurityWarning>,
    next_id: u64,
    maximum: usize,
}

impl Default for SecurityWarningManager {
    fn default() -> Self {
        Self {
            warnings: VecDeque::new(),
            next_id: 1,
            maximum: 1000,
        }
    }
}

impl SecurityWarningManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(
        &mut self,
        severity: WarningSeverity,
        title: impl Into<String>,
        message: impl Into<String>,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        self.warnings
            .push_back(SecurityWarning::new(id, severity, title, message));

        while self.warnings.len() > self.maximum {
            self.warnings.pop_front();
        }

        id
    }

    pub fn get(&self, id: u64) -> Option<&SecurityWarning> {
        self.warnings.iter().find(|warning| warning.id() == id)
    }

    pub fn get_mut(&mut self, id: u64) -> Option<&mut SecurityWarning> {
        self.warnings
            .iter_mut()
            .find(|warning| warning.id() == id)
    }

    pub fn acknowledge(&mut self, id: u64) -> bool {
        let Some(warning) = self.get_mut(id) else {
            return false;
        };

        warning.acknowledge();
        true
    }

    pub fn dismiss(&mut self, id: u64) -> bool {
        let Some(warning) = self.get_mut(id) else {
            return false;
        };

        warning.dismiss();
        true
    }

    pub fn all(&self) -> impl Iterator<Item = &SecurityWarning> {
        self.warnings.iter()
    }

    pub fn pending(&self) -> impl Iterator<Item = &SecurityWarning> {
        self.warnings
            .iter()
            .filter(|warning| !warning.acknowledged())
    }

    pub fn pending_count(&self) -> usize {
        self.warnings
            .iter()
            .filter(|warning| !warning.acknowledged())
            .count()
    }

    pub fn len(&self) -> usize {
        self.warnings.len()
    }

    pub fn is_empty(&self) -> bool {
        self.warnings.is_empty()
    }

    pub fn set_maximum(&mut self, maximum: usize) {
        self.maximum = maximum.max(1);

        while self.warnings.len() > self.maximum {
            self.warnings.pop_front();
        }
    }

    pub fn maximum(&self) -> usize {
        self.maximum
    }

    pub fn clear_acknowledged(&mut self) {
        self.warnings.retain(|warning| !warning.acknowledged());
    }

    pub fn clear(&mut self) {
        self.warnings.clear();
    }
}
