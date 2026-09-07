//! Read-only complete-run projections and caller-declared census bounds.

use super::super::{
    AddressClaim, ArchiveLimits, ArchivedAccounting, ArchivedDisposition, ArchivedInput,
    ArchivedName, ArchivedRun, ArchivedTablePosture, RunArchiveLimits,
};
use crate::identity::ContentAddress;
use crate::report::{InvocationProfile, SelectionOutcome, TargetBinding};

impl RunArchiveLimits {
    /// The byte ceilings and maximum census population, independently declared.
    #[must_use]
    pub const fn declared(bytes: ArchiveLimits, rows: usize) -> Self {
        Self { bytes, rows }
    }

    /// The complete-envelope and framed-member byte ceilings.
    #[must_use]
    pub const fn bytes(self) -> ArchiveLimits {
        self.bytes
    }

    /// The maximum census population.
    #[must_use]
    pub const fn rows(self) -> usize {
        self.rows
    }
}

impl ArchivedAccounting {
    /// The semantic trial claim.
    #[must_use]
    pub const fn trial(&self) -> AddressClaim {
        self.trial
    }

    /// The row revision claim, whose descriptor preimage is absent.
    #[must_use]
    pub const fn row(&self) -> AddressClaim {
        self.row
    }

    /// The subject revision claim.
    #[must_use]
    pub const fn subject(&self) -> AddressClaim {
        self.subject
    }

    /// The check revision claim.
    #[must_use]
    pub const fn check(&self) -> AddressClaim {
        self.check
    }

    /// The readable historical claim name.
    #[must_use]
    pub const fn claim(&self) -> &ArchivedName {
        &self.claim
    }

    /// The complete selection disposition of this row.
    #[must_use]
    pub const fn disposition(&self) -> &ArchivedDisposition {
        &self.disposition
    }
}

impl ArchivedRun {
    /// The exact envelope for caller-owned storage.
    #[must_use]
    pub fn encoded(&self) -> &[u8] {
        &self.encoded
    }

    /// The address derived over this historical envelope.
    #[must_use]
    pub const fn address(&self) -> ContentAddress {
        self.address
    }

    /// Every historical census row in its original order.
    #[must_use]
    pub fn census(&self) -> &[ArchivedAccounting] {
        &self.census
    }

    /// The recorded table posture without authored-world authority.
    #[must_use]
    pub const fn posture(&self) -> &ArchivedTablePosture {
        &self.posture
    }

    /// The recorded selection outcome, independent of attempt conclusions.
    #[must_use]
    pub const fn selection(&self) -> SelectionOutcome {
        self.selection
    }

    /// The invocation budgets, including when the census is empty.
    #[must_use]
    pub const fn invocation(&self) -> InvocationProfile {
        self.invocation
    }

    /// The target and toolchain labels, including when no row is selected.
    #[must_use]
    pub const fn target(&self) -> &TargetBinding {
        &self.target
    }

    /// The complete input standing, independent of selection emptiness.
    #[must_use]
    pub const fn input(&self) -> Option<&ArchivedInput> {
        self.input.as_ref()
    }
}
