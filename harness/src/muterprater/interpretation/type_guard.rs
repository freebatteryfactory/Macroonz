//! The invariant nucleus of evaluation pairs, parity, and interpreted evidence.

use super::{
    EvaluationBinding, EvaluationCall, EvaluationObservation, EvaluationPair,
    EvaluationPairRefusal, EvaluationPairStanding, EvaluationPairStandingMismatch,
    InterpretedMutationEvidence, InterpretedTrust, MeaningCheck, MutationWitness,
    MutationWitnessRefusal, NoMutationParityQualification, NoMutationParityReading,
    NoMutationParityStanding, NoMutationReports, NoMutationResults, ParityQualificationRefusal,
    ProductionBinding, ProductionCall, RejectedNoMutationParity,
};
use crate::descriptor::{CheckRef, RevisionBinding};
use crate::muterprater::{
    ActiveSelection, CompiledProjectionPressure, CompiledSuitePressure, EvaluationCallRefusal,
    EvaluationDirective, EvaluationFamilyRef, EvaluationSurface, EvaluationSurfaceId,
    MutationReport,
};
use crate::properties::{Equivalence, SharedSubstrate};
use crate::report::{TrialConclusion, TrialReport};
use crate::runner::TrialBinding;

impl<Meaning> EvaluationObservation<Meaning> {
    /// Raw output from one evaluation call.
    #[must_use]
    pub const fn observed(meaning: Meaning, firings: u32) -> Self {
        Self { meaning, firings }
    }

    /// The evaluation meaning.
    #[must_use]
    pub const fn meaning(&self) -> &Meaning {
        &self.meaning
    }

    /// How many activation firings the evaluation callable reports.
    #[must_use]
    pub const fn firings(&self) -> u32 {
        self.firings
    }

    /// Split the raw output for receiver validation.
    pub(in crate::muterprater) fn into_parts(self) -> (Meaning, u32) {
        (self.meaning, self.firings)
    }
}

impl<Input, Meaning> ProductionBinding<Input, Meaning> {
    /// The production callable and revision declared for one evaluation family.
    #[must_use]
    pub const fn declared(
        family: EvaluationFamilyRef,
        revision: RevisionBinding,
        call: ProductionCall<Input, Meaning>,
    ) -> Self {
        Self {
            family,
            revision,
            call,
        }
    }

    /// The declared evaluation family.
    #[must_use]
    pub const fn family(&self) -> EvaluationFamilyRef {
        self.family
    }

    /// The production revision binding.
    #[must_use]
    pub const fn revision(&self) -> RevisionBinding {
        self.revision
    }

    /// Run the production callable.
    #[must_use]
    pub fn evaluate(&self, input: &Input) -> Meaning {
        (self.call)(input)
    }
}

impl<Input, Meaning> EvaluationBinding<Input, Meaning> {
    /// Bind the evaluation callable and revision to one exact surface.
    ///
    /// The family and surface identity are derived from `surface`, so a caller keeps no parallel labels coherent.
    #[must_use]
    pub const fn declared(
        surface: &EvaluationSurface,
        revision: RevisionBinding,
        call: EvaluationCall<Input, Meaning>,
    ) -> Self {
        Self {
            family: surface.family(),
            revision,
            surface: surface.identity(),
            call,
        }
    }

    /// The declared evaluation family.
    #[must_use]
    pub const fn family(&self) -> EvaluationFamilyRef {
        self.family
    }

    /// The evaluation revision binding.
    #[must_use]
    pub const fn revision(&self) -> RevisionBinding {
        self.revision
    }

    /// The exact evaluation surface this callable executes.
    #[must_use]
    pub const fn surface(&self) -> EvaluationSurfaceId {
        self.surface
    }

    /// Run the evaluation callable under one surface-bound directive.
    ///
    /// # Errors
    ///
    /// Returns the callable's typed refusal when it does not implement the offered directive.
    pub fn evaluate(
        &self,
        input: &Input,
        directive: EvaluationDirective<'_>,
    ) -> Result<EvaluationObservation<Meaning>, EvaluationCallRefusal> {
        (self.call)(input, directive)
    }
}

