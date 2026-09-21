use std::path::{Path, PathBuf};

use super::{
    CustomShell,
    CustomShellError,
    CustomShellManifest,
    CustomShellRegistry,
};

use crate::resources::shells::{
    ShellResourceEntry,
    ShellResourceManifest,
};

pub struct CustomShellResource {
    manifest: ShellResourceManifest,
    shell: Option<Box<dyn CustomShell>>,
}

impl CustomShellResource {
    pub fn new(manifest: ShellResourceManifest) -> Self {
        Self {
            manifest,
            shell: None,
        }
    }

    pub fn manifest(&self) -> &ShellResourceManifest {
        &self.manifest
    }

    pub fn id(&self) -> &super::CustomShellId {
        self.manifest.id()
    }

    pub fn root_path(&self) -> &Path {
        self.manifest.root_path()
    }

    pub fn executable(&self) -> Option<&str> {
        self.manifest.executable()
    }

    pub fn entrypoint(&self) -> Option<&str> {
        self.manifest.entrypoint()
    }

    pub fn is_loaded(&self) -> bool {
        self.shell.is_some()
    }

    pub fn set_shell(
        &mut self,
        shell: Box<dyn CustomShell>,
    ) {
        self.shell = Some(shell);
    }

    pub fn shell(&self) -> Option<&dyn CustomShell> {
        self.shell.as_deref()
    }

    pub fn shell_mut(&mut self) -> Option<&mut (dyn CustomShell + '_)> {
        match self.shell.as_mut() {
            Some(shell) => Some(shell.as_mut()),
            None => None,
        }
    }

    pub fn unload(&mut self) {
        self.shell = None;
    }
}

pub struct CustomShellResourceManager {
    resources: std::collections::BTreeMap<
        super::CustomShellId,
        CustomShellResource,
    >,
}

impl CustomShellResourceManager {
    pub fn new() -> Self {
        Self {
            resources: std::collections::BTreeMap::new(),
        }
    }

    pub fn register(
        &mut self,
        resource: CustomShellResource,
    ) -> Result<(), CustomShellError> {
        let id = resource.id().clone();

        if self.resources.contains_key(&id) {
            return Err(CustomShellError::AlreadyRegistered(
                id.to_string(),
            ));
        }

        self.resources.insert(id, resource);

        Ok(())
    }

    pub fn unregister(
        &mut self,
        id: &super::CustomShellId,
    ) -> Option<CustomShellResource> {
        self.resources.remove(id)
    }

    pub fn get(
        &self,
        id: &super::CustomShellId,
    ) -> Option<&CustomShellResource> {
        self.resources.get(id)
    }

    pub fn get_mut(
        &mut self,
        id: &super::CustomShellId,
    ) -> Option<&mut CustomShellResource> {
        self.resources.get_mut(id)
    }

    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &CustomShellResource> {
        self.resources.values()
    }

    pub fn iter_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut CustomShellResource> {
        self.resources.values_mut()
    }

    pub fn len(&self) -> usize {
        self.resources.len()
    }

    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }

    pub fn clear(&mut self) {
        self.resources.clear();
    }
}

impl Default for CustomShellResourceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&ShellResourceEntry> for PathBuf {
    fn from(entry: &ShellResourceEntry) -> Self {
        entry.path().clone()
    }
}
