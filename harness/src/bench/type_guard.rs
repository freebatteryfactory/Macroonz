//! The nucleus: every constructor that can refuse, and every reader over a private field.

use super::{
    BenchAttachment, BenchAttachmentRefusal, BenchBinding, BenchBindingRefusal, BenchCall,
    BenchInvocation, BenchMeasurement, BenchOutcome, BenchReading, BenchReferences, BenchReport,
    BenchRow, BenchRowKey, BenchRowRefusal, BenchStage, BenchTable, BenchTableName,
    BenchTableRefusal, BenchVerdictRefusal, ComplexityClaimRef, ContentionPosture, DeclaredBudgets,
    DeclaredBudgetsRefusal, ExactRatio, InputSizeAxis, InputSizeAxisRefusal, PlantedWorseRef,
    PreflightRef, PreflightTrial, SecondaryObservation, SecondaryObservationRefusal,
    WorkConclusion, WorkCount, WorkCurve, WorkCurvePoint, WorkFormula, WorkFormulaRefusal,
    WorkGapStanding, WorkJudge, WorkJudgeBinding, WorkJudgment, WorkJudgmentInput,
    WorkObservationRef, WorkQualificationRefusal, WorkRecorder, WorkRecordingRefusal, WorkloadRef,
};
use crate::bench::encode::derive_row_key;
use crate::clock::{HarnessClock, MeasurementReading};
use crate::descriptor::{NameRefusal, NamespacedName, Provenance};
use crate::identity::ContentAddress;
use crate::report::{TargetBinding, TrialReport};
use crate::runner::{Invocation, TrialBinding};
use std::collections::BTreeMap;
use std::num::NonZeroU32;

/// The two roads and the one reader every namespaced reference here shares, written once.
///
/// Each reference is a type of its own so a workload cannot be handed in where a complexity claim was meant.
/// All they share is how a name is parsed, and that law belongs in one place.
macro_rules! namespaced_reference {
    ($($reference:ident),+ $(,)?) => {
        $(
            impl $reference {
                /// This reference, from the namespace that declares it and the spelling it carries.
                ///
                /// # Errors
                ///
                /// Refuses an empty namespace, then an empty stem.
                pub const fn named(
                    namespace: &'static str,
                    stem: &'static str,
                ) -> Result<Self, NameRefusal> {
                    match NamespacedName::named(namespace, stem) {
                        Ok(name) => Ok(Self(name)),
                        Err(refusal) => Err(refusal),
                    }
                }

                /// This reference, over a name already parsed.
                #[must_use]
                pub const fn over(name: NamespacedName) -> Self {
                    Self(name)
                }

                /// The name this reference carries.
                #[must_use]
                pub const fn name(self) -> NamespacedName {
                    self.0
                }
            }
        )+
    };
}

namespaced_reference!(
    WorkloadRef,
    PreflightRef,
    PlantedWorseRef,
    ComplexityClaimRef,
    WorkObservationRef,
    BenchTableName,
);

impl InputSizeAxis {
    /// At least two distinct sizes, keeping the order they were authored in.
    ///
    /// # Errors
    ///
    /// Refuses an axis shorter than two, then the first repeated size.
    pub fn declared(sizes: Vec<u64>) -> Result<Self, InputSizeAxisRefusal> {
        if sizes.len() < 2usize {
            return Err(InputSizeAxisRefusal::TooShort { found: sizes.len() });
        }
        let mut seen = BTreeMap::new();
        for (duplicate, size) in sizes.iter().copied().enumerate() {
            if let Some(first) = seen.insert(size, duplicate) {
                return Err(InputSizeAxisRefusal::DuplicateSize {
                    size,
                    first,
                    duplicate,
                });
            }
        }
        Ok(Self(sizes))
    }

    /// The sizes, in authored order.
    #[must_use]
    pub fn sizes(&self) -> &[u64] {
        &self.0
    }
}

impl ExactRatio {
    /// The numerator.
    #[must_use]
    pub const fn numerator(self) -> u64 {
        self.numerator
    }

    /// The denominator.
    #[must_use]
    pub const fn denominator(self) -> u64 {
        self.denominator
    }
}

