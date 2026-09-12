use super::types::{
    CaptureTask, PendingProcess, ProcessError, ProcessOutput, ProcessRequest, ProcessRun,
    ProcessStop, Running, Termination,
};
use super::{capture, platform};
use std::process::{Command, Stdio};
use std::time::Duration;

/// Executes one explicit invocation and retains bounded output and cleanup standing.
///
/// The caller supplies an already opened stdin file or selects null input with `None`.
/// The [owner contract](index.html) defines process-group and deadline limits.
///
/// # Errors
/// Refuses an unavailable platform, unrepresentable deadline, allocation failure or process startup failure.
pub fn run(
    request: &ProcessRequest,
    stdin: Option<std::fs::File>,
) -> Result<ProcessRun, ProcessError> {
    if !cfg!(any(windows, target_os = "linux", target_os = "macos")) {
        return Err(ProcessError::Unavailable);
    }
    let started = std::time::Instant::now();
    let deadline = started
        .checked_add(request.limits().execution())
        .ok_or_else(|| {
            ProcessError::InvalidRequest("unrepresentable execution deadline".to_owned())
        })?;
    if deadline.checked_add(request.limits().cleanup()).is_none() {
        return Err(ProcessError::InvalidRequest(
            "unrepresentable cleanup deadline".to_owned(),
        ));
    }
    let stdout_bytes = capture::reserve(request.limits().stdout()).map_err(ProcessError::Start)?;
    let stderr_bytes = capture::reserve(request.limits().stderr()).map_err(ProcessError::Start)?;
    let mut command = Command::new(request.executable());
    command
        .current_dir(request.directory())
        .args(request.arguments())
        .env_clear();
    for (key, value) in request.environment() {
        command.env(key, value);
    }
    command
        .stdin(stdin.map_or_else(Stdio::null, Stdio::from))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = platform::spawn(command).map_err(ProcessError::Start)?;
    let (stdout, stderr) = platform::pipes(&mut child);
    let mut running = Running {
        child,
        stdout: capture::start(stdout, stdout_bytes, request.limits().stdout()),
        stderr: capture::start(stderr, stderr_bytes, request.limits().stderr()),
        stop: ProcessStop::Exited,
        cleanup_error: None,
    };
    loop {
        capture::poll(&mut running.stdout);
        capture::poll(&mut running.stderr);
        if let Some(stop) =
            capture::stop(&running.stdout).or_else(|| capture::stop(&running.stderr))
        {
            running.stop = stop;
            break;
        }
        if std::time::Instant::now() >= deadline {
            running.stop = ProcessStop::Deadline;
            break;
        }
        match platform::exited(&mut running.child) {
            Ok(true) => break,
            Ok(false) => pause(deadline),
            Err(error) => {
                running.stop = ProcessStop::ObservationFailed(error.to_string());
                break;
            }
        }
    }
    Ok(finish(running, request.limits().cleanup()))
}

pub(super) fn finish(mut running: Running, budget: Duration) -> ProcessRun {
    let deadline = std::time::Instant::now().checked_add(budget);
    if budget.is_zero() {
        if let Err(error) = platform::terminate(&mut running.child) {
            running.cleanup_error = Some(error.to_string());
        }
        return ProcessRun::Pending(Box::new(PendingProcess { running }));
    }
    loop {
        match platform::terminate(&mut running.child)
            .and_then(|()| platform::reap(&mut running.child))
        {
            Ok(()) => running.cleanup_error = None,
            Err(error) => running.cleanup_error = Some(error.to_string()),
        }
        capture::poll(&mut running.stdout);
        capture::poll(&mut running.stderr);
        if running.child.termination == Termination::Requested
            && running.child.status.is_some()
            && matches!(&running.stdout, CaptureTask::Finished(_))
            && matches!(&running.stderr, CaptureTask::Finished(_))
        {
            return conclude(running);
        }
        let Some(end) = deadline else {
            running.cleanup_error = Some("unrepresentable cleanup deadline".to_owned());
            return ProcessRun::Pending(Box::new(PendingProcess { running }));
        };
        if std::time::Instant::now() >= end {
            return ProcessRun::Pending(Box::new(PendingProcess { running }));
        }
        pause(end);
    }
}

fn conclude(running: Running) -> ProcessRun {
    let Running {
        child,
        stdout,
        stderr,
        stop,
        cleanup_error,
    } = running;
    match (child.termination, child.status, stdout, stderr) {
        (
            Termination::Requested,
            Some(status),
            CaptureTask::Finished(stdout),
            CaptureTask::Finished(stderr),
        ) => {
            let stop = if stop == ProcessStop::Exited {
                capture::end_stop(&stdout)
                    .or_else(|| capture::end_stop(&stderr))
                    .unwrap_or(stop)
            } else {
                stop
            };
            ProcessRun::Finished(ProcessOutput {
                status,
                stop,
                stdout,
                stderr,
            })
        }
        (_, _, stdout, stderr) => ProcessRun::Pending(Box::new(PendingProcess {
            running: Running {
                child,
                stdout,
                stderr,
                stop,
                cleanup_error,
            },
        })),
    }
}

fn pause(deadline: std::time::Instant) {
    std::thread::sleep(
        deadline
            .saturating_duration_since(std::time::Instant::now())
            .min(Duration::from_millis(5)),
    );
}
