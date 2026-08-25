//! Every public type of the shadow home, declared and nothing else, with the roster stated as data beside them.
//!
//! Construction and reading live in this module's own child `type_guard.rs`.

use crate::descriptor::{DirectBinding, HelperRefusal};

#[path = "type_guard.rs"]
mod guard;

/// Where this helper's family sits among the declaration helpers.
pub const SHADOW_HELPER_POSITION: u32 = 3;

/// One row of the shadow roster: the chosen spelling, its standard-library path, and its shadow path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShadowRow {
    name: &'static str,
    std_path: &'static [&'static str],
    shadow_path: &'static [&'static str],
}

/// The covered names, stated whole.
///
/// A row exists exactly where the shadow library realizes the primitive, and the preemption lane on the harness side witnesses every shadow path at the pinned version, so a row that stopped being true cannot stay quietly in the table.
pub const SHADOW_ROSTER: &[ShadowRow] = &[
    ShadowRow::covered("Arc", &["std", "sync", "Arc"], &["sync", "Arc"]),
    ShadowRow::covered("Mutex", &["std", "sync", "Mutex"], &["sync", "Mutex"]),
    ShadowRow::covered(
        "MutexGuard",
        &["std", "sync", "MutexGuard"],
        &["sync", "MutexGuard"],
    ),
    ShadowRow::covered("RwLock", &["std", "sync", "RwLock"], &["sync", "RwLock"]),
    ShadowRow::covered(
        "RwLockReadGuard",
        &["std", "sync", "RwLockReadGuard"],
        &["sync", "RwLockReadGuard"],
    ),
    ShadowRow::covered(
        "RwLockWriteGuard",
        &["std", "sync", "RwLockWriteGuard"],
        &["sync", "RwLockWriteGuard"],
    ),
    ShadowRow::covered("Condvar", &["std", "sync", "Condvar"], &["sync", "Condvar"]),
    ShadowRow::covered("thread", &["std", "thread"], &["thread"]),
    ShadowRow::covered(
        "AtomicBool",
        &["std", "sync", "atomic", "AtomicBool"],
        &["sync", "atomic", "AtomicBool"],
    ),
    ShadowRow::covered(
        "AtomicU8",
        &["std", "sync", "atomic", "AtomicU8"],
        &["sync", "atomic", "AtomicU8"],
    ),
    ShadowRow::covered(
        "AtomicU16",
        &["std", "sync", "atomic", "AtomicU16"],
        &["sync", "atomic", "AtomicU16"],
    ),
    ShadowRow::covered(
        "AtomicU32",
        &["std", "sync", "atomic", "AtomicU32"],
        &["sync", "atomic", "AtomicU32"],
    ),
    ShadowRow::covered(
        "AtomicU64",
        &["std", "sync", "atomic", "AtomicU64"],
        &["sync", "atomic", "AtomicU64"],
    ),
    ShadowRow::covered(
        "AtomicUsize",
        &["std", "sync", "atomic", "AtomicUsize"],
        &["sync", "atomic", "AtomicUsize"],
    ),
    ShadowRow::covered(
        "AtomicI8",
        &["std", "sync", "atomic", "AtomicI8"],
        &["sync", "atomic", "AtomicI8"],
    ),
    ShadowRow::covered(
        "AtomicI16",
        &["std", "sync", "atomic", "AtomicI16"],
        &["sync", "atomic", "AtomicI16"],
    ),
    ShadowRow::covered(
        "AtomicI32",
        &["std", "sync", "atomic", "AtomicI32"],
        &["sync", "atomic", "AtomicI32"],
    ),
    ShadowRow::covered(
        "AtomicI64",
        &["std", "sync", "atomic", "AtomicI64"],
        &["sync", "atomic", "AtomicI64"],
    ),
    ShadowRow::covered(
        "AtomicIsize",
        &["std", "sync", "atomic", "AtomicIsize"],
        &["sync", "atomic", "AtomicIsize"],
    ),
    ShadowRow::covered(
        "AtomicPtr",
        &["std", "sync", "atomic", "AtomicPtr"],
        &["sync", "atomic", "AtomicPtr"],
    ),
    ShadowRow::covered(
        "Ordering",
        &["std", "sync", "atomic", "Ordering"],
        &["sync", "atomic", "Ordering"],
    ),
    ShadowRow::covered(
        "fence",
        &["std", "sync", "atomic", "fence"],
        &["sync", "atomic", "fence"],
    ),
    ShadowRow::covered("Barrier", &["std", "sync", "Barrier"], &["sync", "Barrier"]),
    ShadowRow::covered(
        "LockResult",
        &["std", "sync", "LockResult"],
        &["sync", "LockResult"],
    ),
    ShadowRow::covered(
        "TryLockResult",
        &["std", "sync", "TryLockResult"],
        &["sync", "TryLockResult"],
    ),
    ShadowRow::covered("mpsc", &["std", "sync", "mpsc"], &["sync", "mpsc"]),
    ShadowRow::covered("thread_local", &["std", "thread_local"], &["thread_local"]),
    ShadowRow::covered(
        "spin_loop",
        &["std", "hint", "spin_loop"],
        &["hint", "spin_loop"],
    ),
];

/// The chosen rows one shadow declaration reads to, in authored order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shadows {
    loom: DirectBinding,
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
