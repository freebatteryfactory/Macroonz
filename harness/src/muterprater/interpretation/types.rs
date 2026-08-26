//! Evaluation pairs, no-mutation parity, interpreted trust, and admitted active evidence.

use crate::descriptor::{CheckRef, ClaimRef, NameRefusal, RevisionBinding};
use crate::muterprater::{
    CompiledProjectionPressure, CompiledSuitePressure, DudPlant, EvaluationCallRefusal,
    EvaluationDirective, EvaluationFamilyRef, EvaluationSurface, EvaluationSurfaceId,
    MutationReport, SelectionRefusal,
};
use crate::properties::{Equivalence, SharedSubstrate, SubstrateRefusal};
use crate::report::{TrialConclusion, TrialReport};
use crate::runner::{ReportRecordingRefusal, TrialBinding};
#[path = "type_guard.rs"]
mod guard;

// ---------------------------------------------------------------------------
// The pair, its witness, and the no-mutation parity.
// ---------------------------------------------------------------------------

/// Why the mandatory no-mutation parity suite could not be declared.
///
/// Dependent checks in a declared order: the substrate names are parsed, then the roster they are declared into.
#[must_use = "a refusal is the reason a parity suite was not declared"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParityRefusal {
    /// A name this lane spells would not parse.
    NameNotParsed(NameRefusal),
    /// The shared-substrate roster refused the substrates it was given.
    SubstrateNotDeclared(SubstrateRefusal),
}

/// The production callable of one evaluation family.
pub type ProductionCall<Input, Meaning> = fn(&Input) -> Meaning;

/// The evaluation callable of one evaluation family.
pub type EvaluationCall<Input, Meaning> =
    for<'surface> fn(
        &Input,
        EvaluationDirective<'surface>,
    ) -> Result<EvaluationObservation<Meaning>, EvaluationCallRefusal>;

/// The check that judges one meaning under the trial binding it is joined to.
pub type MeaningCheck<Meaning> = fn(&Meaning) -> TrialConclusion;

/// Raw output from one evaluation call.
///
/// Caller output rather than admitted evidence: the receiver validates the directive, firing count, trial binding, report, and trust facts before any mutation evidence exists.
pub struct EvaluationObservation<Meaning> {
    meaning: Meaning,
    firings: u32,
}

/// The production callable and revision an owner declares for one evaluation family.
pub struct ProductionBinding<Input, Meaning> {
    family: EvaluationFamilyRef,
    revision: RevisionBinding,
    call: ProductionCall<Input, Meaning>,
}

/// The evaluation callable and revision bound to one exact evaluation surface.
pub struct EvaluationBinding<Input, Meaning> {
    family: EvaluationFamilyRef,
    revision: RevisionBinding,
    surface: EvaluationSurfaceId,
    call: EvaluationCall<Input, Meaning>,
}

/// One production and evaluation binding under a shared owner declaration and equivalence.
///
/// Matching family references prove the declared relationship and not behavioral agreement; only an executed no-mutation parity reading establishes that, for its exact input.
pub struct EvaluationPair<Input, Meaning> {
    production: ProductionBinding<Input, Meaning>,
    evaluation: EvaluationBinding<Input, Meaning>,
    same: Equivalence<Meaning>,
}

/// Why one production and evaluation binding could not be paired.
#[must_use = "a refusal is the reason an evaluation pair was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EvaluationPairRefusal {
    /// The two bindings name different owner families.
    FamilyMismatch {
        /// The production binding's family.
        production: EvaluationFamilyRef,
        /// The evaluation binding's family.
        evaluation: EvaluationFamilyRef,
    },
}

/// The identity and revision facts every reading over one evaluation pair retains.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvaluationPairStanding {
    family: EvaluationFamilyRef,
    production_revision: RevisionBinding,
    evaluation_revision: RevisionBinding,
    surface: EvaluationSurfaceId,
}