impl DeclaredBudgets {
    /// One positive sample count, any warmup count, and one positive gap ratio.
    ///
    /// # Errors
    ///
    /// Refuses zero samples, then a zero numerator, then a zero denominator.
    pub fn declared(
        samples: u32,
        warmups: u32,
        ratio_numerator: u64,
        ratio_denominator: u64,
    ) -> Result<Self, DeclaredBudgetsRefusal> {
        let Some(samples) = NonZeroU32::new(samples) else {
            return Err(DeclaredBudgetsRefusal::NoSamples);
        };
        if ratio_numerator == 0u64 {
            return Err(DeclaredBudgetsRefusal::ZeroRatioNumerator);
        }
        if ratio_denominator == 0u64 {
            return Err(DeclaredBudgetsRefusal::ZeroRatioDenominator);
        }
        Ok(Self {
            samples,
            warmups,
            ratio: ExactRatio {
                numerator: ratio_numerator,
                denominator: ratio_denominator,
            },
        })
    }

    /// How many samples each pass takes.
    #[must_use]
    pub const fn samples(self) -> u32 {
        self.samples.get()
    }

    /// How many calls the timed pass discards first.
    #[must_use]
    pub const fn warmups(self) -> u32 {
        self.warmups
    }

    /// The gap ratio handed to the judge.
    #[must_use]
    pub const fn ratio(self) -> ExactRatio {
        self.ratio
    }
}

impl WorkFormula {
    /// One present formula, in whatever bytes the owner spells it.
    ///
    /// # Errors
    ///
    /// Refuses empty bytes, since a row with no formula carries `None` instead.
    pub fn encoded(bytes: Vec<u8>) -> Result<Self, WorkFormulaRefusal> {
        if bytes.is_empty() {
            return Err(WorkFormulaRefusal::Empty);
        }
        Ok(Self { bytes })
    }

    /// The bytes, exactly as they were declared.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl BenchRowKey {
    pub(in crate::bench) const fn derived(address: ContentAddress) -> Self {
        Self(address)
    }

    /// The address this identity carries.
    #[must_use]
    pub const fn address(self) -> ContentAddress {
        self.0
    }
}

impl BenchReferences {
    /// The four names one row joins.
    #[must_use]
    pub const fn declared(
        workload: WorkloadRef,
        preflight: PreflightRef,
        planted_worse: PlantedWorseRef,
        complexity: ComplexityClaimRef,
    ) -> Self {
        Self {
            workload,
            preflight,
            planted_worse,
            complexity,
        }
    }

    /// The workload being measured.
    #[must_use]
    pub const fn workload(self) -> WorkloadRef {
        self.workload
    }

    /// The correctness preflight.
    #[must_use]
    pub const fn preflight(self) -> PreflightRef {
        self.preflight
    }

    /// The deliberately worse control.
    #[must_use]
    pub const fn planted_worse(self) -> PlantedWorseRef {
        self.planted_worse
    }

    /// The complexity claim.
    #[must_use]
    pub const fn complexity(self) -> ComplexityClaimRef {
        self.complexity
    }
}

impl BenchMeasurement {
    /// The four measurement facts one row declares.
    #[must_use]
    pub fn declared(
        input_sizes: InputSizeAxis,
        budgets: DeclaredBudgets,
        contention: ContentionPosture,
        formula: Option<WorkFormula>,
    ) -> Self {
        Self {
            input_sizes,
            budgets,
            contention,
            formula,
        }
    }

    /// The input-size axis.
    #[must_use]
    pub const fn input_sizes(&self) -> &InputSizeAxis {
        &self.input_sizes
    }

    /// The sample, warmup, and ratio budgets.
    #[must_use]
    pub const fn budgets(&self) -> DeclaredBudgets {
        self.budgets
    }

    /// The declared contention posture.
    #[must_use]
    pub const fn contention(&self) -> ContentionPosture {
        self.contention
    }

    /// The formula bytes, where the row declared any.
    #[must_use]
    pub const fn formula(&self) -> Option<&WorkFormula> {
        self.formula.as_ref()
    }
}

impl BenchRow {
    /// One row, with its identity derived from the whole declaration.
    ///
    /// # Errors
    ///
    /// Refuses only where the canonical encoder cannot hold a member's length in the width it declares.
    pub fn declared(
        references: BenchReferences,
        measurement: BenchMeasurement,
    ) -> Result<Self, BenchRowRefusal> {
        let key = derive_row_key(references, &measurement).map_err(BenchRowRefusal::Encoding)?;
        Ok(Self {
            references,
            measurement,
            key,
        })
    }

