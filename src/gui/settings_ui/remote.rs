use std::collections::BTreeMap;

use crate::config_engine::ConfigValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemoteProtocol {
    Ssh,
    Telnet,
    Serial,
    Local,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemoteSecurity {
    Normal,
    Strict,
    Restricted,
}

#[derive(Debug, Clone)]
pub struct RemoteSettings {
    default_protocol: RemoteProtocol,
    security: RemoteSecurity,
    connection_timeout_seconds: u64,
    keepalive_enabled: bool,
    keepalive_interval_seconds: u64,
    reconnect_enabled: bool,
    reconnect_attempts: u32,
    reconnect_delay_seconds: u64,
    remember_hosts: bool,
    remember_credentials: bool,
    allow_agent_forwarding: bool,
    allow_x11_forwarding: bool,
    allow_port_forwarding: bool,
    verify_host_keys: bool,
    accept_new_host_keys: bool,
    compression: bool,
    connection_logging: bool,
}

impl Default for RemoteSettings {
    fn default() -> Self {
        Self {
            default_protocol: RemoteProtocol::Ssh,
            security: RemoteSecurity::Strict,
            connection_timeout_seconds: 15,
            keepalive_enabled: true,
            keepalive_interval_seconds: 30,
            reconnect_enabled: true,
            reconnect_attempts: 3,
            reconnect_delay_seconds: 5,
            remember_hosts: true,
            remember_credentials: false,
            allow_agent_forwarding: false,
            allow_x11_forwarding: false,
            allow_port_forwarding: false,
            verify_host_keys: true,
            accept_new_host_keys: false,
            compression: false,
            connection_logging: false,
        }
    }
}

impl RemoteSettings {
    pub fn default_protocol(&self) -> RemoteProtocol {
        self.default_protocol
    }

    pub fn set_default_protocol(&mut self, value: RemoteProtocol) {
        self.default_protocol = value;
    }

    pub fn security(&self) -> RemoteSecurity {
        self.security
    }

    pub fn set_security(&mut self, value: RemoteSecurity) {
        self.security = value;
    }

    pub fn connection_timeout_seconds(&self) -> u64 {
        self.connection_timeout_seconds
    }

    pub fn set_connection_timeout_seconds(&mut self, value: u64) {
        self.connection_timeout_seconds = value.clamp(1, 3600);
    }

    pub fn keepalive_enabled(&self) -> bool {
        self.keepalive_enabled
    }

    pub fn set_keepalive_enabled(&mut self, value: bool) {
        self.keepalive_enabled = value;
    }

    pub fn keepalive_interval_seconds(&self) -> u64 {
        self.keepalive_interval_seconds
    }

    pub fn set_keepalive_interval_seconds(&mut self, value: u64) {
        self.keepalive_interval_seconds = value.clamp(1, 3600);
    }

    pub fn reconnect_enabled(&self) -> bool {
        self.reconnect_enabled
    }

    pub fn set_reconnect_enabled(&mut self, value: bool) {
        self.reconnect_enabled = value;
    }

    pub fn reconnect_attempts(&self) -> u32 {
        self.reconnect_attempts
    }

    pub fn set_reconnect_attempts(&mut self, value: u32) {
        self.reconnect_attempts = value.min(100);
    }

    pub fn reconnect_delay_seconds(&self) -> u64 {
        self.reconnect_delay_seconds
    }

    pub fn set_reconnect_delay_seconds(&mut self, value: u64) {
        self.reconnect_delay_seconds = value.clamp(1, 300);
    }

    pub fn remember_hosts(&self) -> bool {
        self.remember_hosts
    }

    pub fn set_remember_hosts(&mut self, value: bool) {
        self.remember_hosts = value;
    }

    pub fn remember_credentials(&self) -> bool {
        self.remember_credentials
    }

    pub fn set_remember_credentials(&mut self, value: bool) {
        self.remember_credentials = value;
    }

    pub fn allow_agent_forwarding(&self) -> bool {
        self.allow_agent_forwarding
    }

    pub fn set_allow_agent_forwarding(&mut self, value: bool) {
        self.allow_agent_forwarding = value;
    }

    pub fn allow_x11_forwarding(&self) -> bool {
        self.allow_x11_forwarding
    }

    pub fn set_allow_x11_forwarding(&mut self, value: bool) {
        self.allow_x11_forwarding = value;
    }

    pub fn allow_port_forwarding(&self) -> bool {
        self.allow_port_forwarding
    }

    pub fn set_allow_port_forwarding(&mut self, value: bool) {
        self.allow_port_forwarding = value;
    }

    pub fn verify_host_keys(&self) -> bool {
        self.verify_host_keys
    }

    pub fn set_verify_host_keys(&mut self, value: bool) {
        self.verify_host_keys = value;
    }

    pub fn accept_new_host_keys(&self) -> bool {
        self.accept_new_host_keys
    }

    pub fn set_accept_new_host_keys(&mut self, value: bool) {
        self.accept_new_host_keys = value;
    }

    pub fn compression(&self) -> bool {
        self.compression
    }

    pub fn set_compression(&mut self, value: bool) {
        self.compression = value;
    }

    pub fn connection_logging(&self) -> bool {
        self.connection_logging
    }

    pub fn set_connection_logging(&mut self, value: bool) {
        self.connection_logging = value;
    }

    pub fn to_config(&self) -> BTreeMap<String, ConfigValue> {
        let mut values = BTreeMap::new();

        values.insert(
            "default_protocol".into(),
            ConfigValue::String(format!("{:?}", self.default_protocol).to_lowercase()),
        );
        values.insert(
            "security".into(),
            ConfigValue::String(format!("{:?}", self.security).to_lowercase()),
        );
        values.insert(
            "connection_timeout_seconds".into(),
            ConfigValue::Integer(self.connection_timeout_seconds as i64),
        );
        values.insert(
            "keepalive_enabled".into(),
            ConfigValue::Boolean(self.keepalive_enabled),
        );
        values.insert(
            "keepalive_interval_seconds".into(),
            ConfigValue::Integer(self.keepalive_interval_seconds as i64),
        );
        values.insert(
            "reconnect_enabled".into(),
            ConfigValue::Boolean(self.reconnect_enabled),
        );
        values.insert(
            "reconnect_attempts".into(),
            ConfigValue::Integer(self.reconnect_attempts as i64),
        );
        values.insert(
            "reconnect_delay_seconds".into(),
            ConfigValue::Integer(self.reconnect_delay_seconds as i64),
        );
        values.insert(
            "remember_hosts".into(),
            ConfigValue::Boolean(self.remember_hosts),
        );
        values.insert(
            "remember_credentials".into(),
            ConfigValue::Boolean(self.remember_credentials),
        );
        values.insert(
            "allow_agent_forwarding".into(),
            ConfigValue::Boolean(self.allow_agent_forwarding),
        );
        values.insert(
            "allow_x11_forwarding".into(),
            ConfigValue::Boolean(self.allow_x11_forwarding),
        );
        values.insert(
            "allow_port_forwarding".into(),
            ConfigValue::Boolean(self.allow_port_forwarding),
        );
        values.insert(
            "verify_host_keys".into(),
            ConfigValue::Boolean(self.verify_host_keys),
        );
        values.insert(
            "accept_new_host_keys".into(),
            ConfigValue::Boolean(self.accept_new_host_keys),
        );
        values.insert("compression".into(), ConfigValue::Boolean(self.compression));
        values.insert(
            "connection_logging".into(),
            ConfigValue::Boolean(self.connection_logging),
        );

        values
    }
}
