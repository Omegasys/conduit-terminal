use crate::resources::{
    ResourceId,
    ResourceKind,
};

use super::change_detector::{
    ResourceChange,
    ResourceChangeKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotReloadAction {
    Reload,
    ReloadDependents,
    RestartComponent,
    Ignore,
}

#[derive(Debug, Clone)]
pub struct HotReloadResult {
    pub resource: Option<ResourceId>,
    pub action: HotReloadAction,
    pub reason: String,
}

impl HotReloadResult {
    pub fn reload(
        resource: ResourceId,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            resource: Some(resource),
            action: HotReloadAction::Reload,
            reason: reason.into(),
        }
    }

    pub fn reload_dependents(
        resource: ResourceId,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            resource: Some(resource),
            action: HotReloadAction::ReloadDependents,
            reason: reason.into(),
        }
    }

    pub fn restart(
        resource: ResourceId,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            resource: Some(resource),
            action: HotReloadAction::RestartComponent,
            reason: reason.into(),
        }
    }

    pub fn ignore(reason: impl Into<String>) -> Self {
        Self {
            resource: None,
            action: HotReloadAction::Ignore,
            reason: reason.into(),
        }
    }
}

#[derive(Debug, Default)]
pub struct HotReloadController;

impl HotReloadController {
    pub fn new() -> Self {
        Self
    }

    pub fn decide(
        &self,
        resource: ResourceId,
        kind: ResourceKind,
        change: &ResourceChange,
    ) -> HotReloadResult {
        match change.kind {
            ResourceChangeKind::Added => {
                Self::added(resource)
            }

            ResourceChangeKind::Modified => {
                Self::modified(resource, kind)
            }

            ResourceChangeKind::Removed => {
                Self::removed(resource, kind)
            }

            ResourceChangeKind::Renamed => {
                Self::renamed(resource, kind)
            }
        }
    }

    fn added(resource: ResourceId) -> HotReloadResult {
        HotReloadResult::reload(
            resource,
            "resource was added and can be loaded dynamically",
        )
    }

    fn modified(
        resource: ResourceId,
        kind: ResourceKind,
    ) -> HotReloadResult {
        match kind {
            ResourceKind::Theme
            | ResourceKind::Profile
            | ResourceKind::Workspace
            | ResourceKind::Layout
            | ResourceKind::Keybinding
            | ResourceKind::Configuration => {
                HotReloadResult::reload(
                    resource,
                    "resource can be reloaded without restarting Conduit",
                )
            }

            ResourceKind::Plugin
            | ResourceKind::Extension => {
                HotReloadResult::restart(
                    resource,
                    "code-bearing resources require component restart",
                )
            }

            ResourceKind::Script => {
                HotReloadResult::reload_dependents(
                    resource,
                    "script modification may affect dependent resources",
                )
            }

            ResourceKind::Unknown => {
                HotReloadResult::ignore(
                    "unknown resource type has no hot-reload policy",
                )
            }
        }
    }

    fn removed(
        resource: ResourceId,
        kind: ResourceKind,
    ) -> HotReloadResult {
        match kind {
            ResourceKind::Plugin
            | ResourceKind::Extension => {
                HotReloadResult::restart(
                    resource,
                    "removed code-bearing resource requires component restart",
                )
            }

            _ => HotReloadResult::reload(
                resource,
                "resource was removed",
            ),
        }
    }

    fn renamed(
        resource: ResourceId,
        _kind: ResourceKind,
    ) -> HotReloadResult {
        HotReloadResult::reload(
            resource,
            "resource was renamed and its registry entry must be refreshed",
        )
    }
}
