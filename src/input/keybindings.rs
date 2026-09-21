use std::collections::HashMap;

use super::keyboard::{
    KeyCode,
    KeyModifiers,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyBindingMode {
    Global,
    Terminal,
    Tui,
    Gui,
    Search,
    CommandPalette,
    Selection,
    AlternateScreen,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BindingAction {
    SendKey,
    SendText(String),
    Copy,
    Paste,
    ExecuteCommand(String),
    Application(String),
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyBinding {
    pub key: KeyCode,
    pub modifiers: KeyModifiers,
    pub mode: KeyBindingMode,
    pub action: BindingAction,
}

impl KeyBinding {
    pub fn new(
        key: KeyCode,
        modifiers: KeyModifiers,
        mode: KeyBindingMode,
        action: BindingAction,
    ) -> Self {
        Self {
            key,
            modifiers,
            mode,
            action,
        }
    }

    pub fn matches(
        &self,
        key: &KeyCode,
        modifiers: &KeyModifiers,
        mode: KeyBindingMode,
    ) -> bool {
        self.key == *key
            && self.modifiers == *modifiers
            && self.mode == mode
    }
}

#[derive(Debug, Default)]
pub struct KeyBindingManager {
    bindings: Vec<KeyBinding>,
    aliases: HashMap<String, String>,
}

impl KeyBindingManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(
        &mut self,
        binding: KeyBinding,
    ) {
        self.bindings.push(binding);
    }

    pub fn remove(
        &mut self,
        key: &KeyCode,
        modifiers: &KeyModifiers,
        mode: KeyBindingMode,
    ) {
        self.bindings.retain(|binding| {
            !binding.matches(
                key,
                modifiers,
                mode,
            )
        });
    }

    pub fn resolve(
        &self,
        key: &KeyCode,
        modifiers: &KeyModifiers,
        mode: KeyBindingMode,
    ) -> Option<&BindingAction> {
        self.bindings
            .iter()
            .find(|binding| {
                binding.matches(
                    key,
                    modifiers,
                    mode,
                )
            })
            .map(|binding| &binding.action)
    }

    pub fn bindings(&self) -> &[KeyBinding] {
        &self.bindings
    }

    pub fn add_alias(
        &mut self,
        name: impl Into<String>,
        target: impl Into<String>,
    ) {
        self.aliases.insert(
            name.into(),
            target.into(),
        );
    }

    pub fn resolve_alias(
        &self,
        name: &str,
    ) -> Option<&str> {
        self.aliases
            .get(name)
            .map(String::as_str)
    }

    pub fn aliases(
        &self,
    ) -> &HashMap<String, String> {
        &self.aliases
    }

    pub fn clear(&mut self) {
        self.bindings.clear();
        self.aliases.clear();
    }
}
