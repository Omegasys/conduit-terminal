use std::fmt;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResourceId(u64);

impl ResourceId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for ResourceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "resource-{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceKind {
    Theme,
    Profile,
    Workspace,
    Plugin,
    Layout,
    Keybinding,
    Configuration,
    Script,
    Extension,
    Unknown,
}

impl ResourceKind {
    pub fn directory_name(self) -> &'static str {
        match self {
            Self::Theme => "themes",
            Self::Profile => "profiles",
            Self::Workspace => "workspaces",
            Self::Plugin => "plugins",
            Self::Layout => "layouts",
            Self::Keybinding => "keybindings",
            Self::Configuration => "config",
            Self::Script => "scripts",
            Self::Extension => "extensions",
            Self::Unknown => "unknown",
        }
    }

    pub fn extension(self) -> Option<&'static str> {
        match self {
            Self::Theme => Some("tmol"),
            Self::Profile => Some("toml"),
            Self::Workspace => Some("toml"),
            Self::Plugin => Some("toml"),
            Self::Layout => Some("toml"),
            Self::Keybinding => Some("toml"),
            Self::Configuration => Some("toml"),
            Self::Script => None,
            Self::Extension => Some("toml"),
            Self::Unknown => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceState {
    Discovered,
    Loading,
    Loaded,
    Modified,
    Invalid,
    Disabled,
    Removed,
}

#[derive(Debug, Clone)]
pub struct ResourceMetadata {
    pub created: Option<SystemTime>,
    pub modified: Option<SystemTime>,
    pub size: u64,
    pub readonly: bool,
}

impl ResourceMetadata {
    pub fn from_path(path: &Path) -> Result<Self, ResourceError> {
        let metadata = std::fs::metadata(path)?;

        Ok(Self {
            created: metadata.created().ok(),
            modified: metadata.modified().ok(),
            size: metadata.len(),
            readonly: metadata.permissions().readonly(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Resource {
    id: ResourceId,
    kind: ResourceKind,
    name: String,
    path: PathBuf,
    state: ResourceState,
    metadata: Option<ResourceMetadata>,
    enabled: bool,
}

impl Resource {
    pub fn new(
        id: ResourceId,
        kind: ResourceKind,
        name: impl Into<String>,
        path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            id,
            kind,
            name: name.into(),
            path: path.into(),
            state: ResourceState::Discovered,
            metadata: None,
            enabled: true,
        }
    }

    pub fn id(&self) -> ResourceId {
        self.id
    }

    pub fn kind(&self) -> ResourceKind {
        self.kind
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn state(&self) -> ResourceState {
        self.state
    }

    pub fn metadata(&self) -> Option<&ResourceMetadata> {
        self.metadata.as_ref()
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn refresh_metadata(&mut self) -> Result<(), ResourceError> {
        self.metadata = Some(ResourceMetadata::from_path(&self.path)?);

        Ok(())
    }

    pub fn set_state(&mut self, state: ResourceState) {
        self.state = state;
    }

    pub fn enable(&mut self) {
        self.enabled = true;

        if self.state == ResourceState::Disabled {
            self.state = ResourceState::Loaded;
        }
    }

    pub fn disable(&mut self) {
        self.enabled = false;
        self.state = ResourceState::Disabled;
    }

    pub fn mark_modified(&mut self) {
        self.state = ResourceState::Modified;
    }

    pub fn mark_invalid(&mut self) {
        self.state = ResourceState::Invalid;
    }

    pub fn mark_removed(&mut self) {
        self.state = ResourceState::Removed;
    }
}

#[derive(Debug)]
pub enum ResourceError {
    Io(std::io::Error),
    InvalidPath(PathBuf),
    InvalidResource(String),
    AlreadyRegistered(ResourceId),
    NotFound(ResourceId),
}

impl fmt::Display for ResourceError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Io(error) => {
                write!(formatter, "resource I/O error: {error}")
            }

            Self::InvalidPath(path) => {
                write!(
                    formatter,
                    "invalid resource path: {}",
                    path.display()
                )
            }

            Self::InvalidResource(reason) => {
                write!(formatter, "invalid resource: {reason}")
            }

            Self::AlreadyRegistered(id) => {
                write!(formatter, "resource already registered: {id}")
            }

            Self::NotFound(id) => {
                write!(formatter, "resource not found: {id}")
            }
        }
    }
}

impl std::error::Error for ResourceError {}

impl From<std::io::Error> for ResourceError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}
