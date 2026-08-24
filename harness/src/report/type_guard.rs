//! The invariant nucleus: every road that reaches a private field of this home.
//!
//! Declared inside `types.rs` as its own child, so it sees the fields the declarations keep private and no sibling file does.
//! An identity that skipped its preimage, a key that skipped its target binding, or a foreign-text field that skipped its bound would have to be written in one of the four files below, and none is.

#[path = "guard_identity.rs"]
mod identities;
#[path = "guard_finding.rs"]
mod findings;
#[path = "guard_record.rs"]
mod records;
#[path = "guard_reading.rs"]
mod readings;
