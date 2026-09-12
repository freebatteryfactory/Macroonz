use super::{
    CapturedOutput, PendingProcess, ProcessError, ProcessLimits, ProcessOutput, ProcessRequest,
    ProcessRun, ProcessStop, ResourceControl,
};
use std::path::PathBuf;
use std::time::Duration;

impl ProcessLimits {
    /// Admits a positive execution budget and representable output bounds.
    ///
    /// A zero cleanup budget submits termination and returns pending custody without polling.
    ///
    /// # Errors
    /// Refuses zero execution time or bounds beyond the addressable allocation range.
    pub fn informed(
        execution: Duration,
        cleanup: Duration,
        stdout: usize,
        stderr: usize,
    ) -> Result<Self, ProcessError> {
        if execution.is_zero()
            || isize::try_from(stdout).is_err()
            || isize::try_from(stderr).is_err()
        {
            return Err(ProcessError::InvalidRequest(
                "positive execution time and addressable output bounds are required".to_owned(),
            ));
        }
        Ok(Self {
            execution,
            cleanup,
            stdout,
            stderr,
        })
    }

    /// Returns the execution budget.
    #[must_use]
    pub const fn execution(self) -> Duration {
        self.execution
    }
    /// Returns the cleanup budget.
    #[must_use]
    pub const fn cleanup(self) -> Duration {
        self.cleanup
    }
    /// Returns the retained stdout bound.
    #[must_use]
    pub const fn stdout(self) -> usize {
        self.stdout
    }
    /// Returns the retained stderr bound.
    #[must_use]
    pub const fn stderr(self) -> usize {
        self.stderr
    }
}

impl ProcessRequest {
    /// Admits explicitly supplied process inputs without consulting the host.
    ///
    /// # Errors
    /// Refuses relative paths, NULs, invalid or duplicate environment keys and unsupported resource controls.
    pub fn informed(
        executable: PathBuf,
        directory: PathBuf,
        arguments: Vec<String>,
        environment: Vec<(String, String)>,
        limits: ProcessLimits,
        resources: &[ResourceControl],
    ) -> Result<Self, ProcessError> {
        if let Some(control) = resources.first() {
            return Err(ProcessError::Unsupported(*control));
        }
        if !executable.is_absolute()
            || !directory.is_absolute()
            || executable.as_os_str().as_encoded_bytes().contains(&0)
            || directory.as_os_str().as_encoded_bytes().contains(&0)
            || arguments.iter().any(|argument| argument.contains('\0'))
        {
            return Err(ProcessError::InvalidRequest(
                "absolute paths and NUL-free arguments are required".to_owned(),
            ));
        }
        let mut keys = std::collections::BTreeSet::new();
        for (key, value) in &environment {
            if key.is_empty()
                || !key.is_ascii()
                || key.contains(['=', '\0'])
                || value.contains('\0')
            {
                return Err(ProcessError::InvalidRequest(
                    "invalid environment entry".to_owned(),
                ));
            }
            let comparison = if cfg!(windows) {
                key.to_ascii_uppercase()
            } else {
                key.clone()
            };
            if !keys.insert(comparison) {
                return Err(ProcessError::InvalidRequest(
                    "duplicate environment key".to_owned(),
                ));
            }
        }
        Ok(Self {
            executable,
            directory,
            arguments,
            environment,
            limits,
        })
    }

    /// Returns the selected executable path.
    #[must_use]
    pub fn executable(&self) -> &std::path::Path {
        &self.executable
    }
    /// Returns the selected working directory.
    #[must_use]
    pub fn directory(&self) -> &std::path::Path {
        &self.directory
    }
    /// Returns the ordered process arguments.
    #[must_use]
    pub fn arguments(&self) -> &[String] {
        &self.arguments
    }
    /// Returns the complete caller-supplied environment.
    #[must_use]
    pub fn environment(&self) -> &[(String, String)] {
        &self.environment
    }
    /// Returns the selected process bounds.
    #[must_use]
    pub const fn limits(&self) -> ProcessLimits {
        self.limits
    }
}

impl CapturedOutput {
    /// Returns the retained prefix.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    /// Returns how capture ended.
    #[must_use]
    pub const fn end(&self) -> &super::CaptureEnd {
        &self.end
    }
}

impl ProcessOutput {
    /// Returns the direct child's operating-system exit status.
    #[must_use]
    pub const fn status(&self) -> std::process::ExitStatus {
        self.status
    }
    /// Returns the execution stop observation.
    #[must_use]
    pub const fn stop(&self) -> &ProcessStop {
        &self.stop
    }
    /// Returns stdout capture.
    #[must_use]
    pub const fn stdout(&self) -> &CapturedOutput {
        &self.stdout
    }
    /// Returns stderr capture.
    #[must_use]
    pub const fn stderr(&self) -> &CapturedOutput {
        &self.stderr
    }
}

impl PendingProcess {
    /// Returns the execution stop observation while cleanup remains unfinished.
    #[must_use]
    pub const fn stop(&self) -> &ProcessStop {
        &self.running.stop
    }
    /// Returns the latest cleanup operation failure, if any.
    #[must_use]
    pub fn cleanup_error(&self) -> Option<&str> {
        self.running.cleanup_error.as_deref()
    }
    /// Consumes the pending owner and retries cleanup for the supplied budget.
    pub fn finish(self, budget: Duration) -> ProcessRun {
        super::super::execute::finish(self.running, budget)
    }
}
