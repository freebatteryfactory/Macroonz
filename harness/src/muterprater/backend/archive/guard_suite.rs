//! Read-only historical suite pressure and its independent limits.

use super::super::{
    ArchivedAdapterProfile, ArchivedBackendManifest, ArchivedSuitePressure, BackendArchiveLimits,
    SuitePressureArchiveLimits,
};
use crate::identity::ContentAddress;
use crate::muterprater::verdict_archive::ArchivedMutation;
use crate::report::archive::ArchiveLimits;

impl SuitePressureArchiveLimits {
    /// Declare independent outer and nested backend ceilings.
    #[must_use]
    pub const fn declared(bytes: ArchiveLimits, backend: BackendArchiveLimits) -> Self {
        Self { bytes, backend }
    }

    /// The outer-envelope and framed-field ceilings.
    #[must_use]
    pub const fn bytes(self) -> ArchiveLimits {
        self.bytes
    }

    /// The complete nested manifest and original-material ceilings.
    #[must_use]
    pub const fn backend(self) -> BackendArchiveLimits {
        self.backend
    }
}

impl ArchivedSuitePressure {
    /// The complete historical pressure envelope.
    #[must_use]
    pub fn encoded(&self) -> &[u8] {
        &self.encoded
    }

    /// The integrity address of the historical envelope.
    #[must_use]
    pub const fn address(&self) -> ContentAddress {
        self.address
    }

    /// The version the historical source claimed its grammar was checked against.
    #[must_use]
    pub fn checked_version(&self) -> &str {
        &self.checked_version
    }

    /// The one profile shared by the historical qualification and manifest.
    #[must_use]
    pub const fn qualification_profile(&self) -> &ArchivedAdapterProfile {
        self.manifest.profile()
    }

    /// The complete historical backend manifest with optional joined original material.
    #[must_use]
    pub const fn manifest(&self) -> &ArchivedBackendManifest {
        &self.manifest
    }

    /// The first kill's zero-based position in the original run order.
    #[must_use]
    pub const fn kill_ordinal(&self) -> u64 {
        self.kill_ordinal
    }

    /// The complete first killed report retained from the manifest's run.
    #[must_use]
    pub const fn kill(&self) -> &ArchivedMutation {
        &self.kill
    }
}
