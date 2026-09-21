use std::time::Duration;

use super::authentication::{
    AuthenticationMethod,
    AuthenticationResult,
};

use super::known_hosts::{
    HostKey,
    HostKeyStatus,
    KnownHosts,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SshState {
    Disconnected,
    Connecting,
    Authenticating,
    Connected,
    Reconnecting,
    Disconnecting,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SshHostKeyPolicy {
    Strict,
    AcceptNew,
    AcceptAny,
}

#[derive(Debug, Clone)]
pub struct SshConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub timeout: Duration,
    pub keepalive_interval: Option<Duration>,
    pub host_key_policy: SshHostKeyPolicy,
    pub authentication: AuthenticationMethod,
}

impl Default for SshConfig {
    fn default() -> Self {
        Self {
            host: String::new(),
            port: 22,
            username: String::new(),
            timeout: Duration::from_secs(10),
            keepalive_interval: Some(Duration::from_secs(30)),
            host_key_policy: SshHostKeyPolicy::Strict,
            authentication: AuthenticationMethod::Agent,
        }
    }
}

impl SshConfig {
    pub fn new(host: impl Into<String>, username: impl Into<String>) -> Self {
        Self {
            host: host.into(),
            username: username.into(),
            ..Self::default()
        }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Debug)]
pub struct SshConnection {
    config: SshConfig,
    state: SshState,
    server_key: Option<HostKey>,
    authenticated: bool,
}

impl SshConnection {
    pub fn new(config: SshConfig) -> Self {
        Self {
            config,
            state: SshState::Disconnected,
            server_key: None,
            authenticated: false,
        }
    }

    pub fn config(&self) -> &SshConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut SshConfig {
        &mut self.config
    }

    pub fn state(&self) -> SshState {
        self.state
    }

    pub fn is_connected(&self) -> bool {
        self.state == SshState::Connected
    }

    pub fn is_authenticated(&self) -> bool {
        self.authenticated
    }

    pub fn server_key(&self) -> Option<&HostKey> {
        self.server_key.as_ref()
    }

    pub fn verify_host(
        &self,
        known_hosts: &KnownHosts,
        key: &HostKey,
    ) -> HostKeyStatus {
        known_hosts.check(&self.config.host, self.config.port, key)
    }

    pub fn begin_connect(&mut self) {
        self.state = SshState::Connecting;
    }

    pub fn begin_authentication(&mut self) {
        self.state = SshState::Authenticating;
    }

    pub fn complete_authentication(
        &mut self,
        result: AuthenticationResult,
    ) -> bool {
        if result.success {
            self.authenticated = true;
            self.state = SshState::Connected;
            true
        } else {
            self.state = SshState::Failed;
            false
        }
    }

    pub fn set_server_key(&mut self, key: HostKey) {
        self.server_key = Some(key);
    }

    pub fn disconnect(&mut self) {
        self.state = SshState::Disconnecting;
        self.authenticated = false;
        self.state = SshState::Disconnected;
    }

    pub fn reconnect(&mut self) {
        self.authenticated = false;
        self.state = SshState::Reconnecting;
    }
}
