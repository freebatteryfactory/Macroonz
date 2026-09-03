//! Every road that builds one of this vocabulary's values, and every reader that hands its seats back.
//!
//! Declared inside `types.rs` as its own child, which is what makes the home's claims structural rather than remembered.
//! A name is parsed here, so a reference that names nothing is not a value anybody can hold.
//! An admitted origin's payload is admitted here, each arm taking only the grounds it earns.
//! A row is born here and commits to its canonical bytes as it is born, so a row that exists has exactly one encoding.
//! A binding is married here, and a table is closed here, so a duplicated trial and an authored candidate are not values that exist.
//!
//! The readers travel with the constructors because they are the same private seats read back.
//! Two hand-written `Clone` realizations sit here rather than in `type_contract.rs` for one mechanical reason: they read private seats, and only this file and `types.rs` can see them.

#[path = "guard_name.rs"]
mod names;
#[path = "guard_row.rs"]
mod rows;
#[path = "guard_attachment.rs"]
mod attachments;
#[path = "guard_table.rs"]
mod tables;
#[path = "guard_schema.rs"]
mod schemas;

pub(crate) use names::namespaced_reference;