    /// The workload being measured.
    #[must_use]
    pub const fn workload(&self) -> WorkloadRef {
        self.references.workload
    }

    /// The input-size axis.
    #[must_use]
    pub const fn input_sizes(&self) -> &InputSizeAxis {
        &self.measurement.input_sizes
    }

    /// The correctness preflight.
    #[must_use]
    pub const fn preflight(&self) -> PreflightRef {
        self.references.preflight
    }

    /// The deliberately worse control.
    #[must_use]
    pub const fn planted_worse(&self) -> PlantedWorseRef {
        self.references.planted_worse
    }

    /// The sample, warmup, and ratio budgets.
    #[must_use]
    pub const fn budgets(&self) -> DeclaredBudgets {
        self.measurement.budgets
    }

    /// The declared contention posture.
    #[must_use]
    pub const fn contention(&self) -> ContentionPosture {
        self.measurement.contention
    }

    /// The formula bytes, where the row declared any.
    #[must_use]
    pub const fn formula(&self) -> Option<&WorkFormula> {
        self.measurement.formula.as_ref()
    }

    /// The complexity claim.
    #[must_use]
    pub const fn complexity(&self) -> ComplexityClaimRef {
        self.references.complexity
    }

    /// The identity derived from the whole declaration.
    #[must_use]
    pub const fn key(&self) -> BenchRowKey {
        self.key
    }
}

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

impl PreflightTrial {
    /// Bind this home's preflight name to a real trial and the invocation it runs under.
    #[must_use]
    pub fn bound(reference: PreflightRef, binding: TrialBinding, invocation: Invocation) -> Self {
        Self {
            reference,
            binding,
            invocation,
        }
    }

    /// The preflight name a row must agree with.
    #[must_use]
    pub const fn reference(&self) -> PreflightRef {
        self.reference
    }

    /// The trial that runs.
    #[must_use]
    pub const fn binding(&self) -> &TrialBinding {
        &self.binding
    }

    /// The invocation it runs under.
    #[must_use]
    pub const fn invocation(&self) -> &Invocation {
        &self.invocation
    }
}

impl BenchBinding {
    /// Join one row to its callables and its preflight.
    ///
    /// # Errors
    ///
    /// Refuses a workload, control, preflight, then complexity name that disagrees, in that order.
    pub fn bound(
        row: BenchRow,
        attachment: BenchAttachment,
        preflight: PreflightTrial,
    ) -> Result<Self, BenchBindingRefusal> {
        if row.workload() != attachment.workload() {
            return Err(BenchBindingRefusal::Workload {
                row: row.workload(),
                attachment: attachment.workload(),
            });
        }
        if row.planted_worse() != attachment.planted_worse_ref() {
            return Err(BenchBindingRefusal::PlantedWorse {
                row: row.planted_worse(),
                attachment: attachment.planted_worse_ref(),
            });
        }
        if row.preflight() != preflight.reference() {
            return Err(BenchBindingRefusal::Preflight {
                row: row.preflight(),
                trial: preflight.reference(),
            });
        }
        if row.complexity() != attachment.judge().complexity() {
            return Err(BenchBindingRefusal::Complexity {
                row: row.complexity(),
                judge: attachment.judge().complexity(),
            });
        }
        Ok(Self {
            row,
            attachment,
            preflight,
        })
    }

    /// The row.
    #[must_use]
    pub const fn row(&self) -> &BenchRow {
        &self.row
    }

    /// The callables and the judge bound to it.
    #[must_use]
    pub const fn attachment(&self) -> &BenchAttachment {
        &self.attachment
    }

    /// The correctness preflight.
    #[must_use]
    pub const fn preflight(&self) -> &PreflightTrial {
        &self.preflight
    }
}

impl BenchTable {
    /// One nonempty table, keeping the order the bindings were authored in.
    ///
    /// # Errors
    ///
    /// Refuses an empty table, then the first repeated row identity.
    pub fn authored(
        name: BenchTableName,
        provenance: Provenance,
        bindings: Vec<BenchBinding>,
    ) -> Result<Self, BenchTableRefusal> {
        if bindings.is_empty() {
            return Err(BenchTableRefusal::Empty);
        }
        let mut seen = BTreeMap::new();
        for (duplicate, binding) in bindings.iter().enumerate() {
            let row = binding.row().key();
            if let Some(first) = seen.insert(row, duplicate) {
                return Err(BenchTableRefusal::DuplicateRow {
                    row,
                    first,
                    duplicate,
                });
            }
        }
        Ok(Self {
            name,
            provenance,
            bindings,
        })
    }

