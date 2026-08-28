//! The invariant nucleus of mutation targets, reports, and run accounting.

use super::{
    ActivationDisposition, ActivationEvidence, ActiveSelection, AlternativeId, BaselineAxis,
    BaselinePrecondition, BaselineQualification, ClaimRef, ContentAddress, CoordinateRefusal,
    DemonstratedRejection, DudPlant, EquivalenceAxis, ExecutionAxis, FamilyAttribution,
    Fingerprint, InconclusiveCause, IntendedRejection, KillRefusal, MUTATION_TARGET_TAG,
    MappingPosture, MaterializationAxis, MutantId, MutationCensus, MutationIdentity,
    MutationOutcome, MutationPointRef, MutationReport, MutationRun, MutationSite, MutationTarget,
    MutationVerdict, OperatorFamily, OperatorFamilyRef, RejectionIdentity, SourceCoordinate,
    TrialFinding, TrialId,
};
use crate::depot::operator_families::OPERATOR_FAMILIES;
use crate::report::{RunAttempt, TrialConclusion, TrialReport, encode_bytes};
/// The version of the external-mutant identity encoding.
///
/// It rides the preimage, so changing how the bytes are cut renames every mutant derived under the old cut rather than letting two encodings be mistaken for one another.
const MUTANT_ENCODING_VERSION: u32 = 1;

/// Whether the baseline every lawful outcome stands on qualified.
fn baseline_qualified(baseline: BaselineAxis) -> bool {
    match baseline {
        BaselineAxis::Qualified => true,
        BaselineAxis::Failed | BaselineAxis::NotRun => false,
    }
}

/// Whether the damage became a thing that could be executed.
fn materialized(materialization: MaterializationAxis) -> bool {
    match materialization {
        MaterializationAxis::Built => true,
        MaterializationAxis::Unviable | MaterializationAxis::ToolFailed => false,
    }
}

/// Whether the witness execution reached a conclusion.
fn witness_completed(execution: ExecutionAxis) -> bool {
    match execution {
        ExecutionAxis::Completed => true,
        ExecutionAxis::NotExecuted
        | ExecutionAxis::TimedOut
        | ExecutionAxis::Crashed
        | ExecutionAxis::InfrastructureFailed => false,
    }
}

// ---------------------------------------------------------------------------
// The mutation target.
// ---------------------------------------------------------------------------

impl SourceCoordinate {
    /// The coordinate an external backend reported.
    ///
    /// # Errors
    ///
    /// Refuses an empty file spelling, because a coordinate that names no file places nothing.
    pub fn reported(file: &str, line: u32, column: u32) -> Result<Self, CoordinateRefusal> {
        if file.is_empty() {
            return Err(CoordinateRefusal::EmptyFile);
        }
        Ok(Self {
            file: file.to_owned(),
            line,
            column,
        })
    }

    /// The file the backend named.
    #[must_use]
    pub fn file(&self) -> &str {
        &self.file
    }

    /// The line the backend named.
    #[must_use]
    pub const fn line(&self) -> u32 {
        self.line
    }

    /// The column the backend named.
    #[must_use]
    pub const fn column(&self) -> u32 {
        self.column
    }
}

impl MutantId {
    /// Derive one external mutant's identity from what the backend reported.
    ///
    /// # The specification
    ///
    /// Two primitives: `u32be(n)` — the integer in four big-endian bytes — and `bytes(x)` — `u64be(len(x))` followed by the bytes of `x`, which is the record vocabulary's framing law ([`crate::report::encode_bytes`]).
    ///
    /// The members, in exactly this order, with no separators and no padding:
    ///
    /// | # | member | encoding |
    /// | - | ------ | -------- |
    /// | 1 | encoding version | `u32be` |
    /// | 2 | file | `bytes(utf8)` |
    /// | 3 | line | `u32be` |
    /// | 4 | column | `u32be` |
    /// | 5 | damage | `bytes(…)` — the backend's own damage text, at full length |
    #[must_use]
    pub fn over(coordinate: &SourceCoordinate, damage: &[u8]) -> Self {
        let mut preimage = Vec::new();
        preimage.extend_from_slice(&MUTANT_ENCODING_VERSION.to_be_bytes());
        encode_bytes(coordinate.file().as_bytes(), &mut preimage);
        preimage.extend_from_slice(&coordinate.line().to_be_bytes());
        preimage.extend_from_slice(&coordinate.column().to_be_bytes());
        encode_bytes(damage, &mut preimage);
        Self(ContentAddress::derived(MUTATION_TARGET_TAG, &preimage))
    }

