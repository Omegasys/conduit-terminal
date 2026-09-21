#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClipboardPermission {
    Read,
    Write,
    WriteSensitive,
    PasteSensitive,
    Clear,
    HistoryRead,
    HistoryWrite,
}

#[derive(Debug, Clone)]
pub struct ClipboardSecurityPolicy {
    allow_read: bool,
    allow_write: bool,
    allow_write_sensitive: bool,
    allow_paste_sensitive: bool,
    allow_clear: bool,
    allow_history_read: bool,
    allow_history_write: bool,
}

impl Default for ClipboardSecurityPolicy {
    fn default() -> Self {
        Self {
            allow_read: true,
            allow_write: true,
            allow_write_sensitive: false,
            allow_paste_sensitive: false,
            allow_clear: true,
            allow_history_read: true,
            allow_history_write: true,
        }
    }
}

impl ClipboardSecurityPolicy {
    pub fn allows(&self, permission: ClipboardPermission) -> bool {
        match permission {
            ClipboardPermission::Read => self.allow_read,
            ClipboardPermission::Write => self.allow_write,
            ClipboardPermission::WriteSensitive => self.allow_write_sensitive,
            ClipboardPermission::PasteSensitive => self.allow_paste_sensitive,
            ClipboardPermission::Clear => self.allow_clear,
            ClipboardPermission::HistoryRead => self.allow_history_read,
            ClipboardPermission::HistoryWrite => self.allow_history_write,
        }
    }

    pub fn set(&mut self, permission: ClipboardPermission, allowed: bool) {
        match permission {
            ClipboardPermission::Read => self.allow_read = allowed,
            ClipboardPermission::Write => self.allow_write = allowed,
            ClipboardPermission::WriteSensitive => self.allow_write_sensitive = allowed,
            ClipboardPermission::PasteSensitive => self.allow_paste_sensitive = allowed,
            ClipboardPermission::Clear => self.allow_clear = allowed,
            ClipboardPermission::HistoryRead => self.allow_history_read = allowed,
            ClipboardPermission::HistoryWrite => self.allow_history_write = allowed,
        }
    }

    pub fn secure_defaults() -> Self {
        Self::default()
    }

    pub fn allow_all() -> Self {
        Self {
            allow_read: true,
            allow_write: true,
            allow_write_sensitive: true,
            allow_paste_sensitive: true,
            allow_clear: true,
            allow_history_read: true,
            allow_history_write: true,
        }
    }
}
