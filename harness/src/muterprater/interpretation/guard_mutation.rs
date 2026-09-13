//! Construction and projections of scoped mutation qualification and judgment.

use super::super::{
    MutationAssessment, MutationWitness, MutationWitnessQualificationRefusal,
    MutationWitnessReading, QualifiedMutation, RejectedMutationWitness,
};
use crate::muterprater::{CompiledMutationObservation, MutationReport};
use crate::properties::{Agreement, SharedSubstrate};
use crate::report::TrialReport;
use core::num::NonZeroU32;

impl<'scope, Input, Meaning> QualifiedMutation<'scope, Input, Meaning> {
    pub(in crate::muterprater) fn qualified(
        compiled: &'scope CompiledMutationObservation<'scope, Input, Meaning>,
        meanings: [&'scope Meaning; 4],
        firings: NonZeroU32,
        substrate: SharedSubstrate,
        difference: Agreement,
    ) -> Self {
        let [baseline, selected, compiled_baseline, compiled_selected] = meanings;
        Self {
            compiled,
            baseline,
            selected,
            compiled_baseline,
            compiled_selected,
            firings,
            substrate,
            difference,
        }
    }

    /// The complete compiled and interpreted observations.
    #[must_use]
    pub const fn compiled(&self) -> &'scope CompiledMutationObservation<'scope, Input, Meaning> {
        self.compiled
    }

    /// The unchanged evaluation meaning.
    #[must_use]
    pub const fn baseline(&self) -> &'scope Meaning {
        self.baseline
    }

    /// The selected evaluation meaning.
    #[must_use]
    pub const fn selected(&self) -> &'scope Meaning {
        self.selected
    }

    /// The unchanged compiled meaning.
    #[must_use]
    pub const fn compiled_baseline(&self) -> &'scope Meaning {
        self.compiled_baseline
    }

    /// The selected compiled meaning.
    #[must_use]
    pub const fn compiled_selected(&self) -> &'scope Meaning {
        self.compiled_selected
    }

    /// The reported positive selection firing count.
    #[must_use]
    pub const fn firings(&self) -> NonZeroU32 {
        self.firings
    }

    /// The shared foundations explicitly declared for the road comparisons.
    #[must_use]
    pub const fn substrate(&self) -> &SharedSubstrate {
        &self.substrate
    }

    /// The owner relation's comparison of unchanged and selected behavior on this input.
    #[must_use]
    pub const fn difference(&self) -> Agreement {
        self.difference
    }
}

impl<'scope, Input, Meaning> MutationWitnessReading<'scope, Input, Meaning> {
    pub(in crate::muterprater) fn observed(
        qualification: &'scope QualifiedMutation<'scope, Input, Meaning>,
        witness: MutationWitness<Meaning>,
        reports: [TrialReport; 5],
    ) -> Self {
        let [
            production,
            baseline,
            compiled_baseline,
            compiled_selected,
            selected,
        ] = reports;
        Self {
            qualification,
            witness,
            production,
            baseline,
            compiled_baseline,
            compiled_selected,
            selected,
        }
    }

    /// The independently qualified execution being judged.
    #[must_use]
    pub const fn qualification(&self) -> &'scope QualifiedMutation<'scope, Input, Meaning> {
        self.qualification
    }

    /// The exact witness whose callable supplied these judgments.
    #[must_use]
    pub const fn witness(&self) -> &MutationWitness<Meaning> {
        &self.witness
    }

    /// The witness's ordinary production report.
    #[must_use]
    pub const fn production_report(&self) -> &TrialReport {
        &self.production
    }

    /// The witness's unchanged evaluation report.
    #[must_use]
    pub const fn baseline_report(&self) -> &TrialReport {
        &self.baseline
    }

    /// The witness's unchanged compiled report.
    #[must_use]
    pub const fn compiled_baseline_report(&self) -> &TrialReport {
        &self.compiled_baseline
    }

    /// The witness's selected compiled report.
    #[must_use]
    pub const fn compiled_selected_report(&self) -> &TrialReport {
        &self.compiled_selected
    }

    /// The witness's selected evaluation report.
    #[must_use]
    pub const fn selected_report(&self) -> &TrialReport {
        &self.selected
    }
}

impl<'scope, Input, Meaning> RejectedMutationWitness<'scope, Input, Meaning> {
    pub(in crate::muterprater) fn rejected(
        reading: MutationWitnessReading<'scope, Input, Meaning>,
        cause: MutationWitnessQualificationRefusal,
    ) -> Self {
        Self {
            reading: Box::new(reading),
            cause,
        }
    }

    /// The complete judgments that did not qualify.
    #[must_use]
    pub const fn reading(&self) -> &MutationWitnessReading<'scope, Input, Meaning> {
        &self.reading
    }

    /// The first qualification obligation that failed.
    pub const fn cause(&self) -> MutationWitnessQualificationRefusal {
        self.cause
    }
}

impl<'scope, Input, Meaning> MutationAssessment<'scope, Input, Meaning> {
    pub(in crate::muterprater) fn qualified(
        reading: MutationWitnessReading<'scope, Input, Meaning>,
        mutation: MutationReport,
    ) -> Self {
        Self { reading, mutation }
    }

    /// The complete execution and witness evidence this assessment retains.
    #[must_use]
    pub const fn reading(&self) -> &MutationWitnessReading<'scope, Input, Meaning> {
        &self.reading
    }

    /// The report derived from the qualified evidence.
    #[must_use]
    pub const fn mutation(&self) -> &MutationReport {
        &self.mutation
    }
}
