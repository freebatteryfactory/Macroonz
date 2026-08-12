//! Reading the repository.
//!
//! Nothing here decides whether the repository is lawful. These modules turn the
//! tree and the files in it into facts — paths, manifest entries, README rows —
//! and `crate::checks` judges those facts. Splitting the reading from the
//! judging is what lets a law be proven against fixture text instead of against
//! the tree it guards.
//!
//! The modules are declared in dependency order: the shared vocabulary, then the
//! walker every reader stands on, then the two file readers.

pub(crate) mod types;

pub(crate) mod walk;

pub(crate) mod manifest;

pub(crate) mod readme;
