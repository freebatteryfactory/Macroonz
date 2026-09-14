//! Read-only historical attachment projections.

use super::super::{
    ArchivedBinding, ArchivedName, ArchivedProvenance, ArchivedRevisionBinding, ArchivedRow,
};
use crate::descriptor::RevisionPosture;

impl ArchivedRevisionBinding {
    /// The original revision address claim, without a live revision mint.
    #[must_use]
    pub const fn revision(&self) -> &[u8; 32] {
        &self.revision
    }

    /// The original declared revision posture.
    #[must_use]
    pub const fn posture(&self) -> RevisionPosture {
        self.posture
    }
}

impl ArchivedBinding {
    /// The exact nested encoding for caller-owned storage.
    #[must_use]
    pub fn encoded(&self) -> &[u8] {
        &self.encoded
    }

    /// The complete historical descriptor row.
    #[must_use]
    pub const fn row(&self) -> &ArchivedRow {
        &self.row
    }

    /// The attachment's historical subject name.
    #[must_use]
    pub const fn subject(&self) -> &ArchivedName {
        &self.subject
    }

    /// The attachment's historical check name.
    #[must_use]
    pub const fn check(&self) -> &ArchivedName {
        &self.check
    }

    /// The attachment's subject revision claim and posture.
    #[must_use]
    pub const fn subject_revision(&self) -> ArchivedRevisionBinding {
        self.subject_revision
    }

    /// The attachment's check revision claim and posture.
    #[must_use]
    pub const fn check_revision(&self) -> ArchivedRevisionBinding {
        self.check_revision
    }

    /// The retained producer standing without a currency assertion.
    #[must_use]
    pub const fn provenance(&self) -> &ArchivedProvenance {
        &self.provenance
    }
}

impl ArchivedRow {
    /// Derive an integrity address from the complete historical semantic coordinates.
    pub(crate) fn trial_key_address(&self) -> crate::identity::ContentAddress {
        let preimage = crate::descriptor::encode::encode_trial_names(
            parts(self.claim()),
            parts(self.subject()),
            parts(self.check()),
            parts(self.population()),
        );
        crate::descriptor::encode::derive_trial_key(&preimage)
    }
}

fn parts(name: &ArchivedName) -> (&str, &str) {
    (name.namespace(), name.stem())
}
