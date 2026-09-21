use std::collections::HashMap;

use super::{
    errors::{CustomProtocolError, CustomProtocolResult},
    protocol::{CustomTerminalProtocol, ProtocolId},
};

pub struct ProtocolRegistration {
    pub protocol: Box<dyn CustomTerminalProtocol>,
    pub enabled: bool,
}

impl ProtocolRegistration {
    pub fn new(protocol: Box<dyn CustomTerminalProtocol>) -> Self {
        Self {
            protocol,
            enabled: true,
        }
    }

    pub fn id(&self) -> ProtocolId {
        self.protocol.id()
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }
}

pub struct CustomProtocolRegistry {
    protocols: HashMap<ProtocolId, ProtocolRegistration>,
}

impl CustomProtocolRegistry {
    pub fn new() -> Self {
        Self {
            protocols: HashMap::new(),
        }
    }

    pub fn register(
        &mut self,
        protocol: Box<dyn CustomTerminalProtocol>,
    ) -> CustomProtocolResult<()> {
        let id = protocol.id();

        if self.protocols.contains_key(&id) {
            return Err(CustomProtocolError::AlreadyRegistered(id.to_string()));
        }

        self.protocols.insert(id, ProtocolRegistration::new(protocol));

        Ok(())
    }

    pub fn unregister(
        &mut self,
        id: &ProtocolId,
    ) -> CustomProtocolResult<()> {
        self.protocols
            .remove(id)
            .map(|_| ())
            .ok_or_else(|| CustomProtocolError::NotFound(id.to_string()))
    }

    pub fn get(
        &self,
        id: &ProtocolId,
    ) -> Option<&ProtocolRegistration> {
        self.protocols.get(id)
    }

    pub fn get_mut(
        &mut self,
        id: &ProtocolId,
    ) -> Option<&mut ProtocolRegistration> {
        self.protocols.get_mut(id)
    }

    pub fn contains(&self, id: &ProtocolId) -> bool {
        self.protocols.contains_key(id)
    }

    pub fn enable(&mut self, id: &ProtocolId) -> CustomProtocolResult<()> {
        self.get_mut(id)
            .ok_or_else(|| CustomProtocolError::NotFound(id.to_string()))?
            .enable();

        Ok(())
    }

    pub fn disable(&mut self, id: &ProtocolId) -> CustomProtocolResult<()> {
        self.get_mut(id)
            .ok_or_else(|| CustomProtocolError::NotFound(id.to_string()))?
            .disable();

        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = &ProtocolRegistration> {
        self.protocols.values()
    }

    pub fn len(&self) -> usize {
        self.protocols.len()
    }

    pub fn is_empty(&self) -> bool {
        self.protocols.is_empty()
    }

    pub fn clear(&mut self) {
        self.protocols.clear();
    }
}

impl Default for CustomProtocolRegistry {
    fn default() -> Self {
        Self::new()
    }
}
