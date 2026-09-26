use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KittyCapability {
    Graphics,
    Keyboard,
    Clipboard,
    TextSizing,
    Cursor,
    DragDrop,
    FileTransfer,
    Notifications,
    Colors,
    Underlines,
    Regions,
}

#[derive(Debug, Default)]
pub struct KittyProtocol {
    capabilities: HashSet<KittyCapability>,
}

impl KittyProtocol {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enable(&mut self, capability: KittyCapability) {
        self.capabilities.insert(capability);
    }

    pub fn disable(&mut self, capability: KittyCapability) {
        self.capabilities.remove(&capability);
    }

    pub fn supports(&self, capability: KittyCapability) -> bool {
        self.capabilities.contains(&capability)
    }

    pub fn capabilities(&self) -> &HashSet<KittyCapability> {
        &self.capabilities
    }

    pub fn enable_all(&mut self) {
        for capability in [
            KittyCapability::Graphics,
            KittyCapability::Keyboard,
            KittyCapability::Clipboard,
            KittyCapability::TextSizing,
            KittyCapability::Cursor,
            KittyCapability::DragDrop,
            KittyCapability::FileTransfer,
            KittyCapability::Notifications,
            KittyCapability::Colors,
            KittyCapability::Underlines,
            KittyCapability::Regions,
        ] {
            self.enable(capability);
        }
    }
}
