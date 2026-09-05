//! The target, verdict chain, mutation record, and run accounting.

use crate::depot::types::OperatorFamily;
use crate::descriptor::{ClaimRef, MutationPointRef};
use crate::identity::{DomainTag, IdentityProfileVersion};
use crate::muterprater::{ActivationSite, ActiveSelection, AlternativeId};
use crate::report::{Fingerprint, ForeignText, TrialFinding, TrialId};

macro_rules! with_mutation_verdicts {
    ($callback:ident) => {
        $callback! {
            /// The suite rejected the damaged subject.
            Killed => killed,
            /// The suite accepted a damage whose firing was observed under the exact selection and witness.
            Survived => survived,
            /// Nothing was learned about the suite from this mutant.
            Inconclusive => inconclusive,
        }
    };
}

#[path = "type_guard.rs"]
mod guard;

// ---------------------------------------------------------------------------
// The verdict chain.
// ---------------------------------------------------------------------------

/// What the unmutated subject's own suite did before any damage was inflicted.
///
/// An unchanged passing baseline is the precondition every kill stands on, so this axis is read before any other.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BaselineAxis {
    /// The unchanged subject's suite ran and passed.
    Qualified,
    /// The unchanged subject's suite ran and did not pass.
    Failed,
    /// The unchanged subject's suite was not run at all.
    NotRun,
}

/// Whether one damage became a thing that could be executed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MaterializationAxis {
    /// The damaged subject built.
    Built,
    /// The damage does not typecheck, or the site admits no such alternative.
    Unviable,
    /// The backend failed while materializing the damage, so nothing was established about it.
    ToolFailed,
}

/// What the backend or evaluation callable reported about one planted damage firing.
///
/// A damage nothing reached says nothing about the suite that did not catch it, which is why this axis stands between materialization and the verdict.
/// [`ActivationAxis::UnobservableUnderBackend`] is a fact about the backend and never about the damage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActivationAxis {
    /// An execution channel reported a positive firing count for the damage.
    Observed,
    /// The backend exposes an activation channel and supplied no positive observation.
    NotObserved,
    /// The backend offers no activation channel, so firing is unobservable under it.
    UnobservableUnderBackend,
}

/// What became of the witness execution over the damaged subject.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExecutionAxis {
    /// The witness ran to a conclusion.
    Completed,
    /// The witness was not executed.
    NotExecuted,
    /// The witness passed the time bound it was given.
    TimedOut,
    /// The witness process died.
    Crashed,
    /// The harness or the backend failed around the witness, so nothing was learned.
    InfrastructureFailed,
}

macro_rules! declare_mutation_verdicts {
    ($($(#[$variant_meta:meta])* $variant:ident => $seat:ident),+ $(,)?) => {
        /// What one mutant earned, at axis width.
        ///
        /// The record carries [`MutationOutcome`], whose arms carry the evidence each one requires; this is the word a census counts.
        /// A mutant unobservable under its backend can never earn [`MutationVerdict::Survived`], and that is a refusal in the record's constructors rather than a rule somebody follows.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum MutationVerdict {
            $($(#[$variant_meta])* $variant),+
        }
    };
}

with_mutation_verdicts!(declare_mutation_verdicts);

/// What was established about the damaged subject meaning what the lawful one means.
///
/// [`EquivalenceAxis::ProvenInScope`] claims equivalence over the scope the proof was taken in and never in general.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquivalenceAxis {
    /// No equivalence question was put.
    NotAssessed,
    /// The damaged subject was proven equivalent, in the scope the proof was taken in.
    ProvenInScope,
    /// The damaged subject was shown to differ from the lawful one.
    Refuted,
    /// The equivalence question was put and not answered.
    Inconclusive,
}

// ---------------------------------------------------------------------------
// The mutation target.
// ---------------------------------------------------------------------------

/// The domain tag every external mutant identity is derived under.
pub const MUTATION_TARGET_TAG: DomainTag =
    DomainTag::declared("mutation-target", IdentityProfileVersion::declared(1));

/// Where in a source text an external backend placed one damage.
///
/// Owned text rather than a [`crate::descriptor::NamespacedName`], because the spelling is a tool's output and not a name anybody authored.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SourceCoordinate {
    file: String,
    line: u32,
    column: u32,
}

/// Why one source coordinate was refused.
#[must_use = "a refusal is the reason a coordinate was not read"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CoordinateRefusal {
    /// The coordinate names no file, so it places nothing.
    EmptyFile,
}

