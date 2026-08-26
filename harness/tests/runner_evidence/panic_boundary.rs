//! Claims over subject-panic capture, hook chaining, thread correlation, nesting, and the process ceiling.

use super::support::{LaneFailure, binding, invocation, refused};
use macroonz_harness::report::{FailureClass, RunAttempt, TrialConclusion, TrialFinding};
use macroonz_harness::runner::{Invocation, SUBJECT_PANIC_CAUSE, run_one};
use std::io::Write;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Barrier, OnceLock};

const HOOK_CHILD_TEST: &str = "panic_boundary::hook_claim_child";
const ABORT_CHILD_TEST: &str = "panic_boundary::abort_claim_child";

static PRIOR_HOOK_CALLS: AtomicUsize = AtomicUsize::new(0usize);
static CONCURRENT_GATE: OnceLock<Barrier> = OnceLock::new();

struct ForeignPayload;

fn panics_with_text(_: &Invocation) -> TrialConclusion {
    let mut empty: Vec<u8> = Vec::new();
    let _removed = empty.remove(0usize);
    refused()
}

fn panics_with_foreign_payload(_: &Invocation) -> TrialConclusion {
    std::panic::resume_unwind(Box::new(ForeignPayload))
}

fn waits_then_vector_panics(_: &Invocation) -> TrialConclusion {
    let Some(gate) = CONCURRENT_GATE.get() else {
        return refused();
    };
    gate.wait();
    let mut empty: Vec<u8> = Vec::new();
    let _removed = empty.remove(0usize);
    refused()
}

fn waits_then_string_panics(_: &Invocation) -> TrialConclusion {
    let Some(gate) = CONCURRENT_GATE.get() else {
        return refused();
    };
    gate.wait();
    let mut empty = String::new();
    let _removed = empty.remove(1usize);
    refused()
}

fn nested_subject(_: &Invocation) -> TrialConclusion {
    let Ok(inner) = binding("nested-inner", panics_with_text) else {
        return refused();
    };
    let report = run_one(&inner, &invocation());
    match report.attempt() {
        RunAttempt::Executed(TrialConclusion::Refused(finding))
            if finding.class() == FailureClass::SubjectPanic
                && finding.cause() == SUBJECT_PANIC_CAUSE =>
        {
            TrialConclusion::Passed
        }
        RunAttempt::Executed(_)
        | RunAttempt::SkippedWithReason(_)
        | RunAttempt::TimedOut
        | RunAttempt::InfrastructureFailed(_) => refused(),
    }
}

fn aborts(_: &Invocation) -> TrialConclusion {
    let mut stdout = std::io::stdout().lock();
    if stdout
        .write_all(b"runner-abort-entered\n")
        .and_then(|()| stdout.flush())
        .is_err()
    {
        return refused();
    }
    std::process::abort()
}

fn subject_finding(
    report: &macroonz_harness::report::TrialReport,
) -> Result<&TrialFinding, LaneFailure> {
    let RunAttempt::Executed(TrialConclusion::Refused(finding)) = report.attempt() else {
        return Err(LaneFailure::Missing("typed subject-panic finding"));
    };
    assert_eq!(finding.class(), FailureClass::SubjectPanic);
    assert_eq!(finding.cause(), SUBJECT_PANIC_CAUSE);
    Ok(finding)
}

fn run_panicking(
    stem: &'static str,
    call: fn(&Invocation) -> TrialConclusion,
) -> Result<macroonz_harness::report::TrialReport, LaneFailure> {
    Ok(run_one(&binding(stem, call)?, &invocation()))
}

