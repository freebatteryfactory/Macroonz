use super::types::{PlatformChild, Termination};
use std::io;
use std::process::{ChildStderr, ChildStdout, Command};

#[cfg(windows)]
pub(super) fn spawn(command: Command) -> io::Result<PlatformChild> {
    use process_wrap::std::{CommandWrap, CreationFlags, JobObject};
    let flags = CreationFlags(windows::Win32::System::Threading::CREATE_NO_WINDOW);
    let mut wrapped = CommandWrap::from(command);
    wrapped.wrap(flags).wrap(JobObject);
    Ok(PlatformChild {
        inner: wrapped.spawn()?,
        termination: Termination::Required,
        status: None,
    })
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub(super) fn spawn(mut command: Command) -> io::Result<PlatformChild> {
    use std::os::unix::process::CommandExt;
    command.process_group(0);
    Ok(PlatformChild {
        inner: command.spawn()?,
        termination: Termination::Required,
        status: None,
    })
}

#[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
pub(super) fn spawn(_command: Command) -> io::Result<PlatformChild> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "native process backend unavailable",
    ))
}

pub(super) fn pipes(child: &mut PlatformChild) -> (Option<ChildStdout>, Option<ChildStderr>) {
    #[cfg(windows)]
    {
        (child.inner.stdout().take(), child.inner.stderr().take())
    }
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        (child.inner.stdout.take(), child.inner.stderr.take())
    }
    #[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
    {
        let _child = child;
        (None, None)
    }
}

pub(super) fn exited(child: &mut PlatformChild) -> io::Result<bool> {
    #[cfg(windows)]
    {
        child.status = child.inner.inner_mut().try_wait()?;
        Ok(child.status.is_some())
    }
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        use rustix::process::{WaitId, WaitIdOptions, waitid};
        let observation = waitid(
            WaitId::Pid(rustix::process::Pid::from_child(&child.inner)),
            WaitIdOptions::EXITED | WaitIdOptions::NOHANG | WaitIdOptions::NOWAIT,
        )?;
        Ok(observation.is_some())
    }
    #[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
    {
        let _child = child;
        Err(io::ErrorKind::Unsupported.into())
    }
}

pub(super) fn terminate(child: &mut PlatformChild) -> io::Result<()> {
    if child.termination == Termination::Requested {
        return Ok(());
    }
    #[cfg(windows)]
    child.inner.start_kill()?;
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        let pid = rustix::process::Pid::from_child(&child.inner);
        if pid.is_init() {
            return Err(io::Error::other("refused init process group"));
        }
        match rustix::process::kill_process_group(pid, rustix::process::Signal::KILL) {
            Ok(()) | Err(rustix::io::Errno::SRCH) => {}
            Err(error) => return Err(error.into()),
        }
    }
    child.termination = Termination::Requested;
    Ok(())
}

pub(super) fn reap(child: &mut PlatformChild) -> io::Result<()> {
    if child.termination != Termination::Requested || child.status.is_some() {
        return Ok(());
    }
    #[cfg(windows)]
    {
        child.status = child.inner.inner_mut().try_wait()?;
    }
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        child.status = child.inner.try_wait()?;
    }
    Ok(())
}
