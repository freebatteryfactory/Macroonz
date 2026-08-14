//! The repository laws.
//!
//! One module per family, each carrying its own constants, its own reading of
//! what an offence is, and the planted reversals that prove it can fail. A check
//! that cannot fail is not a check, so no family here ships without one, and
//! where a half of a law is unplanted the module that owns it says so.
//!
//! The families are declared in the order `main.rs` registers them, so the
//! declaration list and the run order can be read against each other.

pub(crate) mod parity;

pub(crate) mod hygiene;

pub(crate) mod toolchain;

pub(crate) mod dependency;

pub(crate) mod supply_chain;

pub(crate) mod placement;

pub(crate) mod obligations;

pub(crate) mod coupling;

pub(crate) mod seal;

pub(crate) mod vocabulary;

#[cfg(test)]
pub(crate) mod scratch;
