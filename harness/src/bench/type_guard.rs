//! The benchmark receiver's invariant nucleus and borrowed read surface.

use super::{
    BenchAttachment, BenchAttachmentRefusal, BenchBinding, BenchBindingRefusal, BenchCall,
    BenchInvocation, BenchMeasurement, BenchOutcome, BenchReading, BenchReferences, BenchReport,
    BenchRow, BenchRowKey, BenchRowRefusal, BenchStage, BenchTable, BenchTableName,
    BenchTableRefusal, BenchVerdictRefusal, ComplexityClaimRef, ContentAddress, ContentionPosture,
    DeclaredBudgets, DeclaredBudgetsRefusal, ExactRatio, InputSizeAxis, InputSizeAxisRefusal,
    Invocation, NamespacedName, NonZeroU32, PlantedWorseRef, PreflightRef, PreflightTrial,
    Provenance, SecondaryObservation, SecondaryObservationRefusal, TargetBinding, TrialBinding,
    TrialReport, WorkConclusion, WorkCount, WorkCurve, WorkCurvePoint, WorkFormula,
    WorkFormulaRefusal, WorkGapStanding, WorkJudge, WorkJudgeBinding, WorkJudgment,
    WorkJudgmentInput, WorkObservationRef, WorkQualificationRefusal, WorkRecorder,
    WorkRecordingRefusal, WorkloadRef,
};
use crate::bench::encode::derive_row_key;
use crate::clock::{HarnessClock, MeasurementReading};
use crate::descriptor::NameRefusal;
use std::collections::BTreeMap;

macro_rules! named_reference {
    ($($reference:ident),+ $(,)?) => {
        $(
            impl $reference {
                /// Parse this reference from its declaring namespace and spelling.
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

                /// This reference over an already-parsed name.
                #[must_use]
                pub const fn over(name: NamespacedName) -> Self {
                    Self(name)
                }

                /// The namespaced name this reference carries.
                #[must_use]
                pub const fn name(self) -> NamespacedName {
                    self.0
                }
            }
        )+
    };
}

named_reference!(
    WorkloadRef,
    PreflightRef,
    PlantedWorseRef,
    ComplexityClaimRef,
    WorkObservationRef,
    BenchTableName,
);

impl InputSizeAxis {
    /// Parse at least two distinct input sizes while retaining authored order.
    ///
    /// # Errors
    ///
    /// Refuses a roster shorter than two, then the first repeated size.
    pub fn declared(sizes: Vec<u64>) -> Result<Self, InputSizeAxisRefusal> {
        if sizes.len() < 2usize {
            return Err(InputSizeAxisRefusal::TooShort { found: sizes.len() });
        }
        let mut positions = BTreeMap::new();
        for (duplicate, size) in sizes.iter().copied().enumerate() {
            if let Some(first) = positions.insert(size, duplicate) {
                return Err(InputSizeAxisRefusal::DuplicateSize {
                    size,
                    first,
                    duplicate,
                });
            }
        }
        Ok(Self(sizes))
    }

    /// The input sizes in authored order.
    #[must_use]
    pub fn sizes(&self) -> &[u64] {
        &self.0
    }
}

impl ExactRatio {
    /// The ratio's numerator.
    #[must_use]
    pub const fn numerator(self) -> u64 {
        self.numerator
    }

    /// The ratio's denominator.
    #[must_use]
    pub const fn denominator(self) -> u64 {
        self.denominator
    }
}

impl DeclaredBudgets {
    /// Parse one positive sample count, a zero-or-more warmup count, and one positive exact ratio.
    ///
    /// # Errors
    ///
    /// Refuses zero samples, then a zero ratio numerator, then a zero ratio denominator.
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

    /// The positive primary sample count.
    #[must_use]
    pub const fn samples(self) -> u32 {
        self.samples.get()
    }

    /// The declared warmup count.
    #[must_use]
    pub const fn warmups(self) -> u32 {
        self.warmups
    }

    /// The exact ratio supplied to the work judge.
    #[must_use]
    pub const fn ratio(self) -> ExactRatio {
        self.ratio
    }
}

