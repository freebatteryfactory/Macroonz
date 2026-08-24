//! The preemption road, exercised from outside: a real lost-update race is found by exhaustive bounded exploration, and the corrected model survives the same walk.
//!
//! The models are written directly against loom's shadow types, which is the adopter's own posture; the pin lane holds the declared [`LOOM_PIN`] and the workspace manifest's `=`-requirement together, so the evidence row and the compiled scheduler cannot drift apart in silence.

use loom::sync::Arc;
use loom::sync::atomic::{AtomicUsize, Ordering};
use loom::thread;
use macroonz_harness::preemption::{
    LOOM_PIN, PreemptionBound, PreemptionBounds, PreemptionBoundsRefusal, PreemptionVerdict,
    explored,
};

/// The workspace manifest, read at compile time, where the loom pin is declared.
const ROOT_MANIFEST: &str = include_str!("../../../Cargo.toml");

/// A lost update: two threads read, then write, so one increment can vanish.
fn racy_model() {
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
        assert!(handle.join().is_ok());
    }
    assert_eq!(value.load(Ordering::SeqCst), 2usize);
}

/// The same counter with the read and the write fused, which no interleaving can break.
fn fused_model() {
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
        assert!(handle.join().is_ok());
    }
    assert_eq!(value.load(Ordering::SeqCst), 2usize);
}

/// The lane's one declared budget.
fn bounds() -> Result<PreemptionBounds, PreemptionBoundsRefusal> {
    PreemptionBounds::declared(PreemptionBound::AtMost(2u32), 1_000u32)
}

/// The lost update is found, and loom's own report crosses the boundary as foreign text.
#[test]
fn the_racy_counter_is_caught_with_looms_report() -> Result<(), PreemptionBoundsRefusal> {
    let reading = explored(bounds()?, racy_model);
    assert_eq!(reading.bounds(), bounds()?);
    assert!(
        matches!(
            reading.verdict(),
            PreemptionVerdict::ModelBroke { report: Some(_) }
        ),
        "the racy model was not caught with a report"
    );
    Ok(())
}

/// The fused counter holds over every interleaving the bounds admit.
#[test]
fn the_fused_counter_holds_over_the_bounded_space() -> Result<(), PreemptionBoundsRefusal> {
    let reading = explored(bounds()?, fused_model);
    assert_eq!(reading.verdict(), &PreemptionVerdict::AllInterleavingsHeld);
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

/// The witnessed spellings, one per roster row, in roster order.
const WITNESSED: [&str; 22] = [
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
];

/// The shadow roster's rows and this lane's witnessed spellings are one list, so a row can neither appear nor vanish unwitnessed.
#[test]
fn the_shadow_roster_is_witnessed_against_the_pinned_loom() {
    witnessed_sync_faces(None, None, None, None, None, None, None);
    witnessed_unsigned_faces(None, None, None, None, None, None);
    witnessed_signed_faces(None, None, None, None, None, None);
    witnessed_odd_faces(thread::yield_now, None, loom::sync::atomic::fence);
    let stated: Vec<&str> = macroonz::descriptor::shadow::SHADOW_ROSTER
        .iter()
        .map(macroonz::descriptor::shadow::ShadowRow::name)
        .collect();
    assert_eq!(stated, WITNESSED);
}
