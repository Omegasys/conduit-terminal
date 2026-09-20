use std::collections::BTreeMap;

use super::color::Color;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum SemanticColor {
    TerminalForeground,
    TerminalBackground,
    Cursor,
    Selection,
    SearchMatch,
    ActiveTab,
    InactiveTab,
    TabText,
    PaneBorder,
    ActivePaneBorder,
    MenuBackground,
    MenuForeground,
    Accent,
    Success,
    Warning,
    Error,
    Information,
    Security,
    SecurityWarning,
    Link,
    Disabled,
    StatusBar,
    StatusBarText,
    CommandPalette,
    FlowNode,
    FlowEdge,
}

#[derive(Clone, Debug, Default)]
pub struct SemanticColorMap {
    colors: BTreeMap<SemanticColor, Color>,
}

impl SemanticColorMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, semantic: SemanticColor, color: Color) {
        self.colors.insert(semantic, color);
    }

    pub fn get(&self, semantic: SemanticColor) -> Option<Color> {
        self.colors.get(&semantic).copied()
    }

    pub fn remove(&mut self, semantic: SemanticColor) -> Option<Color> {
        self.colors.remove(&semantic)
    }

    pub fn contains(&self, semantic: SemanticColor) -> bool {
        self.colors.contains_key(&semantic)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&SemanticColor, &Color)> {
        self.colors.iter()
    }

    pub fn len(&self) -> usize {
        self.colors.len()
    }

    pub fn is_empty(&self) -> bool {
        self.colors.is_empty()
    }
}