fn concurrent_findings() -> Result<(), LaneFailure> {
    if CONCURRENT_GATE.set(Barrier::new(3usize)).is_err() {
        return Err(LaneFailure::Missing("one concurrent barrier"));
    }
    let first = binding("concurrent-first", waits_then_vector_panics)?;
    let second = binding("concurrent-second", waits_then_string_panics)?;
    let first_invocation = invocation();
    let second_invocation = invocation();
    let first_thread = std::thread::spawn(move || run_one(&first, &first_invocation));
    let second_thread = std::thread::spawn(move || run_one(&second, &second_invocation));
    let Some(gate) = CONCURRENT_GATE.get() else {
        return Err(LaneFailure::Missing("concurrent barrier"));
    };
    gate.wait();
    let Ok(first_report) = first_thread.join() else {
        return Err(LaneFailure::Missing("first joined report"));
    };
    let Ok(second_report) = second_thread.join() else {
        return Err(LaneFailure::Missing("second joined report"));
    };
    let first_material = subject_finding(&first_report)?
        .foreign()
        .ok_or(LaneFailure::Missing("concurrent vector panic material"))?
        .shown();
    assert!(first_material.contains("removal index"));
    assert!(first_material.contains(file!()));
    let second_material = subject_finding(&second_report)?
        .foreign()
        .ok_or(LaneFailure::Missing("concurrent string panic material"))?
        .shown();
    assert!(second_material.contains("start byte index"));
    assert!(second_material.contains(file!()));
    Ok(())
}

fn child_hook_claims() -> Result<(), LaneFailure> {
    PRIOR_HOOK_CALLS.store(0usize, Ordering::SeqCst);
    let preceding = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |information| {
        PRIOR_HOOK_CALLS.fetch_add(1usize, Ordering::SeqCst);
        preceding(information);
    }));
    let text = run_panicking("text-panic", panics_with_text)?;
    let text_material = subject_finding(&text)?
        .foreign()
        .ok_or(LaneFailure::Missing("text panic material"))?
        .shown();
    assert!(text_material.contains("removal index"));
    assert!(text_material.contains(file!()));
    let foreign = run_panicking("foreign-panic", panics_with_foreign_payload)?;
    assert!(subject_finding(&foreign)?.foreign().is_none());
    concurrent_findings()?;
    let nested = run_panicking("nested", nested_subject)?;
    assert_eq!(
        nested.attempt(),
        &RunAttempt::Executed(TrialConclusion::Passed)
    );
    assert_eq!(PRIOR_HOOK_CALLS.load(Ordering::SeqCst), 4usize);
    let mut stdout = std::io::stdout().lock();
    stdout.write_all(b"runner-hook-claims-passed\n")?;
    stdout.flush()?;
    Ok(())
}

fn child_output(test: &str) -> Result<std::process::Output, LaneFailure> {
    let executable = std::env::current_exe()?;
    Ok(Command::new(executable)
        .arg("--exact")
        .arg(test)
        .arg("--ignored")
        .arg("--nocapture")
        .output()?)
}

/// Process-isolated child for hook installation, payload, nesting, and concurrency claims.
#[test]
#[ignore = "driven by the parent process-isolation claim"]
fn hook_claim_child() -> Result<(), LaneFailure> {
    child_hook_claims()
}

/// Process-isolated child that must terminate before returning from the runner.
#[test]
#[ignore = "driven by the parent process-isolation claim"]
fn abort_claim_child() -> Result<(), LaneFailure> {
    let _never_returns = run_panicking("abort", aborts)?;
    let mut stdout = std::io::stdout().lock();
    stdout.write_all(b"runner-abort-returned\n")?;
    stdout.flush()?;
    Ok(())
}

/// Claim: one chained hook preserves the preceding host hook while typed and foreign payloads remain safely distinct across nested and concurrent runs.
///
/// Evidence ceiling: this fresh child process controls first installation of the process-global hook; a later replacement may lawfully remove origin capture.
#[test]
fn subject_panics_preserve_hook_payload_origin_and_thread_custody() -> Result<(), LaneFailure> {
    let output = child_output(HOOK_CHILD_TEST)?;
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("runner-hook-claims-passed"));
    Ok(())
}

/// Claim: an abort cannot be flattened into an in-process trial finding because it terminates the process before the runner returns.
///
/// Evidence ceiling: stack overflow shares the non-unwind ceiling but is not induced because platform stack exhaustion is not a stable test input.
#[test]
fn abort_establishes_the_in_process_evidence_ceiling() -> Result<(), LaneFailure> {
    let output = child_output(ABORT_CHILD_TEST)?;
    assert!(!output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("runner-abort-entered"));
    assert!(!stdout.contains("runner-abort-returned"));
    Ok(())
}
