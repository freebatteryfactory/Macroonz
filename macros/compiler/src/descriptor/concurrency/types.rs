//! Every public type of the concurrency declaration home, declared and nothing else.
//!
//! Construction and reading live in this module's own child `type_guard.rs`.

use crate::descriptor::{DirectBinding, HelperRefusal};

#[path = "type_guard.rs"]
mod guard;

/// One declared exploration: its spelling, and the four facts that make its findings replayable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExplorationRow {
    name: String,
    population: String,
    interleavings: u32,
    samples: u32,
    seed: u64,
}

/// The complete payload one concurrency declaration reads to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcurrencyDeclaration {
    harness: DirectBinding,
    module: String,
    namespace: String,
    rows: Vec<ExplorationRow>,
}

/// What a concurrency request produces: one direct declaration-site unit carrying the exploration module.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConcurrencyModule;

/// How one concurrency declaration was not read.
#[must_use = "a concurrency capture refusal names the cause and the token it was established at"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConcurrencyCaptureError(HelperRefusal);
