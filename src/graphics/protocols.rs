#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GraphicsProtocolKind {
    Sixel,
    Kitty,
    Iterm2,
}

#[derive(Debug, Clone)]
pub enum GraphicsCommand {
    Begin {
        protocol: GraphicsProtocolKind,
    },
    Data(Vec<u8>),
    End,
    Delete {
        image_id: u64,
    },
    Clear,
}

pub trait GraphicsProtocol: Send {
    fn kind(&self) -> GraphicsProtocolKind;

    fn feed(&mut self, data: &[u8]) -> Result<Vec<GraphicsCommand>, String>;

    fn reset(&mut self);

    fn active(&self) -> bool;
}

#[derive(Debug, Clone, Default)]
pub struct GraphicsProtocolRegistry {
    protocols: Vec<GraphicsProtocolKind>,
}

impl GraphicsProtocolRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, protocol: GraphicsProtocolKind) {
        if !self.protocols.contains(&protocol) {
            self.protocols.push(protocol);
        }
    }

    pub fn unregister(&mut self, protocol: GraphicsProtocolKind) {
        self.protocols.retain(|item| *item != protocol);
    }

    pub fn contains(&self, protocol: GraphicsProtocolKind) -> bool {
        self.protocols.contains(&protocol)
    }

    pub fn protocols(&self) -> &[GraphicsProtocolKind] {
        &self.protocols
    }

    pub fn clear(&mut self) {
        self.protocols.clear();
    }

    pub fn register_defaults(&mut self) {
        self.register(GraphicsProtocolKind::Sixel);
        self.register(GraphicsProtocolKind::Kitty);
        self.register(GraphicsProtocolKind::Iterm2);
    }
}
