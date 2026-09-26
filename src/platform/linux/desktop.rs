use std::env;

/// Linux desktop environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopEnvironment {
    Gnome,
    Kde,
    Xfce,
    Cinnamon,
    Mate,
    Lxde,
    Lxqt,
    Budgie,
    Cosmic,
    Deepin,
    Pantheon,
    Enlightenment,
    Hyprland,
    Sway,
    Unknown,
}

impl DesktopEnvironment {
    pub fn detect() -> Self {
        let value = env::var("XDG_CURRENT_DESKTOP")
            .or_else(|_| env::var("XDG_SESSION_DESKTOP"))
            .unwrap_or_default()
            .to_ascii_lowercase();

        if value.contains("gnome") {
            Self::Gnome
        } else if value.contains("kde") || value.contains("plasma") {
            Self::Kde
        } else if value.contains("xfce") {
            Self::Xfce
        } else if value.contains("cinnamon") {
            Self::Cinnamon
        } else if value.contains("mate") {
            Self::Mate
        } else if value.contains("lxqt") {
            Self::Lxqt
        } else if value.contains("lxde") {
            Self::Lxde
        } else if value.contains("budgie") {
            Self::Budgie
        } else if value.contains("cosmic") {
            Self::Cosmic
        } else if value.contains("deepin") {
            Self::Deepin
        } else if value.contains("pantheon") {
            Self::Pantheon
        } else if value.contains("enlightenment") {
            Self::Enlightenment
        } else if value.contains("hyprland") {
            Self::Hyprland
        } else if value.contains("sway") {
            Self::Sway
        } else {
            Self::Unknown
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Self::Gnome => "GNOME",
            Self::Kde => "KDE Plasma",
            Self::Xfce => "Xfce",
            Self::Cinnamon => "Cinnamon",
            Self::Mate => "MATE",
            Self::Lxde => "LXDE",
            Self::Lxqt => "LXQt",
            Self::Budgie => "Budgie",
            Self::Cosmic => "COSMIC",
            Self::Deepin => "Deepin",
            Self::Pantheon => "Pantheon",
            Self::Enlightenment => "Enlightenment",
            Self::Hyprland => "Hyprland",
            Self::Sway => "Sway",
            Self::Unknown => "Unknown",
        }
    }
}

/// Linux display server.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayServer {
    Wayland,
    X11,
    Unknown,
}

impl DisplayServer {
    pub fn detect() -> Self {
        match env::var("XDG_SESSION_TYPE")
            .unwrap_or_default()
            .to_ascii_lowercase()
            .as_str()
        {
            "wayland" => Self::Wayland,
            "x11" => Self::X11,
            _ if env::var_os("WAYLAND_DISPLAY").is_some() => Self::Wayland,
            _ if env::var_os("DISPLAY").is_some() => Self::X11,
            _ => Self::Unknown,
        }
    }
}

/// Current Linux desktop session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinuxSession {
    pub desktop: DesktopEnvironment,
    pub display_server: DisplayServer,
    pub session_type: Option<String>,
    pub desktop_name: Option<String>,
}

impl LinuxSession {
    pub fn detect() -> Self {
        Self {
            desktop: DesktopEnvironment::detect(),
            display_server: DisplayServer::detect(),
            session_type: env::var("XDG_SESSION_TYPE").ok(),
            desktop_name: env::var("XDG_CURRENT_DESKTOP")
                .or_else(|_| env::var("XDG_SESSION_DESKTOP"))
                .ok(),
        }
    }
}

/// High-level Linux desktop integration.
#[derive(Debug, Clone)]
pub struct LinuxDesktop {
    session: LinuxSession,
}

impl Default for LinuxDesktop {
    fn default() -> Self {
        Self::detect()
    }
}

impl LinuxDesktop {
    pub fn detect() -> Self {
        Self {
            session: LinuxSession::detect(),
        }
    }

    pub fn session(&self) -> &LinuxSession {
        &self.session
    }

    pub fn desktop(&self) -> DesktopEnvironment {
        self.session.desktop
    }

    pub fn display_server(&self) -> DisplayServer {
        self.session.display_server
    }

    pub fn is_wayland(&self) -> bool {
        self.display_server() == DisplayServer::Wayland
    }

    pub fn is_x11(&self) -> bool {
        self.display_server() == DisplayServer::X11
    }
}
