/// Mouse button.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Back,
    Forward,
}

/// Mouse action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseAction {
    Select,
    Open,
    ContextMenu,
    Paste,
    ExtendSelection,
    Drag,
    Resize,
    Scroll,
    None,
}

/// Position of the mouse pointer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MousePosition {
    pub x: f64,
    pub y: f64,
}

impl MousePosition {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// Current mouse state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MouseState {
    position: MousePosition,
    pressed_button: Option<MouseButton>,
    wheel_delta_x: f64,
    wheel_delta_y: f64,
}

impl Default for MouseState {
    fn default() -> Self {
        Self {
            position: MousePosition::new(0.0, 0.0),
            pressed_button: None,
            wheel_delta_x: 0.0,
            wheel_delta_y: 0.0,
        }
    }
}

impl MouseState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn position(&self) -> MousePosition {
        self.position
    }

    pub fn pressed_button(&self) -> Option<MouseButton> {
        self.pressed_button
    }

    pub fn wheel_delta(&self) -> (f64, f64) {
        (self.wheel_delta_x, self.wheel_delta_y)
    }

    pub fn move_to(&mut self, position: MousePosition) {
        self.position = position;
    }

    pub fn press(&mut self, button: MouseButton) {
        self.pressed_button = Some(button);
    }

    pub fn release(&mut self) {
        self.pressed_button = None;
    }

    pub fn scroll(&mut self, x: f64, y: f64) {
        self.wheel_delta_x = x;
        self.wheel_delta_y = y;
    }

    pub fn clear_wheel_delta(&mut self) {
        self.wheel_delta_x = 0.0;
        self.wheel_delta_y = 0.0;
    }
}

/// GUI mouse configuration.
#[derive(Debug, Clone)]
pub struct MouseSettings {
    pub focus_follows_mouse: bool,
    pub middle_click_paste: bool,
    pub right_click_context_menu: bool,
    pub natural_scrolling: bool,
    pub scroll_multiplier: f64,
    pub double_click_interval_ms: u64,
}

impl Default for MouseSettings {
    fn default() -> Self {
        Self {
            focus_follows_mouse: false,
            middle_click_paste: true,
            right_click_context_menu: true,
            natural_scrolling: false,
            scroll_multiplier: 1.0,
            double_click_interval_ms: 400,
        }
    }
}
