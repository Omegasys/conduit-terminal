use super::flow::FlowPosition;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FlowNodeId(u64);

impl FlowNodeId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowNodeKind {
    Application,
    Window,
    Tab,
    Pane,
    Session,
    Shell,
    Process,
    Command,
    Plugin,
    Event,
    Input,
    Output,
    File,
    Network,
    External,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowNodeState {
    Created,
    Active,
    Waiting,
    Blocked,
    Error,
    Stopped,
    Hidden,
}

#[derive(Debug, Clone)]
pub struct FlowNode {
    id: FlowNodeId,
    kind: FlowNodeKind,
    state: FlowNodeState,
    title: String,
    subtitle: Option<String>,
    position: FlowPosition,
    width: f32,
    height: f32,
    visible: bool,
    selected: bool,
}

impl FlowNode {
    pub fn new(
        kind: FlowNodeKind,
        title: impl Into<String>,
    ) -> Self {
        Self {
            id: FlowNodeId::new(0),
            kind,
            state: FlowNodeState::Created,
            title: title.into(),
            subtitle: None,
            position: FlowPosition::origin(),
            width: 180.0,
            height: 80.0,
            visible: true,
            selected: false,
        }
    }

    pub(crate) fn set_id(&mut self, id: FlowNodeId) {
        self.id = id;
    }

    pub fn id(&self) -> FlowNodeId {
        self.id
    }

    pub fn kind(&self) -> FlowNodeKind {
        self.kind
    }

    pub fn state(&self) -> FlowNodeState {
        self.state
    }

    pub fn set_state(&mut self, state: FlowNodeState) {
        self.state = state;
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn subtitle(&self) -> Option<&str> {
        self.subtitle.as_deref()
    }

    pub fn set_subtitle(&mut self, subtitle: Option<String>) {
        self.subtitle = subtitle;
    }

    pub fn position(&self) -> FlowPosition {
        self.position
    }

    pub fn set_position(&mut self, position: FlowPosition) {
        self.position = position;
    }

    pub fn move_by(&mut self, x: f32, y: f32) {
        self.position = self.position.offset(x, y);
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.width = width.max(1.0);
        self.height = height.max(1.0);
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn selected(&self) -> bool {
        self.selected
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        x >= self.position.x
            && x <= self.position.x + self.width
            && y >= self.position.y
            && y <= self.position.y + self.height
    }
}
