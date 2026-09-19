//! Process and terminal signal handling.
//!
//! The signal layer provides a platform-neutral API for the rest of
//! Conduit while keeping Unix-specific signal operations isolated here.

/// Signals that Conduit may send to a session process.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Signal {
    Interrupt,
    Terminate,
    Hangup,
    Kill,
    Stop,
    Continue,
    WindowResize,
}

impl Signal {
    #[cfg(unix)]
    fn as_raw(self) -> libc::c_int {
        match self {
            Self::Interrupt => libc::SIGINT,
            Self::Terminate => libc::SIGTERM,
            Self::Hangup => libc::SIGHUP,
            Self::Kill => libc::SIGKILL,
            Self::Stop => libc::SIGSTOP,
            Self::Continue => libc::SIGCONT,

            // Window resize is normally delivered through TIOCSWINSZ
            // followed by SIGWINCH.
            Self::WindowResize => libc::SIGWINCH,
        }
    }
}

/// Sends a signal to a process.
#[cfg(unix)]
pub fn send(pid: libc::pid_t, signal: Signal) -> std::io::Result<()> {
    let result = unsafe {
        libc::kill(pid, signal.as_raw())
    };

    if result != 0 {
        return Err(std::io::Error::last_os_error());
    }

    Ok(())
}

/// Sends an interrupt signal to a process.
#[cfg(unix)]
pub fn interrupt(pid: libc::pid_t) -> std::io::Result<()> {
    send(pid, Signal::Interrupt)
}

/// Sends a termination signal to a process.
#[cfg(unix)]
pub fn terminate(pid: libc::pid_t) -> std::io::Result<()> {
    send(pid, Signal::Terminate)
}

/// Sends a hangup signal to a process.
#[cfg(unix)]
pub fn hangup(pid: libc::pid_t) -> std::io::Result<()> {
    send(pid, Signal::Hangup)
}

/// Forcefully kills a process.
#[cfg(unix)]
pub fn kill(pid: libc::pid_t) -> std::io::Result<()> {
    send(pid, Signal::Kill)
}

/// Stops a process.
#[cfg(unix)]
pub fn stop(pid: libc::pid_t) -> std::io::Result<()> {
    send(pid, Signal::Stop)
}

/// Continues a stopped process.
#[cfg(unix)]
pub fn continue_process(pid: libc::pid_t) -> std::io::Result<()> {
    send(pid, Signal::Continue)
}
