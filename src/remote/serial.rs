use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SerialParity {
    None,
    Even,
    Odd,
    Mark,
    Space,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SerialStopBits {
    One,
    OnePointFive,
    Two,
}

#[derive(Debug, Clone)]
pub struct SerialConfig {
    pub device: String,
    pub baud_rate: u32,
    pub data_bits: u8,
    pub parity: SerialParity,
    pub stop_bits: SerialStopBits,
    pub flow_control: bool,
    pub timeout: Duration,
}

impl Default for SerialConfig {
    fn default() -> Self {
        Self {
            device: String::new(),
            baud_rate: 115_200,
            data_bits: 8,
            parity: SerialParity::None,
            stop_bits: SerialStopBits::One,
            flow_control: false,
            timeout: Duration::from_secs(1),
        }
    }
}

#[derive(Debug)]
pub struct SerialConnection {
    config: SerialConfig,
    connected: bool,
}

impl SerialConnection {
    pub fn new(config: SerialConfig) -> Self {
        Self {
            config,
            connected: false,
        }
    }

    pub fn config(&self) -> &SerialConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut SerialConfig {
        &mut self.config
    }

    pub fn connect(&mut self) -> Result<(), String> {
        if self.config.device.is_empty() {
            return Err("serial device has not been configured".into());
        }

        self.connected = true;
        Ok(())
    }

    pub fn disconnect(&mut self) {
        self.connected = false;
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub fn send(&self, _data: &[u8]) -> Result<(), String> {
        if !self.connected {
            return Err("serial connection is not connected".into());
        }

        Ok(())
    }
}
