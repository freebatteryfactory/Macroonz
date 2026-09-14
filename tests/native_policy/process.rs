//! Independent operating-system controls through the public process entrance.

use macroonz::native_process::{
    self, CaptureEnd, ProcessError, ProcessLimits, ProcessOutput, ProcessRequest, ProcessRun,
    ProcessStop, ResourceControl,
};
use std::fs::File;
use std::io::{BufRead, Write};
use std::path::Path;
use std::time::{Duration, Instant};

pub(super) fn arguments(entry: &str) -> Vec<String> {
    [
        "--exact",
        entry,
        "--ignored",
        "--nocapture",
        "--test-threads=1",
    ]
    .map(str::to_owned)
    .to_vec()
}

pub(super) fn invocation(root: &Path, limits: ProcessLimits) -> Result<ProcessRequest, String> {
    ProcessRequest::informed(
        std::env::current_exe().map_err(|error| error.to_string())?,
        root.to_path_buf(),
        arguments("process_child::subject"),
        Vec::new(),
        limits,
        &[],
    )
    .map_err(|error| error.to_string())
}

pub(super) fn input(root: &Path, mode: &str) -> Result<File, String> {
    let path = root.join(format!("{mode}.input"));
    std::fs::write(&path, format!("{mode}\n{}\n", root.display()))
        .map_err(|error| error.to_string())?;
    File::open(path).map_err(|error| error.to_string())
}

pub(super) fn completed(result: ProcessRun) -> Result<ProcessOutput, String> {
    match result {
        ProcessRun::Finished(output) => Ok(output),
        ProcessRun::Pending(pending) => {
            let detail = format!(
                "unexpected pending cleanup: {:?}; {:?}",
                pending.stop(),
                pending.cleanup_error()
            );
            let retried = pending.finish(Duration::from_secs(5));
            drop(retried);
            Err(detail)
        }
    }
}

pub(super) fn limits(
    execution: Duration,
    cleanup: Duration,
    stdout: usize,
    stderr: usize,
) -> Result<ProcessLimits, String> {
    ProcessLimits::informed(execution, cleanup, stdout, stderr).map_err(|error| error.to_string())
}

#[test]
fn concurrent_capture_drains_stderr_before_stdout_without_deadlock() -> Result<(), String> {
    let root = super::check::scratch()?;
    let request = invocation(
        &root,
        limits(
            Duration::from_secs(8),
            Duration::from_secs(3),
            500_000,
            500_000,
        )?,
    )?;
    let output = completed(
        native_process::run(&request, Some(input(&root, "finite")?))
            .map_err(|error| error.to_string())?,
    )?;
    assert_eq!(output.stop(), &ProcessStop::Exited);
    assert!(output.status().success());
    assert_eq!(output.stdout().end(), &CaptureEnd::Eof);
    assert_eq!(output.stderr().end(), &CaptureEnd::Eof);
    let expected = vec![0x80; 400_000];
    assert_eq!(
        output
            .stdout()
            .bytes()
            .split(u8::is_ascii)
            .filter(|chunk| !chunk.is_empty())
            .collect::<Vec<_>>(),
        [expected.as_slice()]
    );
    assert_eq!(output.stderr().bytes(), vec![0x81; 300_000]);
    Ok(())
}

#[test]
fn floods_stop_at_independent_pipe_bounds() -> Result<(), String> {
    for mode in ["flood-out", "flood-err"] {
        let root = super::check::scratch()?;
        let request = invocation(
            &root,
            limits(Duration::from_secs(5), Duration::from_secs(3), 4096, 3072)?,
        )?;
        let started = Instant::now();
        let output = completed(
            native_process::run(&request, Some(input(&root, mode)?))
                .map_err(|error| error.to_string())?,
        )?;
        assert!(started.elapsed() < Duration::from_secs(5));
        assert_eq!(output.stop(), &ProcessStop::OutputLimit);
        assert!(output.stdout().bytes().len() <= 4096);
        assert!(output.stderr().bytes().len() <= 3072);
        let capture = if mode == "flood-out" {
            output.stdout()
        } else {
            output.stderr()
        };
        assert_eq!(capture.end(), &CaptureEnd::LimitExceeded);
        assert_eq!(
            capture.bytes().len(),
            if mode == "flood-out" { 4096 } else { 3072 }
        );
    }
    Ok(())
}