    /// The identity's address, for comparison and for rendering.
    #[must_use]
    pub const fn address(&self) -> &ContentAddress {
        &self.0
    }
}

impl MutationIdentity {
    /// The mutation point this identity names, where it names one.
    ///
    /// An external identity names a coordinate rather than a point, so it answers nothing here.
    #[must_use]
    pub const fn point(self) -> Option<MutationPointRef> {
        match self {
            Self::External(_) => None,
            Self::Interpreted {
                point,
                alternative: _,
            }
            | Self::CompiledProjection {
                point,
                alternative: _,
            } => Some(point),
        }
    }

    /// The admitted alternative this identity names, where it names one.
    #[must_use]
    pub const fn alternative(self) -> Option<AlternativeId> {
        match self {
            Self::External(_) => None,
            Self::Interpreted {
                point: _,
                alternative,
            }
            | Self::CompiledProjection {
                point: _,
                alternative,
            } => Some(alternative),
        }
    }
}

impl OperatorFamilyRef {
    /// The reference the bank declares under this slug, where the bank declares one.
    #[must_use]
    pub fn of_slug(slug: &str) -> Option<Self> {
        OPERATOR_FAMILIES
            .iter()
            .copied()
            .find(|family| family.slug() == slug)
            .map(Self)
    }

    /// The bank row this reference carries.
    #[must_use]
    pub const fn family(self) -> OperatorFamily {
        self.0
    }

    /// The family's stable slug.
    #[must_use]
    pub const fn slug(self) -> &'static str {
        self.0.slug()
    }
}

impl MutationTarget {
    /// One damaged thing this lane pressed.
    #[must_use]
    pub(in crate::muterprater) fn pressed(
        identity: MutationIdentity,
        family: FamilyAttribution,
        site: MutationSite,
        owner: MappingPosture,
    ) -> Self {
        Self {
            identity,
            family,
            site,
            owner,
        }
    }

    /// How the damaged thing is identified.
    #[must_use]
    pub const fn identity(&self) -> MutationIdentity {
        self.identity
    }

    /// Which operator family the damage realizes, where the bank names one.
    #[must_use]
    pub const fn family(&self) -> FamilyAttribution {
        self.family
    }

    /// Where the damage lives.
    #[must_use]
    pub const fn site(&self) -> &MutationSite {
        &self.site
    }

    /// Whether the origin reading named the claim that owns the site.
    #[must_use]
    pub const fn owner(&self) -> MappingPosture {
        self.owner
    }

    /// The claim that owns the site, where the reading named one.
    #[must_use]
    pub const fn owning_claim(&self) -> Option<ClaimRef> {
        match self.owner {
            MappingPosture::Mapped(claim) => Some(claim),
            MappingPosture::OwnerUnmapped => None,
        }
    }
}

// ---------------------------------------------------------------------------
// Activation evidence.
// ---------------------------------------------------------------------------

impl ActivationEvidence {
    /// The positive firing reading one evaluation callback reported for a planted damage.
    ///
    /// A zero count returns absence, so the receiver can return the exact [`DudPlant`] finding instead.
    pub(in crate::muterprater) fn observed(
        selection: ActiveSelection,
        witness: TrialId,
        firings: u32,
    ) -> Option<Self> {
        if firings == 0_u32 {
            return None;
        }
        Some(Self {
            selection,
            witness,
            firings,
        })
    }

    /// The point whose selected damage received the positive-count report.
    #[must_use]
    pub const fn point(self) -> MutationPointRef {
        self.selection.point()
    }

    /// The exact active selection bound to the positive-count report.
    #[must_use]
    pub const fn selection(self) -> ActiveSelection {
        self.selection
    }

    /// The trial bound to the positive-count report.
    #[must_use]
    pub const fn witness(self) -> TrialId {
        self.witness
    }