impl<Input, Meaning> EvaluationPair<Input, Meaning> {
    /// Join production and evaluation bindings under one declared family and equivalence.
    ///
    /// # Errors
    ///
    /// Refuses bindings naming different evaluation families.
    pub fn paired(
        production: ProductionBinding<Input, Meaning>,
        evaluation: EvaluationBinding<Input, Meaning>,
        same: Equivalence<Meaning>,
    ) -> Result<Self, EvaluationPairRefusal> {
        if production.family() != evaluation.family() {
            return Err(EvaluationPairRefusal::FamilyMismatch {
                production: production.family(),
                evaluation: evaluation.family(),
            });
        }
        Ok(Self {
            production,
            evaluation,
            same,
        })
    }

    /// The production binding.
    #[must_use]
    pub const fn production(&self) -> &ProductionBinding<Input, Meaning> {
        &self.production
    }

    /// The evaluation binding.
    #[must_use]
    pub const fn evaluation(&self) -> &EvaluationBinding<Input, Meaning> {
        &self.evaluation
    }

    /// The owner-declared equivalence over the two meanings.
    #[must_use]
    pub const fn equivalence(&self) -> Equivalence<Meaning> {
        self.same
    }

    /// The identity and revision facts this pair retains in evidence.
    #[must_use]
    pub const fn standing(&self) -> EvaluationPairStanding {
        EvaluationPairStanding {
            family: self.production.family(),
            production_revision: self.production.revision(),
            evaluation_revision: self.evaluation.revision(),
            surface: self.evaluation.surface(),
        }
    }
}

impl EvaluationPairStanding {
    /// Project one exact standing disagreement, without weakening whole-standing admission.
    pub(in crate::muterprater) fn mismatch(
        self,
        found: Self,
    ) -> Option<EvaluationPairStandingMismatch> {
        if self == found {
            return None;
        }
        if self.family != found.family {
            return Some(EvaluationPairStandingMismatch::Family {
                expected: self.family,
                found: found.family,
            });
        }
        if self.production_revision != found.production_revision {
            return Some(EvaluationPairStandingMismatch::ProductionRevision {
                expected: self.production_revision,
                found: found.production_revision,
            });
        }
        if self.evaluation_revision != found.evaluation_revision {
            return Some(EvaluationPairStandingMismatch::EvaluationRevision {
                expected: self.evaluation_revision,
                found: found.evaluation_revision,
            });
        }
        if self.surface != found.surface {
            return Some(EvaluationPairStandingMismatch::Surface {
                expected: self.surface,
                found: found.surface,
            });
        }
        Some(EvaluationPairStandingMismatch::StandingChanged)
    }

    /// The evaluation family shared by both bindings.
    #[must_use]
    pub const fn family(self) -> EvaluationFamilyRef {
        self.family
    }

    /// The production revision.
    #[must_use]
    pub const fn production_revision(self) -> RevisionBinding {
        self.production_revision
    }

    /// The evaluation revision.
    #[must_use]
    pub const fn evaluation_revision(self) -> RevisionBinding {
        self.evaluation_revision
    }

    /// The exact evaluation surface.
    #[must_use]
    pub const fn surface(self) -> EvaluationSurfaceId {
        self.surface
    }
}

impl<Meaning> MutationWitness<Meaning> {
    /// Join one trial binding to the identity and callable of the check its executions report through.
    ///
    /// # Errors
    ///
    /// Refuses a check identity other than the one the trial row retains.
    /// The function-pointer shape excludes captured state and cannot establish that the callable's behavior matches its declared identity; the execution lane observes that.
    pub fn bound(
        binding: TrialBinding,
        check_ref: CheckRef,
        check: MeaningCheck<Meaning>,
    ) -> Result<Self, MutationWitnessRefusal> {
        let expected = binding.row().check();
        if check_ref != expected {
            return Err(MutationWitnessRefusal::CheckMismatch {
                expected,
                found: check_ref,
            });
        }
        Ok(Self { binding, check })
    }

    /// The exact trial binding the receiver reports through.
    #[must_use]
    pub const fn binding(&self) -> &TrialBinding {
        &self.binding
    }

    /// The check identity bound to the callable.
    #[must_use]
    pub const fn check_ref(&self) -> CheckRef {
        self.binding.row().check()
    }

    /// Judge one produced meaning under the declared check callable.
    #[must_use]
    pub fn conclude(&self, meaning: &Meaning) -> TrialConclusion {
        (self.check)(meaning)
    }
}

// ---------------------------------------------------------------------------
// The no-mutation parity.
// ---------------------------------------------------------------------------

impl<Meaning> NoMutationResults<Meaning> {
    /// The production meaning, the no-mutation evaluation meaning, and the evaluation firing count.
    pub(in crate::muterprater) const fn observed(
        production: Meaning,
        evaluation: Meaning,
        evaluation_firings: u32,
    ) -> Self {
        Self {
            production,
            evaluation,
            evaluation_firings,
        }
    }

