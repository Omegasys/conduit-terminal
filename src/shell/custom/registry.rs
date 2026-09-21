use std::collections::HashMap;

use super::{
    errors::{
        CustomShellError,
        CustomShellResult,
    },
    shell::{
        CustomShell,
        CustomShellId,
    },
};

pub struct ShellRegistration {
    pub shell: Box<dyn CustomShell>,
    pub enabled: bool,
}

impl ShellRegistration {
    pub fn new(
        shell: Box<dyn CustomShell>,
    ) -> Self {
        Self {
            shell,
            enabled: true,
        }
    }

    pub fn id(&self) -> CustomShellId {
        self.shell.id()
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

pub struct CustomShellRegistry {
    shells:
        HashMap<CustomShellId, ShellRegistration>,
}

impl CustomShellRegistry {
    pub fn new() -> Self {
        Self {
            shells: HashMap::new(),
        }
    }

    pub fn register(
        &mut self,
        shell: Box<dyn CustomShell>,
    ) -> CustomShellResult<()> {
        let id = shell.id();

        if self.shells.contains_key(&id) {
            return Err(
                CustomShellError::AlreadyRegistered(
                    id.to_string(),
                ),
            );
        }

        self.shells.insert(
            id,
            ShellRegistration::new(shell),
        );

        Ok(())
    }

    pub fn unregister(
        &mut self,
        id: &CustomShellId,
    ) -> CustomShellResult<()> {
        self.shells
            .remove(id)
            .map(|_| ())
            .ok_or_else(|| {
                CustomShellError::NotFound(
                    id.to_string(),
                )
            })
    }

    pub fn get(
        &self,
        id: &CustomShellId,
    ) -> Option<&ShellRegistration> {
        self.shells.get(id)
    }

    pub fn get_mut(
        &mut self,
        id: &CustomShellId,
    ) -> Option<&mut ShellRegistration> {
        self.shells.get_mut(id)
    }

    pub fn enable(
        &mut self,
        id: &CustomShellId,
    ) -> CustomShellResult<()> {
        let registration =
            self.get_mut(id).ok_or_else(|| {
                CustomShellError::NotFound(
                    id.to_string(),
                )
            })?;

        registration.enable();

        Ok(())
    }

    pub fn disable(
        &mut self,
        id: &CustomShellId,
    ) -> CustomShellResult<()> {
        let registration =
            self.get_mut(id).ok_or_else(|| {
                CustomShellError::NotFound(
                    id.to_string(),
                )
            })?;

        registration.disable();

        Ok(())
    }

    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &ShellRegistration> {
        self.shells.values()
    }

    pub fn len(&self) -> usize {
        self.shells.len()
    }

    pub fn is_empty(&self) -> bool {
        self.shells.is_empty()
    }

    pub fn clear(&mut self) {
        self.shells.clear();
    }
}

impl Default for CustomShellRegistry {
    fn default() -> Self {
        Self::new()
    }
}
