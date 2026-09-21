use super::manager::{
    Connection,
    ConnectionId,
    ConnectionState,
    ConnectionType,
};

#[derive(Debug, Clone)]
pub struct SerialConnectionConfig {
    pub device: String,
    pub baud_rate: u32,
    pub data_bits: u8,
    pub parity: SerialParity,
    pub stop_bits: SerialStopBits,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SerialParity {
    None,
    Even,
    Odd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SerialStopBits {
    One,
    Two,
}

impl Default for SerialConnectionConfig {
    fn default() -> Self {
        Self {
            device: String::new(),
            baud_rate: 115_200,
            data_bits: 8,
            parity: SerialParity::None,
            stop_bits: SerialStopBits::One,
        }
    }
}

pub struct SerialConnection {
    id: ConnectionId,
    config: SerialConnectionConfig,
    state: ConnectionState,
}

impl SerialConnection {
    pub fn new(
        id: ConnectionId,
        config: SerialConnectionConfig,
    ) -> Self {
        Self {
            id,
            config,
            state: ConnectionState::Disconnected,
        }
    }

    pub fn config(&self) -> &SerialConnectionConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut SerialConnectionConfig {
        &mut self.config
    }
}

impl Connection for SerialConnection {
    fn id(&self) -> ConnectionId {
        self.id
    }

    fn connection_type(&self) -> ConnectionType {
        ConnectionType::Serial
    }

    fn state(&self) -> ConnectionState {
        self.state
    }

    fn connect(&mut self) -> Result<(), String> {
        if self.config.device.is_empty() {
            return Err("serial device has not been configured".into());
        }

        self.state = ConnectionState::Connecting;
        self.state = ConnectionState::Connected;

        Ok(())
    }

    fn disconnect(&mut self) -> Result<(), String> {
        self.state = ConnectionState::Disconnecting;
        self.state = ConnectionState::Disconnected;

        Ok(())
    }
}