    /// The production meaning.
    #[must_use]
    pub const fn production(&self) -> &Meaning {
        &self.production
    }

    /// The evaluation meaning under no mutation.
    #[must_use]
    pub const fn evaluation(&self) -> &Meaning {
        &self.evaluation
    }

    /// How many activation firings the no-mutation call reported.
    #[must_use]
    pub const fn evaluation_firings(&self) -> u32 {
        self.evaluation_firings
    }
}

impl NoMutationReports {
    /// Retain the production and evaluation reports in their semantic roles.
    pub(in crate::muterprater) fn recorded(
        production: TrialReport,
        evaluation: TrialReport,
    ) -> Self {
        Self {
            production,
            evaluation,
        }
    }

    /// The production report.
    const fn production(&self) -> &TrialReport {
        &self.production
    }

    /// The evaluation report.
    const fn evaluation(&self) -> &TrialReport {
        &self.evaluation
    }
}

impl<'pair, 'input, Input, Meaning> NoMutationParityReading<'pair, 'input, Input, Meaning> {
    /// Record one complete no-mutation comparison, after both observations joined the same trial binding.
    pub(in crate::muterprater) fn recorded(
        pair: &'pair EvaluationPair<Input, Meaning>,
        witness: MutationWitness<Meaning>,
        input: &'input Input,
        results: NoMutationResults<Meaning>,
        substrate: SharedSubstrate,
        conclusion: TrialConclusion,
        reports: NoMutationReports,
    ) -> Self {
        Self {
            pair,
            witness,
            input,
            results,
            substrate,
            conclusion,
            reports,
        }
    }

    /// The exact pair that ran.
    #[must_use]
    pub const fn pair(&self) -> &'pair EvaluationPair<Input, Meaning> {
        self.pair
    }

    /// The exact trial binding, check identity, and check callable both roads used.
    #[must_use]
    pub const fn witness(&self) -> &MutationWitness<Meaning> {
        &self.witness
    }

    /// The exact input both roads received.
    #[must_use]
    pub const fn input(&self) -> &'input Input {
        self.input
    }

    /// The production meaning.
    #[must_use]
    pub const fn production(&self) -> &Meaning {
        self.results.production()
    }

    /// The evaluation meaning under no mutation.
    #[must_use]
    pub const fn evaluation(&self) -> &Meaning {
        self.results.evaluation()
    }

    /// How many activation firings the no-mutation call reported.
    #[must_use]
    pub const fn evaluation_firings(&self) -> u32 {
        self.results.evaluation_firings()
    }

    /// The foundations both roads share.
    #[must_use]
    pub const fn substrate(&self) -> &SharedSubstrate {
        &self.substrate
    }

    /// The owner-declared equivalence's conclusion.
    #[must_use]
    pub const fn conclusion(&self) -> &TrialConclusion {
        &self.conclusion
    }

    /// The production execution report.
    #[must_use]
    pub const fn production_report(&self) -> &TrialReport {
        self.reports.production()
    }

    /// The no-mutation evaluation execution report.
    #[must_use]
    pub const fn evaluation_report(&self) -> &TrialReport {
        self.reports.evaluation()
    }
}

impl<'pair, 'input, Input, Meaning> NoMutationParityQualification<'pair, 'input, Input, Meaning> {
    /// A no-mutation reading that both reports, zero activation, and semantic agreement qualified.
    pub(in crate::muterprater) fn qualified(
        reading: NoMutationParityReading<'pair, 'input, Input, Meaning>,
    ) -> Self {
        Self { reading }
    }

    /// The complete reading this qualification stands on.
    #[must_use]
    pub const fn reading(&self) -> &NoMutationParityReading<'pair, 'input, Input, Meaning> {
        &self.reading
    }
}

impl<'pair, 'input, Input, Meaning> RejectedNoMutationParity<'pair, 'input, Input, Meaning> {
    /// A complete no-mutation reading that did not qualify.
    pub(in crate::muterprater) fn rejected(
        cause: ParityQualificationRefusal,
        reading: NoMutationParityReading<'pair, 'input, Input, Meaning>,
    ) -> Self {
        Self { cause, reading }
    }

    /// Why the reading did not qualify.
    pub const fn cause(&self) -> ParityQualificationRefusal {
        self.cause
    }

