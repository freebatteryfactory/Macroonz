//! The canonical preimage of every identity this home mints, and the one
//! length framing they are all written through.
//!
//! A preimage is the exact byte string handed to the identity substrate. The
//! specifications here are COMPLETE: an independent implementation needs what
//! each function documents and nothing else, which is what makes a published
//! identity something a second party can re-derive rather than take on trust.
//!
//! One framing for the whole home rather than one per identity is what keeps
//! the concatenation collision from being reintroduced locally: a member
//! written without its length would let two different member sequences cut into
//! one byte string.
//!
//! Nothing here reads an ambient fact. Every byte comes from a value the caller
//! already holds, and a name owned by another home is written from that home's
//! own two declared parts rather than from anything this file invents.

use super::{
    CheckRevisionId, FailureClass, FindingCause, InvocationProfile, ReplayCapsule, ReplayPosture,
    SubjectRevisionId, TargetBinding, TrialCoordinates, TrialId, TrialProfile,
};
use crate::descriptor::NamespacedName;

/// Append one length as eight big-endian bytes.
///
/// A fixed width rather than a varint, because a canonical encoding that
/// admitted two spellings of one length would admit two preimages for one
/// value.
pub fn encode_length(length: usize, into: &mut Vec<u8>) {
    into.extend_from_slice(&u64::try_from(length).unwrap_or(u64::MAX).to_be_bytes());
}

/// Append one length-prefixed byte string: the eight-byte length, then the
/// bytes.
pub fn encode_bytes(material: &[u8], into: &mut Vec<u8>) {
    encode_length(material.len(), into);
    into.extend_from_slice(material);
}

/// Append one namespaced name: its namespace, then its stem, each framed.
///
/// Two framed members rather than a joined spelling, so no pair of namespace
/// and stem can be re-cut into a different pair that encodes identically.
fn encode_name(name: NamespacedName, into: &mut Vec<u8>) {
    encode_bytes(name.namespace().as_bytes(), into);
    encode_bytes(name.stem().as_bytes(), into);
}

impl TrialProfile {
    /// The discriminant byte a preimage carries for this profile.
    ///
    /// A slot rather than the Rust spelling: renaming the variant leaves every
    /// identity derived under it with its name, and moving a slot is an
    /// identity-profile version bump.
    #[must_use]
    pub const fn slot(self) -> u8 {
        match self {
            Self::Unprofiled => 0,
        }
    }
}

impl FailureClass {
    /// The discriminant byte a preimage carries for this class.
    #[must_use]
    pub const fn slot(self) -> u8 {
        match self {
            Self::RefusedByCheck => 0,
            Self::PropertyDisagreement => 1,
            Self::OracleDisagreement => 2,
            Self::SubjectPanic => 3,
            Self::BudgetExhausted => 4,
        }
    }
}

impl ReplayPosture {
    /// The discriminant byte a preimage carries for this posture.
    #[must_use]
    pub const fn slot(self) -> u8 {
        match self {
            Self::ExactDerived => 0,
            Self::DeclaredByAuthor => 1,
            Self::UnavailableBecauseUntracked => 2,
        }
    }
}

/// The COMPLETE preimage of one [`TrialId`].
///
/// Two primitives:
///
/// - `u32be(n)` / `u64be(n)` — the integer in four or eight big-endian bytes.
/// - `bytes(x)` — `u64be(x.len())` followed by the bytes of `x`.
///
/// A namespaced name is `bytes(namespace)` followed by `bytes(stem)`.
///
/// The members, in exactly this order, with no separators and no padding:
///
/// | # | member | encoding |
/// | - | ------ | -------- |
/// | 1 | claim | the reference's namespaced name |
/// | 2 | subject route | the reference's namespaced name |
/// | 3 | mechanism | the check reference's namespaced name |
/// | 4 | population | the reference's namespaced name |
/// | 5 | profile | one byte, [`TrialProfile::slot`] |
///
/// Nothing about where the trial is written appears anywhere in it, which is
/// the encoding half of the promise that a trial identity survives a move. The
/// execution suite is absent for the same reason the descriptor-side trial key
/// leaves it out: two rows differing only by suite are one trial under two
/// seats.
#[must_use]
pub fn trial_preimage(coordinates: TrialCoordinates) -> Vec<u8> {
    let mut bytes = Vec::new();
    encode_name(coordinates.claim().name(), &mut bytes);
    encode_name(coordinates.subject().name(), &mut bytes);
    encode_name(coordinates.mechanism().name(), &mut bytes);
    encode_name(coordinates.population().name(), &mut bytes);
    bytes.push(coordinates.profile().slot());
    bytes
}

