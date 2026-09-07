//! Read-only access to historical values and declared resource bounds.

use super::{
    AddressClaim, ArchiveLimits, ArchiveRefusal, ArchivedCapsule, ArchivedExecution,
    ArchivedFingerprint, ArchivedInput, ArchivedProfile, CAPSULE_ARCHIVE_TAG,
};
use crate::identity::ContentAddress;
use crate::report::{FailureClass, InvocationProfile, ReplayPosture, TargetBinding};

#[path = "read_identity.rs"]
mod identity;

#[path = "read_trial.rs"]
mod trial;
#[path = "guard_trial.rs"]
mod trial_readings;

#[path = "read_run.rs"]
mod run;
#[path = "guard_run.rs"]
mod run_readings;

#[path = "read.rs"]
mod read;

pub use read::read_capsule;
pub use run::read_run;
pub use trial::read_trial;

pub(crate) use identity::{
    claim, cursor, envelope, execution, fingerprint, finish, frame, posture, text,
};
pub(crate) use run::name;

impl ArchiveLimits {
    /// The complete-envelope and per-frame ceilings.
    #[must_use]
    pub const fn declared(envelope: usize, field: usize) -> Self {
        Self { envelope, field }
    }

    /// The complete-envelope ceiling.
    #[must_use]
    pub const fn envelope(self) -> usize {
        self.envelope
    }

    /// The ceiling on each framed member.
    #[must_use]
    pub const fn field(self) -> usize {
        self.field
    }
}

impl AddressClaim {
    /// The claimed bytes, without a content-address mint.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl ArchivedProfile {
    /// The exact historical spelling.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The historical version.
    #[must_use]
    pub const fn version(&self) -> u32 {
        self.version
    }
}

impl ArchivedInput {
    /// The historical profile namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// The historical local profile and version.
    #[must_use]
    pub const fn profile(&self) -> &ArchivedProfile {
        &self.profile
    }

    /// The claimed input schema.
    #[must_use]
    pub const fn schema(&self) -> AddressClaim {
        self.schema
    }

    /// The original case claim, distinct from the reached witness.
    #[must_use]
    pub const fn case(&self) -> AddressClaim {
        self.case
    }

    /// The historical decoder revision claim.
    #[must_use]
    pub const fn decoder(&self) -> AddressClaim {
        self.decoder
    }

    /// The recorded decoder posture, without current execution authority.
    #[must_use]
    pub const fn posture(&self) -> ReplayPosture {
        self.posture
    }
}

impl ArchivedExecution {
    /// The address derived from the supplied canonical key preimage.
    #[must_use]
    pub const fn address(&self) -> ContentAddress {
        self.address
    }

    /// The semantic trial claim.
    #[must_use]
    pub const fn trial(&self) -> AddressClaim {
        self.trial
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

    /// The recorded invocation budgets.
    #[must_use]
    pub const fn invocation(&self) -> InvocationProfile {
        self.invocation
    }

    /// The historical target and toolchain labels.
    #[must_use]
    pub const fn target(&self) -> &TargetBinding {
        &self.target
    }

    /// The original input standing, absent on the unit-input road.
    #[must_use]
    pub const fn input(&self) -> Option<&ArchivedInput> {
        self.input.as_ref()
    }
}

impl ArchivedFingerprint {
    /// The address derived from the supplied fingerprint preimage.
    #[must_use]
    pub const fn address(&self) -> ContentAddress {
        self.address
    }

    /// The trial claim the fingerprint is joined to.
    #[must_use]
    pub const fn trial(&self) -> AddressClaim {
        self.trial
    }

    /// The caller-owned cause family spelling.
    #[must_use]
    pub fn family(&self) -> &str {
        &self.family
    }

    /// The caller-owned local cause spelling.
    #[must_use]
    pub fn local(&self) -> &str {
        &self.local
    }

    /// The recorded failure class.
    #[must_use]
    pub const fn class(&self) -> FailureClass {
        self.class
    }
}

impl ArchivedCapsule {
    /// The exact envelope for caller-owned storage.
    #[must_use]
    pub fn encoded(&self) -> &[u8] {
        &self.encoded
    }

    /// The address of this historical envelope.
    #[must_use]
    pub const fn address(&self) -> ContentAddress {
        self.address
    }

    /// The capsule identity derived from its supplied canonical preimage.
    #[must_use]
    pub const fn identity(&self) -> ContentAddress {
        self.identity
    }

    /// The original failing execution's historical coordinates.
    #[must_use]
    pub const fn key(&self) -> &ArchivedExecution {
        &self.key
    }

    /// The historical failure identity retained during reduction.
    #[must_use]
    pub const fn fingerprint(&self) -> &ArchivedFingerprint {
        &self.fingerprint
    }

    /// The reached witness bytes, ready for independent input admission.
    #[must_use]
    pub fn input(&self) -> &[u8] {
        &self.input
    }

    /// The historical generation profile.
    #[must_use]
    pub const fn generation(&self) -> &ArchivedProfile {
        &self.generation
    }

    /// The historical minimization profile.
    #[must_use]
    pub const fn minimization(&self) -> &ArchivedProfile {
        &self.minimization
    }

    /// The generated-support schema claim.
    #[must_use]
    pub const fn schema(&self) -> AddressClaim {
        self.schema
    }

    /// The source's replay-ceiling claim, never current reproduction authority.
    #[must_use]
    pub const fn claimed_posture(&self) -> ReplayPosture {
        self.posture
    }
}
