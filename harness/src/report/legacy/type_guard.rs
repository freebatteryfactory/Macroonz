//! Source admission and readers for historical claims.

use super::{
    Admission, DigestSeed, KeySeed, LegacyField, LegacyInputProfile, LegacyLimits, LegacyPresence,
    LegacyProfile, LegacyRecord, LegacyRefusal, Nullable, NumberSeed, Object, ProfileSeed,
    RecordSeed, TextSeed, WitnessSeed,
};
use crate::report::archive::AddressClaim;

#[path = "read.rs"]
mod read;
#[path = "admit.rs"]
mod admit;
#[path = "type_contract.rs"]
mod contract;

pub use read::read_record;

impl<'kind> LegacyProfile<'kind> {
    /// The exact labels to require when their source headers are present.
    #[must_use]
    pub const fn declared(kind: &'kind str, schema: u64) -> Self {
        Self { kind, schema }
    }

    /// The independently expected kind label.
    #[must_use]
    pub const fn kind(self) -> &'kind str {
        self.kind
    }

    /// The independently expected schema label.
    #[must_use]
    pub const fn schema(self) -> u64 {
        self.schema
    }
}

impl LegacyLimits {
    /// The source, text, witness, member, depth and retained-byte ceilings.
    #[must_use]
    pub const fn declared(
        source: usize,
        text: usize,
        witness: usize,
        members: usize,
        depth: usize,
        retained: usize,
    ) -> Self {
        Self {
            source,
            text,
            witness,
            members,
            depth,
            retained,
        }
    }

    /// The declared source ceiling.
    #[must_use]
    pub const fn source(self) -> usize {
        self.source
    }

    /// The declared text ceiling.
    #[must_use]
    pub const fn text(self) -> usize {
        self.text
    }

    /// The declared witness ceiling.
    #[must_use]
    pub const fn witness(self) -> usize {
        self.witness
    }

    /// The declared members ceiling.
    #[must_use]
    pub const fn members(self) -> usize {
        self.members
    }

    /// The declared depth ceiling.
    #[must_use]
    pub const fn depth(self) -> usize {
        self.depth
    }

    /// The declared retained ceiling.
    #[must_use]
    pub const fn retained(self) -> usize {
        self.retained
    }
}

impl LegacyRecord {
    /// The address of the exact JSON source, without writer authentication.
    #[must_use]
    pub fn source_address(&self) -> crate::identity::ContentAddress {
        crate::identity::ContentAddress::derived(super::LEGACY_SOURCE_TAG, &self.source)
    }

    /// The exact supplied JSON, ready for caller-owned storage.
    #[must_use]
    pub fn source(&self) -> &[u8] {
        &self.source
    }

    /// The supplied kind header.
    #[must_use]
    pub const fn kind(&self) -> &LegacyPresence<String> {
        &self.kind
    }

    /// The supplied schema header.
    #[must_use]
    pub const fn schema(&self) -> &LegacyPresence<u64> {
        &self.schema
    }

    /// The exact saved witness, including an explicitly empty specimen.
    #[must_use]
    pub const fn witness(&self) -> &LegacyPresence<Vec<u8>> {
        &self.witness
    }

    /// The partial historical input convention.
    #[must_use]
    pub const fn input_profile(&self) -> &LegacyPresence<LegacyInputProfile> {
        &self.input_profile
    }

    /// The source's trial label.
    #[must_use]
    pub const fn trial_name(&self) -> &LegacyPresence<String> {
        &self.trial_name
    }

    /// The source's subject label.
    #[must_use]
    pub const fn subject_name(&self) -> &LegacyPresence<String> {
        &self.subject_name
    }

    /// The source's check label.
    #[must_use]
    pub const fn check_name(&self) -> &LegacyPresence<String> {
        &self.check_name
    }

    /// The source's opaque subject revision marker.
    #[must_use]
    pub const fn subject_revision(&self) -> &LegacyPresence<u64> {
        &self.subject_revision
    }

    /// The source's opaque check revision marker.
    #[must_use]
    pub const fn check_revision(&self) -> &LegacyPresence<u64> {
        &self.check_revision
    }

    /// The source's target label.
    #[must_use]
    pub const fn target(&self) -> &LegacyPresence<String> {
        &self.target
    }

    /// The source's toolchain label.
    #[must_use]
    pub const fn toolchain(&self) -> &LegacyPresence<String> {
        &self.toolchain
    }

    /// The claimed execution digest without its preimage.
    #[must_use]
    pub const fn execution_digest(&self) -> &LegacyPresence<AddressClaim> {
        &self.execution_digest
    }

    /// The claimed fingerprint digest without its preimage.
    #[must_use]
    pub const fn fingerprint_digest(&self) -> &LegacyPresence<AddressClaim> {
        &self.fingerprint_digest
    }

    /// The source's uninterpreted outcome label.
    #[must_use]
    pub const fn reported_outcome(&self) -> &LegacyPresence<String> {
        &self.reported_outcome
    }
}

impl LegacyInputProfile {
    /// The profile's source name without current-name admission.
    #[must_use]
    pub const fn name(&self) -> &LegacyPresence<String> {
        &self.name
    }

    /// The source's numeric version marker.
    #[must_use]
    pub const fn revision(&self) -> &LegacyPresence<u64> {
        &self.revision
    }
}