crate::identity::content_address_reference! {
    /// One external mutant's identity, over the coordinate and damage text the backend reported.
    ///
    /// Two runs of one backend over one unchanged tree name the same mutant, and a moved line names a different one — which is what a coordinate affords.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct MutantId;
}

/// How one damaged thing is identified, by the lane that damaged it.
///
/// The two selected-projection arms name the same authored point and alternative under different execution roads, and their report provenance stays separate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MutationIdentity {
    /// An external backend's mutant, addressed by its reported coordinate and damage.
    External(MutantId),
    /// A point on an evaluation surface, addressed by the reference its producer authored.
    Interpreted {
        /// The stable point the producer discovered.
        point: MutationPointRef,
        /// The stable mutation meaning selected at that point.
        alternative: AlternativeId,
    },
    /// A separately materialized production-shaped selected projection.
    CompiledProjection {
        /// The stable point the producer discovered.
        point: MutationPointRef,
        /// The stable mutation meaning baked into the compiled artifact.
        alternative: AlternativeId,
    },
}

/// Where one damage lives, as the lane that placed it can say.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MutationSite {
    /// A source coordinate an external backend reported.
    Reported(SourceCoordinate),
    /// The named activation site a producer declared.
    Declared(ActivationSite),
}

/// One row of the operator-family bank, cited by the slug that bank declares.
///
/// [`OperatorFamilyRef::of_slug`] resolves against the bank's own roster, so a reference can never name a family the bank does not declare.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OperatorFamilyRef(OperatorFamily);

/// Whether one damage is one the operator-family bank names.
///
/// A backend applies its own operators, and attributing one of them to a family the bank never declared would be this lane inventing a fact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FamilyAttribution {
    /// The damage realizes a family the bank declares.
    Declared(OperatorFamilyRef),
    /// The damage is not one the bank names, so no family is claimed for it.
    OutsideTheBank,
}

/// Whether the origin reading named the claim that owns one damage's site.
///
/// The owning claim rides the mapped arm, so a mapped target without a claim is not a value anybody can hold.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MappingPosture {
    /// The reading named this claim as the owner of the damage's site.
    Mapped(ClaimRef),
    /// No mapping was available for the site.
    OwnerUnmapped,
}

/// One damaged thing a lane pressed: its identity, the family it realizes, where it lives, and whether its owning claim is known.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MutationTarget {
    identity: MutationIdentity,
    family: FamilyAttribution,
    site: MutationSite,
    owner: MappingPosture,
}

// ---------------------------------------------------------------------------
// Activation evidence.
// ---------------------------------------------------------------------------

/// A positive firing count reported for one exact selection and witness trial.
///
/// A zero count produces [`DudPlant`] instead, so a silent plant cannot enter the observed arm.
/// The count is callback output: this value binds it to a selection and a witness and does not instrument the callback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ActivationEvidence {
    selection: ActiveSelection,
    witness: TrialId,
    firings: u32,
}

/// A plant whose evaluation callback reported zero firings for its exact selection and witness trial.
#[must_use = "a dud plant is a finding, never a silent pass"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DudPlant {
    selection: ActiveSelection,
    witness: TrialId,
}

/// The activation axis with the evidence its observed arm requires.
///
/// The bare axis is [`ActivationAxis`], and the projection between the two is declared once in `type_contract.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActivationDisposition {
    /// A positive activation observation, with the selection, witness, and count it was bound to.
    Observed(ActivationEvidence),
    /// The backend exposes an activation channel and supplied no positive observation.
    NotObserved,
    /// The backend offers no activation channel at all.
    UnobservableUnderBackend,
}

// ---------------------------------------------------------------------------
// The rejection a kill stands on.
// ---------------------------------------------------------------------------

/// One rejection this harness's own engine demonstrated: the trial that refused, and the finding it refused with.
///
/// The finding is carried whole, so [`DemonstratedRejection::fingerprint`] derives the failure identity rather than remembering it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DemonstratedRejection {
    trial: TrialId,
    finding: TrialFinding,
}

/// The rejection one witness execution answered a damaged subject with.
///
/// [`IntendedRejection::Demonstrated`] names a trial and a finding, so it carries a failure fingerprint.
/// [`IntendedRejection::ReportedByBackend`] is an external tool's word and names neither, so a kill standing on it claims what the backend stated and nothing more.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IntendedRejection {
    /// This harness's engine ran the witness and the check refused.
    Demonstrated(DemonstratedRejection),
    /// An external backend ran its own command and reported a rejection.
    ReportedByBackend {
        /// The backend's own line, bounded and marked.
        stated: ForeignText,
    },
}

