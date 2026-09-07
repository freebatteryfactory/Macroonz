//! The invariant nucleus: every road that reaches a private field of this home.
//!
//! Declared inside `types.rs` as its own child, so it sees the fields the declarations keep private and no sibling file does.
//! Each subject file owns the constructors for its private relationships.

#[path = "guard_identity.rs"]
mod identities;
#[path = "guard_input.rs"]
mod input;
#[path = "guard_finding.rs"]
mod findings;
#[path = "guard_record.rs"]
mod records;
#[path = "guard_reading.rs"]
mod readings;
