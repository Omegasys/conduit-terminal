use super::keyboard::{
    KeyCode,
    KeyModifiers,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShortcutAction {
    Copy,
    Paste,
    Cut,
    SelectAll,

    NewWindow,
    CloseWindow,

    NewTab,
    CloseTab,
    NextTab,
    PreviousTab,

    SplitHorizontal,
    SplitVertical,
    ClosePane,

    IncreaseFontSize,
    DecreaseFontSize,
    ResetFontSize,

    ToggleFullscreen,
    ToggleSidebar,

    OpenSettings,
    OpenCommandPalette,

    Search,

    Quit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shortcut {
    pub key: KeyCode,
    pub modifiers: KeyModifiers,
    pub action: ShortcutAction,
}

impl Shortcut {
    pub fn new(
        key: KeyCode,
        modifiers: KeyModifiers,
        action: ShortcutAction,
    ) -> Self {
        Self {
            key,
            modifiers,
            action,
        }
    }

    pub fn matches(
        &self,
        key: &KeyCode,
        modifiers: &KeyModifiers,
    ) -> bool {
        &self.key == key
            && self.modifiers == *modifiers
    }
}

#[derive(Debug, Default)]
pub struct ShortcutManager {
    shortcuts: Vec<Shortcut>,
}

impl ShortcutManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(
        &mut self,
        shortcut: Shortcut,
    ) {
        self.shortcuts.push(shortcut);
    }

    pub fn remove_action(
        &mut self,
        action: ShortcutAction,
    ) {
        self.shortcuts
            .retain(|shortcut| shortcut.action != action);
    }

    pub fn find(
        &self,
        key: &KeyCode,
        modifiers: &KeyModifiers,
    ) -> Option<ShortcutAction> {
        self.shortcuts
            .iter()
            .find(|shortcut| {
                shortcut.matches(key, modifiers)
            })
            .map(|shortcut| shortcut.action)
    }

    pub fn shortcuts(&self) -> &[Shortcut] {
        &self.shortcuts
    }

    pub fn clear(&mut self) {
        self.shortcuts.clear();
    }

    pub fn load_defaults(&mut self) {
        self.register(Shortcut::new(
            KeyCode::Character('c'),
            KeyModifiers::default().control(),
            ShortcutAction::Copy,
        ));

        self.register(Shortcut::new(
            KeyCode::Character('v'),
            KeyModifiers::default().control(),
            ShortcutAction::Paste,
        ));

        self.register(Shortcut::new(
            KeyCode::Character('t'),
            KeyModifiers::default().control(),
            ShortcutAction::NewTab,
        ));

        self.register(Shortcut::new(
            KeyCode::Character('w'),
            KeyModifiers::default().control(),
            ShortcutAction::CloseTab,
        ));

        self.register(Shortcut::new(
            KeyCode::Character(','),
            KeyModifiers::default().control(),
            ShortcutAction::OpenSettings,
        ));

        self.register(Shortcut::new(
            KeyCode::Character('p'),
            KeyModifiers::default()
                .control()
                .shift(),
            ShortcutAction::OpenCommandPalette,
        ));

        self.register(Shortcut::new(
            KeyCode::Character('f'),
            KeyModifiers::default().control(),
            ShortcutAction::Search,
        ));

        self.register(Shortcut::new(
            KeyCode::Character('q'),
            KeyModifiers::default()
                .control()
                .shift(),
            ShortcutAction::Quit,
        ));
    }
}
