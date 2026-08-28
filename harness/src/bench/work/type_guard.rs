//! The work nucleus: scoped recording, owner judgment, and executable attachment construction.

use super::{
    BenchAttachment, BenchAttachmentRefusal, BenchCall, SecondaryObservation,
    SecondaryObservationRefusal, WorkConclusion, WorkCount, WorkCurve, WorkCurvePoint,
    WorkGapStanding, WorkJudge, WorkJudgeBinding, WorkJudgment, WorkJudgmentInput,
    WorkObservationRef, WorkQualificationRefusal, WorkRecorder, WorkRecordingRefusal,
};
use crate::bench::declaration::{
    ComplexityClaimRef, DeclaredBudgets, PlantedWorseRef, WorkFormula, WorkloadRef,
};
use crate::clock::MeasurementReading;
use std::collections::BTreeMap;

impl WorkJudgmentInput<'_> {
    pub(in crate::bench) const fn over<'reading>(
        formula: Option<&'reading WorkFormula>,
        complexity: ComplexityClaimRef,
        budgets: DeclaredBudgets,
        measured: &'reading WorkCurve,
        planted_worse: &'reading WorkCurve,
    ) -> WorkJudgmentInput<'reading> {
        WorkJudgmentInput {
            formula,
            complexity,
            budgets,
            measured,
            planted_worse,
        }
    }

    /// The row's formula bytes, where it declared any.
    #[must_use]
    pub const fn formula(&self) -> Option<&WorkFormula> {
        self.formula
    }

    /// The row's complexity claim.
    #[must_use]
    pub const fn complexity(&self) -> ComplexityClaimRef {
        self.complexity
    }

    /// The row's budgets, including the exact gap ratio.
    #[must_use]
    pub const fn budgets(&self) -> DeclaredBudgets {
        self.budgets
    }

    /// The measured curve.
    #[must_use]
    pub const fn measured(&self) -> &WorkCurve {
        self.measured
    }

    /// The control's curve.
    #[must_use]
    pub const fn planted_worse(&self) -> &WorkCurve {
        self.planted_worse
    }
}

impl WorkJudgeBinding {
    /// Bind one judge to the complexity claim it reads.
    #[must_use]
    pub const fn bound(complexity: ComplexityClaimRef, judge: WorkJudge) -> Self {
        Self { complexity, judge }
    }

    /// The complexity claim this judge reads.
    #[must_use]
    pub const fn complexity(self) -> ComplexityClaimRef {
        self.complexity
    }

    /// The judge itself.
    #[must_use]
    pub const fn judge(self) -> WorkJudge {
        self.judge
    }
}

impl WorkJudgment {
    /// State all three readings at once, which is the only way to state any of them.
    pub const fn stated(
        measured: WorkConclusion,
        planted_worse: WorkConclusion,
        gap: WorkGapStanding,
    ) -> Self {
        Self {
            measured,
            planted_worse,
            gap,
        }
    }

    /// What the judge concluded about the measured curve.
    pub const fn measured(self) -> WorkConclusion {
        self.measured
    }

    /// What the judge concluded about the control.
    pub const fn planted_worse(self) -> WorkConclusion {
        self.planted_worse
    }

    /// How the declared gap read.
    #[must_use]
    pub const fn gap(self) -> WorkGapStanding {
        self.gap
    }

    /// Whether these three readings qualify the row for timing.
    ///
    /// # Errors
    ///
    /// Refuses an inactive control before it looks at the measured curve at all.
    pub const fn qualification(self) -> Result<(), WorkQualificationRefusal> {
        if !matches!(self.planted_worse, WorkConclusion::Refused(_))
            || !matches!(self.gap, WorkGapStanding::Distinguished)
        {
            return Err(WorkQualificationRefusal::PlantedWorseNotDistinguished {
                planted_worse: self.planted_worse,
                gap: self.gap,
            });
        }
        if !matches!(self.measured, WorkConclusion::Satisfied) {
            return Err(WorkQualificationRefusal::MeasuredRefused(self.measured));
        }
        Ok(())
    }

    /// The same reading as [`qualification`](Self::qualification), without the reason.
    #[must_use]
    pub const fn qualifies(self) -> bool {
        self.qualification().is_ok()
    }
}

impl WorkCount {
    /// The observation being counted.
    #[must_use]
    pub const fn observation(self) -> WorkObservationRef {
        self.observation
    }

    /// The exact count.
    #[must_use]
    pub const fn count(self) -> u64 {
        self.count
    }
}

