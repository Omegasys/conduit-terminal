#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Back,
    Forward,
    Other(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseEventKind {
    Move,
    Press,
    Release,
    Drag,
    Scroll,
    Enter,
    Leave,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MouseModifiers {
    pub shift: bool,
    pub control: bool,
    pub alt: bool,
    pub super_key: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MouseEvent {
    pub kind: MouseEventKind,
    pub button: Option<MouseButton>,
    pub x: f64,
    pub y: f64,
    pub delta_x: f64,
    pub delta_y: f64,
    pub modifiers: MouseModifiers,
}

impl MouseEvent {
    pub fn new(
        kind: MouseEventKind,
        x: f64,
        y: f64,
    ) -> Self {
        Self {
            kind,
            button: None,
            x,
            y,
            delta_x: 0.0,
            delta_y: 0.0,
            modifiers: MouseModifiers::default(),
        }
    }

    pub fn button(
        mut self,
        button: MouseButton,
    ) -> Self {
        self.button = Some(button);
        self
    }

    pub fn delta(
        mut self,
        delta_x: f64,
        delta_y: f64,
    ) -> Self {
        self.delta_x = delta_x;
        self.delta_y = delta_y;
        self
    }

    pub fn modifiers(
        mut self,
        modifiers: MouseModifiers,
    ) -> Self {
        self.modifiers = modifiers;
        self
    }

    pub fn is_button_event(&self) -> bool {
        matches!(
            self.kind,
            MouseEventKind::Press
                | MouseEventKind::Release
        )
    }

    pub fn is_motion(&self) -> bool {
        matches!(
            self.kind,
            MouseEventKind::Move
                | MouseEventKind::Drag
        )
    }
}

#[derive(Debug, Default)]
pub struct MouseState {
    x: f64,
    y: f64,
    buttons: Vec<MouseButton>,
    modifiers: MouseModifiers,
}

impl MouseState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(
        &mut self,
        event: &MouseEvent,
    ) {
        self.x = event.x;
        self.y = event.y;
        self.modifiers = event.modifiers;

        if let Some(button) = event.button {
            match event.kind {
                MouseEventKind::Press => {
                    if !self.buttons.contains(&button) {
                        self.buttons.push(button);
                    }
                }

                MouseEventKind::Release => {
                    self.buttons.retain(
                        |item| *item != button,
                    );
                }

                _ => {}
            }
        }
    }

    pub fn position(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    pub fn buttons(&self) -> &[MouseButton] {
        &self.buttons
    }

    pub fn is_pressed(
        &self,
        button: &MouseButton,
    ) -> bool {
        self.buttons.contains(button)
    }

    pub fn modifiers(&self) -> MouseModifiers {
        self.modifiers
    }

    pub fn clear(&mut self) {
        self.buttons.clear();
        self.modifiers = MouseModifiers::default();
    }
}
