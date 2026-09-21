use std::fmt;
use std::path::{Path, PathBuf};

use crate::resources::{
    Resource,
    ResourceKind,
    ResourceManager,
};

/// Actions supported by the `conduit resources` command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceCommandAction {
    List,
    Show,
    Discover,
    Reload,
    Validate,
    Enable,
    Disable,
    Remove,
    Register,
}

impl ResourceCommandAction {
    pub fn name(&self) -> &'static str {
        match self {
            Self::List => "list",
            Self::Show => "show",
            Self::Discover => "discover",
            Self::Reload => "reload",
            Self::Validate => "validate",
            Self::Enable => "enable",
            Self::Disable => "disable",
            Self::Remove => "remove",
            Self::Register => "register",
        }
    }

    pub fn from_name(value: &str) -> Option<Self> {
        match value {
            "list" => Some(Self::List),
            "show" => Some(Self::Show),
            "discover" => Some(Self::Discover),
            "reload" => Some(Self::Reload),
            "validate" => Some(Self::Validate),
            "enable" => Some(Self::Enable),
            "disable" => Some(Self::Disable),
            "remove" => Some(Self::Remove),
            "register" => Some(Self::Register),
            _ => None,
        }
    }
}

impl fmt::Display for ResourceCommandAction {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

/// A parsed resource-management command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceCommand {
    action: ResourceCommandAction,
    resource: Option<String>,
    kind: Option<ResourceKind>,
    path: Option<PathBuf>,
}

impl ResourceCommand {
    pub fn new(action: ResourceCommandAction) -> Self {
        Self {
            action,
            resource: None,
            kind: None,
            path: None,
        }
    }

    pub fn action(&self) -> &ResourceCommandAction {
        &self.action
    }

    pub fn resource(&self) -> Option<&str> {
        self.resource.as_deref()
    }

    pub fn kind(&self) -> Option<&ResourceKind> {
        self.kind.as_ref()
    }

    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    pub fn set_resource<S>(&mut self, resource: S)
    where
        S: Into<String>,
    {
        self.resource = Some(resource.into());
    }

    pub fn with_resource<S>(mut self, resource: S) -> Self
    where
        S: Into<String>,
    {
        self.set_resource(resource);
        self
    }

    pub fn set_kind(&mut self, kind: ResourceKind) {
        self.kind = Some(kind);
    }

    pub fn with_kind(mut self, kind: ResourceKind) -> Self {
        self.set_kind(kind);
        self
    }

    pub fn set_path<P>(&mut self, path: P)
    where
        P: Into<PathBuf>,
    {
        self.path = Some(path.into());
    }

    pub fn with_path<P>(mut self, path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        self.set_path(path);
        self
    }

    pub fn requires_resource(&self) -> bool {
        matches!(
            self.action,
            ResourceCommandAction::Show
                | ResourceCommandAction::Validate
                | ResourceCommandAction::Enable
                | ResourceCommandAction::Disable
                | ResourceCommandAction::Remove
        )
    }

    pub fn requires_path(&self) -> bool {
        matches!(self.action, ResourceCommandAction::Register)
    }

    pub fn applies_to(&self, resource: &Resource) -> bool {
        if let Some(name) = self.resource() {
            if resource.name() != name {
                return false;
            }
        }

        if let Some(kind) = self.kind() {
            if resource.kind() != *kind {
                return false;
            }
        }

        true
    }

    pub fn find<'a>(&self, manager: &'a ResourceManager) -> Option<&'a Resource> {
        if let Some(name) = self.resource() {
            manager.find_by_name(name)
        } else {
            None
        }
    }
}
