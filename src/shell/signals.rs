/// Portable representation of common process signals.
///
/// Numeric values are provided for Unix environments, but Conduit should
/// generally use this enum rather than passing raw signal numbers around.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShellSignal {
    Interrupt,
    Terminate,
    Hangup,
    Quit,
    Stop,
    Continue,
    Kill,
    WindowResize,
    User1,
    User2,
}

impl ShellSignal {
    #[cfg(unix)]
    pub const fn unix_number(self) -> i32 {
        match self {
            Self::Hangup => 1,
            Self::Interrupt => 2,
            Self::Quit => 3,
            Self::Kill => 9,
            Self::Terminate => 15,
            Self::Stop => 19,
            Self::Continue => 18,
            Self::User1 => 10,
            Self::User2 => 12,
            Self::WindowResize => 28,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Self::Interrupt => "INT",
            Self::Terminate => "TERM",
            Self::Hangup => "HUP",
            Self::Quit => "QUIT",
            Self::Stop => "STOP",
            Self::Continue => "CONT",
            Self::Kill => "KILL",
            Self::WindowResize => "WINCH",
            Self::User1 => "USR1",
            Self::User2 => "USR2",
        }
    }
}

/// Policy controlling which signals Conduit may forward.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignalPolicy {
    pub forward_interrupt: bool,
    pub forward_terminate: bool,
    pub forward_hangup: bool,
    pub forward_resize: bool,
}

impl SignalPolicy {
    pub const fn permissive() -> Self {
        Self {
            forward_interrupt: true,
            forward_terminate: true,
            forward_hangup: true,
            forward_resize: true,
        }
    }

    pub const fn terminal_default() -> Self {
        Self {
            forward_interrupt: true,
            forward_terminate: true,
            forward_hangup: true,
            forward_resize: true,
        }
    }

    pub const fn deny_all() -> Self {
        Self {
            forward_interrupt: false,
            forward_terminate: false,
            forward_hangup: false,
            forward_resize: false,
        }
    }

    pub const fn allows(self, signal: ShellSignal) -> bool {
        match signal {
            ShellSignal::Interrupt => self.forward_interrupt,
            ShellSignal::Terminate => self.forward_terminate,
            ShellSignal::Hangup => self.forward_hangup,
            ShellSignal::WindowResize => self.forward_resize,
            _ => true,
        }
    }
}

impl Default for SignalPolicy {
    fn default() -> Self {
        Self::terminal_default()
    }
}

/// Platform-independent signal request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignalRequest {
    pub signal: ShellSignal,
    pub process_id: u32,
}

impl SignalRequest {
    pub const fn new(process_id: u32, signal: ShellSignal) -> Self {
        Self {
            signal,
            process_id,
        }
    }
}
