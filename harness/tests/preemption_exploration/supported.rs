//! Supported-backend claims over the preemption road.
//!
//! The models are written directly against loom's shadow types, which is the adopter's own posture; the pin lane holds the declared [`LOOM_PIN`] and the workspace manifest's `=`-requirement together, so the evidence row and the compiled scheduler cannot drift apart in silence.

use loom::sync::Arc;
use loom::sync::atomic::{AtomicUsize, Ordering};
use loom::thread;
use macroonz_harness::preemption::{
    IncompleteExploration, LOOM_PIN, MODEL_BROKE, PreemptionBound, PreemptionBounds,
    PreemptionBoundsRefusal, PreemptionModelFailure, PreemptionModelResult, PreemptionOutcome,
    PreemptionVerdict, attempted, explored,
};
use macroonz_harness::report::{FailureClass, InfrastructureFault, RunAttempt, TrialConclusion};
use std::io::Write;
use std::process::{Command, Output};

/// The workspace manifest, read at compile time, where the loom pin is declared.
const ROOT_MANIFEST: &str = include_str!("../../../Cargo.toml");

/// The ignored exact child that observes intentional branch exhaustion without backtrace generation.
const BRANCH_EXHAUSTION_CHILD: &str = "supported::branch_exhaustion_is_typed_child";

/// The child marker written only after the typed branch-exhaustion assertions hold.
const BRANCH_EXHAUSTION_CHILD_MARKER: &[u8] = b"macroonz-branch-exhaustion-typed\n";

/// Every environment seat Loom 0.7.2 reads while constructing its builder.
const LOOM_ENVIRONMENT: [&str; 8] = [
    "LOOM_CHECKPOINT_FILE",
    "LOOM_CHECKPOINT_INTERVAL",
    "LOOM_LOCATION",
    "LOOM_LOG",
    "LOOM_MAX_BRANCHES",
    "LOOM_MAX_DURATION",
    "LOOM_MAX_PERMUTATIONS",
    "LOOM_MAX_PREEMPTIONS",
];

loom::thread_local! {
    /// One shadowed thread-local, declared through loom's own macro and read inside a model.
    static SEAT: u8 = 7u8;
}

/// A lost update: two threads read, then write, so one increment can vanish.
fn racy_model() -> PreemptionModelResult {
    let value = Arc::new(AtomicUsize::new(0));
    let handles: Vec<_> = (0usize..2usize)
        .map(|_| {
            let value = Arc::clone(&value);
            thread::spawn(move || {
                let seen = value.load(Ordering::SeqCst);
                value.store(seen.saturating_add(1usize), Ordering::SeqCst);
            })
        })
        .collect();
    for handle in handles {
        if handle.join().is_err() {
            return Err(PreemptionModelFailure::reported(b"a worker did not join"));
        }
    }
    if value.load(Ordering::SeqCst) == 2usize {
        Ok(())
    } else {
        Err(PreemptionModelFailure::reported(b"the update was lost"))
    }
}

/// The same counter with the read and the write fused, which no interleaving can break.
///
/// The model also reads the loom thread-local, which is only reachable inside a model and so is witnessed here.
fn fused_model() -> PreemptionModelResult {
    if !SEAT.with(|seat| *seat == 7u8) {
        return Err(PreemptionModelFailure::reported(
            b"the shadow thread-local changed",
        ));
    }
    let value = Arc::new(AtomicUsize::new(0));
    let handles: Vec<_> = (0usize..2usize)
        .map(|_| {
            let value = Arc::clone(&value);
            thread::spawn(move || {
                value.fetch_add(1usize, Ordering::SeqCst);
            })
        })
        .collect();
    for handle in handles {
        if handle.join().is_err() {
            return Err(PreemptionModelFailure::reported(b"a worker did not join"));
        }
    }
    if value.load(Ordering::SeqCst) == 2usize {
        Ok(())
    } else {
        Err(PreemptionModelFailure::reported(
            b"the fused update was lost",
        ))
    }
}

