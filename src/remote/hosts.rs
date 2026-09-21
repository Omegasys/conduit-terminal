use std::net::IpAddr;

#[derive(Debug, Clone)]
pub enum HostAddress {
    Hostname(String),
    Ip(IpAddr),
}

impl HostAddress {
    pub fn as_string(&self) -> String {
        match self {
            Self::Hostname(host) => host.clone(),
            Self::Ip(address) => address.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Host {
    pub name: String,
    pub address: HostAddress,
    pub port: u16,
    pub username: Option<String>,
    pub aliases: Vec<String>,
}

impl Host {
    pub fn new(
        name: impl Into<String>,
        address: HostAddress,
    ) -> Self {
        Self {
            name: name.into(),
            address,
            port: 22,
            username: None,
            aliases: Vec::new(),
        }
    }

    pub fn endpoint(&self) -> String {
        format!(
            "{}:{}",
            self.address.as_string(),
            self.port
        )
    }

    pub fn matches(&self, value: &str) -> bool {
        self.name == value
            || self.address.as_string() == value
            || self.aliases.iter().any(|alias| alias == value)
    }
}

#[derive(Debug, Default)]
pub struct HostManager {
    hosts: Vec<Host>,
}

impl HostManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, host: Host) {
        self.remove(&host.name);
        self.hosts.push(host);
    }

    pub fn get(&self, name: &str) -> Option<&Host> {
        self.hosts.iter().find(|host| host.matches(name))
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Host> {
        self.hosts.iter_mut().find(|host| host.matches(name))
    }

    pub fn remove(&mut self, name: &str) -> bool {
        if let Some(index) = self
            .hosts
            .iter()
            .position(|host| host.matches(name))
        {
            self.hosts.remove(index);
            true
        } else {
            false
        }
    }

    pub fn hosts(&self) -> &[Host] {
        &self.hosts
    }

    pub fn clear(&mut self) {
        self.hosts.clear();
    }
}
