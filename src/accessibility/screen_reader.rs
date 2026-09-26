//! Screen-reader accessibility tree and state.

use std::collections::HashMap;

/// Semantic role of an accessible UI node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessibilityRole {
    Window,
    Dialog,
    Menu,
    MenuItem,
    Button,
    CheckBox,
    RadioButton,
    TextField,
    Terminal,
    Tab,
    TabList,
    Pane,
    List,
    ListItem,
    StatusBar,
    Toolbar,
    ScrollBar,
    Image,
    Text,
    Unknown,
}

/// State associated with an accessible node.
#[derive(Debug, Clone, Default)]
pub struct AccessibilityState {
    pub focused: bool,
    pub selected: bool,
    pub expanded: bool,
    pub disabled: bool,
    pub checked: bool,
    pub busy: bool,
    pub value: Option<String>,
}

impl AccessibilityState {
    pub fn focus(&mut self) {
        self.focused = true;
    }

    pub fn blur(&mut self) {
        self.focused = false;
    }

    pub fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    pub fn set_expanded(&mut self, expanded: bool) {
        self.expanded = expanded;
    }

    pub fn set_checked(&mut self, checked: bool) {
        self.checked = checked;
    }
}

/// A node in Conduit's platform-neutral accessibility tree.
#[derive(Debug, Clone)]
pub struct ScreenReaderNode {
    pub id: u64,
    pub role: AccessibilityRole,
    pub name: String,
    pub description: Option<String>,
    pub state: AccessibilityState,
    pub children: Vec<u64>,
}

impl ScreenReaderNode {
    pub fn new(
        id: u64,
        role: AccessibilityRole,
        name: impl Into<String>,
    ) -> Self {
        Self {
            id,
            role,
            name: name.into(),
            description: None,
            state: AccessibilityState::default(),
            children: Vec::new(),
        }
    }

    pub fn describe(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn add_child(&mut self, child_id: u64) {
        if !self.children.contains(&child_id) {
            self.children.push(child_id);
        }
    }

    /// Produces a concise phrase suitable for speech output.
    pub fn spoken_description(&self) -> String {
        let mut result = format!("{}: {}", role_name(self.role), self.name);

        if let Some(description) = &self.description {
            if !description.is_empty() {
                result.push_str(". ");
                result.push_str(description);
            }
        }

        if self.state.disabled {
            result.push_str(". Disabled");
        }

        if self.state.selected {
            result.push_str(". Selected");
        }

        if self.state.checked {
            result.push_str(". Checked");
        }

        if self.state.expanded {
            result.push_str(". Expanded");
        }

        if let Some(value) = &self.state.value {
            result.push_str(". ");
            result.push_str(value);
        }

        result
    }
}

fn role_name(role: AccessibilityRole) -> &'static str {
    match role {
        AccessibilityRole::Window => "Window",
        AccessibilityRole::Dialog => "Dialog",
        AccessibilityRole::Menu => "Menu",
        AccessibilityRole::MenuItem => "Menu item",
        AccessibilityRole::Button => "Button",
        AccessibilityRole::CheckBox => "Checkbox",
        AccessibilityRole::RadioButton => "Radio button",
        AccessibilityRole::TextField => "Text field",
        AccessibilityRole::Terminal => "Terminal",
        AccessibilityRole::Tab => "Tab",
        AccessibilityRole::TabList => "Tab list",
        AccessibilityRole::Pane => "Pane",
        AccessibilityRole::List => "List",
        AccessibilityRole::ListItem => "List item",
        AccessibilityRole::StatusBar => "Status bar",
        AccessibilityRole::Toolbar => "Toolbar",
        AccessibilityRole::ScrollBar => "Scrollbar",
        AccessibilityRole::Image => "Image",
        AccessibilityRole::Text => "Text",
        AccessibilityRole::Unknown => "Element",
    }
}

/// Platform-neutral screen-reader manager.
#[derive(Debug, Clone, Default)]
pub struct ScreenReader {
    enabled: bool,
    nodes: HashMap<u64, ScreenReaderNode>,
    focused_node: Option<u64>,
    root_node: Option<u64>,
}

impl ScreenReader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn set_root(&mut self, node_id: u64) {
        self.root_node = Some(node_id);
    }

    pub fn root(&self) -> Option<u64> {
        self.root_node
    }

    pub fn register(&mut self, node: ScreenReaderNode) {
        self.nodes.insert(node.id, node);
    }

    pub fn remove(&mut self, node_id: u64) {
        self.nodes.remove(&node_id);

        if self.focused_node == Some(node_id) {
            self.focused_node = None;
        }

        if self.root_node == Some(node_id) {
            self.root_node = None;
        }

        for node in self.nodes.values_mut() {
            node.children.retain(|child| *child != node_id);
        }
    }

    pub fn node(&self, node_id: u64) -> Option<&ScreenReaderNode> {
        self.nodes.get(&node_id)
    }

    pub fn node_mut(&mut self, node_id: u64) -> Option<&mut ScreenReaderNode> {
        self.nodes.get_mut(&node_id)
    }

    pub fn focus(&mut self, node_id: u64) -> Option<String> {
        if !self.nodes.contains_key(&node_id) {
            return None;
        }

        if let Some(previous) = self.focused_node {
            if let Some(node) = self.nodes.get_mut(&previous) {
                node.state.blur();
            }
        }

        let description = if let Some(node) = self.nodes.get_mut(&node_id) {
            node.state.focus();
            Some(node.spoken_description())
        } else {
            None
        };

        self.focused_node = Some(node_id);
        description
    }

    pub fn focused(&self) -> Option<u64> {
        self.focused_node
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.focused_node = None;
        self.root_node = None;
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}