impl WorkFormula {
    /// Parse one present owner-declared work-formula representation.
    ///
    /// # Errors
    ///
    /// Refuses empty bytes; a row with no formula carries `None` instead.
    pub fn encoded(bytes: Vec<u8>) -> Result<Self, WorkFormulaRefusal> {
        if bytes.is_empty() {
            return Err(WorkFormulaRefusal::Empty);
        }
        Ok(Self { bytes })
    }

    /// The exact bytes the owner declared for this formula.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl BenchReferences {
    /// The semantic references one benchmark row joins.
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

    /// The measured workload reference.
    #[must_use]
    pub const fn workload(self) -> WorkloadRef {
        self.workload
    }

    /// The correctness-preflight reference.
    #[must_use]
    pub const fn preflight(self) -> PreflightRef {
        self.preflight
    }

    /// The planted-worse reference.
    #[must_use]
    pub const fn planted_worse(self) -> PlantedWorseRef {
        self.planted_worse
    }

    /// The neutral complexity-claim reference.
    #[must_use]
    pub const fn complexity(self) -> ComplexityClaimRef {
        self.complexity
    }
}

impl BenchMeasurement {
    /// The measurement facts one benchmark row declares.
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

    /// The authored input-size axis.
    #[must_use]
    pub const fn input_sizes(&self) -> &InputSizeAxis {
        &self.input_sizes
    }

    /// The benchmark budgets.
    #[must_use]
    pub const fn budgets(&self) -> DeclaredBudgets {
        self.budgets
    }

    /// The declared contention posture.
    #[must_use]
    pub const fn contention(&self) -> ContentionPosture {
        self.contention
    }

    /// The optional owner-declared work-formula bytes.
    #[must_use]
    pub const fn formula(&self) -> Option<&WorkFormula> {
        self.formula.as_ref()
    }
}

impl BenchRowKey {
    pub(in crate::bench) const fn derived(address: ContentAddress) -> Self {
        Self(address)
    }

    /// The row identity's derived address.
    #[must_use]
    pub const fn address(self) -> ContentAddress {
        self.0
    }
}

impl BenchRow {
    /// Build one immutable row and derive its key from the complete eight-field declaration.
    ///
    /// # Errors
    ///
    /// Refuses only where the canonical encoder cannot represent a member length in its declared width.
    pub fn declared(
        references: BenchReferences,
        measurement: BenchMeasurement,
    ) -> Result<Self, BenchRowRefusal> {
        let key = derive_row_key(references, &measurement).map_err(BenchRowRefusal::Encoding)?;
        Ok(Self {
            workload: references.workload(),
            input_sizes: measurement.input_sizes,
            preflight: references.preflight(),
            planted_worse: references.planted_worse(),
            budgets: measurement.budgets,
            contention: measurement.contention,
            formula: measurement.formula,
            complexity: references.complexity(),
            key,
        })
    }

    /// The measured workload reference.
    #[must_use]
    pub const fn workload(&self) -> WorkloadRef {
        self.workload
    }

    /// The authored input-size axis.
    #[must_use]
    pub const fn input_sizes(&self) -> &InputSizeAxis {
        &self.input_sizes
    }

    /// The correctness-preflight reference.
    #[must_use]
    pub const fn preflight(&self) -> PreflightRef {
        self.preflight
    }

    /// The planted-worse reference.
    #[must_use]
    pub const fn planted_worse(&self) -> PlantedWorseRef {
        self.planted_worse
    }

    /// The declared benchmark budgets.
    #[must_use]
    pub const fn budgets(&self) -> DeclaredBudgets {
        self.budgets
    }

    /// The declared contention posture.
    #[must_use]
    pub const fn contention(&self) -> ContentionPosture {
        self.contention
    }

    /// The optional owner-declared work-formula bytes.
    #[must_use]
    pub const fn formula(&self) -> Option<&WorkFormula> {
        self.formula.as_ref()
    }

    /// The neutral complexity-claim reference.
    #[must_use]
    pub const fn complexity(&self) -> ComplexityClaimRef {
        self.complexity
    }

    /// The identity derived from the complete row declaration.
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

    /// The row's exact optional formula.
    #[must_use]
    pub const fn formula(&self) -> Option<&WorkFormula> {
        self.formula
    }

    /// The row's neutral complexity claim.
    #[must_use]
    pub const fn complexity(&self) -> ComplexityClaimRef {
        self.complexity
    }