/// A larger fused counter with explicit yields on both sides of each atomic operation.
fn longer_fused_model() -> PreemptionModelResult {
    let value = Arc::new(AtomicUsize::new(0));
    let handles: Vec<_> = (0usize..3usize)
        .map(|_| {
            let value = Arc::clone(&value);
            thread::spawn(move || {
                thread::yield_now();
                value.fetch_add(1usize, Ordering::SeqCst);
                thread::yield_now();
            })
        })
        .collect();
    for handle in handles {
        if handle.join().is_err() {
            return Err(PreemptionModelFailure::reported(b"a worker did not join"));
        }
    }
    if value.load(Ordering::SeqCst) == 3usize {
        Ok(())
    } else {
        Err(PreemptionModelFailure::reported(
            b"the longer fused update was lost",
        ))
    }
}

/// An async model: one thread races an async block driven by loom's own executor.
///
/// The claim is that the futures face is real — `block_on` participates in the exploration like any other loom operation.
fn futures_model() -> PreemptionModelResult {
    let value = Arc::new(AtomicUsize::new(0));
    let cloned = Arc::clone(&value);
    let handle = thread::spawn(move || {
        cloned.fetch_add(1usize, Ordering::SeqCst);
    });
    let seen = loom::future::block_on(async { value.fetch_add(1usize, Ordering::SeqCst) });
    if seen > 1usize {
        return Err(PreemptionModelFailure::reported(
            b"the async observation was outside its range",
        ));
    }
    if handle.join().is_err() {
        return Err(PreemptionModelFailure::reported(b"a worker did not join"));
    }
    if value.load(Ordering::SeqCst) == 2usize {
        Ok(())
    } else {
        Err(PreemptionModelFailure::reported(
            b"the async update was lost",
        ))
    }
}

/// An undeclared panic carrying the same words as the model's typed refusal.
fn lookalike_panicking_model() -> PreemptionModelResult {
    if std::thread::panicking() {
        return Err(PreemptionModelFailure::unreported());
    }
    std::panic::resume_unwind(Box::new("the update was lost"))
}

/// One scheduler branch with no live shadow allocation during unwind.
fn branching_model() -> PreemptionModelResult {
    thread::yield_now();
    if std::hint::black_box(false) {
        Err(PreemptionModelFailure::unreported())
    } else {
        Ok(())
    }
}

/// The lane's one declared budget.
fn bounds() -> Result<PreemptionBounds, PreemptionBoundsRefusal> {
    PreemptionBounds::declared(PreemptionBound::AtMost(2u32), 1_000u32)
}

/// Run one ignored child claim under declared environment changes and no inherited Loom seat.
fn child_with_environment(name: &str, environment: &[(&str, &str)]) -> std::io::Result<Output> {
    let mut command = Command::new(std::env::current_exe()?);
    command.args(["--exact", name, "--ignored", "--nocapture"]);
    for variable in LOOM_ENVIRONMENT {
        command.env_remove(variable);
    }
    for &(variable, value) in environment {
        command.env(variable, value);
    }
    command.output()
}

/// The lost update is found through the model-owned typed refusal road.
#[test]
fn the_racy_counter_is_caught_with_its_typed_report() -> Result<(), PreemptionBoundsRefusal> {
    let reading = explored(bounds()?, racy_model);
    assert_eq!(reading.bounds(), bounds()?);
    assert!(matches!(
        reading.outcome(),
        PreemptionOutcome::Completed(PreemptionVerdict::ModelBroke { report: Some(report) })
            if report.bytes() == b"the update was lost"
    ));
    let attempt = attempted(&reading);
    assert!(
        matches!(attempt, RunAttempt::Executed(TrialConclusion::Refused(_))),
        "the broke verdict did not become an executed refusal"
    );
    if let RunAttempt::Executed(TrialConclusion::Refused(finding)) = attempt {
        assert_eq!(finding.cause(), MODEL_BROKE);
        assert_eq!(finding.class(), FailureClass::RefusedByCheck);
        assert!(finding.foreign().is_some());
    }
    Ok(())
}

