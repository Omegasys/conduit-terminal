use std::env;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, Default)]
pub struct PathResolver;

impl PathResolver {
    pub fn new() -> Self {
        Self
    }

    pub fn home(&self) -> Option<PathBuf> {
        dirs_home()
    }

    pub fn resolve(&self, path: impl AsRef<Path>) -> io::Result<PathBuf> {
        let path = path.as_ref();

        let expanded = if path == Path::new("~") {
            self.home()
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::NotFound,
                        "home directory is unavailable",
                    )
                })?
        } else if let Some(stripped) = path
            .to_str()
            .and_then(|value| value.strip_prefix("~/"))
        {
            self.home()
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::NotFound,
                        "home directory is unavailable",
                    )
                })?
                .join(stripped)
        } else {
            path.to_path_buf()
        };

        if expanded.is_absolute() {
            Ok(expanded)
        } else {
            Ok(env::current_dir()?.join(expanded))
        }
    }

    pub fn canonicalize(
        &self,
        path: impl AsRef<Path>,
    ) -> io::Result<PathBuf> {
        self.resolve(path)?.canonicalize()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PathDisplay;

impl PathDisplay {
    pub fn new() -> Self {
        Self
    }

    pub fn display(&self, path: &Path) -> String {
        if let Some(home) = dirs_home() {
            if let Ok(relative) = path.strip_prefix(&home) {
                return if relative.as_os_str().is_empty() {
                    "~".to_owned()
                } else {
                    format!("~/{}", relative.display())
                };
            }
        }

        path.display().to_string()
    }
}

fn dirs_home() -> Option<PathBuf> {
    if let Some(home) = env::var_os("HOME") {
        return Some(PathBuf::from(home));
    }

    env::var_os("USERPROFILE").map(PathBuf::from)
}
