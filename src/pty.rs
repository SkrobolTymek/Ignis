#[cfg(unix)]
mod unix_pty {
    use nix::{
        unistd::{fork, ForkResult},
        fcntl::{self, OFlag},
        sys::stat::Mode,
        errno::Errno,
    };
    use std::os::unix::io::RawFd;
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum PtyError {
        #[error("PTY operation failed: {0}")]
        OperationFailed(#[from] nix::Error),
        #[error("Fork failed")]
        ForkFailed,
    }

    pub struct Pty {
        master: RawFd,
    }

    impl Pty {
        pub fn new() -> Result<Self, PtyError> {
            let master = unsafe { libc::posix_openpt(OFlag::O_RDWR.bits()) };
            if master == -1 {
                return Err(PtyError::OperationFailed(nix::Error::last()));
            }

            unsafe {
                if libc::grantpt(master) != 0 || libc::unlockpt(master) != 0 {
                    libc::close(master);
                    return Err(PtyError::OperationFailed(nix::Error::last()));
                }
            }

            match fork()? {
                ForkResult::Parent { child: _ } => Ok(Self { master }),
                ForkResult::Child => {
                    let slave_name = unsafe {
                        std::ffi::CStr::from_ptr(libc::ptsname(master))
                            .to_string_lossy()
                            .into_owned()
                    };

                    let slave = fcntl::open(&slave_name, OFlag::O_RDWR, Mode::empty())?;

                    nix::unistd::dup2(slave, libc::STDIN_FILENO)?;
                    nix::unistd::dup2(slave, libc::STDOUT_FILENO)?;
                    nix::unistd::dup2(slave, libc::STDERR_FILENO)?;

                    if slave > libc::STDERR_FILENO {
                        unsafe { libc::close(slave) };
                    }

                    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
                    nix::unistd::execvp(shell.as_str(), &[])?;

                    unreachable!();
                }
            }
        }

        pub fn send_command(&self, command: &str) {
            unsafe {
                let mut file = std::fs::File::from_raw_fd(self.master);
                let _ = file.write_all(command.as_bytes());
                let _ = file.write_all(b"\n");
                std::mem::forget(file);
            }
        }
    }
}

#[cfg(windows)]
mod windows_pty {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum PtyError {
        #[error("PTY not supported on Windows")]
        Unsupported,
    }

    pub struct Pty;

    impl Pty {
        pub fn new() -> Result<Self, PtyError> {
            Err(PtyError::Unsupported)
        }

        pub fn send_command(&self, _command: &str) {}
    }
}

#[cfg(unix)]
pub use unix_pty::{Pty, PtyError};

#[cfg(windows)]
pub use windows_pty::{Pty, PtyError};