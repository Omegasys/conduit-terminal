#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateChannel {
    Stable,
    Beta,
    Nightly,
}

impl Default for UpdateChannel {
    fn default() -> Self {
        Self::Stable
    }
}

#[derive(Debug, Clone)]
pub struct PluginUpdate {
    pub plugin_id: String,
    pub current_version: String,
    pub available_version: String,
    pub channel: UpdateChannel,
    pub download_url: Option<String>,
    pub release_notes: Option<String>,
}

impl PluginUpdate {
    pub fn new<S: Into<String>>(
        plugin_id: S,
        current_version: S,
        available_version: S,
    ) -> Self {
        Self {
            plugin_id: plugin_id.into(),
            current_version: current_version.into(),
            available_version: available_version.into(),
            channel: UpdateChannel::Stable,
            download_url: None,
            release_notes: None,
        }
    }

    pub fn is_newer(&self) -> bool {
        self.current_version != self.available_version
    }
}

#[derive(Debug, Default)]
pub struct PluginUpdateManager {
    updates: Vec<PluginUpdate>,
    channel: UpdateChannel,
}

impl PluginUpdateManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn channel(&self) -> UpdateChannel {
        self.channel
    }

    pub fn set_channel(
        &mut self,
        channel: UpdateChannel,
    ) {
        self.channel = channel;
    }

    pub fn add(&mut self, update: PluginUpdate) {
        self.updates.push(update);
    }

    pub fn get(
        &self,
        plugin_id: &str,
    ) -> Option<&PluginUpdate> {
        self.updates
            .iter()
            .find(|update| update.plugin_id == plugin_id)
    }

    pub fn updates(&self) -> &[PluginUpdate] {
        &self.updates
    }

    pub fn available(&self) -> impl Iterator<Item = &PluginUpdate> {
        self.updates.iter().filter(|update| update.is_newer())
    }

    pub fn remove(
        &mut self,
        plugin_id: &str,
    ) -> Option<PluginUpdate> {
        let index = self
            .updates
            .iter()
            .position(|update| update.plugin_id == plugin_id)?;

        Some(self.updates.remove(index))
    }

    pub fn clear(&mut self) {
        self.updates.clear();
    }
}
