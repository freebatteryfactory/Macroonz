//! Structural rewrite descriptors, rosters, candidates, and admission posture.

use crate::muterprater::{MissingTrustEvidence, OperatorFamilyRef, ScopeShape};
#[path = "type_guard.rs"]
mod guard;

// ---------------------------------------------------------------------------
// The rewrite lane.
// ---------------------------------------------------------------------------

/// One rewrite-mutation descriptor: the shape a damage matches, the shape it rewrites to, and the operator family the pair realizes.
///
/// Data rows, never programs: a descriptor states a pattern and its rewrite as text a structural rewriter reads, and nothing here compiles, executes, or interprets either side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RewriteDescriptor {
    family: OperatorFamilyRef,
    pattern: &'static str,
    rewrite: &'static str,
}

/// Why one rewrite descriptor was refused.
///
/// Dependent checks in a declared order: the pattern, then the rewrite, then the pair.
#[must_use = "a refusal is the reason a rewrite descriptor was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RewriteRefusal {
    /// The pattern is empty, so the descriptor matches nothing.
    EmptyPattern,
    /// The rewrite is empty, so the descriptor states no damage.
    EmptyRewrite,
    /// The pattern and the rewrite are one shape, so applying it damages nothing.
    RewriteIsPattern,
}

/// The rewrite lane's declared descriptors.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RewriteRoster {
    descriptors: Vec<RewriteDescriptor>,
}

/// Why one rewrite roster was refused.
#[must_use = "a refusal is the reason a rewrite roster was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RosterRefusal {
    /// The roster states no descriptor at all.
    EmptyRoster,
    /// Two entries state one pattern-and-rewrite pair.
    DuplicateDescriptor {
        /// The second entry's position in the roster.
        at: usize,
    },
}

/// The trust posture every rewrite-produced descriptor stands under.
///
/// Rewrite-produced descriptors are admitted last, as candidates the harness audits and never as evidence on their own.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RewriteTrust {
    /// The descriptor awaits the harness's audit.
    AuditPending,
}

/// One rewrite descriptor planned for audit, with the scope it was planned under and the trust it stands under.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RewriteCandidate {
    descriptor: RewriteDescriptor,
    scope: ScopeShape,
    trust: RewriteTrust,
}

/// Why rewrite descriptors may not enter the interpreted audit road.
#[must_use = "a refusal is the reason the rewrite audit road was withheld"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RewriteWithheld {
    /// The interpreted lane, which is what makes rewrite families cheap, is unavailable.
    InterpreterUnavailable,
    /// The trust order still owes this evidence.
    TrustNotOpened(MissingTrustEvidence),
}

/// Whether rewrite descriptors may enter the interpreted audit road.
///
/// Admission here is execution availability and not evidence: a descriptor stays [`RewriteTrust::AuditPending`] until an actual execution establishes what a later evidence owner requires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RewriteAdmission {
    /// The audit road is available under a generic suite bite and exact selection-scoped projection pressure.
    Admitted,
    /// The audit road is unavailable for a stated reason.
    Withheld(RewriteWithheld),
}

// ---------------------------------------------------------------------------
// The artifact-mutation seed roster.
// ---------------------------------------------------------------------------

/// One deliberate damage the artifact-mutation mode inflicts on a lawful rendered artifact.
///
/// Each arm is a lie a damaged rendering tells about the declaration it claims to project, and every one of them is this harness's own — a producer that writes its own exam is rehearsed only against the defects it already imagined.
/// The roster is seed material rather than a lane: the surgery that realizes one is authored where the anchors are, so a damage is cut against the anchors a generator emits rather than against spellings a hand restated beside them.
///
/// # Nonclaims
///
/// It says nothing about which reader catches a damage.
/// That ownership belongs to the readers that exist ([`crate::oracle`]) and is stated there, against a seat that can hold it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArtifactMutation {
    /// The emitted members are written in reverse of the order the declaration states.
    OrderPermuted,
    /// Every emitted member is written under the first member's key, so members the declaration keeps distinct share one identity.
    IdentityRecycled,
    /// One planned output is deleted from the artifact.
    PlannedOutputOmitted,
    /// An output nobody planned is appended.
    UnplannedOutputAdded,
    /// The implementation targets a different type than the one declared.
    ImplTargetAltered,
    /// The declared body shape is changed.
    ShapeAltered,
    /// A planned output is emitted twice.
    OutputDuplicated,
    /// The trait path names a contract the declaration did not realize.
    TraitPathWrong,
    /// A decoy carrying the anchored bytes is planted in a comment while the real constant is damaged.
    DecoyInComment,
    /// One planned member constant is emitted twice inside one implementation.
    ImplMemberDuplicated,
    /// A member nobody planned is added inside one implementation.
    ImplMemberUnexpected,
    /// A declared value is carried through a constructor the declaration did not name.
    ConstructorPathAltered,
    /// The implementation is written under a posture the declaration did not name.
    ImplPostureAltered,
    /// An attribute that decides something is added to an implementation.
    MeaningBearingAttributeAdded,
    /// The artifact stops being well-formed Rust.
    MalformedRust,
}

/// The artifact-mutation roster, in the order this home states it.
///
/// A declared table rather than a derived one, so a plan reads the damages in an order written down once here.
/// A slice rather than a sized array: a consumer whose artifacts break in ways this table does not name declares its own slice, and nothing here closes the width.
pub const ARTIFACT_MUTATIONS: &[ArtifactMutation] = &[
    ArtifactMutation::OrderPermuted,
    ArtifactMutation::IdentityRecycled,
    ArtifactMutation::PlannedOutputOmitted,
    ArtifactMutation::UnplannedOutputAdded,
    ArtifactMutation::ImplTargetAltered,
    ArtifactMutation::ShapeAltered,
    ArtifactMutation::OutputDuplicated,
    ArtifactMutation::TraitPathWrong,
    ArtifactMutation::DecoyInComment,
    ArtifactMutation::ImplMemberDuplicated,
    ArtifactMutation::ImplMemberUnexpected,
    ArtifactMutation::ConstructorPathAltered,
    ArtifactMutation::ImplPostureAltered,
    ArtifactMutation::MeaningBearingAttributeAdded,
    ArtifactMutation::MalformedRust,
];