    /// The positive firing count reported by the evaluation callback or backend.
    #[must_use]
    pub const fn firings(self) -> u32 {
        self.firings
    }
}

impl DudPlant {
    /// The exact selection and trial for which the evaluation callback reported zero firings.
    pub(in crate::muterprater) const fn unfired(
        selection: ActiveSelection,
        witness: TrialId,
    ) -> Self {
        Self { selection, witness }
    }

    /// The point whose selected damage received a reported count of zero.
    #[must_use]
    pub const fn point(self) -> MutationPointRef {
        self.selection.point()
    }

    /// The exact active selection that received a reported count of zero.
    #[must_use]
    pub const fn selection(self) -> ActiveSelection {
        self.selection
    }

    /// The trial bound to the callback's zero-count report.
    #[must_use]
    pub const fn witness(self) -> TrialId {
        self.witness
    }
}

impl ActivationDisposition {
    /// The evidence, where the disposition carries any.
    #[must_use]
    pub const fn evidence(self) -> Option<ActivationEvidence> {
        match self {
            Self::Observed(evidence) => Some(evidence),
            Self::NotObserved | Self::UnobservableUnderBackend => None,
        }
    }
}

// ---------------------------------------------------------------------------
// The rejection a kill stands on.
// ---------------------------------------------------------------------------

impl DemonstratedRejection {
    /// The rejection this harness's own engine demonstrated.
    #[must_use]
    pub(in crate::muterprater) fn demonstrated(trial: TrialId, finding: TrialFinding) -> Self {
        Self { trial, finding }
    }

    /// The trial that refused.
    #[must_use]
    pub const fn trial(&self) -> TrialId {
        self.trial
    }

    /// The finding it refused with.
    #[must_use]
    pub const fn finding(&self) -> &TrialFinding {
        &self.finding
    }

    /// The failure identity, derived from the trial and the finding.
    #[must_use]
    pub fn fingerprint(&self) -> Fingerprint {
        Fingerprint::of(self.trial, &self.finding)
    }
}

impl IntendedRejection {
    /// The failure identity this rejection carries, where it carries one.
    #[must_use]
    pub fn identity(&self) -> RejectionIdentity {
        match self {
            Self::Demonstrated(rejection) => {
                RejectionIdentity::Fingerprinted(rejection.fingerprint())
            }
            Self::ReportedByBackend { stated: _ } => RejectionIdentity::Unfingerprinted,
        }
    }
}

// ---------------------------------------------------------------------------
// The per-mutant record.
// ---------------------------------------------------------------------------

impl MutationReport {
    /// The record of one mutant this lane killed lawfully.
    ///
    /// # Errors
    ///
    /// Refuses, in a declared dependent order: a baseline that is not a qualified unchanged pass, a damage that did not materialize, an activation not observed under a backend that can observe firing, and a witness that did not complete.
    /// The unobservable-under-backend arm is admitted at its stated ceiling, where a kill asserts witness rejection and never observed activation.
    pub(in crate::muterprater) fn killed(
        target: MutationTarget,
        baseline: BaselineAxis,
        materialization: MaterializationAxis,
        activation: ActivationDisposition,
        execution: ExecutionAxis,
        rejection: IntendedRejection,
        equivalence: EquivalenceAxis,
    ) -> Result<Self, KillRefusal> {
        if !baseline_qualified(baseline) {
            return Err(KillRefusal::BaselineNotQualified(baseline));
        }
        if !materialized(materialization) {
            return Err(KillRefusal::NotMaterialized(materialization));
        }
        match activation {
            ActivationDisposition::Observed(_)
            | ActivationDisposition::UnobservableUnderBackend => {}
            ActivationDisposition::NotObserved => {
                return Err(KillRefusal::ActivationNotObserved);
            }
        }
        if !witness_completed(execution) {
            return Err(KillRefusal::WitnessDidNotComplete(execution));
        }
        Ok(Self {
            target,
            baseline,
            materialization,
            activation,
            execution,
            outcome: MutationOutcome::Killed(rejection),
            equivalence,
        })
    }