/// The fused counter holds over every interleaving the bounds admit.
#[test]
fn the_fused_counter_holds_over_the_bounded_space() -> Result<(), PreemptionBoundsRefusal> {
    let reading = explored(bounds()?, fused_model);
    assert_eq!(
        reading.outcome(),
        &PreemptionOutcome::Completed(PreemptionVerdict::AllInterleavingsHeld)
    );
    assert_eq!(
        attempted(&reading),
        RunAttempt::Executed(TrialConclusion::Passed)
    );
    Ok(())
}

/// Three yielded workers hold over the longer declared schedule campaign without claiming schedules beyond its preemption and branch ceilings.
#[test]
#[ignore = "long deterministic local preemption campaign; run explicitly"]
fn the_longer_fused_counter_holds_over_its_bounded_space() -> Result<(), PreemptionBoundsRefusal> {
    let campaign = PreemptionBounds::declared(PreemptionBound::AtMost(4u32), 100_000u32)?;
    let reading = explored(campaign, longer_fused_model);
    assert_eq!(reading.bounds(), campaign);
    assert_eq!(
        reading.outcome(),
        &PreemptionOutcome::Completed(PreemptionVerdict::AllInterleavingsHeld)
    );
    assert_eq!(
        attempted(&reading),
        RunAttempt::Executed(TrialConclusion::Passed)
    );
    Ok(())
}

/// The async model explores clean under `block_on`, so the futures face is load-bearing.
#[test]
fn an_async_model_explores_under_block_on() -> Result<(), PreemptionBoundsRefusal> {
    let reading = explored(bounds()?, futures_model);
    assert_eq!(
        reading.outcome(),
        &PreemptionOutcome::Completed(PreemptionVerdict::AllInterleavingsHeld)
    );
    Ok(())
}

/// An untyped unwind cannot wear the exact model-break badge or cross onto the subject rail, even when its text matches.
#[test]
fn lookalike_panic_text_stays_infrastructure_unresolved() -> Result<(), PreemptionBoundsRefusal> {
    let reading = explored(bounds()?, lookalike_panicking_model);
    assert!(matches!(
        reading.outcome(),
        PreemptionOutcome::Incomplete(IncompleteExploration::ExecutionUnresolved {
            report: Some(report)
        }) if report.bytes() == b"the update was lost"
    ));
    let attempt = attempted(&reading);
    assert!(matches!(
        attempt,
        RunAttempt::InfrastructureFailed(ref failure)
            if failure.fault() == InfrastructureFault::BackendExecutionUnresolved
                && failure.foreign().is_some()
    ));
    Ok(())
}

/// The exact child retains intentional branch exhaustion on the typed infrastructure rail.
#[test]
#[ignore = "driven by the parent under a scoped diagnostic environment"]
fn branch_exhaustion_is_typed_child() -> std::io::Result<()> {
    let narrow =
        PreemptionBounds::declared(PreemptionBound::AtMost(2u32), 1u32).map_err(|refusal| {
            std::io::Error::other(format!("the narrow bounds refused: {refusal:?}"))
        })?;
    let reading = explored(narrow, branching_model);
    assert!(matches!(
        reading.outcome(),
        PreemptionOutcome::Incomplete(IncompleteExploration::ExecutionUnresolved {
            report: Some(_)
        })
    ));
    let attempt = attempted(&reading);
    assert!(matches!(
        attempt,
        RunAttempt::InfrastructureFailed(ref failure)
            if failure.fault() == InfrastructureFault::BackendExecutionUnresolved
                && failure.foreign().is_some()
    ));
    let mut stdout = std::io::stdout().lock();
    stdout.write_all(BRANCH_EXHAUSTION_CHILD_MARKER)?;
    stdout.flush()?;
    Ok(())
}

