use std::net::IpAddr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForwardingDirection {
    Local,
    Remote,
    Dynamic,
}

#[derive(Debug, Clone)]
pub struct PortForward {
    pub id: u64,
    pub direction: ForwardingDirection,
    pub bind_address: IpAddr,
    pub bind_port: u16,
    pub target_host: String,
    pub target_port: u16,
    pub active: bool,
}

impl PortForward {
    pub fn new(
        id: u64,
        direction: ForwardingDirection,
        bind_address: IpAddr,
        bind_port: u16,
        target_host: impl Into<String>,
        target_port: u16,
    ) -> Self {
        Self {
            id,
            direction,
            bind_address,
            bind_port,
            target_host: target_host.into(),
            target_port,
            active: false,
        }
    }

    pub fn target(&self) -> String {
        format!("{}:{}", self.target_host, self.target_port)
    }

    pub fn bind(&self) -> String {
        format!("{}:{}", self.bind_address, self.bind_port)
    }
}

#[derive(Debug, Default)]
pub struct PortForwardingManager {
    forwards: Vec<PortForward>,
    next_id: u64,
}

impl PortForwardingManager {
    pub fn new() -> Self {
        Self {
            forwards: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add(
        &mut self,
        direction: ForwardingDirection,
        bind_address: IpAddr,
        bind_port: u16,
        target_host: impl Into<String>,
        target_port: u16,
    ) -> u64 {
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);

        self.forwards.push(PortForward::new(
            id,
            direction,
            bind_address,
            bind_port,
            target_host,
            target_port,
        ));

        id
    }

    pub fn get(&self, id: u64) -> Option<&PortForward> {
        self.forwards.iter().find(|forward| forward.id == id)
    }

    pub fn get_mut(&mut self, id: u64) -> Option<&mut PortForward> {
        self.forwards.iter_mut().find(|forward| forward.id == id)
    }

    pub fn start(&mut self, id: u64) -> Result<(), String> {
        let forward = self
            .get_mut(id)
            .ok_or_else(|| "port forward not found".to_string())?;

        forward.active = true;
        Ok(())
    }

    pub fn stop(&mut self, id: u64) -> Result<(), String> {
        let forward = self
            .get_mut(id)
            .ok_or_else(|| "port forward not found".to_string())?;

        forward.active = false;
        Ok(())
    }

    pub fn remove(&mut self, id: u64) -> bool {
        if let Some(index) = self.forwards.iter().position(|f| f.id == id) {
            self.forwards.remove(index);
            true
        } else {
            false
        }
    }

    pub fn forwards(&self) -> &[PortForward] {
        &self.forwards
    }
}
