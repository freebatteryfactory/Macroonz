//! Findings, the text they carry in from outside, the failure identity they are named by, and the reproduction account a reduction earns.

use crate::descriptor::{GeneratedSupportSchemaId, RevisionPosture};
use crate::identity::ContentAddress;
use crate::report::{
    ExecutionKey, FINGERPRINT_TAG, FOREIGN_TEXT_MAX_BYTES, FailureClass, FindingCause,
    FindingLocation, Fingerprint, ForeignText, GenerationProfile, MinimizationProfile,
    REPLAY_CAPSULE_TAG, ReplayCapsule, ReplayPosture, TextFidelity, TrialFinding, TrialId,
    TrialRunStanding, Truncation, fingerprint_preimage, replay_capsule_preimage,
};

impl GenerationProfile {
    /// The generation profile under its declared name and version.
    #[must_use]
    pub const fn declared(name: &'static str, version: u32) -> Self {
        Self { name, version }
    }

    /// The declared name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        self.name
    }

    /// The declared version.
    #[must_use]
    pub const fn version(self) -> u32 {
        self.version
    }
}

impl MinimizationProfile {
    /// The minimization profile under its declared name and version.
    #[must_use]
    pub const fn declared(name: &'static str, version: u32) -> Self {
        Self { name, version }
    }

    /// The declared name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        self.name
    }

    /// The declared version.
    #[must_use]
    pub const fn version(self) -> u32 {
        self.version
    }
}

impl ReplayCapsule {
    /// Mint one run-bound account once the reduction owner has joined every input.
    ///
    /// Crate-visible for [`crate::generate::capture_replay`] alone: a public caller reaches a capsule by first earning [`crate::generate::ReductionEvidence`], never by supplying these seats.
    #[must_use]
    pub(crate) fn captured(
        standing: &TrialRunStanding,
        input: &[u8],
        fingerprint: Fingerprint,
        generation: GenerationProfile,
        minimization: MinimizationProfile,
        schema: GeneratedSupportSchemaId,
        posture: ReplayPosture,
    ) -> Self {
        Self {
            key: standing.key().clone(),
            input: input.to_vec(),
            fingerprint,
            generation,
            minimization,
            schema,
            posture,
        }
    }

    /// The execution this capsule reproduces.
    #[must_use]
    pub const fn key(&self) -> &ExecutionKey {
        &self.key
    }

    /// The exact input bytes the execution was handed.
    #[must_use]
    pub fn input(&self) -> &[u8] {
        &self.input
    }

    /// The failure fingerprint this input preserved during reduction.
    #[must_use]
    pub const fn fingerprint(&self) -> Fingerprint {
        self.fingerprint
    }

    /// The generation profile that produced the input.
    #[must_use]
    pub const fn generation(&self) -> GenerationProfile {
        self.generation
    }

    /// The minimization profile that reduced the input.
    #[must_use]
    pub const fn minimization(&self) -> MinimizationProfile {
        self.minimization
    }

    /// The generated-support schema identity the input conforms to.
    #[must_use]
    pub const fn schema(&self) -> GeneratedSupportSchemaId {
        self.schema
    }

    /// What reproducing this capsule can claim.
    #[must_use]
    pub const fn posture(&self) -> ReplayPosture {
        self.posture
    }

    /// The capsule's own identity, derived from its complete preimage.
    #[must_use]
    pub fn identity(&self) -> ContentAddress {
        ContentAddress::derived(REPLAY_CAPSULE_TAG, &replay_capsule_preimage(self))
    }
}

impl ReplayPosture {
    /// Meet this ceiling with one more callable revision posture.
    ///
    /// Derived keeps the standing ceiling, declared forecloses an exact derived claim, and untracked makes exact reproduction unavailable.
    /// Crate-visible because only an owner assembling run evidence computes the meet; a caller reads the result off the evidence.
    #[must_use]
    pub(crate) const fn meet_revision(self, revision: RevisionPosture) -> Self {
        match (self, revision) {
            (Self::UnavailableBecauseUntracked, _) | (_, RevisionPosture::Untracked) => {
                Self::UnavailableBecauseUntracked
            }
            (Self::DeclaredByAuthor, RevisionPosture::Derived | RevisionPosture::Declared)
            | (Self::ExactDerived, RevisionPosture::Declared) => Self::DeclaredByAuthor,
            (Self::ExactDerived, RevisionPosture::Derived) => Self::ExactDerived,
        }
    }
}