/// The failure identity a rejection carries, where it carries one.
///
/// [`RejectionIdentity::Unfingerprinted`] states that the rejection named neither a trial nor a cause, not that a value is missing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RejectionIdentity {
    /// The rejection names a fingerprint, derived from its trial and finding.
    Fingerprinted(Fingerprint),
    /// The rejection carries no trial and no cause, so it names no failure.
    Unfingerprinted,
}

// ---------------------------------------------------------------------------
// The per-mutant record, and the run over it.
// ---------------------------------------------------------------------------

/// Why nothing was learned about the suite from one mutant.
///
/// Every arm names a link of the verdict chain that did not hold, and none of them is a softer survivor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InconclusiveCause {
    /// The unchanged baseline did not qualify, so no rejection under it would prove anything.
    BaselineNotQualified,
    /// The damage never became a thing that could be executed.
    NotMaterialized,
    /// The backend can observe firing and reported none, so the suite was never asked.
    NotActivated,
    /// The witness execution did not complete.
    WitnessIncomplete,
    /// The backend cannot observe firing and the witness did not reject.
    UnobservableAndUnrejected,
    /// The damaged subject was proven equivalent in scope, so no suite could have rejected it.
    ProvenEquivalentInScope,
}

/// The verdict one mutant earned, with the evidence each arm requires.
///
/// The killed arm carries the rejection that killed it, so a kill asserted without one is unrepresentable.
/// The survived arm carries nothing, because it is the absence of a rejection after a positive firing count was bound to the selection and witness.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MutationOutcome {
    /// The witness rejected the damaged subject, and this is the rejection.
    Killed(IntendedRejection),
    /// A damage with an observed firing was accepted by its witness.
    Survived,
    /// Nothing was learned about the suite from this mutant.
    Inconclusive(InconclusiveCause),
}

/// One mutant's complete record: the target, every axis of the verdict chain, and the outcome the chain earned.
///
/// There is no loose public constructor: the wrapped-backend adapter, the interpreted receiver, and the compiled-projection road each mint only the records their own evidence affords.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MutationReport {
    target: MutationTarget,
    baseline: BaselineAxis,
    materialization: MaterializationAxis,
    activation: ActivationDisposition,
    execution: ExecutionAxis,
    outcome: MutationOutcome,
    equivalence: EquivalenceAxis,
}

/// Why one mutant's record could not be minted as a lawful kill.
///
/// Dependent checks in a declared order — baseline, materialization, activation, execution — so exactly one cause is true of any refused kill.
#[must_use = "a refusal is the reason a kill was not minted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KillRefusal {
    /// The unchanged baseline did not qualify, so a rejection under it proves nothing.
    BaselineNotQualified(BaselineAxis),
    /// The damage never materialized, so there was nothing for a witness to reject.
    NotMaterialized(MaterializationAxis),
    /// The backend can observe firing and supplied no positive observation.
    ActivationNotObserved,
    /// The witness execution did not complete, so its rejection is not the suite's answer.
    WitnessDidNotComplete(ExecutionAxis),
}

macro_rules! declare_mutation_census {
    ($($(#[$variant_meta:meta])* $variant:ident => $seat:ident),+ $(,)?) => {
        crate::census::declare_census! {
            /// The accounting over one pressure run's mutants.
            ///
            /// One seat per arm of [`MutationVerdict`], and [`MutationCensus::pressed`] is their sum rather than a total that could disagree with its parts.
            /// It counts mutants under one run, and it is not the trial, generation, or bench-sample census.
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct MutationCensus {
                count: u32,
                seat: MutationCensusSeat,
                context {}
                fields {
                    $( $variant => $seat, )+
                }
            }
        }
    };
}

with_mutation_verdicts!(declare_mutation_census);

/// That the unchanged subject's own suite ran and passed.
///
/// The precondition every kill stands on, carried as a value so that "was the baseline good" is not a question anywhere downstream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BaselineQualification {
    axis: BaselineAxis,
}

/// Why one baseline did not qualify.
#[must_use = "a refusal is the reason a baseline did not qualify"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BaselinePrecondition {
    /// The unchanged subject's suite ran and did not pass.
    BaselineFailed,
    /// The unchanged subject's suite was not run at all.
    BaselineNotRun,
}

/// One pressure run's complete record: the baseline it stood on, every mutant's report, and the census over them.
///
/// The baseline is a [`BaselineQualification`] rather than an axis reading, so a run whose baseline did not qualify is not a run this value describes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutationRun {
    baseline: BaselineQualification,
    reports: Vec<MutationReport>,
    census: MutationCensus,
}