/// The exact member by which two evaluation-pair standings disagree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvaluationPairStandingMismatch {
    /// The standings name different evaluation families.
    Family {
        /// The required family.
        expected: EvaluationFamilyRef,
        /// The offered family.
        found: EvaluationFamilyRef,
    },
    /// The standings name different production revisions.
    ProductionRevision {
        /// The required production revision.
        expected: RevisionBinding,
        /// The offered production revision.
        found: RevisionBinding,
    },
    /// The standings name different evaluation revisions.
    EvaluationRevision {
        /// The required evaluation revision.
        expected: RevisionBinding,
        /// The offered evaluation revision.
        found: RevisionBinding,
    },
    /// The standings name different evaluation surfaces.
    Surface {
        /// The required surface.
        expected: EvaluationSurfaceId,
        /// The offered surface.
        found: EvaluationSurfaceId,
    },
    /// The standings differ beyond the individually projected members.
    StandingChanged,
}

/// One trial binding joined to the check identity and callable that judge mutation executions through it.
pub struct MutationWitness<Meaning> {
    binding: TrialBinding,
    check: MeaningCheck<Meaning>,
}

/// Why one mutation witness could not bind its check identity to its trial.
#[must_use = "a refusal is the reason a mutation witness was not bound"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MutationWitnessRefusal {
    /// The offered check identity is not the one the trial row retains.
    CheckMismatch {
        /// The check identity retained by the row.
        expected: CheckRef,
        /// The check identity offered with the callable.
        found: CheckRef,
    },
}

/// The three facts one no-mutation observation compares.
pub struct NoMutationResults<Meaning> {
    production: Meaning,
    evaluation: Meaning,
    evaluation_firings: u32,
}

/// The production and evaluation reports of one no-mutation comparison, kept in their semantic roles.
pub(in crate::muterprater) struct NoMutationReports {
    production: TrialReport,
    evaluation: TrialReport,
}

/// The exact input, results, substrate, conclusion, and reports of one no-mutation comparison.
pub struct NoMutationParityReading<'pair, 'input, Input, Meaning> {
    pair: &'pair EvaluationPair<Input, Meaning>,
    witness: MutationWitness<Meaning>,
    input: &'input Input,
    results: NoMutationResults<Meaning>,
    substrate: SharedSubstrate,
    conclusion: TrialConclusion,
    reports: NoMutationReports,
}

/// Why one no-mutation observation could not be recorded.
#[must_use = "a refusal is the reason no no-mutation reading was recorded"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NoMutationObservationRefusal {
    /// The shared-substrate declaration could not be built.
    Substrate(ParityRefusal),
    /// The evaluation callable refused the no-mutation directive.
    EvaluationCall(EvaluationCallRefusal),
    /// The production observation could not join its trial binding.
    ProductionReport(ReportRecordingRefusal),
    /// The evaluation observation could not join its trial binding.
    EvaluationReport(ReportRecordingRefusal),
}

/// Why one complete no-mutation reading did not qualify interpreted evidence.
#[must_use = "a refusal is the reason no-mutation parity did not qualify"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParityQualificationRefusal {
    /// The production report did not earn a lawful lens verdict.
    ProductionDidNotQualify,
    /// The evaluation report did not earn a lawful lens verdict.
    EvaluationDidNotQualify,
    /// The no-mutation directive reported an activation.
    NoMutationActivated {
        /// How many firings the evaluation callable reported.
        firings: u32,
    },
    /// The owner-declared equivalence refused the two meanings.
    MeaningsDisagreed,
}

/// One no-mutation reading that earned scoped parity qualification.
pub struct NoMutationParityQualification<'pair, 'input, Input, Meaning> {
    reading: NoMutationParityReading<'pair, 'input, Input, Meaning>,
}

/// One complete no-mutation reading that did not earn qualification, kept whole beside the reason.
pub struct RejectedNoMutationParity<'pair, 'input, Input, Meaning> {
    cause: ParityQualificationRefusal,
    reading: NoMutationParityReading<'pair, 'input, Input, Meaning>,
}

/// The qualification disposition of one complete no-mutation reading.
pub enum NoMutationParityStanding<'pair, 'input, Input, Meaning> {
    /// The reading earned parity qualification for its exact pair and input.
    Qualified(NoMutationParityQualification<'pair, 'input, Input, Meaning>),
    /// The reading remains available with the exact reason it did not qualify.
    Rejected(RejectedNoMutationParity<'pair, 'input, Input, Meaning>),
}
// ---------------------------------------------------------------------------
// The interpreted lane's trust boundary.
// ---------------------------------------------------------------------------

