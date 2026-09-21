use crate::colors::Rgba;

/// Terminal cursor shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorShape {
    Block,
    Beam,
    Underline,
    HollowBlock,
    HollowBeam,
}

impl Default for CursorShape {
    fn default() -> Self {
        Self::Block
    }
}

/// Cursor blink state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorBlinkState {
    Disabled,
    Visible,
    Hidden,
}

impl Default for CursorBlinkState {
    fn default() -> Self {
        Self::Visible
    }
}

/// Terminal cursor description.
#[derive(Debug, Clone)]
pub struct Cursor {
    position: [u32; 2],
    size: [u32; 2],
    shape: CursorShape,
    color: Rgba,
    opacity: f32,
    blink_enabled: bool,
    blink_interval_ms: u64,
    blink_state: CursorBlinkState,
    focused: bool,
}

impl Default for Cursor {
    fn default() -> Self {
        Self {
            position: [0, 0],
            size: [1, 1],
            shape: CursorShape::Block,
            color: Rgba::WHITE,
            opacity: 1.0,
            blink_enabled: true,
            blink_interval_ms: 500,
            blink_state: CursorBlinkState::Visible,
            focused: true,
        }
    }
}

impl Cursor {
    pub fn position(&self) -> [u32; 2] {
        self.position
    }

    pub fn size(&self) -> [u32; 2] {
        self.size
    }

    pub fn shape(&self) -> CursorShape {
        self.shape
    }

    pub fn color(&self) -> Rgba {
        self.color
    }

    pub fn opacity(&self) -> f32 {
        self.opacity
    }

    pub fn blink_enabled(&self) -> bool {
        self.blink_enabled
    }

    pub fn blink_interval_ms(&self) -> u64 {
        self.blink_interval_ms
    }

    pub fn blink_state(&self) -> CursorBlinkState {
        self.blink_state
    }

    pub fn focused(&self) -> bool {
        self.focused
    }

    pub fn set_position(&mut self, column: u32, row: u32) {
        self.position = [column, row];
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.size = [width.max(1), height.max(1)];
    }

    pub fn set_shape(&mut self, shape: CursorShape) {
        self.shape = shape;
    }

    pub fn set_color(&mut self, color: Rgba) {
        self.color = color;
    }

    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity.clamp(0.0, 1.0);
    }

    pub fn set_blink_enabled(&mut self, enabled: bool) {
        self.blink_enabled = enabled;

        if !enabled {
            self.blink_state = CursorBlinkState::Visible;
        }
    }

    pub fn set_blink_interval(&mut self, milliseconds: u64) {
        self.blink_interval_ms = milliseconds.max(50);
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;

        if !focused {
            self.blink_state = CursorBlinkState::Visible;
        }
    }

    pub fn set_blink_state(&mut self, state: CursorBlinkState) {
        self.blink_state = state;
    }

    pub fn toggle_blink(&mut self) {
        if !self.blink_enabled {
            self.blink_state = CursorBlinkState::Visible;
            return;
        }

        self.blink_state = match self.blink_state {
            CursorBlinkState::Visible => CursorBlinkState::Hidden,
            CursorBlinkState::Hidden => CursorBlinkState::Visible,
            CursorBlinkState::Disabled => CursorBlinkState::Visible,
        };
    }

    pub fn visible(&self) -> bool {
        self.blink_state == CursorBlinkState::Visible
            && self.opacity > 0.0
    }
}