    /// The record of one mutant that established nothing about the suite.
    ///
    /// Total, and deliberately so: any chain can fail to establish anything, and the cause names which link did not hold.
    #[must_use]
    pub(in crate::muterprater) fn inconclusive(
        target: MutationTarget,
        baseline: BaselineAxis,
        materialization: MaterializationAxis,
        activation: ActivationDisposition,
        execution: ExecutionAxis,
        cause: InconclusiveCause,
        equivalence: EquivalenceAxis,
    ) -> Self {
        Self {
            target,
            baseline,
            materialization,
            activation,
            execution,
            outcome: MutationOutcome::Inconclusive(cause),
            equivalence,
        }
    }

    /// Derive one interpreted report from an activated execution under a qualified no-mutation baseline.
    #[must_use]
    pub(in crate::muterprater) fn interpreted(
        target: MutationTarget,
        activation: ActivationEvidence,
        report: &TrialReport,
    ) -> Self {
        let (execution, outcome) = match report.attempt() {
            RunAttempt::Executed(TrialConclusion::Passed) => {
                (ExecutionAxis::Completed, MutationOutcome::Survived)
            }
            RunAttempt::Executed(TrialConclusion::Refused(finding)) => (
                ExecutionAxis::Completed,
                MutationOutcome::Killed(IntendedRejection::Demonstrated(DemonstratedRejection {
                    trial: report.trial(),
                    finding: finding.clone(),
                })),
            ),
            RunAttempt::SkippedWithReason(_) => (
                ExecutionAxis::NotExecuted,
                MutationOutcome::Inconclusive(InconclusiveCause::WitnessIncomplete),
            ),
            RunAttempt::TimedOut => (
                ExecutionAxis::TimedOut,
                MutationOutcome::Inconclusive(InconclusiveCause::WitnessIncomplete),
            ),
            RunAttempt::InfrastructureFailed(_) => (
                ExecutionAxis::InfrastructureFailed,
                MutationOutcome::Inconclusive(InconclusiveCause::WitnessIncomplete),
            ),
        };
        Self {
            target,
            baseline: BaselineAxis::Qualified,
            materialization: MaterializationAxis::Built,
            activation: ActivationDisposition::Observed(activation),
            execution,
            outcome,
            equivalence: EquivalenceAxis::NotAssessed,
        }
    }

    /// Derive one compiled-projection kill from the report that rejected it.
    ///
    /// Returns no value unless the report completed with a typed refusal.
    /// Compiled projection pressure has no activation channel, so its axis states that ceiling while retaining the exact trial and finding the runner admitted.
    pub(in crate::muterprater) fn compiled_projection(
        target: MutationTarget,
        report: &TrialReport,
    ) -> Option<Self> {
        let RunAttempt::Executed(TrialConclusion::Refused(finding)) = report.attempt() else {
            return None;
        };
        Some(Self {
            target,
            baseline: BaselineAxis::Qualified,
            materialization: MaterializationAxis::Built,
            activation: ActivationDisposition::UnobservableUnderBackend,
            execution: ExecutionAxis::Completed,
            outcome: MutationOutcome::Killed(IntendedRejection::Demonstrated(
                DemonstratedRejection {
                    trial: report.trial(),
                    finding: finding.clone(),
                },
            )),
            equivalence: EquivalenceAxis::NotAssessed,
        })
    }

    /// What was damaged.
    #[must_use]
    pub const fn target(&self) -> &MutationTarget {
        &self.target
    }

    /// What the unchanged subject's suite did.
    #[must_use]
    pub const fn baseline(&self) -> BaselineAxis {
        self.baseline
    }

    /// Whether the damage became executable.
    #[must_use]
    pub const fn materialization(&self) -> MaterializationAxis {
        self.materialization
    }

    /// What was established about the damage firing.
    #[must_use]
    pub const fn activation(&self) -> ActivationDisposition {
        self.activation
    }

    /// What became of the witness execution.
    #[must_use]
    pub const fn execution(&self) -> ExecutionAxis {
        self.execution
    }

    /// The outcome with its evidence.
    #[must_use]
    pub const fn outcome(&self) -> &MutationOutcome {
        &self.outcome
    }

    /// The outcome at axis width.
    #[must_use]
    pub fn verdict(&self) -> MutationVerdict {
        MutationVerdict::from(&self.outcome)
    }