    /// The complete reading that did not qualify.
    #[must_use]
    pub const fn reading(&self) -> &NoMutationParityReading<'pair, 'input, Input, Meaning> {
        &self.reading
    }
}

impl<'pair, 'input, Input, Meaning> NoMutationParityStanding<'pair, 'input, Input, Meaning> {
    /// The qualification, where this reading earned one.
    #[must_use]
    pub const fn qualification(
        &self,
    ) -> Option<&NoMutationParityQualification<'pair, 'input, Input, Meaning>> {
        match self {
            Self::Qualified(qualification) => Some(qualification),
            Self::Rejected(_) => None,
        }
    }

    /// The rejected reading, where qualification was refused.
    #[must_use]
    pub const fn rejection(
        &self,
    ) -> Option<&RejectedNoMutationParity<'pair, 'input, Input, Meaning>> {
        match self {
            Self::Qualified(_) => None,
            Self::Rejected(rejection) => Some(rejection),
        }
    }
}
impl<'surface, 'suite, 'projection, 'parity, 'pair, 'input, Input, Meaning>
    InterpretedTrust<'surface, 'suite, 'projection, 'parity, 'pair, 'input, Input, Meaning>
{
    /// Open interpreted execution over one surface, generic suite bite, and exact selection pressure.
    pub(in crate::muterprater) fn opened(
        surface: &'surface EvaluationSurface,
        suite: &'suite CompiledSuitePressure,
        projection: &'projection CompiledProjectionPressure<'parity, 'pair, 'input, Input, Meaning>,
    ) -> Self {
        Self {
            surface,
            suite,
            projection,
        }
    }

    /// The exact evaluation surface interpreted selection runs over.
    #[must_use]
    pub const fn surface(&self) -> &'surface EvaluationSurface {
        self.surface
    }

    /// The generic compiled suite bite, retained without evaluation-pair authority.
    #[must_use]
    pub const fn suite(&self) -> &'suite CompiledSuitePressure {
        self.suite
    }

    /// The exact compiled selected-projection pressure for this selection.
    #[must_use]
    pub const fn projection(
        &self,
    ) -> &'projection CompiledProjectionPressure<'parity, 'pair, 'input, Input, Meaning> {
        self.projection
    }

    /// The no-mutation qualification the exact projection pressure retains.
    #[must_use]
    pub const fn parity(
        &self,
    ) -> &'parity NoMutationParityQualification<'pair, 'input, Input, Meaning> {
        self.projection.parity()
    }

    /// The only surface-issued selection this trust authorizes.
    #[must_use]
    pub const fn selection(&self) -> ActiveSelection {
        self.projection.standing().selection()
    }

    /// Duplicate this borrowed trust statement for one admitted evidence record.
    pub(in crate::muterprater) fn duplicate(&self) -> Self {
        Self {
            surface: self.surface,
            suite: self.suite,
            projection: self.projection,
        }
    }
}

impl<'surface, 'suite, 'projection, 'parity, 'pair, 'input, Input, Meaning>
    InterpretedMutationEvidence<
        'surface,
        'suite,
        'projection,
        'parity,
        'pair,
        'input,
        Input,
        Meaning,
    >
{
    /// One active execution, admitted under the trust boundary that made it evidence.
    pub(in crate::muterprater) fn admitted(
        trust: InterpretedTrust<
            'surface,
            'suite,
            'projection,
            'parity,
            'pair,
            'input,
            Input,
            Meaning,
        >,
        meaning: Meaning,
        report: TrialReport,
        mutation: MutationReport,
    ) -> Self {
        Self {
            trust,
            meaning,
            report,
            mutation,
        }
    }

    /// The trust evidence this interpreted result was admitted under.
    #[must_use]
    pub const fn trust(
        &self,
    ) -> &InterpretedTrust<'surface, 'suite, 'projection, 'parity, 'pair, 'input, Input, Meaning>
    {
        &self.trust
    }

    /// The exact active selection that ran.
    #[must_use]
    pub const fn selection(&self) -> ActiveSelection {
        self.trust.selection()
    }

    /// The meaning the active evaluation callable returned.
    #[must_use]
    pub const fn meaning(&self) -> &Meaning {
        &self.meaning
    }

    /// The trial report admitted through the report spine.
    #[must_use]
    pub const fn report(&self) -> &TrialReport {
        &self.report
    }

    /// The mutation report derived from the active execution.
    #[must_use]
    pub const fn mutation(&self) -> &MutationReport {
        &self.mutation
    }
}
