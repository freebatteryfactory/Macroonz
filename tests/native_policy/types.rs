//! The paired policy texts admitted by the single-entry derivation.

#[path = "type_guard.rs"]
mod guard;

pub(super) struct PolicyProfiles {
    strict: String,
    native: String,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum StorageDependencies {
    Absent,
    Native,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum HarnessDependencies {
    Absent,
    Judgment,
    Preemption,
}