    /// The row's declared benchmark budgets and exact ratio.
    #[must_use]
    pub const fn budgets(&self) -> DeclaredBudgets {
        self.budgets
    }

    /// The measured primary work curve.
    #[must_use]
    pub const fn measured(&self) -> &WorkCurve {
        self.measured
    }

    /// The deliberately worse primary work curve.
    #[must_use]
    pub const fn planted_worse(&self) -> &WorkCurve {
        self.planted_worse
    }
}

impl WorkJudgeBinding {
    /// Bind one capture-free judge to the neutral complexity claim it reads.
    #[must_use]
    pub const fn bound(complexity: ComplexityClaimRef, judge: WorkJudge) -> Self {
        Self { complexity, judge }
    }

    /// The neutral complexity claim this judge reads.
    #[must_use]
    pub const fn complexity(self) -> ComplexityClaimRef {
        self.complexity
    }

    /// The capture-free work judge.
    #[must_use]
    pub const fn judge(self) -> WorkJudge {
        self.judge
    }
}

impl WorkJudgment {
    /// State the measured, planted-worse, and declared-gap readings together.
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

    /// The measured curve's conclusion.
    pub const fn measured(self) -> WorkConclusion {
        self.measured
    }

    /// The planted-worse curve's conclusion.
    pub const fn planted_worse(self) -> WorkConclusion {
        self.planted_worse
    }

    /// Whether the exact declared gap distinguished the curves.
    #[must_use]
    pub const fn gap(self) -> WorkGapStanding {
        self.gap
    }

    /// Read whether all three primary readings qualify this row for secondary observation.
    ///
    /// # Errors
    ///
    /// Refuses an inactive planted-worse control before a refused measured curve.
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

    /// Whether the relational reading qualifies this row for secondary observation.
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

    /// The exact accumulated count.
    #[must_use]
    pub const fn count(self) -> u64 {
        self.count
    }
}

impl WorkCurvePoint {
    /// The input size this point records.
    #[must_use]
    pub const fn input_size(&self) -> u64 {
        self.input_size
    }

    /// The point's work counts in declared observation order.
    #[must_use]
    pub fn counts(&self) -> &[WorkCount] {
        &self.counts
    }
}

impl WorkCurve {
    pub(in crate::bench) fn recorded(points: Vec<WorkCurvePoint>) -> Self {
        Self { points }
    }

    /// The curve's points in authored input-axis order.
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

    /// The work curve recorded by the timed pass.
    #[must_use]
    pub const fn work(&self) -> &WorkCurve {
        &self.work
    }

    /// The same work judge's accepted reading of the timed pass.
    pub const fn judgment(&self) -> WorkJudgment {
        self.judgment
    }

    /// Caller-clock readings in input-axis, then sample order.
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

