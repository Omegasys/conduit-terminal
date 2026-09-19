use std::collections::HashMap;

/// Keyboard modifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyModifier {
    Shift,
    Control,
    Alt,
    Super,
}

/// A keyboard key.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    Character(String),
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
    Up,
    Down,
    Left,
    Right,
    Function(u8),
    Space,
}

/// A complete keyboard shortcut.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyCombination {
    pub modifiers: Vec<KeyModifier>,
    pub key: Key,
}

impl KeyCombination {
    pub fn new(key: Key) -> Self {
        Self {
            modifiers: Vec::new(),
            key,
        }
    }

    pub fn with_modifier(mut self, modifier: KeyModifier) -> Self {
        if !self.modifiers.contains(&modifier) {
            self.modifiers.push(modifier);
        }

        self
    }

    pub fn has_modifier(&self, modifier: KeyModifier) -> bool {
        self.modifiers.contains(&modifier)
    }
}

/// Keyboard action exposed by the GUI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyboardAction {
    ExecuteCommand(String),
    FocusNextPane,
    FocusPreviousPane,
    NewTab,
    CloseTab,
    NewWindow,
    CloseWindow,
    NextTab,
    PreviousTab,
    ToggleCommandPalette,
    ToggleSidebar,
    OpenSettings,
    Copy,
    Paste,
    SelectAll,
    Custom(String),
}

/// Keyboard shortcut binding.
#[derive(Debug, Clone)]
pub struct KeyBinding {
    combination: KeyCombination,
    action: KeyboardAction,
    enabled: bool,
}

impl KeyBinding {
    pub fn new(
        combination: KeyCombination,
        action: KeyboardAction,
    ) -> Self {
        Self {
            combination,
            action,
            enabled: true,
        }
    }

    pub fn combination(&self) -> &KeyCombination {
        &self.combination
    }

    pub fn action(&self) -> &KeyboardAction {
        &self.action
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

/// Manages GUI keyboard shortcuts.
#[derive(Debug, Default)]
pub struct KeyboardManager {
    bindings: Vec<KeyBinding>,
    command_aliases: HashMap<String, KeyboardAction>,
}

impl KeyboardManager {
    pub fn new() -> Self {
        Self {
            bindings: Vec::new(),
            command_aliases: HashMap::new(),
        }
    }

    pub fn bind(&mut self, binding: KeyBinding) {
        self.bindings.push(binding);
    }

    pub fn unbind(&mut self, combination: &KeyCombination) {
        self.bindings
            .retain(|binding| binding.combination() != combination);
    }

    pub fn resolve(&self, combination: &KeyCombination) -> Option<&KeyboardAction> {
        self.bindings
            .iter()
            .find(|binding| {
                binding.is_enabled() && binding.combination() == combination
            })
            .map(KeyBinding::action)
    }

    pub fn add_command_alias(
        &mut self,
        alias: impl Into<String>,
        action: KeyboardAction,
    ) {
        self.command_aliases.insert(alias.into(), action);
    }

    pub fn resolve_alias(&self, alias: &str) -> Option<&KeyboardAction> {
        self.command_aliases.get(alias)
    }

    pub fn bindings(&self) -> &[KeyBinding] {
        &self.bindings
    }

    pub fn clear(&mut self) {
        self.bindings.clear();
        self.command_aliases.clear();
    }

    pub fn len(&self) -> usize {
        self.bindings.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }
}
