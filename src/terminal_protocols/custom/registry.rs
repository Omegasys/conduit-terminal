use std::collections::HashMap;

use super::{
    errors::{
        CustomProtocolError,
        CustomProtocolResult,
    },
    protocol::{
        CustomProtocol,
        CustomProtocolId,
    },
};

pub struct ProtocolRegistration {
    protocol: Box<dyn CustomProtocol>,
    enabled: bool,
}

impl ProtocolRegistration {
    pub fn new(
        protocol: Box<dyn CustomProtocol>,
    ) -> Self {
        Self {
            protocol,
            enabled: true,
        }
    }

    pub fn protocol(&self) -> &dyn CustomProtocol {
        self.protocol.as_ref()
    }

    pub fn protocol_mut(
        &mut self,
    ) -> &mut dyn CustomProtocol {
        self.protocol.as_mut()
    }

    pub fn id(&self) -> CustomProtocolId {
        self.protocol.id()
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

#[derive(Default)]
pub struct CustomProtocolRegistry {
    protocols: HashMap<
        CustomProtocolId,
        ProtocolRegistration,
    >,
}

impl CustomProtocolRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(
        &mut self,
        protocol: Box<dyn CustomProtocol>,
    ) -> CustomProtocolResult<()> {
        let id = protocol.id();

        if self.protocols.contains_key(&id) {
            return Err(CustomProtocolError::AlreadyRegistered(
                id.to_string(),
            ));
        }

        self.protocols.insert(
            id,
            ProtocolRegistration::new(protocol),
        );

        Ok(())
    }

    pub fn unregister(
        &mut self,
        id: &CustomProtocolId,
    ) -> Option<ProtocolRegistration> {
        self.protocols.remove(id)
    }

    pub fn get(
        &self,
        id: &CustomProtocolId,
    ) -> Option<&ProtocolRegistration> {
        self.protocols.get(id)
    }

    pub fn get_mut(
        &mut self,
        id: &CustomProtocolId,
    ) -> Option<&mut ProtocolRegistration> {
        self.protocols.get_mut(id)
    }

    pub fn enable(
        &mut self,
        id: &CustomProtocolId,
    ) -> CustomProtocolResult<()> {
        let protocol = self
            .get_mut(id)
            .ok_or_else(|| {
                CustomProtocolError::NotFound(
                    id.to_string(),
                )
            })?;

        protocol.enable();

        Ok(())
    }

    pub fn disable(
        &mut self,
        id: &CustomProtocolId,
    ) -> CustomProtocolResult<()> {
        let protocol = self
            .get_mut(id)
            .ok_or_else(|| {
                CustomProtocolError::NotFound(
                    id.to_string(),
                )
            })?;

        protocol.disable();

        Ok(())
    }

    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &ProtocolRegistration> {
        self.protocols.values()
    }

    pub fn iter_enabled(
        &self,
    ) -> impl Iterator<Item = &ProtocolRegistration> {
        self.protocols
            .values()
            .filter(|protocol| protocol.enabled())
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