    /// Add exact units to one observation declared by this recorder's binding.
    ///
    /// # Errors
    ///
    /// Refuses an observation outside the scoped roster, then checked-add overflow.
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
    /// Bind the measured and planted-worse callables, work judge, and complete observation roster.
    ///
    /// # Errors
    ///
    /// Refuses an empty observation roster, then the first duplicate in authored order.
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
        let mut positions = BTreeMap::new();
        for (duplicate, observation) in observations.iter().copied().enumerate() {
            if let Some(first) = positions.insert(observation, duplicate) {
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

    /// The measured workload reference.
    #[must_use]
    pub const fn workload(&self) -> WorkloadRef {
        self.workload
    }

    /// The measured callable.
    #[must_use]
    pub const fn measured(&self) -> BenchCall {
        self.measured
    }

    /// The planted-worse reference.
    #[must_use]
    pub const fn planted_worse_ref(&self) -> PlantedWorseRef {
        self.planted_worse_ref
    }

    /// The planted-worse callable.
    #[must_use]
    pub const fn planted_worse(&self) -> BenchCall {
        self.planted_worse
    }

    /// The owner-bound relational work judge.
    #[must_use]
    pub const fn judge(&self) -> WorkJudgeBinding {
        self.judge
    }

    /// The complete work-observation roster in authored order.
    #[must_use]
    pub fn observations(&self) -> &[WorkObservationRef] {
        &self.observations
    }
}

impl PreflightTrial {
    /// Bind one preflight reference to a real trial binding and invocation.
    #[must_use]
    pub fn bound(reference: PreflightRef, binding: TrialBinding, invocation: Invocation) -> Self {
        Self {
            reference,
            binding,
            invocation,
        }
    }

    /// The benchmark-owned preflight reference.
    #[must_use]
    pub const fn reference(&self) -> PreflightRef {
        self.reference
    }

    /// The real trial binding used for preflight.
    #[must_use]
    pub const fn binding(&self) -> &TrialBinding {
        &self.binding
    }

    /// The real trial invocation used for preflight.
    #[must_use]
    pub const fn invocation(&self) -> &Invocation {
        &self.invocation
    }
}

impl BenchBinding {
    /// Join one row to callables and a preflight under matching semantic references.
    ///
    /// # Errors
    ///
    /// Refuses workload, planted-worse, preflight, then complexity mismatch.
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

    /// The immutable row declaration.
    #[must_use]
    pub const fn row(&self) -> &BenchRow {
        &self.row
    }

    /// The bound benchmark callables and observations.
    #[must_use]
    pub const fn attachment(&self) -> &BenchAttachment {
        &self.attachment
    }

    /// The real correctness preflight.
    #[must_use]
    pub const fn preflight(&self) -> &PreflightTrial {
        &self.preflight
    }
}

impl BenchTable {
    /// Build one nonempty benchmark table while retaining authored order.
    ///
    /// # Errors
    ///
    /// Refuses an empty denominator, then the first duplicate complete row identity.
    pub fn authored(
        name: BenchTableName,
        provenance: Provenance,
        bindings: Vec<BenchBinding>,
    ) -> Result<Self, BenchTableRefusal> {
        if bindings.is_empty() {
            return Err(BenchTableRefusal::Empty);
        }
        let mut positions = BTreeMap::new();
        for (duplicate, binding) in bindings.iter().enumerate() {
            let row = binding.row().key();
            if let Some(first) = positions.insert(row, duplicate) {
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

    /// The table's authored name.
    #[must_use]
    pub const fn name(&self) -> BenchTableName {
        self.name
    }

    /// The table's producer posture.
    #[must_use]
    pub const fn provenance(&self) -> Provenance {
        self.provenance
    }

    /// Every binding in authored order.
    #[must_use]
    pub fn bindings(&self) -> &[BenchBinding] {
        &self.bindings
    }

    /// The denominator derived from the admitted bindings.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bindings.len()
    }

    /// Whether this table has no row; always false for an admitted table.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }
}

impl BenchInvocation {
    /// Declare the target, caller clock, and contention posture for one complete table run.
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

    /// The target and toolchain this host run declares.
    #[must_use]
    pub const fn target(&self) -> &TargetBinding {
        &self.target
    }

    /// The caller's secondary-observation clock.
    #[must_use]
    pub const fn clock(&self) -> HarnessClock {
        self.clock
    }

    /// The host's declared contention posture.
    #[must_use]
    pub const fn contention(&self) -> ContentionPosture {
        self.contention
    }
}

impl BenchOutcome {
    /// The stage this outcome occupies.
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

    /// The complete immutable row declaration this reading executed.
    #[must_use]
    pub const fn row(&self) -> &BenchRow {
        &self.row
    }

    /// The target and toolchain this reading stood on.
    #[must_use]
    pub const fn target(&self) -> &TargetBinding {
        &self.target
    }

    /// The exact retained correctness-preflight report.
    #[must_use]
    pub const fn preflight(&self) -> &TrialReport {
        &self.preflight
    }

    /// The row's stage-shaped benchmark outcome.
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

    /// The authored table this report records.
    #[must_use]
    pub const fn table(&self) -> BenchTableName {
        self.table
    }

    /// The table's producer posture.
    #[must_use]
    pub const fn provenance(&self) -> Provenance {
        self.provenance
    }

    /// One reading per authored binding, in table order.
    #[must_use]
    pub fn readings(&self) -> &[BenchReading] {
        &self.readings
    }

    /// The denominator derived from the complete reading census.
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

    /// The stage at which that row stopped.
    #[must_use]
    pub const fn stage(self) -> BenchStage {
        self.stage
    }
}
