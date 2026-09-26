use std::collections::BTreeMap;

#[derive(Debug, Clone, Default)]
pub struct ProtocolDiagnostics {
    enabled: BTreeMap<String, bool>,
    processed_sequences: u64,
    rejected_sequences: u64,
    malformed_sequences: u64,
    last_protocol: Option<String>,
}

impl ProtocolDiagnostics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_enabled(
        &mut self,
        protocol: impl Into<String>,
        enabled: bool,
    ) {
        self.enabled.insert(protocol.into(), enabled);
    }

    pub fn is_enabled(&self, protocol: &str) -> bool {
        self.enabled
            .get(protocol)
            .copied()
            .unwrap_or(false)
    }

    pub fn record_sequence(&mut self, protocol: impl Into<String>) {
        self.processed_sequences =
            self.processed_sequences.saturating_add(1);

        self.last_protocol = Some(protocol.into());
    }

    pub fn record_rejected(&mut self) {
        self.rejected_sequences =
            self.rejected_sequences.saturating_add(1);
    }

    pub fn record_malformed(&mut self) {
        self.malformed_sequences =
            self.malformed_sequences.saturating_add(1);
    }

    pub fn processed_sequences(&self) -> u64 {
        self.processed_sequences
    }

    pub fn rejected_sequences(&self) -> u64 {
        self.rejected_sequences
    }

    pub fn malformed_sequences(&self) -> u64 {
        self.malformed_sequences
    }

    pub fn last_protocol(&self) -> Option<&str> {
        self.last_protocol.as_deref()
    }

    pub fn protocols(
        &self,
    ) -> impl Iterator<Item = (&String, &bool)> {
        self.enabled.iter()
    }
}
