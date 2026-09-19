use std::env;
use std::path::PathBuf;

use super::resource::ResourceKind;

#[derive(Debug, Clone)]
pub struct ResourcePaths {
    config_dir: PathBuf,
    data_dir: PathBuf,
    cache_dir: PathBuf,
    resource_dir: PathBuf,
}

impl ResourcePaths {
    pub fn discover() -> Self {
        let home = env::var_os("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."));

        let config_dir = env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| home.join(".config"))
            .join("conduit");

        let data_dir = env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| home.join(".local").join("share"))
            .join("conduit");

        let cache_dir = env::var_os("XDG_CACHE_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| home.join(".cache"))
            .join("conduit");

        let resource_dir = data_dir.join("resources");

        Self {
            config_dir,
            data_dir,
            cache_dir,
            resource_dir,
        }
    }

    pub fn config_dir(&self) -> &PathBuf {
        &self.config_dir
    }

    pub fn data_dir(&self) -> &PathBuf {
        &self.data_dir
    }

    pub fn cache_dir(&self) -> &PathBuf {
        &self.cache_dir
    }

    pub fn resource_dir(&self) -> &PathBuf {
        &self.resource_dir
    }

    pub fn kind_dir(&self, kind: ResourceKind) -> PathBuf {
        self.resource_dir.join(kind.directory_name())
    }

    pub fn config_file(&self) -> PathBuf {
        self.config_dir.join("config.toml")
    }

    pub fn profiles_dir(&self) -> PathBuf {
        self.config_dir.join("profiles")
    }

    pub fn workspaces_dir(&self) -> PathBuf {
        self.data_dir.join("workspaces")
    }

    pub fn themes_dir(&self) -> PathBuf {
        self.resource_dir.join("themes")
    }

    pub fn plugins_dir(&self) -> PathBuf {
        self.resource_dir.join("plugins")
    }

    pub fn layouts_dir(&self) -> PathBuf {
        self.resource_dir.join("layouts")
    }

    pub fn keybindings_dir(&self) -> PathBuf {
        self.resource_dir.join("keybindings")
    }

    pub fn ensure_directories(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.config_dir)?;
        std::fs::create_dir_all(&self.data_dir)?;
        std::fs::create_dir_all(&self.cache_dir)?;
        std::fs::create_dir_all(&self.resource_dir)?;

        for kind in [
            ResourceKind::Theme,
            ResourceKind::Profile,
            ResourceKind::Workspace,
            ResourceKind::Plugin,
            ResourceKind::Layout,
            ResourceKind::Keybinding,
            ResourceKind::Extension,
        ] {
            std::fs::create_dir_all(self.kind_dir(kind))?;
        }

        Ok(())
    }
}

impl Default for ResourcePaths {
    fn default() -> Self {
        Self::discover()
    }
}
