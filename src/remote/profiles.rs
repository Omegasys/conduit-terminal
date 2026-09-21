use super::authentication::AuthenticationMethod;
use super::ssh::SshConfig;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RemoteProfileId(String);

impl RemoteProfileId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct RemoteProfile {
    pub id: RemoteProfileId,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub authentication: AuthenticationMethod,
    pub startup_command: Option<String>,
    pub environment: Vec<(String, String)>,
}

impl RemoteProfile {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        host: impl Into<String>,
        username: impl Into<String>,
    ) -> Self {
        Self {
            id: RemoteProfileId::new(id),
            name: name.into(),
            host: host.into(),
            port: 22,
            username: username.into(),
            authentication: AuthenticationMethod::Agent,
            startup_command: None,
            environment: Vec::new(),
        }
    }

    pub fn ssh_config(&self) -> SshConfig {
        SshConfig {
            host: self.host.clone(),
            port: self.port,
            username: self.username.clone(),
            authentication: self.authentication.clone(),
            ..SshConfig::default()
        }
    }
}

#[derive(Debug, Default)]
pub struct RemoteProfileManager {
    profiles: Vec<RemoteProfile>,
}

impl RemoteProfileManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, profile: RemoteProfile) {
        self.remove(profile.id.as_str());
        self.profiles.push(profile);
    }

    pub fn get(&self, id: &str) -> Option<&RemoteProfile> {
        self.profiles
            .iter()
            .find(|profile| profile.id.as_str() == id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut RemoteProfile> {
        self.profiles
            .iter_mut()
            .find(|profile| profile.id.as_str() == id)
    }

    pub fn remove(&mut self, id: &str) -> bool {
        if let Some(index) = self
            .profiles
            .iter()
            .position(|profile| profile.id.as_str() == id)
        {
            self.profiles.remove(index);
            true
        } else {
            false
        }
    }

    pub fn profiles(&self) -> &[RemoteProfile] {
        &self.profiles
    }

    pub fn clear(&mut self) {
        self.profiles.clear();
    }
}