    /// What was established about equivalence.
    #[must_use]
    pub const fn equivalence(&self) -> EquivalenceAxis {
        self.equivalence
    }
}

// ---------------------------------------------------------------------------
// The run, and the accounting over it.
// ---------------------------------------------------------------------------

impl MutationCensus {
    /// The accounting over one run's mutants, counted from the records themselves.
    #[must_use]
    pub fn over(reports: &[MutationReport]) -> Self {
        let mut killed: u32 = 0;
        let mut survived: u32 = 0;
        let mut inconclusive: u32 = 0;
        for report in reports {
            match report.verdict() {
                MutationVerdict::Killed => killed = killed.saturating_add(1),
                MutationVerdict::Survived => survived = survived.saturating_add(1),
                MutationVerdict::Inconclusive => inconclusive = inconclusive.saturating_add(1),
            }
        }
        Self {
            killed,
            survived,
            inconclusive,
        }
    }

    /// How many mutants the suite rejected.
    #[must_use]
    pub const fn killed(self) -> u32 {
        self.killed
    }

    /// How many mutants the suite accepted after a positive firing observation.
    #[must_use]
    pub const fn survived(self) -> u32 {
        self.survived
    }

    /// How many mutants established nothing.
    #[must_use]
    pub const fn inconclusive(self) -> u32 {
        self.inconclusive
    }

    /// How many mutants the run pressed, as the sum of its parts.
    #[must_use]
    pub const fn pressed(self) -> u32 {
        self.killed
            .saturating_add(self.survived)
            .saturating_add(self.inconclusive)
    }
}

impl BaselineQualification {
    /// The precondition, read from the baseline axis one run recorded.
    ///
    /// # Errors
    ///
    /// Refuses a baseline that ran and did not pass, then one that was not run at all.
    pub(in crate::muterprater) const fn read(
        axis: BaselineAxis,
    ) -> Result<Self, BaselinePrecondition> {
        match axis {
            BaselineAxis::Qualified => Ok(Self { axis }),
            BaselineAxis::Failed => Err(BaselinePrecondition::BaselineFailed),
            BaselineAxis::NotRun => Err(BaselinePrecondition::BaselineNotRun),
        }
    }

    /// The axis reading this qualification was read from.
    #[must_use]
    pub const fn axis(self) -> BaselineAxis {
        self.axis
    }
}

impl MutationRun {
    /// One pressure run's record, with the census counted from the reports.
    #[must_use]
    pub(in crate::muterprater) fn recorded(
        baseline: BaselineQualification,
        reports: Vec<MutationReport>,
    ) -> Self {
        let census = MutationCensus::over(&reports);
        Self {
            baseline,
            reports,
            census,
        }
    }

    /// The qualified baseline the run stood on.
    #[must_use]
    pub const fn baseline(&self) -> BaselineQualification {
        self.baseline
    }

    /// Every mutant's record, in the order the run pressed them.
    #[must_use]
    pub fn reports(&self) -> &[MutationReport] {
        &self.reports
    }

    /// The accounting over them.
    #[must_use]
    pub const fn census(&self) -> MutationCensus {
        self.census
    }

    /// Every mutant this run killed, in the order the run pressed them.
    ///
    /// The roster a trust-opening fact is read out of: a run with no kill has shown no property biting.
    pub fn kills(&self) -> impl Iterator<Item = &MutationReport> {
        self.reports
            .iter()
            .filter(|report| report.verdict() == MutationVerdict::Killed)
    }

    /// Every mutant this run did not kill, whatever the reason.
    ///
    /// The roster a reader means by "what got through", which under a backend with no activation channel is inconclusive rather than survived.
    pub fn non_kills(&self) -> impl Iterator<Item = &MutationReport> {
        self.reports
            .iter()
            .filter(|report| report.verdict() != MutationVerdict::Killed)
    }

    /// Every mutant this run classified as survived under its activation ceiling.
    pub fn survivors(&self) -> impl Iterator<Item = &MutationReport> {
        self.reports
            .iter()
            .filter(|report| report.verdict() == MutationVerdict::Survived)
    }
}