/// The COMPLETE preimage of one [`ExecutionKey`](super::ExecutionKey).
///
/// The members, in exactly this order:
///
/// | # | member | encoding |
/// | - | ------ | -------- |
/// | 1 | trial identity | `bytes(…)` of the full thirty-two |
/// | 2 | subject revision | `bytes(…)` of the full thirty-two |
/// | 3 | check revision | `bytes(…)` of the full thirty-two |
/// | 4 | case budget | `u32be` |
/// | 5 | byte budget | `u64be` |
/// | 6 | time budget | `u64be` |
/// | 7 | target triple | `bytes(utf8)` |
/// | 8 | toolchain identity | `bytes(utf8)` |
///
/// The target binding is members seven and eight unconditionally: there is no
/// shape of this preimage in which a run's target is absent, so a key derived
/// on one target cannot equal a key derived on another.
#[must_use]
pub fn execution_key_preimage(
    trial: TrialId,
    subject: SubjectRevisionId,
    check: CheckRevisionId,
    invocation: InvocationProfile,
    target: &TargetBinding,
) -> Vec<u8> {
    let mut bytes = Vec::new();
    encode_bytes(trial.address().as_bytes(), &mut bytes);
    encode_bytes(subject.address().as_bytes(), &mut bytes);
    encode_bytes(check.address().as_bytes(), &mut bytes);
    bytes.extend_from_slice(&invocation.cases().cases().to_be_bytes());
    bytes.extend_from_slice(&invocation.bytes().bytes().to_be_bytes());
    bytes.extend_from_slice(&invocation.time().nanoseconds().to_be_bytes());
    encode_bytes(target.target().spelling().as_bytes(), &mut bytes);
    encode_bytes(target.toolchain().spelling().as_bytes(), &mut bytes);
    bytes
}

/// The COMPLETE preimage of one [`ReplayCapsule`]'s identity.
///
/// The members, in exactly this order:
///
/// | # | member | encoding |
/// | - | ------ | -------- |
/// | 1 | execution key | `bytes(…)` of the key's derived address |
/// | 2 | input | `bytes(…)` — the exact input at full length, never a fold |
/// | 3 | generation profile | `bytes(utf8)` |
/// | 4 | generation version | `u32be` |
/// | 5 | minimization profile | `bytes(utf8)` |
/// | 6 | minimization version | `u32be` |
/// | 7 | generated-support schema | `bytes(…)` of its address, full thirty-two |
/// | 8 | replay posture | one byte, [`ReplayPosture::slot`] |
#[must_use]
pub fn replay_capsule_preimage(capsule: &ReplayCapsule) -> Vec<u8> {
    let mut bytes = Vec::new();
    encode_bytes(capsule.key().address().as_bytes(), &mut bytes);
    encode_bytes(capsule.input(), &mut bytes);
    encode_bytes(capsule.generation().name().as_bytes(), &mut bytes);
    bytes.extend_from_slice(&capsule.generation().version().to_be_bytes());
    encode_bytes(capsule.minimization().name().as_bytes(), &mut bytes);
    bytes.extend_from_slice(&capsule.minimization().version().to_be_bytes());
    encode_bytes(capsule.schema().address().as_bytes(), &mut bytes);
    bytes.push(capsule.posture().slot());
    bytes
}

/// The COMPLETE preimage of one failure fingerprint.
///
/// The members, in exactly this order:
///
/// | # | member | encoding |
/// | - | ------ | -------- |
/// | 1 | trial identity | `bytes(…)` of the full thirty-two |
/// | 2 | cause family | `bytes(utf8)` |
/// | 3 | cause local key | `bytes(utf8)` |
/// | 4 | failure class | one byte, [`FailureClass::slot`] |
///
/// The input that reached the failure is deliberately absent: a fingerprint
/// that moved with the input could not deduplicate two finds of one defect, and
/// could not survive the minimization that is required to preserve it.
#[must_use]
pub fn fingerprint_preimage(trial: TrialId, cause: FindingCause, class: FailureClass) -> Vec<u8> {
    let mut bytes = Vec::new();
    encode_bytes(trial.address().as_bytes(), &mut bytes);
    encode_bytes(cause.family().as_bytes(), &mut bytes);
    encode_bytes(cause.local().as_bytes(), &mut bytes);
    bytes.push(class.slot());
    bytes
}