#[test]
fn deadline_terminates_a_silent_child_and_reaps_it() -> Result<(), String> {
    let root = super::check::scratch()?;
    let request = invocation(
        &root,
        limits(
            Duration::from_millis(300),
            Duration::from_secs(3),
            2048,
            2048,
        )?,
    )?;
    let started = Instant::now();
    let output = completed(
        native_process::run(&request, Some(input(&root, "sleep")?))
            .map_err(|error| error.to_string())?,
    )?;
    assert!(started.elapsed() >= Duration::from_millis(300));
    assert!(started.elapsed() < Duration::from_secs(3));
    assert_eq!(output.stop(), &ProcessStop::Deadline);
    assert!(!output.status().success());
    assert!(String::from_utf8_lossy(output.stdout().bytes()).contains("subject-ready"));
    assert_eq!(output.stdout().end(), &CaptureEnd::Eof);
    assert_eq!(output.stderr().end(), &CaptureEnd::Eof);
    Ok(())
}

#[test]
fn unsupported_controls_and_malformed_requests_refuse_before_spawn() -> Result<(), String> {
    let root = super::check::scratch()?;
    let bounds = limits(Duration::from_secs(1), Duration::ZERO, 0, 0)?;
    for control in [
        ResourceControl::MemoryBytes(1024),
        ResourceControl::CpuTime(Duration::from_secs(1)),
        ResourceControl::DenyNetwork,
        ResourceControl::PreventGroupEscape,
    ] {
        assert!(
            matches!(ProcessRequest::informed(root.join("missing"), root.clone(), Vec::new(), Vec::new(), bounds, &[control]), Err(ProcessError::Unsupported(found)) if found == control)
        );
    }
    for (executable, directory, arguments, environment) in [
        ("relative".into(), root.clone(), Vec::new(), Vec::new()),
        (
            root.join("missing"),
            "relative".into(),
            Vec::new(),
            Vec::new(),
        ),
        (
            root.join("missing"),
            root.clone(),
            vec!["bad\0arg".to_owned()],
            Vec::new(),
        ),
        (
            root.join("missing"),
            root.clone(),
            Vec::new(),
            vec![("a=b".to_owned(), "value".to_owned())],
        ),
        (
            root.join("missing"),
            root.clone(),
            Vec::new(),
            vec![
                ("A".to_owned(), "1".to_owned()),
                ("A".to_owned(), "2".to_owned()),
            ],
        ),
    ] {
        assert!(matches!(
            ProcessRequest::informed(executable, directory, arguments, environment, bounds, &[]),
            Err(ProcessError::InvalidRequest(_))
        ));
    }
    assert!(ProcessLimits::informed(Duration::ZERO, Duration::ZERO, 0, 0).is_err());
    assert!(
        ProcessLimits::informed(Duration::from_secs(1), Duration::ZERO, usize::MAX, 0).is_err()
    );
    let missing = ProcessRequest::informed(
        root.join("missing"),
        root,
        Vec::new(),
        Vec::new(),
        bounds,
        &[],
    )
    .map_err(|error| error.to_string())?;
    assert!(matches!(
        native_process::run(&missing, None),
        Err(ProcessError::Start(_))
    ));
    Ok(())
}

#[test]
fn nonzero_exit_is_preserved_without_becoming_a_subject_verdict() -> Result<(), String> {
    let root = super::check::scratch()?;
    let request = invocation(
        &root,
        limits(Duration::from_secs(3), Duration::from_secs(3), 4096, 4096)?,
    )?;
    let output = completed(
        native_process::run(&request, Some(input(&root, "fail")?))
            .map_err(|error| error.to_string())?,
    )?;
    assert_eq!(output.stop(), &ProcessStop::Exited);
    assert!(!output.status().success());
    assert!(String::from_utf8_lossy(output.stderr().bytes()).contains("declared-child-failure"));
    Ok(())
}

#[test]
fn zero_output_bound_detects_a_write_without_retaining_bytes() -> Result<(), String> {
    let root = super::check::scratch()?;
    let request = invocation(
        &root,
        limits(Duration::from_secs(3), Duration::from_secs(3), 0, 0)?,
    )?;
    let output = completed(
        native_process::run(&request, Some(input(&root, "finite")?))
            .map_err(|error| error.to_string())?,
    )?;
    assert_eq!(output.stop(), &ProcessStop::OutputLimit);
    assert!(output.stdout().bytes().is_empty());
    assert!(output.stderr().bytes().is_empty());
    Ok(())
}

pub(super) fn ready(reader: impl BufRead) -> Result<(), String> {
    for line in reader.lines().take(8) {
        if line
            .map_err(|error| error.to_string())?
            .contains("subject-ready")
        {
            return Ok(());
        }
    }
    Err("no child readiness marker".to_owned())
}

pub(super) fn mark_ready() -> Result<(), String> {
    let mut out = std::io::stdout().lock();
    writeln!(out, "subject-ready").map_err(|error| error.to_string())?;
    out.flush().map_err(|error| error.to_string())
}
