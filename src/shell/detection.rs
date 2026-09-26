//! Shell detection.
//!
//! Detection is intentionally conservative. Conduit identifies well-known
//! shell executables and otherwise reports an unknown shell rather than
//! pretending that an unknown shell is Bash-compatible.

use std::{
    env,
    fs,
    path::{Path, PathBuf},
};

/// Known shell families.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShellKind {
    Bash,
    Zsh,
    Fish,
    Dash,
    Ash,
    Yash,
    HeirloomSh,
    Ksh,
    Mksh,
    Csh,
    Tcsh,
    Nushell,
    Elvish,
    Xonsh,
    Oil,
    Ysh,
    Rc,
    Es,
    PowerShell,
    Sh,
    Unknown,
}

impl ShellKind {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Bash => "bash",
            Self::Zsh => "zsh",
            Self::Fish => "fish",
            Self::Dash => "dash",
            Self::Ash => "ash",
            Self::Yash => "yash",
            Self::HeirloomSh => "heirloom-sh",
            Self::Ksh => "ksh",
            Self::Mksh => "mksh",
            Self::Csh => "csh",
            Self::Tcsh => "tcsh",
            Self::Nushell => "nushell",
            Self::Elvish => "elvish",
            Self::Xonsh => "xonsh",
            Self::Oil => "oil",
            Self::Ysh => "ysh",
            Self::Rc => "rc",
            Self::Es => "es",
            Self::PowerShell => "powershell",
            Self::Sh => "sh",
            Self::Unknown => "unknown",
        }
    }

    pub fn from_executable(path: &Path) -> Self {
        let name = path
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase();

        match name.as_str() {
            "bash" => Self::Bash,
            "zsh" => Self::Zsh,
            "fish" => Self::Fish,
            "dash" => Self::Dash,
            "ash" => Self::Ash,
            "yash" => Self::Yash,
            "sh" => Self::Sh,
            "ksh" | "ksh93" => Self::Ksh,
            "mksh" => Self::Mksh,
            "csh" => Self::Csh,
            "tcsh" => Self::Tcsh,
            "nu" | "nushell" => Self::Nushell,
            "elvish" => Self::Elvish,
            "xonsh" => Self::Xonsh,
            "oil" => Self::Oil,
            "ysh" => Self::Ysh,
            "rc" => Self::Rc,
            "es" => Self::Es,
            "pwsh" | "powershell" => Self::PowerShell,
            "heirloom-sh" | "heirloomsh" => Self::HeirloomSh,
            _ => Self::Unknown,
        }
    }
}

/// Result of shell detection.
#[derive(Debug, Clone)]
pub struct DetectedShell {
    pub kind: ShellKind,
    pub executable: PathBuf,
    pub version: Option<String>,
    pub from_environment: bool,
}

impl DetectedShell {
    pub fn name(&self) -> &'static str {
        self.kind.name()
    }

    pub fn is_known(&self) -> bool {
        self.kind != ShellKind::Unknown
    }
}

/// Detects shells available to Conduit.
#[derive(Debug, Clone, Default)]
pub struct ShellDetector;

impl ShellDetector {
    pub fn new() -> Self {
        Self
    }

    /// Detect the user's current shell using the SHELL environment variable.
    pub fn detect_current(&self) -> Option<DetectedShell> {
        let shell = env::var_os("SHELL")?;
        let path = PathBuf::from(shell);

        if !path.exists() {
            return None;
        }

        Some(self.detect_path(&path, true))
    }

    /// Detect a shell from an executable path.
    pub fn detect_path(
        &self,
        path: &Path,
        from_environment: bool,
    ) -> DetectedShell {
        let kind = ShellKind::from_executable(path);

        let version = if path.exists() {
            detect_version(path, &kind)
        } else {
            None
        };

        DetectedShell {
            kind,
            executable: path.to_path_buf(),
            version,
            from_environment,
        }
    }

    /// Search PATH for a named shell.
    pub fn find(&self, name: &str) -> Option<DetectedShell> {
        let paths = env::var_os("PATH")?;

        for directory in env::split_paths(&paths) {
            let candidate = directory.join(name);

            if is_executable(&candidate) {
                return Some(self.detect_path(&candidate, false));
            }
        }

        None
    }

    /// Return common shells available on the current system.
    pub fn available(&self) -> Vec<DetectedShell> {
        let names = [
            "bash",
            "zsh",
            "fish",
            "dash",
            "ash",
            "yash",
            "ksh",
            "mksh",
            "csh",
            "tcsh",
            "nu",
            "elvish",
            "xonsh",
            "oil",
            "ysh",
            "rc",
            "es",
            "pwsh",
        ];

        names
            .iter()
            .filter_map(|name| self.find(name))
            .collect()
    }
}

fn is_executable(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        return fs::metadata(path)
            .map(|metadata| metadata.permissions().mode() & 0o111 != 0)
            .unwrap_or(false);
    }

    #[cfg(not(unix))]
    {
        true
    }
}

fn detect_version(
    path: &Path,
    kind: &ShellKind,
) -> Option<String> {
    let argument = match kind {
        ShellKind::Nushell => "--version",
        ShellKind::PowerShell => "--version",
        ShellKind::Elvish => "--version",
        ShellKind::Xonsh => "--version",
        _ => "--version",
    };

    let output = std::process::Command::new(path)
        .arg(argument)
        .output()
        .ok()?;

    let text = String::from_utf8_lossy(&output.stdout);

    text.lines()
        .chain(String::from_utf8_lossy(&output.stderr).lines())
        .find(|line| !line.trim().is_empty())
        .map(|line| line.trim().to_string())
}
