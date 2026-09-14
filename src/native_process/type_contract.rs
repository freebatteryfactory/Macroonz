use super::types::{PlatformChild, ProcessError};

impl std::fmt::Display for ProcessError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRequest(reason) => write!(formatter, "invalid process request: {reason}"),
            Self::Unavailable => formatter.write_str("native process execution unavailable"),
            Self::Unsupported(control) => {
                write!(formatter, "unsupported process control: {control:?}")
            }
            Self::Start(error) => write!(formatter, "process startup failed: {error}"),
        }
    }
}

impl std::error::Error for ProcessError {}

impl Drop for PlatformChild {
    fn drop(&mut self) {
        drop(super::platform::terminate(self));
        drop(super::platform::reap(self));
    }
}