/// A budget the backend cannot complete under remains infrastructure-incomplete rather than a subject panic.
///
/// The parent retains the workflow's diagnostic posture while the exact child disables only backtrace generation for the intentional Loom panic that must unwind into the typed reading.
#[test]
fn branch_exhaustion_stays_infrastructure_unresolved() -> std::io::Result<()> {
    let output = child_with_environment(
        BRANCH_EXHAUSTION_CHILD,
        &[("RUST_BACKTRACE", "0"), ("RUST_LIB_BACKTRACE", "0")],
    )?;
    assert!(
        output.status.success(),
        "the scoped branch-exhaustion child failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        output
            .stdout
            .windows(BRANCH_EXHAUSTION_CHILD_MARKER.len())
            .any(|window| window == BRANCH_EXHAUSTION_CHILD_MARKER),
        "the scoped branch-exhaustion child returned without its completion marker"
    );
    Ok(())
}

/// An invalid ambient backend seat refuses before execution and remains an initialization fault with bounded foreign material.
#[test]
#[ignore = "driven in a child process with a declared invalid Loom environment"]
fn invalid_environment_is_typed_child() -> Result<(), PreemptionBoundsRefusal> {
    let reading = explored(bounds()?, fused_model);
    assert!(matches!(
        reading.outcome(),
        PreemptionOutcome::Incomplete(IncompleteExploration::InitializationFailed {
            report: Some(_)
        })
    ));
    assert!(matches!(
        attempted(&reading),
        RunAttempt::InfrastructureFailed(ref failure)
            if failure.fault() == InfrastructureFault::BackendInitializationFailed
                && failure.foreign().is_some()
    ));
    Ok(())
}

/// Valid ambient early-stop seats cannot create a zero-execution pass because the road overwrites them before checking.
#[test]
#[ignore = "driven in a child process with declared ambient Loom early stops"]
fn ambient_early_stops_are_overwritten_child() -> Result<(), PreemptionBoundsRefusal> {
    let reading = explored(bounds()?, racy_model);
    assert!(matches!(
        reading.outcome(),
        PreemptionOutcome::Completed(PreemptionVerdict::ModelBroke { report: Some(report) })
            if report.bytes() == b"the update was lost"
    ));
    Ok(())
}

/// Invalid construction input and valid early-stop input are isolated in child processes, so neither can leak across tests.
#[test]
fn ambient_backend_configuration_cannot_counterfeit_a_verdict() -> std::io::Result<()> {
    let invalid = child_with_environment(
        "supported::invalid_environment_is_typed_child",
        &[("LOOM_MAX_BRANCHES", "not-a-number")],
    )?;
    assert!(
        invalid.status.success(),
        "the invalid-environment child failed: {}",
        String::from_utf8_lossy(&invalid.stderr)
    );

    let early_stop = child_with_environment(
        "supported::ambient_early_stops_are_overwritten_child",
        &[
            ("LOOM_CHECKPOINT_INTERVAL", "1"),
            ("LOOM_MAX_BRANCHES", "1"),
            ("LOOM_MAX_DURATION", "0"),
            ("LOOM_MAX_PERMUTATIONS", "1"),
            ("LOOM_MAX_PREEMPTIONS", "0"),
        ],
    )?;
    assert!(
        early_stop.status.success(),
        "the ambient-early-stop child failed: {}",
        String::from_utf8_lossy(&early_stop.stderr)
    );
    Ok(())
}

/// A branch budget of zero could never take a step, and the bounds say so.
#[test]
fn a_zero_branch_budget_refuses() {
    assert_eq!(
        PreemptionBounds::declared(PreemptionBound::Exhaustive, 0u32),
        Err(PreemptionBoundsRefusal::ZeroBranches)
    );
}

/// The declared pin and the manifest's requirement spell one loom.
#[test]
fn the_declared_pin_mirrors_the_manifest() {
    let requirement = format!("loom = {{ version = \"={LOOM_PIN}\"");
    assert!(
        ROOT_MANIFEST.contains(&requirement),
        "the workspace manifest does not declare loom at the pinned {LOOM_PIN}"
    );
    assert_eq!(loom::MAX_THREADS, 5usize);
}

