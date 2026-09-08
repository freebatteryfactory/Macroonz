//! The paired policy texts admitted by the single-entry derivation.

#[path = "type_guard.rs"]
mod guard;

pub(super) struct PolicyProfiles {
    strict: String,
    native: String,
}
