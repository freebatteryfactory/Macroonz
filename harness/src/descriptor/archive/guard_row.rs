//! Read-only projections of a complete historical descriptor.

use super::{ArchivedName, ArchivedOrigin, ArchivedRow};

impl ArchivedRow {
    /// The exact canonical descriptor preimage.
    #[must_use]
    pub fn canonical_bytes(&self) -> &[u8] {
        &self.canonical
    }

    /// The historical behavior claim.
    #[must_use]
    pub const fn claim(&self) -> &ArchivedName {
        &self.fields.claim
    }

    /// The historical aggregate execution seat.
    #[must_use]
    pub const fn execution_suite(&self) -> &ArchivedName {
        &self.fields.execution_suite
    }

    /// The historical subject route.
    #[must_use]
    pub const fn subject(&self) -> &ArchivedName {
        &self.fields.subject
    }

    /// The historical check name.
    #[must_use]
    pub const fn check(&self) -> &ArchivedName {
        &self.fields.check
    }

    /// The historical input population.
    #[must_use]
    pub const fn population(&self) -> &ArchivedName {
        &self.fields.population
    }

    /// The historical origin, without producer or human admission authority.
    #[must_use]
    pub const fn origin(&self) -> &ArchivedOrigin {
        &self.origin
    }

    /// The historical roles in canonical namespace-and-stem order.
    #[must_use]
    pub fn roles(&self) -> &[ArchivedName] {
        &self.fields.roles
    }

    /// The historical tags in canonical namespace-and-stem order.
    #[must_use]
    pub fn tags(&self) -> &[ArchivedName] {
        &self.fields.tags
    }
}
