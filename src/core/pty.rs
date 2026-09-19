//! Unix pseudo-terminal implementation.
//!
//! The PTY is the boundary between Conduit and the operating-system
//! terminal process. Terminal output flows:
//
//! shell/process -> PTY master -> terminal parser -> Screen
//!
//! Input flows:
//
//! keyboard -> Terminal -> PTY master -> shell/process

use std::io;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd};

/// PTY configuration.
#[derive(Debug, Clone)]
pub struct PtyConfig {
    pub rows: u16,
    pub columns: u16,
}

impl Default for PtyConfig {
    fn default() -> Self {
        Self {
            rows: 24,
            columns: 80,
        }
    }
}

/// Unix PTY master.
pub struct Pty {
    master: OwnedFd,
    pid: libc::pid_t,
}

impl Pty {
    /// Creates a new PTY and launches the requested program.
    #[cfg(unix)]
    pub fn spawn(
        program: &str,
        arguments: &[String],
        config: &PtyConfig,
        working_directory: Option<&str>,
        environment: &[(String, String)],
    ) -> io::Result<Self> {
        let mut master: RawFd = -1;
        let mut slave: RawFd = -1;

        let mut window_size = libc::winsize {
            ws_row: config.rows,
            ws_col: config.columns,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };

        let result = unsafe {
            libc::openpty(
                &mut master,
                &mut slave,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut window_size,
            )
        };

        if result != 0 {
            return Err(io::Error::last_os_error());
        }

        let pid = unsafe { libc::fork() };

        if pid < 0 {
            unsafe {
                libc::close(master);
                libc::close(slave);
            }

            return Err(io::Error::last_os_error());
        }

        if pid == 0 {
            child_process(
                slave,
                program,
                arguments,
                working_directory,
                environment,
            );
        }

        unsafe {
            libc::close(slave);
        }

        let master = unsafe { OwnedFd::from_raw_fd(master) };

        set_nonblocking(master.as_raw_fd())?;

        Ok(Self { master, pid })
    }

    /// Returns the child process ID.
    pub fn pid(&self) -> libc::pid_t {
        self.pid
    }

    /// Reads available bytes from the PTY.
    ///
    /// An empty vector means that no data is currently available when
    /// the PTY is operating in nonblocking mode.
    pub fn read(&self, buffer: &mut [u8]) -> io::Result<usize> {
        let result = unsafe {
            libc::read(
                self.master.as_raw_fd(),
                buffer.as_mut_ptr() as *mut libc::c_void,
                buffer.len(),
            )
        };

        if result < 0 {
            let error = io::Error::last_os_error();

            if error.kind() == io::ErrorKind::WouldBlock {
                return Ok(0);
            }

            return Err(error);
        }

        Ok(result as usize)
    }

    /// Writes bytes to the child process through the PTY.
    pub fn write(&self, data: &[u8]) -> io::Result<usize> {
        let result = unsafe {
            libc::write(
                self.master.as_raw_fd(),
                data.as_ptr() as *const libc::c_void,
                data.len(),
            )
        };

        if result < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(result as usize)
    }

    /// Resizes the terminal window.
    pub fn resize(&self, rows: u16, columns: u16) -> io::Result<()> {
        let window_size = libc::winsize {
            ws_row: rows,
            ws_col: columns,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };

        let result = unsafe {
            libc::ioctl(
                self.master.as_raw_fd(),
                libc::TIOCSWINSZ,
                &window_size,
            )
        };

        if result != 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    /// Returns the underlying file descriptor.
    pub fn as_raw_fd(&self) -> RawFd {
        self.master.as_raw_fd()
    }
}

#[cfg(unix)]
fn child_process(
    slave: RawFd,
    program: &str,
    arguments: &[String],
    working_directory: Option<&str>,
    environment: &[(String, String)],
) -> ! {
    unsafe {
        libc::close(STDIN_FILENO);

        libc::dup2(slave, STDIN_FILENO);
        libc::dup2(slave, STDOUT_FILENO);
        libc::dup2(slave, STDERR_FILENO);

        if slave > STDERR_FILENO {
            libc::close(slave);
        }

        libc::setsid();

        #[cfg(any(target_os = "linux", target_os = "android"))]
        {
            libc::ioctl(STDIN_FILENO, libc::TIOCSCTTY, 0);
        }
    }

    if let Some(directory) = working_directory {
        if let Ok(path) = std::ffi::CString::new(directory) {
            unsafe {
                libc::chdir(path.as_ptr());
            }
        }
    }

    for (key, value) in environment {
        let Ok(key) = std::ffi::CString::new(key.as_str()) else {
            continue;
        };

        let Ok(value) = std::ffi::CString::new(value.as_str()) else {
            continue;
        };

        unsafe {
            libc::setenv(key.as_ptr(), value.as_ptr(), 1);
        }
    }

    let Ok(program_c) = std::ffi::CString::new(program) else {
        unsafe {
            libc::_exit(127);
        }
    };

    let mut argv = Vec::with_capacity(arguments.len() + 2);

    argv.push(program_c.clone());

    for argument in arguments {
        let Ok(argument) = std::ffi::CString::new(argument.as_str()) else {
            unsafe {
                libc::_exit(127);
            }
        };

        argv.push(argument);
    }

    let mut pointers: Vec<*const libc::c_char> =
        argv.iter().map(|argument| argument.as_ptr()).collect();

    pointers.push(std::ptr::null());

    unsafe {
        libc::execvp(program_c.as_ptr(), pointers.as_ptr());

        libc::_exit(127);
    }
}

#[cfg(unix)]
fn set_nonblocking(fd: RawFd) -> io::Result<()> {
    let flags = unsafe { libc::fcntl(fd, libc::F_GETFL) };

    if flags < 0 {
        return Err(io::Error::last_os_error());
    }

    let result = unsafe {
        libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK)
    };

    if result < 0 {
        return Err(io::Error::last_os_error());
    }

    Ok(())
}

const STDIN_FILENO: RawFd = 0;
const STDERR_FILENO: RawFd = 2;