/// The roster's `sync` faces, witnessed in parameter position at the pinned version.
///
/// Compiling these signatures is the claim: a roster row whose shadow path stopped being true would stop this target building, under the ordinary wall, with no second command.
fn witnessed_sync_faces(
    _arc: Option<Arc<u8>>,
    _mutex: Option<loom::sync::Mutex<u8>>,
    _mutex_guard: Option<loom::sync::MutexGuard<'static, u8>>,
    _rw_lock: Option<loom::sync::RwLock<u8>>,
    _read_guard: Option<loom::sync::RwLockReadGuard<'static, u8>>,
    _write_guard: Option<loom::sync::RwLockWriteGuard<'static, u8>>,
    _condvar: Option<loom::sync::Condvar>,
) {
}

/// The roster's unsigned atomic faces, on the same terms.
fn witnessed_unsigned_faces(
    _flag: Option<loom::sync::atomic::AtomicBool>,
    _narrow: Option<loom::sync::atomic::AtomicU8>,
    _short: Option<loom::sync::atomic::AtomicU16>,
    _wide: Option<loom::sync::atomic::AtomicU32>,
    _wider: Option<loom::sync::atomic::AtomicU64>,
    _sized: Option<AtomicUsize>,
) {
}

/// The roster's signed and pointer atomic faces, on the same terms.
fn witnessed_signed_faces(
    _narrow: Option<loom::sync::atomic::AtomicI8>,
    _short: Option<loom::sync::atomic::AtomicI16>,
    _wide: Option<loom::sync::atomic::AtomicI32>,
    _wider: Option<loom::sync::atomic::AtomicI64>,
    _sized: Option<loom::sync::atomic::AtomicIsize>,
    _pointer: Option<loom::sync::atomic::AtomicPtr<u8>>,
) {
}

/// The roster's remaining faces: the thread module, the ordering, and the fence.
fn witnessed_odd_faces(_yielding: fn(), _ordering: Option<Ordering>, _fencing: fn(Ordering)) {}

/// The roster's later rows: the barrier, the lock results, the channel module, and the spin hint.
///
/// The `thread_local` row's shadow face is a macro, witnessed by the module-level declaration above and its read inside the fused model.
fn witnessed_late_faces(
    _barrier: Option<loom::sync::Barrier>,
    _lock: Option<loom::sync::LockResult<u8>>,
    _try_lock: Option<loom::sync::TryLockResult<u8>>,
    _channeling: fn() -> (loom::sync::mpsc::Sender<u8>, loom::sync::mpsc::Receiver<u8>),
    _spinning: fn(),
) {
}

/// The witnessed spellings, one per roster row, in roster order.
const WITNESSED: [&str; 28] = [
    "Arc",
    "Mutex",
    "MutexGuard",
    "RwLock",
    "RwLockReadGuard",
    "RwLockWriteGuard",
    "Condvar",
    "thread",
    "AtomicBool",
    "AtomicU8",
    "AtomicU16",
    "AtomicU32",
    "AtomicU64",
    "AtomicUsize",
    "AtomicI8",
    "AtomicI16",
    "AtomicI32",
    "AtomicI64",
    "AtomicIsize",
    "AtomicPtr",
    "Ordering",
    "fence",
    "Barrier",
    "LockResult",
    "TryLockResult",
    "mpsc",
    "thread_local",
    "spin_loop",
];

/// The shadow roster's rows and this lane's witnessed spellings are one list, so a row can neither appear nor vanish unwitnessed.
#[test]
fn the_shadow_roster_is_witnessed_against_the_pinned_loom() {
    witnessed_sync_faces(None, None, None, None, None, None, None);
    witnessed_unsigned_faces(None, None, None, None, None, None);
    witnessed_signed_faces(None, None, None, None, None, None);
    witnessed_odd_faces(thread::yield_now, None, loom::sync::atomic::fence);
    witnessed_late_faces(
        None,
        None,
        None,
        loom::sync::mpsc::channel::<u8>,
        loom::hint::spin_loop,
    );
    let stated: Vec<&str> = macroonz_compiler::descriptor::shadow::SHADOW_ROSTER
        .iter()
        .map(macroonz_compiler::descriptor::shadow::ShadowRow::name)
        .collect();
    assert_eq!(stated, WITNESSED);
}
