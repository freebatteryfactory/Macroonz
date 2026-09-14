//! Read-only historical trial projections.

use super::super::{
    ArchivedAttempt, ArchivedExecution, ArchivedFinding, ArchivedFingerprint, ArchivedForeignText,
    ArchivedMeasurement, ArchivedSite, ArchivedTrial, ArchivedTruncation,
};
use crate::clock::ClockAttribution;
use crate::identity::ContentAddress;
use crate::report::{ReplayPosture, TextFidelity};

impl ArchivedSite {
    /// The historical module spelling.
    #[must_use]
    pub fn module_path(&self) -> &str {
        &self.module_path
    }

    /// The historical source file.
    #[must_use]
    pub fn file(&self) -> &str {
        &self.file
    }

    /// The historical source line.
    #[must_use]
    pub const fn line(&self) -> u32 {
        self.line
    }

    /// The historical display name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl ArchivedForeignText {
    /// The exact retained bytes.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// The original retention counts.
    #[must_use]
    pub const fn truncation(&self) -> ArchivedTruncation {
        self.truncation
    }

    /// The rendering fidelity of the retained bytes.
    #[must_use]
    pub const fn fidelity(&self) -> TextFidelity {
        self.fidelity
    }
}

impl ArchivedFinding {
    /// The failure identity derived from the supplied preimage.
    #[must_use]
    pub const fn fingerprint(&self) -> &ArchivedFingerprint {
        &self.fingerprint
    }

    /// The historical refusal file.
    #[must_use]
    pub fn file(&self) -> &str {
        &self.file
    }

    /// The historical refusal line.
    #[must_use]
    pub const fn line(&self) -> u32 {
        self.line
    }

    /// The exact foreign material retained at the refusal.
    #[must_use]
    pub const fn foreign(&self) -> Option<&ArchivedForeignText> {
        self.foreign.as_ref()
    }
}

impl ArchivedTrial {
    /// The exact envelope for caller-owned storage.
    #[must_use]
    pub fn encoded(&self) -> &[u8] {
        &self.encoded
    }

    /// The address derived from this historical envelope.
    #[must_use]
    pub const fn address(&self) -> ContentAddress {
        self.address
    }

    /// The historical execution coordinates.
    #[must_use]
    pub const fn key(&self) -> &ArchivedExecution {
        &self.key
    }

    /// The source's replay ceiling without current reproduction authority.
    #[must_use]
    pub const fn claimed_posture(&self) -> ReplayPosture {
        self.posture
    }

    /// The original source site.
    #[must_use]
    pub const fn site(&self) -> &ArchivedSite {
        &self.site
    }

    /// The full historical attempt.
    #[must_use]
    pub const fn attempt(&self) -> &ArchivedAttempt {
        &self.attempt
    }

    /// The independent historical wall reading.
    #[must_use]
    pub const fn measurement(&self) -> ArchivedMeasurement {
        self.measurement
    }

    /// The historical source classification, without current native-clock authority.
    #[must_use]
    pub const fn clock_attribution(&self) -> ClockAttribution {
        self.clock_attribution
    }
}