/// Which of the trust order's facts the interpreted lane is still owed.
///
/// Generic suite pressure proves the external suite bit somewhere under its adapter profile and carries no evaluation pair; exact projection pressure owns one qualified pair and one surface-issued selection.
/// Every arm names an absent or mismatched strict value rather than a weak one the gate tries to upgrade.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MissingTrustEvidence {
    /// No generic compiled suite pressure demonstrated that the suite bites.
    CompiledSuitePressure,
    /// No exact compiled selected-projection pressure exists for a selection.
    CompiledProjectionPressure,
    /// The exact projection pressure belongs to another evaluation surface.
    ProjectionPressureForAnotherSurface,
}

/// The generic suite bite and exact selection pressure that open interpreted execution for one surface.
pub struct InterpretedTrust<'surface, 'suite, 'projection, 'parity, 'pair, 'input, Input, Meaning> {
    surface: &'surface EvaluationSurface,
    suite: &'suite CompiledSuitePressure,
    projection: &'projection CompiledProjectionPressure<'parity, 'pair, 'input, Input, Meaning>,
}

/// The availability of interpreted evidence for one evaluation surface.
///
/// A surface alone earns no trust: availability takes a generic compiled suite bite plus exact projection pressure whose qualification, pair standing, and selection all belong to this surface.
pub enum InterpreterAvailability<
    'surface,
    'suite,
    'projection,
    'parity,
    'pair,
    'input,
    Input,
    Meaning,
> {
    /// A conforming evaluation surface exists and trust has opened.
    Available(
        InterpretedTrust<'surface, 'suite, 'projection, 'parity, 'pair, 'input, Input, Meaning>,
    ),
    /// No conforming evaluation surface exists, producer-authored or hand-authored.
    NoConformingSurface,
    /// A surface exists and the trust order still owes this evidence.
    TrustNotOpened {
        /// What the staging is still owed.
        missing: MissingTrustEvidence,
    },
}

/// The admitted interpreted result of the one active selection an opened trust boundary retains.
pub struct InterpretedMutationEvidence<
    'surface,
    'suite,
    'projection,
    'parity,
    'pair,
    'input,
    Input,
    Meaning,
> {
    trust: InterpretedTrust<'surface, 'suite, 'projection, 'parity, 'pair, 'input, Input, Meaning>,
    meaning: Meaning,
    report: TrialReport,
    mutation: MutationReport,
}

/// Why one interpreted execution produced no admitted mutation evidence.
#[must_use = "a refusal is the reason interpreted mutation evidence was not built"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterpretedExecutionRefusal {
    /// The supplied invocation does not reproduce the compiled projection's execution key.
    InvocationForAnotherExecution,
    /// The active selection does not belong to the opened surface.
    Selection(SelectionRefusal),
    /// The witness trial belongs to a claim other than the selected point's owner.
    WitnessForAnotherClaim {
        /// The claim that owns the selected point.
        expected: ClaimRef,
        /// The claim carried by the offered trial binding.
        found: ClaimRef,
    },
    /// The evaluation callable omitted the exact surface-issued branch.
    EvaluationCall(EvaluationCallRefusal),
    /// The evaluation callback reported zero firings for the selected damage.
    ///
    /// Boxed because the dud retains the exact surface-issued selection and trial, and the refusing side of a `Result` should not widen its passing side.
    DudPlant(Box<DudPlant>),
    /// The observation could not join its exact trial binding.
    Report(ReportRecordingRefusal),
}
// ---------------------------------------------------------------------------
// The names this home spells.
// ---------------------------------------------------------------------------

/// The owner every name this home spells is declared under.
pub const MUTERPRATER_NAMESPACE: &str = "muterprater";

/// The spelling of the declaration both no-mutation parity roads are projected from.
pub const PARITY_DECLARATION_SUBSTRATE: &str = "one-declaration";

/// The spelling of the rendering engine both no-mutation parity roads stand on.
pub const PARITY_RENDERING_SUBSTRATE: &str = "rendering-engine";

/// The spelling the no-mutation parity's road pairing is declared under.
pub const NO_MUTATION_PAIRING: &str = "no-mutation-parity";
