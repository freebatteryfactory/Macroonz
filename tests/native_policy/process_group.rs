//! Descendant cleanup observed by an independently held operating-system file lock.

use super::process::{completed, input, invocation, limits};
use macroonz::native_process::{self, ProcessRun, ProcessStop};
use std::time::{Duration, Instant};

#[test]
fn termination_reaches_grandchildren_after_deadline_and_after_leader_exit() -> Result<(), String> {
    for mode in ["parent-exit", "parent-sleep"] {
        let root = super::check::scratch()?;
        let request = invocation(
            &root,
            limits(Duration::from_secs(2), Duration::from_secs(3), 4096, 4096)?,
        )?;
        let started = Instant::now();
        let output = completed(
            native_process::run(&request, Some(input(&root, mode)?))
                .map_err(|error| error.to_string())?,
        )?;
        assert!(started.elapsed() < Duration::from_secs(5));
        assert_eq!(
            output.stop(),
            if mode == "parent-exit" {
                &ProcessStop::Exited
            } else {
                &ProcessStop::Deadline
            }
        );
        assert!(String::from_utf8_lossy(output.stdout().bytes()).contains("subject-ready"));
        assert!(root.join("grandchild.pid").is_file());
        let held = std::fs::OpenOptions::new()
            .write(true)
            .open(root.join("held.lock"))
            .map_err(|error| error.to_string())?;
        held.try_lock()
            .map_err(|error| format!("grandchild retained lock after cleanup: {error}"))?;
    }
    Ok(())
}

#[test]
fn zero_cleanup_budget_preserves_retryable_custody() -> Result<(), String> {
    let root = super::check::scratch()?;
    let request = invocation(
        &root,
        limits(Duration::from_millis(300), Duration::ZERO, 4096, 4096)?,
    )?;
    let result = native_process::run(&request, Some(input(&root, "sleep")?))
        .map_err(|error| error.to_string())?;
    let output = match result {
        ProcessRun::Pending(pending) => {
            assert_eq!(pending.stop(), &ProcessStop::Deadline);
            completed(pending.finish(Duration::from_secs(3)))?
        }
        ProcessRun::Finished(_) => {
            return Err("zero cleanup budget must retain pending observation custody".to_owned());
        }
    };
    assert_eq!(output.stop(), &ProcessStop::Deadline);
    assert!(!output.status().success());
    Ok(())
}
