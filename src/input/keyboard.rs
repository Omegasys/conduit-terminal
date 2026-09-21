use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    Character(char),

    Enter,
    Escape,
    Tab,
    Backspace,
    Delete,

    Insert,
    Home,
    End,
    PageUp,
    PageDown,

    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,

    CapsLock,
    NumLock,
    ScrollLock,

    PrintScreen,
    Pause,

    Function(u8),

    Space,

    Unknown,
}

impl KeyCode {
    pub fn is_function_key(&self) -> bool {
        matches!(self, Self::Function(_))
    }

    pub fn is_navigation_key(&self) -> bool {
        matches!(
            self,
            Self::Home
                | Self::End
                | Self::PageUp
                | Self::PageDown
                | Self::ArrowUp
                | Self::ArrowDown
                | Self::ArrowLeft
                | Self::ArrowRight
        )
    }

    pub fn character(&self) -> Option<char> {
        match self {
            Self::Character(character) => Some(*character),
            Self::Space => Some(' '),
            _ => None,
        }
    }
}

impl fmt::Display for KeyCode {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Character(character) => {
                write!(formatter, "{character}")
            }
            Self::Enter => write!(formatter, "Enter"),
            Self::Escape => write!(formatter, "Escape"),
            Self::Tab => write!(formatter, "Tab"),
            Self::Backspace => write!(formatter, "Backspace"),
            Self::Delete => write!(formatter, "Delete"),
            Self::Insert => write!(formatter, "Insert"),
            Self::Home => write!(formatter, "Home"),
            Self::End => write!(formatter, "End"),
            Self::PageUp => write!(formatter, "PageUp"),
            Self::PageDown => write!(formatter, "PageDown"),
            Self::ArrowUp => write!(formatter, "ArrowUp"),
            Self::ArrowDown => write!(formatter, "ArrowDown"),
            Self::ArrowLeft => write!(formatter, "ArrowLeft"),
            Self::ArrowRight => write!(formatter, "ArrowRight"),
            Self::CapsLock => write!(formatter, "CapsLock"),
            Self::NumLock => write!(formatter, "NumLock"),
            Self::ScrollLock => write!(formatter, "ScrollLock"),
            Self::PrintScreen => write!(formatter, "PrintScreen"),
            Self::Pause => write!(formatter, "Pause"),
            Self::Function(number) => write!(formatter, "F{number}"),
            Self::Space => write!(formatter, "Space"),
            Self::Unknown => write!(formatter, "Unknown"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct KeyModifiers {
    pub shift: bool,
    pub control: bool,
    pub alt: bool,
    pub super_key: bool,
    pub meta: bool,
}

impl KeyModifiers {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn shift(mut self) -> Self {
        self.shift = true;
        self
    }

    pub fn control(mut self) -> Self {
        self.control = true;
        self
    }

    pub fn alt(mut self) -> Self {
        self.alt = true;
        self
    }

    pub fn super_key(mut self) -> Self {
        self.super_key = true;
        self
    }

    pub fn meta(mut self) -> Self {
        self.meta = true;
        self
    }

    pub fn any(&self) -> bool {
        self.shift
            || self.control
            || self.alt
            || self.super_key
            || self.meta
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyEventKind {
    Press,
    Release,
    Repeat,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyEvent {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
    pub kind: KeyEventKind,
    pub text: Option<String>,
}

impl KeyEvent {
    pub fn press(code: KeyCode) -> Self {
        Self {
            code,
            modifiers: KeyModifiers::default(),
            kind: KeyEventKind::Press,
            text: None,
        }
    }

    pub fn with_modifiers(
        mut self,
        modifiers: KeyModifiers,
    ) -> Self {
        self.modifiers = modifiers;
        self
    }

    pub fn with_text(
        mut self,
        text: impl Into<String>,
    ) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn is_press(&self) -> bool {
        self.kind == KeyEventKind::Press
    }

    pub fn is_release(&self) -> bool {
        self.kind == KeyEventKind::Release
    }

    pub fn is_repeat(&self) -> bool {
        self.kind == KeyEventKind::Repeat
    }
}

#[derive(Debug, Default)]
pub struct KeyboardState {
    pressed: Vec<KeyCode>,
    modifiers: KeyModifiers,
}

impl KeyboardState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, event: &KeyEvent) {
        self.modifiers = event.modifiers;

        match event.kind {
            KeyEventKind::Press | KeyEventKind::Repeat => {
                if !self.pressed.contains(&event.code) {
                    self.pressed.push(event.code);
                }
            }

            KeyEventKind::Release => {
                self.pressed.retain(|key| *key != event.code);
            }
        }
    }

    pub fn is_pressed(
        &self,
        key: &KeyCode,
    ) -> bool {
        self.pressed.contains(key)
    }

    pub fn pressed(&self) -> &[KeyCode] {
        &self.pressed
    }

    pub fn modifiers(&self) -> KeyModifiers {
        self.modifiers
    }

    pub fn clear(&mut self) {
        self.pressed.clear();
        self.modifiers = KeyModifiers::default();
    }
}