    /// The table's name.
    #[must_use]
    pub const fn name(&self) -> BenchTableName {
        self.name
    }

    /// Whether these rows were written by hand or produced.
    #[must_use]
    pub const fn provenance(&self) -> Provenance {
        self.provenance
    }

    /// Every binding, in authored order.
    #[must_use]
    pub fn bindings(&self) -> &[BenchBinding] {
        &self.bindings
    }

    /// How many rows this table admitted.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bindings.len()
    }

    /// Always false, since an empty table is not admitted.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }
}

impl BenchInvocation {
    /// Declare the target, the clock, and the contention posture for one whole table run.
    #[must_use]
    pub fn declared(
        target: TargetBinding,
        clock: HarnessClock,
        contention: ContentionPosture,
    ) -> Self {
        Self {
            target,
            clock,
            contention,
        }
    }

    /// The target and toolchain this run declares.
    #[must_use]
    pub const fn target(&self) -> &TargetBinding {
        &self.target
    }

    /// The clock the timed pass reads.
    #[must_use]
    pub const fn clock(&self) -> HarnessClock {
        self.clock
    }

    /// The contention posture this run declares.
    #[must_use]
    pub const fn contention(&self) -> ContentionPosture {
        self.contention
    }
}

impl BenchOutcome {
    /// The stage this outcome occupies, without its evidence.
    #[must_use]
    pub const fn stage(&self) -> BenchStage {
        match self {
            Self::PreflightRefused => BenchStage::PreflightRefused,
            Self::PlantedWorseNotDistinguished { .. } => BenchStage::PlantedWorseNotDistinguished,
            Self::PrimaryWorkRefused { .. } => BenchStage::PrimaryWorkRefused,
            Self::Qualified { .. } => BenchStage::Qualified,
        }
    }
}

impl BenchReading {
    pub(in crate::bench) fn recorded(
        row: &BenchRow,
        target: TargetBinding,
        preflight: TrialReport,
        outcome: BenchOutcome,
    ) -> Self {
        Self {
            row: row.clone(),
            target,
            preflight,
            outcome,
        }
    }

    /// The whole row this reading executed.
    #[must_use]
    pub const fn row(&self) -> &BenchRow {
        &self.row
    }

    /// The target it stood on.
    #[must_use]
    pub const fn target(&self) -> &TargetBinding {
        &self.target
    }

    /// The correctness preflight's own report.
    #[must_use]
    pub const fn preflight(&self) -> &TrialReport {
        &self.preflight
    }

    /// How far the row got, and with what evidence.
    #[must_use]
    pub const fn outcome(&self) -> &BenchOutcome {
        &self.outcome
    }
}

impl BenchReport {
    pub(in crate::bench) fn recorded(
        table: BenchTableName,
        provenance: Provenance,
        readings: Vec<BenchReading>,
    ) -> Self {
        Self {
            table,
            provenance,
            readings,
        }
    }

    /// The table this report records.
    #[must_use]
    pub const fn table(&self) -> BenchTableName {
        self.table
    }

    /// Whether that table was written by hand or produced.
    #[must_use]
    pub const fn provenance(&self) -> Provenance {
        self.provenance
    }

    /// One reading per authored binding, in table order.
    #[must_use]
    pub fn readings(&self) -> &[BenchReading] {
        &self.readings
    }

    /// How many readings this report holds.
    #[must_use]
    pub fn denominator(&self) -> usize {
        self.readings.len()
    }
}

impl BenchVerdictRefusal {
    pub(in crate::bench) const fn row_not_qualified(row: BenchRowKey, stage: BenchStage) -> Self {
        Self { row, stage }
    }

    /// The first row that did not qualify.
    #[must_use]
    pub const fn row(self) -> BenchRowKey {
        self.row
    }

    /// Where that row stopped.
    #[must_use]
    pub const fn stage(self) -> BenchStage {
        self.stage
    }
}