impl FindingCause {
    /// Cite one cause by the pair of declared names its owner wrote down.
    #[must_use]
    pub const fn named(family: &'static str, local: &'static str) -> Self {
        Self { family, local }
    }

    /// The cause family.
    #[must_use]
    pub const fn family(self) -> &'static str {
        self.family
    }

    /// The local key inside that family.
    #[must_use]
    pub const fn local(self) -> &'static str {
        self.local
    }
}

impl FindingLocation {
    /// Where a refusal was raised.
    #[must_use]
    pub const fn at(file: &'static str, line: u32) -> Self {
        Self { file, line }
    }

    /// The file.
    #[must_use]
    pub const fn file(self) -> &'static str {
        self.file
    }

    /// The line.
    #[must_use]
    pub const fn line(self) -> u32 {
        self.line
    }
}

impl ForeignText {
    /// Admit foreign material at the bound, recording what was cut and whether rendering it loses bytes.
    ///
    /// Total: material past the bound is cut rather than refused, and the cut is a typed fact on the value, so nothing shows a reader a shortened rendering that looks whole.
    /// A cut can land mid-sequence, which is what [`TextFidelity::LossyReplacement`] states.
    #[must_use]
    pub fn admitted(material: &[u8]) -> Self {
        let bytes: Vec<u8> = material
            .iter()
            .copied()
            .take(FOREIGN_TEXT_MAX_BYTES)
            .collect();
        let truncation = if material.len() > FOREIGN_TEXT_MAX_BYTES {
            Truncation::TruncatedAt {
                admitted: bytes.len(),
                offered: material.len(),
            }
        } else {
            Truncation::Complete
        };
        let fidelity = if core::str::from_utf8(&bytes).is_ok() {
            TextFidelity::Exact
        } else {
            TextFidelity::LossyReplacement
        };
        Self {
            bytes,
            truncation,
            fidelity,
        }
    }

    /// The admitted bytes.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Whether everything offered was admitted.
    #[must_use]
    pub const fn truncation(&self) -> Truncation {
        self.truncation
    }

    /// Whether rendering the admitted bytes as text loses anything.
    #[must_use]
    pub const fn fidelity(&self) -> TextFidelity {
        self.fidelity
    }

    /// The rendering, for a caller to show a person.
    ///
    /// The one lawful use of the bytes, and a one-way road: nothing in the harness reads this back.
    #[must_use]
    pub fn shown(&self) -> String {
        String::from_utf8_lossy(&self.bytes).into_owned()
    }
}

impl TrialFinding {
    /// The typed refusal one check returned.
    #[must_use]
    pub fn established(
        class: FailureClass,
        cause: FindingCause,
        located: FindingLocation,
        foreign: Option<ForeignText>,
    ) -> Self {
        Self {
            class,
            cause,
            located,
            foreign,
        }
    }

    /// The normalized failure class.
    #[must_use]
    pub const fn class(&self) -> FailureClass {
        self.class
    }

    /// The typed cause.
    #[must_use]
    pub const fn cause(&self) -> FindingCause {
        self.cause
    }

    /// Where the refusal was raised.
    #[must_use]
    pub const fn located(&self) -> FindingLocation {
        self.located
    }

    /// The foreign text the finding carried in, where there was any.
    #[must_use]
    pub fn foreign(&self) -> Option<&ForeignText> {
        self.foreign.as_ref()
    }
}

impl Fingerprint {
    /// The failure identity of one finding under one trial.
    #[must_use]
    pub const fn of(trial: TrialId, finding: &TrialFinding) -> Self {
        Self {
            trial,
            cause: finding.cause,
            class: finding.class,
        }
    }

    /// The failure identity of one cause and class under one trial.
    #[must_use]
    pub const fn over(trial: TrialId, cause: FindingCause, class: FailureClass) -> Self {
        Self {
            trial,
            cause,
            class,
        }
    }

    /// The trial the failure was found under.
    #[must_use]
    pub const fn trial(self) -> TrialId {
        self.trial
    }

    /// The typed cause.
    #[must_use]
    pub const fn cause(self) -> FindingCause {
        self.cause
    }

    /// The normalized failure class.
    #[must_use]
    pub const fn class(self) -> FailureClass {
        self.class
    }

    /// The fingerprint's address, derived from its three coordinates.
    #[must_use]
    pub fn address(self) -> ContentAddress {
        ContentAddress::derived(
            FINGERPRINT_TAG,
            &fingerprint_preimage(self.trial, self.cause, self.class),
        )
    }
}
