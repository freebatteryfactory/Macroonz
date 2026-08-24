//! Every public type of the shadow home, declared and nothing else, with the roster stated as data beside them.
//!
//! Construction and reading live in this module's own child `type_guard.rs`.

use crate::descriptor::HelperRefusal;

#[path = "type_guard.rs"]
mod guard;

/// Where this helper's family sits among the declaration helpers.
pub const SHADOW_HELPER_POSITION: u32 = 3;

/// One row of the shadow roster: the chosen spelling, its standard-library path, and its shadow path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShadowRow {
    name: &'static str,
    std_path: &'static [&'static str],
    loom_path: &'static [&'static str],
}

/// The covered names, stated whole.
///
/// A row exists exactly where the shadow library realizes the primitive, and the preemption lane on the harness side witnesses every shadow path at the pinned version, so a row that stopped being true cannot stay quietly in the table.
pub const SHADOW_ROSTER: &[ShadowRow] = &[
    ShadowRow::covered("Arc", &["std", "sync", "Arc"], &["loom", "sync", "Arc"]),
    ShadowRow::covered(
        "Mutex",
        &["std", "sync", "Mutex"],
        &["loom", "sync", "Mutex"],
    ),
    ShadowRow::covered(
        "MutexGuard",
        &["std", "sync", "MutexGuard"],
        &["loom", "sync", "MutexGuard"],
    ),
    ShadowRow::covered(
        "RwLock",
        &["std", "sync", "RwLock"],
        &["loom", "sync", "RwLock"],
    ),
    ShadowRow::covered(
        "RwLockReadGuard",
        &["std", "sync", "RwLockReadGuard"],
        &["loom", "sync", "RwLockReadGuard"],
    ),
    ShadowRow::covered(
        "RwLockWriteGuard",
        &["std", "sync", "RwLockWriteGuard"],
        &["loom", "sync", "RwLockWriteGuard"],
    ),
    ShadowRow::covered(
        "Condvar",
        &["std", "sync", "Condvar"],
        &["loom", "sync", "Condvar"],
    ),
    ShadowRow::covered("thread", &["std", "thread"], &["loom", "thread"]),
    ShadowRow::covered(
        "AtomicBool",
        &["std", "sync", "atomic", "AtomicBool"],
        &["loom", "sync", "atomic", "AtomicBool"],
    ),
    ShadowRow::covered(
        "AtomicU8",
        &["std", "sync", "atomic", "AtomicU8"],
        &["loom", "sync", "atomic", "AtomicU8"],
    ),
    ShadowRow::covered(
        "AtomicU16",
        &["std", "sync", "atomic", "AtomicU16"],
        &["loom", "sync", "atomic", "AtomicU16"],
    ),
    ShadowRow::covered(
        "AtomicU32",
        &["std", "sync", "atomic", "AtomicU32"],
        &["loom", "sync", "atomic", "AtomicU32"],
    ),
    ShadowRow::covered(
        "AtomicU64",
        &["std", "sync", "atomic", "AtomicU64"],
        &["loom", "sync", "atomic", "AtomicU64"],
    ),
    ShadowRow::covered(
        "AtomicUsize",
        &["std", "sync", "atomic", "AtomicUsize"],
        &["loom", "sync", "atomic", "AtomicUsize"],
    ),
    ShadowRow::covered(
        "AtomicI8",
        &["std", "sync", "atomic", "AtomicI8"],
        &["loom", "sync", "atomic", "AtomicI8"],
    ),
    ShadowRow::covered(
        "AtomicI16",
        &["std", "sync", "atomic", "AtomicI16"],
        &["loom", "sync", "atomic", "AtomicI16"],
    ),
    ShadowRow::covered(
        "AtomicI32",
        &["std", "sync", "atomic", "AtomicI32"],
        &["loom", "sync", "atomic", "AtomicI32"],
    ),
    ShadowRow::covered(
        "AtomicI64",
        &["std", "sync", "atomic", "AtomicI64"],
        &["loom", "sync", "atomic", "AtomicI64"],
    ),
    ShadowRow::covered(
        "AtomicIsize",
        &["std", "sync", "atomic", "AtomicIsize"],
        &["loom", "sync", "atomic", "AtomicIsize"],
    ),
    ShadowRow::covered(
        "AtomicPtr",
        &["std", "sync", "atomic", "AtomicPtr"],
        &["loom", "sync", "atomic", "AtomicPtr"],
    ),
    ShadowRow::covered(
        "Ordering",
        &["std", "sync", "atomic", "Ordering"],
        &["loom", "sync", "atomic", "Ordering"],
    ),
    ShadowRow::covered(
        "fence",
        &["std", "sync", "atomic", "fence"],
        &["loom", "sync", "atomic", "fence"],
    ),
    ShadowRow::covered(
        "Barrier",
        &["std", "sync", "Barrier"],
        &["loom", "sync", "Barrier"],
    ),
    ShadowRow::covered(
        "LockResult",
        &["std", "sync", "LockResult"],
        &["loom", "sync", "LockResult"],
    ),
    ShadowRow::covered(
        "TryLockResult",
        &["std", "sync", "TryLockResult"],
        &["loom", "sync", "TryLockResult"],
    ),
    ShadowRow::covered("mpsc", &["std", "sync", "mpsc"], &["loom", "sync", "mpsc"]),
    ShadowRow::covered(
        "thread_local",
        &["std", "thread_local"],
        &["loom", "thread_local"],
    ),
    ShadowRow::covered(
        "spin_loop",
        &["std", "hint", "spin_loop"],
        &["loom", "hint", "spin_loop"],
    ),
];

/// The chosen rows one shadow declaration reads to, in authored order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shadows {
    chosen: Vec<ShadowRow>,
}

/// What a shadow request produces: one direct declaration-site unit carrying both faces of every chosen name.
///
/// A marker for the compiler to be generic over, like every kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShadowFace;

/// How one shadow declaration was not read.
///
/// Its own type, because a diagnostic's family tag is a fact about the type: this grammar is a declaration's shadow reading, and the trial, mutation, and bench grammars each carry their own.
#[must_use = "a shadow capture refusal names the cause and the token it was established at"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShadowCaptureError(HelperRefusal);
