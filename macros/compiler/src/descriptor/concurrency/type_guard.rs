//! Constructors and readers for the concurrency declaration vocabulary.

use super::{ConcurrencyCaptureError, ConcurrencyDeclaration, ExplorationRow};
use crate::descriptor::{CaptureCause, CaptureIssue, DirectBinding, Grammar, HelperRefusal};
use crate::token::SpanHandle;

impl ExplorationRow {
    /// One declared exploration, minted only by the capture reading.
    #[must_use]
    pub(crate) const fn declared(
        name: String,
        population: String,
        interleavings: u32,
        samples: u32,
        seed: u64,
    ) -> Self {
        Self {
            name,
            population,
            interleavings,
            samples,
            seed,
        }
    }

    /// The spelling this exploration's function is named by.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The population's stem, owned under the declaration's namespace.
    #[must_use]
    pub fn population(&self) -> &str {
        &self.population
    }

    /// The exhaustive ceiling, at the width the harness bound declares.
    #[must_use]
    pub const fn interleavings(&self) -> u32 {
        self.interleavings
    }

    /// The sample count beyond the ceiling, at the width the harness bound declares.
    #[must_use]
    pub const fn samples(&self) -> u32 {
        self.samples
    }

    /// The declared seed.
    #[must_use]
    pub const fn seed(&self) -> u64 {
        self.seed
    }
}

impl ConcurrencyDeclaration {
    /// The complete payload, minted only by the capture reading.
    #[must_use]
    pub(crate) const fn read(
        harness: DirectBinding,
        module: String,
        namespace: String,
        rows: Vec<ExplorationRow>,
    ) -> Self {
        Self {
            harness,
            module,
            namespace,
            rows,
        }
    }

    /// The physical path to the harness vocabulary this direct projection targets.
    #[must_use]
    pub const fn harness(&self) -> &DirectBinding {
        &self.harness
    }

    /// The module the exploration functions land in.
    #[must_use]
    pub fn module(&self) -> &str {
        &self.module
    }

    /// The namespace every declared name is owned under.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// The rows, in authored order.
    #[must_use]
    pub fn rows(&self) -> &[ExplorationRow] {
        &self.rows
    }
}

impl ConcurrencyCaptureError {
    /// One refusal the concurrency grammar's own reading established.
    pub const fn grammar_refused(grammar: Grammar, cause: CaptureCause, at: SpanHandle) -> Self {
        Self(HelperRefusal::grammar_refused(grammar, cause, at))
    }

    /// One refusal from the direct-binding reading, retained without flattening its owner.
    pub const fn binding_refused(grammar: Grammar, issue: CaptureIssue, at: SpanHandle) -> Self {
        Self(HelperRefusal::capture_refused(grammar, issue, at))
    }

    /// The refusal itself.
    pub const fn refusal(&self) -> &HelperRefusal {
        &self.0
    }
}
