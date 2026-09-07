//! Historical descriptor bounds, name admission and read-only projections.

use super::{
    ArchivedCandidate, ArchivedName, ArchivedOrigin, ArchivedRow, ArchivedSynthesis,
    CandidateArchiveLimits,
};
use crate::descriptor::NameRefusal;

#[path = "read.rs"]
mod read;

#[path = "read_row.rs"]
mod row;

#[path = "guard_row.rs"]
mod row_readers;

#[path = "read_binding.rs"]
mod binding;
#[path = "guard_binding.rs"]
mod binding_readers;

pub use binding::read_binding;
pub(crate) use binding::read_revision;

pub use read::{read_candidate, retain_candidate};
pub use row::{read_row, retain_row};

impl ArchivedName {
    /// Append this historical name through the descriptor owner's canonical grammar.
    pub(crate) fn encode_into(&self, into: &mut Vec<u8>) {
        crate::descriptor::encode::encode_name(self.namespace(), self.stem(), into);
    }

    /// Preserve caller-bounded name components without granting static-name authority.
    pub(crate) fn named(namespace: &str, stem: &str) -> Result<Self, NameRefusal> {
        if namespace.is_empty() {
            return Err(NameRefusal::EmptyNamespace);
        }
        if stem.is_empty() {
            return Err(NameRefusal::EmptyStem);
        }
        Ok(Self {
            namespace: namespace.to_owned(),
            stem: stem.to_owned(),
        })
    }

    /// The exact historical namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// The exact historical local spelling.
    #[must_use]
    pub fn stem(&self) -> &str {
        &self.stem
    }
}

impl CandidateArchiveLimits {
    /// The canonical-byte, name-component and per-roster population ceilings.
    #[must_use]
    pub const fn declared(bytes: usize, field: usize, labels: usize) -> Self {
        Self {
            bytes,
            field,
            labels,
        }
    }

    /// The maximum complete canonical byte count.
    #[must_use]
    pub const fn bytes(self) -> usize {
        self.bytes
    }

    /// The maximum byte count of each name component.
    #[must_use]
    pub const fn field(self) -> usize {
        self.field
    }

    /// The maximum population of each role or tag roster.
    #[must_use]
    pub const fn labels(self) -> usize {
        self.labels
    }
}

impl ArchivedCandidate {
    /// The exact canonical row preimage retained for its enclosing record.
    #[must_use]
    pub fn canonical_bytes(&self) -> &[u8] {
        &self.canonical
    }

    /// The historical behavior claim.
    #[must_use]
    pub const fn claim(&self) -> &ArchivedName {
        &self.claim
    }

    /// The historical aggregate execution seat.
    #[must_use]
    pub const fn execution_suite(&self) -> &ArchivedName {
        &self.execution_suite
    }

    /// The historical subject route.
    #[must_use]
    pub const fn subject(&self) -> &ArchivedName {
        &self.subject
    }

    /// The historical check name.
    #[must_use]
    pub const fn check(&self) -> &ArchivedName {
        &self.check
    }

    /// The historical input population.
    #[must_use]
    pub const fn population(&self) -> &ArchivedName {
        &self.population
    }

    /// The historical synthesis opening, without mutation or proof evidence.
    #[must_use]
    pub const fn synthesis(&self) -> &ArchivedSynthesis {
        &self.synthesis
    }

    /// The historical roles in canonical namespace-and-stem order.
    #[must_use]
    pub fn roles(&self) -> &[ArchivedName] {
        &self.roles
    }

    /// The historical tags in canonical namespace-and-stem order.
    #[must_use]
    pub fn tags(&self) -> &[ArchivedName] {
        &self.tags
    }
}