impl WorkCurvePoint {
    /// The input size this point was recorded at.
    #[must_use]
    pub const fn input_size(&self) -> u64 {
        self.input_size
    }

    /// The counts, in the order the binding declared its observations.
    #[must_use]
    pub fn counts(&self) -> &[WorkCount] {
        &self.counts
    }
}

impl WorkCurve {
    pub(in crate::bench) fn recorded(points: Vec<WorkCurvePoint>) -> Self {
        Self { points }
    }

    /// The points, in authored axis order.
    #[must_use]
    pub fn points(&self) -> &[WorkCurvePoint] {
        &self.points
    }
}

impl SecondaryObservation {
    pub(in crate::bench) fn recorded(
        work: WorkCurve,
        judgment: WorkJudgment,
        measurements: Vec<MeasurementReading>,
    ) -> Result<Self, SecondaryObservationRefusal> {
        judgment
            .qualification()
            .map_err(SecondaryObservationRefusal::Judgment)?;
        Ok(Self {
            work,
            judgment,
            measurements,
        })
    }

    /// The curve the timed pass recorded for itself.
    #[must_use]
    pub const fn work(&self) -> &WorkCurve {
        &self.work
    }

    /// The same judge's reading of that curve.
    pub const fn judgment(&self) -> WorkJudgment {
        self.judgment
    }

    /// The clock readings, in axis order and then sample order.
    pub fn measurements(&self) -> &[MeasurementReading] {
        &self.measurements
    }
}

impl WorkRecorder {
    pub(in crate::bench) fn scoped(observations: &[WorkObservationRef]) -> Self {
        Self {
            counts: observations
                .iter()
                .copied()
                .map(|observation| WorkCount {
                    observation,
                    count: 0u64,
                })
                .collect(),
        }
    }

    /// Add units to one observation this recorder was scoped to.
    ///
    /// # Errors
    ///
    /// Refuses an observation outside the scoped roster, then an addition that would overflow.
    pub fn record(
        &mut self,
        observation: WorkObservationRef,
        units: u64,
    ) -> Result<(), WorkRecordingRefusal> {
        let Some(count) = self
            .counts
            .iter_mut()
            .find(|count| count.observation == observation)
        else {
            return Err(WorkRecordingRefusal::UnknownObservation(observation));
        };
        let Some(next) = count.count.checked_add(units) else {
            return Err(WorkRecordingRefusal::CountOverflow {
                observation,
                current: count.count,
                addition: units,
            });
        };
        count.count = next;
        Ok(())
    }

    pub(in crate::bench) fn finish(self, input_size: u64) -> WorkCurvePoint {
        WorkCurvePoint {
            input_size,
            counts: self.counts,
        }
    }
}

impl BenchAttachment {
    /// Bind both callables, the judge, and the complete observation roster.
    ///
    /// # Errors
    ///
    /// Refuses an empty roster, then the first repeat in authored order.
    pub fn attached(
        workload: WorkloadRef,
        measured: BenchCall,
        planted_worse_ref: PlantedWorseRef,
        planted_worse: BenchCall,
        judge: WorkJudgeBinding,
        observations: Vec<WorkObservationRef>,
    ) -> Result<Self, BenchAttachmentRefusal> {
        if observations.is_empty() {
            return Err(BenchAttachmentRefusal::NoObservation);
        }
        let mut seen = BTreeMap::new();
        for (duplicate, observation) in observations.iter().copied().enumerate() {
            if let Some(first) = seen.insert(observation, duplicate) {
                return Err(BenchAttachmentRefusal::DuplicateObservation {
                    observation,
                    first,
                    duplicate,
                });
            }
        }
        Ok(Self {
            workload,
            measured,
            planted_worse_ref,
            planted_worse,
            judge,
            observations,
        })
    }

    /// The workload these callables claim to be.
    #[must_use]
    pub const fn workload(&self) -> WorkloadRef {
        self.workload
    }

    /// The measured callable.
    #[must_use]
    pub const fn measured(&self) -> BenchCall {
        self.measured
    }

    /// The control's name.
    #[must_use]
    pub const fn planted_worse_ref(&self) -> PlantedWorseRef {
        self.planted_worse_ref
    }

    /// The control's callable.
    #[must_use]
    pub const fn planted_worse(&self) -> BenchCall {
        self.planted_worse
    }

    /// The bound judge.
    #[must_use]
    pub const fn judge(&self) -> WorkJudgeBinding {
        self.judge
    }

    /// The observation roster, in authored order.
    #[must_use]
    pub fn observations(&self) -> &[WorkObservationRef] {
        &self.observations
    }
}
