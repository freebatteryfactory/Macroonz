//! The documented schedule caller must execute through the harness-only root posture.

use std::path::Path;

#[test]
fn public_schedule_example_executes_faults_counterexample_replay_and_bounds() -> Result<(), String>
{
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let scratch = crate::check::scratch()?;
    let output = crate::check::cargo(
        root,
        root,
        &scratch,
        "schedule-example",
        &[
            "run",
            "--example",
            "schedule_workflow",
            "--no-default-features",
            "--features",
            "harness",
            "--locked",
            "--offline",
            "-j1",
        ],
    )?;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, b"network: quiet and four fault forms agree with exact deliveries\n\
        concurrency: withdraw-first fails and replays; guarded histories hold\n\
        evidence: exhaustive and sampled standings stay distinct; empty pressure and zero work refuse\n");
    Ok(())
}
