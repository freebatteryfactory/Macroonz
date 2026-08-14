//! Reading the repository.
//!
//! Nothing here decides whether the repository is lawful. These modules turn the
//! tree into FACTS — one immutable snapshot, built once — and `crate::checks`
//! judges those facts. Splitting the reading from the judging is what lets a law
//! be proven against fixture text instead of against the tree it guards.
//!
//! Each module is authoritative for one language and claims nothing outside it:
//! [`cargo`] for Cargo's syntax and for what cargo resolves, [`markdown`] for
//! document structure, [`rust`] for Rust syntax. [`snapshot`] is the one place
//! that touches the filesystem or starts a process; [`types`] is the vocabulary
//! the readings and the laws share.
//!
//! The modules are declared in dependency order: the shared vocabulary, then the
//! snapshot every reading is carried in, then the three decoders.

pub(crate) mod types;

pub(crate) mod snapshot;

pub(crate) mod cargo;

pub(crate) mod markdown;

pub(crate) mod rust;
