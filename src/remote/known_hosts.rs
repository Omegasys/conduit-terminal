#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostKeyAlgorithm {
    Ed25519,
    Ecdsa,
    Rsa,
    Dsa,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostKey {
    pub algorithm: HostKeyAlgorithm,
    pub fingerprint: String,
}

impl HostKey {
    pub fn new(
        algorithm: HostKeyAlgorithm,
        fingerprint: impl Into<String>,
    ) -> Self {
        Self {
            algorithm,
            fingerprint: fingerprint.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostKeyStatus {
    Unknown,
    Trusted,
    Changed,
    Revoked,
}

#[derive(Debug, Clone)]
pub struct KnownHostsEntry {
    pub host: String,
    pub port: u16,
    pub key: HostKey,
    pub comment: Option<String>,
}

impl KnownHostsEntry {
    pub fn new(
        host: impl Into<String>,
        port: u16,
        key: HostKey,
    ) -> Self {
        Self {
            host: host.into(),
            port,
            key,
            comment: None,
        }
    }

    pub fn matches(
        &self,
        host: &str,
        port: u16,
    ) -> bool {
        self.host == host && self.port == port
    }
}

#[derive(Debug, Default)]
pub struct KnownHosts {
    entries: Vec<KnownHostsEntry>,
}

impl KnownHosts {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, entry: KnownHostsEntry) {
        self.entries
            .retain(|existing| {
                !existing.matches(&entry.host, entry.port)
            });

        self.entries.push(entry);
    }

    pub fn check(
        &self,
        host: &str,
        port: u16,
        key: &HostKey,
    ) -> HostKeyStatus {
        let entry = self
            .entries
            .iter()
            .find(|entry| entry.matches(host, port));

        match entry {
            None => HostKeyStatus::Unknown,

            Some(entry) if entry.key == *key => {
                HostKeyStatus::Trusted
            }

            Some(_) => HostKeyStatus::Changed,
        }
    }

    pub fn get(
        &self,
        host: &str,
        port: u16,
    ) -> Option<&KnownHostsEntry> {
        self.entries
            .iter()
            .find(|entry| entry.matches(host, port))
    }

    pub fn remove(
        &mut self,
        host: &str,
        port: u16,
    ) -> bool {
        let original = self.entries.len();

        self.entries
            .retain(|entry| !entry.matches(host, port));

        original != self.entries.len()
    }

    pub fn entries(&self) -> &[KnownHostsEntry] {
        &self.entries
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
