use std::collections::HashMap;

use super::actions::MenuAction;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Accelerator {
    key: String,
    modifiers: Vec<String>,
}

impl Accelerator {
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            modifiers: Vec::new(),
        }
    }

    pub fn with_modifier(mut self, modifier: impl Into<String>) -> Self {
        self.modifiers.push(modifier.into());
        self
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn modifiers(&self) -> &[String] {
        &self.modifiers
    }

    pub fn display(&self) -> String {
        let mut parts = self.modifiers.clone();
        parts.push(self.key.clone());
        parts.join("+")
    }

    pub fn parse(value: &str) -> Self {
        let mut parts = value
            .split('+')
            .map(str::trim)
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>();

        let key = parts.pop().unwrap_or_default().to_string();

        Self {
            key,
            modifiers: parts.into_iter().map(String::from).collect(),
        }
    }
}

#[derive(Debug, Default)]
pub struct AcceleratorManager {
    bindings: HashMap<Accelerator, MenuAction>,
}

impl AcceleratorManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, accelerator: Accelerator, action: MenuAction) {
        self.bindings.insert(accelerator, action);
    }

    pub fn register_string(
        &mut self,
        accelerator: impl AsRef<str>,
        action: MenuAction,
    ) {
        self.register(Accelerator::parse(accelerator.as_ref()), action);
    }

    pub fn unregister(&mut self, accelerator: &Accelerator) -> Option<MenuAction> {
        self.bindings.remove(accelerator)
    }

    pub fn resolve(&self, accelerator: &Accelerator) -> Option<&MenuAction> {
        self.bindings.get(accelerator)
    }

    pub fn resolve_string(&self, accelerator: &str) -> Option<&MenuAction> {
        self.resolve(&Accelerator::parse(accelerator))
    }

    pub fn bindings(&self) -> &HashMap<Accelerator, MenuAction> {
        &self.bindings
    }

    pub fn clear(&mut self) {
        self.bindings.clear();
    }

    pub fn len(&self) -> usize {
        self.bindings.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }
}
