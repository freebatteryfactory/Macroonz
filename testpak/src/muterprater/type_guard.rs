//! The proof-pressure engine's invariant nucleus: every road that builds one of
//! this home's values, and every reader that hands its seats back.
//!
//! Declared inside `types.rs` as its own child, which is what makes this home's
//! claims structural rather than remembered. A kill is minted HERE, so a kill
//! standing on an unqualified baseline is not a value that exists. A survivor is
//! minted HERE, so a mutant nothing observed fire — and a mutant under a backend
//! that cannot observe firing at all — can never earn the word. A dud plant is
//! refused HERE, so activation evidence always has a firing behind it. A
//! duplicate is refused HERE, so "not a duplicate" is a comparison rather than a
//! paragraph. And a proposal is married to its ground HERE, so evidence that
//! does not fit the ground is not a proposal anybody can offer.

use super::{
    ActivationDisposition, ActivationEvidence, ActivationSite, ActiveMutant, ActiveSelection,
    AdapterProfile, AlternativeIndex, AnnouncedRoster, BackendVersion, BackendVersionPosture,
    BackendVersionRefusal, BaselineAxis, BaselinePrecondition, BaselineQualification,
    BudgetRefusal, CandidateSketch, CheckGap, ClaimCeiling, CoordinateRefusal, Demonstration,
    DemonstratedRejection, DiffPath, DiffPathRefusal, DischargeEvidence, DudPlant,
    DuplicateEvidence, DuplicateRefusal, EquivalenceAxis, EvaluationSurface, ExecutionAxis,
    ExplanationRefusal, FamilyAttribution, GrammarVersion, InconclusiveCause, InferredObligation,
    IntendedRejection, KillRefusal, MUTATION_TARGET_TAG, MappingPosture, MaterializationAxis,
    MutantId, MutationCensus, MutationIdentity, MutationOutcome, MutationPoint, MutationReport,
    MutationRun, MutationSite, MutationTarget, MutationVerdict, NoComparisonReason, ObligationLane,
    OperatorFamilyRef, OracleClass, OwedClaim, OwedClaimRefusal, OwedDeclaration, PROPOSAL_TAG,
    ParityStanding, PlanRefusal, PlannedDamage, PlannedRun, PointRefusal, PressureBudget,
    PressureLane, ProofDelta, ProofDeltaRefusal, ProofPlan, ProofRefusal, ProofShape, Proposal,
    ProposalDestination, ProposalGround, ProposalRefusal, ReadingSource, RejectionIdentity,
    RewriteCandidate, RewriteDescriptor, RewriteRefusal, RewriteRoster, RewriteTrust,
    RosterRefusal, ScopeShape, ScopedInvocation, SelectionRefusal, SinkRefusal, SourceCoordinate,
    StoredProposalRef, SurfaceRefusal, SurvivalRefusal, SurvivorExplanation, UnparsedLine,
    WrapReading, WrapRefusal, WrappedBackend,
};
use crate::depot::operator_families::OPERATOR_FAMILIES;
use crate::depot::types::OperatorFamily;
use crate::descriptor::{
    AdmissionGround, CheckRef, ClaimRef, Classification, ExecutionSuite, MutationPointRef,
    NameRefusal, NamespacedName, Origin, PopulationRef, ProposalId, Row, SubjectRoute,
    SynthesisFacts, TablePosture,
};
use crate::identity::ContentAddress;
use crate::report::{
    ClaimExercise, ExecutionKey, Fingerprint, ForeignText, InvocationProfile, ReplayCapsule,
    RunAttempt, RunReport, TrialConclusion, TrialFinding, TrialId, encode_bytes,
};
use crate::runner::Selection;
use std::collections::BTreeSet;

/// The version of the external-mutant identity encoding itself.
///
/// It rides the preimage, so changing how the bytes are cut renames every mutant
/// derived under the old cut rather than letting two encodings be mistaken for
/// one another.
const MUTANT_ENCODING_VERSION: u32 = 1;

/// The version of the proposal identity encoding itself.
const PROPOSAL_ENCODING_VERSION: u32 = 1;

// ---------------------------------------------------------------------------
// The mutation target.
// ---------------------------------------------------------------------------

impl SourceCoordinate {
    /// The coordinate an external backend reported.
    ///
    /// # Errors
    ///
    /// Refuses an empty file spelling, because a coordinate that names no file
    /// places nothing.
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
    /// Two primitives: `u32be(n)` — the integer in four big-endian bytes — and
    /// `bytes(x)` — `u64be(len(x))` followed by the bytes of `x`, the record
    /// vocabulary's one framing law ([`crate::report::encode_bytes`]).
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
    ///
    /// # Nonclaims
    ///
    /// The identity commits to what the backend REPORTED. A mutant whose line
    /// moved is a different mutant under this naming, which is honest: the
    /// coordinate is the whole of what arrived.
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
    /// An external identity names a coordinate rather than a point, so it
    /// answers nothing here.
    #[must_use]
    pub const fn point(self) -> Option<MutationPointRef> {
        match self {
            Self::External(_) => None,
            Self::Interpreted(point) => Some(point),
        }
    }
}

impl OperatorFamilyRef {
    /// The reference over one row of the bank, already read.
    #[must_use]
    pub const fn declared(family: OperatorFamily) -> Self {
        Self(family)
    }

    /// The reference the bank declares under this slug, where the bank declares
    /// one.
    ///
    /// Resolved against the bank's own roster, so a reference can never name a
    /// family the bank does not declare.
    #[must_use]
    pub fn of_slug(slug: &str) -> Option<Self> {
        OPERATOR_FAMILIES
            .into_iter()
            .find(|family| family.slug == slug)
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
        self.0.slug
    }
}

impl MutationTarget {
    /// One damaged thing this lane pressed.
    #[must_use]
    pub fn pressed(
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

    /// Whether the origin-graph reading named the claim that owns the site.
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
// Activation evidence, and the dud plant.
// ---------------------------------------------------------------------------

impl ActivationEvidence {
    /// The evidence that one planted damage fired.
    ///
    /// # Errors
    ///
    /// Refuses a firing count of zero and hands back the [`DudPlant`] finding
    /// instead: a plant that never fired is a finding, never a silent pass.
    pub fn observed(
        point: MutationPointRef,
        witness: TrialId,
        firings: u32,
    ) -> Result<Self, DudPlant> {
        if firings == 0_u32 {
            return Err(DudPlant { point, witness });
        }
        Ok(Self {
            point,
            witness,
            firings,
        })
    }

    /// The point whose damage fired.
    #[must_use]
    pub const fn point(self) -> MutationPointRef {
        self.point
    }

    /// The trial whose execution observed the firing.
    #[must_use]
    pub const fn witness(self) -> TrialId {
        self.witness
    }

    /// How many times the damage was observed to fire.
    #[must_use]
    pub const fn firings(self) -> u32 {
        self.firings
    }
}

impl DudPlant {
    /// The point whose damage never fired.
    #[must_use]
    pub const fn point(self) -> MutationPointRef {
        self.point
    }

    /// The trial that was supposed to reach it.
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
    pub fn demonstrated(trial: TrialId, finding: TrialFinding) -> Self {
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

/// The baseline every lawful outcome stands on, read once for both roads.
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
        ExecutionAxis::TimedOut | ExecutionAxis::Crashed | ExecutionAxis::InfrastructureFailed => {
            false
        }
    }
}

impl MutationReport {
    /// The record of one mutant this lane killed lawfully.
    ///
    /// # Errors
    ///
    /// Refuses, in a declared dependent order: a baseline that is not a
    /// qualified unchanged pass, a damage that did not materialize, an
    /// activation that was not observed under a backend that CAN observe
    /// firing, and a witness that did not complete. The
    /// unobservable-under-backend arm is admitted, at its stated ceiling — a
    /// kill under it asserts witness rejection, never observed activation.
    pub fn killed(
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
            ActivationDisposition::Observed(_) | ActivationDisposition::UnobservableUnderBackend => {
            }
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

    /// The record of one mutant this lane proved survived.
    ///
    /// # Errors
    ///
    /// Refuses, in a declared dependent order: an unqualified baseline, a damage
    /// that did not materialize, an activation that was not observed — an
    /// unactivated mutant is not a survivor — an activation that is unobservable
    /// under the backend, a witness that did not complete, and a damage proven
    /// equivalent in scope. The unobservable arm is the structural rule: a
    /// mutant nothing could have observed firing can never earn survived, and
    /// its non-kill result is [`MutationOutcome::Inconclusive`].
    pub fn survived(
        target: MutationTarget,
        baseline: BaselineAxis,
        materialization: MaterializationAxis,
        activation: ActivationDisposition,
        execution: ExecutionAxis,
        equivalence: EquivalenceAxis,
    ) -> Result<Self, SurvivalRefusal> {
        if !baseline_qualified(baseline) {
            return Err(SurvivalRefusal::BaselineNotQualified(baseline));
        }
        if !materialized(materialization) {
            return Err(SurvivalRefusal::NotMaterialized(materialization));
        }
        match activation {
            ActivationDisposition::Observed(_) => {}
            ActivationDisposition::NotObserved => {
                return Err(SurvivalRefusal::ActivationNotObserved);
            }
            ActivationDisposition::UnobservableUnderBackend => {
                return Err(SurvivalRefusal::ActivationUnobservable);
            }
        }
        if !witness_completed(execution) {
            return Err(SurvivalRefusal::WitnessDidNotComplete(execution));
        }
        match equivalence {
            EquivalenceAxis::NotAssessed
            | EquivalenceAxis::Refuted
            | EquivalenceAxis::Inconclusive => {}
            EquivalenceAxis::ProvenInScope => {
                return Err(SurvivalRefusal::ProvenEquivalentInScope);
            }
        }
        Ok(Self {
            target,
            baseline,
            materialization,
            activation,
            execution,
            outcome: MutationOutcome::Survived,
            equivalence,
        })
    }

    /// The record of one mutant that established nothing about the suite.
    ///
    /// Total, and deliberately so: any chain can fail to establish anything, and
    /// the cause names which link did not hold.
    #[must_use]
    pub fn inconclusive(
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

impl MutationCensus {
    /// The accounting over one run's mutants, counted from the records
    /// themselves.
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

    /// How many mutants the suite accepted while their damage was proven to
    /// fire.
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
    /// Refuses a baseline that ran and did not pass, then one that was not run
    /// at all. A mutant "caught" by an already-failing suite proves nothing, so
    /// there is no road from an unqualified reading to this value.
    pub const fn read(axis: BaselineAxis) -> Result<Self, BaselinePrecondition> {
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
    pub fn recorded(baseline: BaselineQualification, reports: Vec<MutationReport>) -> Self {
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

    /// Every mutant this run did not kill, whatever the reason.
    ///
    /// The roster a reader means by "what got through", which under a backend
    /// with no activation channel is inconclusive rather than survived.
    pub fn non_kills(&self) -> impl Iterator<Item = &MutationReport> {
        self.reports
            .iter()
            .filter(|report| report.verdict() != MutationVerdict::Killed)
    }

    /// Every mutant this run proved survived.
    pub fn survivors(&self) -> impl Iterator<Item = &MutationReport> {
        self.reports
            .iter()
            .filter(|report| report.verdict() == MutationVerdict::Survived)
    }
}

// ---------------------------------------------------------------------------
// The wrap lane's reading vocabulary.
// ---------------------------------------------------------------------------

impl BackendVersion {
    /// The version the party that ran the backend states.
    ///
    /// # Errors
    ///
    /// Refuses an empty spelling, which states no version.
    pub fn stated(spelling: &str) -> Result<Self, BackendVersionRefusal> {
        if spelling.is_empty() {
            return Err(BackendVersionRefusal::EmptySpelling);
        }
        Ok(Self(spelling.to_owned()))
    }

    /// The spelling that party stated.
    #[must_use]
    pub fn spelling(&self) -> &str {
        &self.0
    }
}

impl GrammarVersion {
    /// The version an adapter's own page states for its line grammar.
    #[must_use]
    pub const fn adapter(version: u32) -> Self {
        Self(version)
    }

    /// The number the adapter states.
    #[must_use]
    pub const fn number(self) -> u32 {
        self.0
    }
}

impl AdapterProfile {
    /// What one reading is stated under.
    #[must_use]
    pub fn stated(
        backend: WrappedBackend,
        version: BackendVersionPosture,
        source: ReadingSource,
        grammar: GrammarVersion,
    ) -> Self {
        Self {
            backend,
            version,
            source,
            grammar,
        }
    }

    /// The backend the reading was taken from.
    #[must_use]
    pub const fn backend(&self) -> WrappedBackend {
        self.backend
    }

    /// Whether the party that ran that backend stated its version.
    #[must_use]
    pub const fn version(&self) -> &BackendVersionPosture {
        &self.version
    }

    /// Which of the backend's outputs the reading was taken from.
    #[must_use]
    pub const fn source(&self) -> ReadingSource {
        self.source
    }

    /// The adapter grammar version the reading was taken under.
    #[must_use]
    pub const fn grammar(&self) -> GrammarVersion {
        self.grammar
    }

    /// The most a reading under this profile can establish.
    ///
    /// Read from the source rather than stored, so a profile can never grant
    /// more than the output it was taken from affords.
    #[must_use]
    pub fn ceiling(&self) -> ClaimCeiling {
        ClaimCeiling::from(self.source)
    }
}

impl UnparsedLine {
    /// One line of a backend's output this parser could not read.
    ///
    /// The material is admitted through the record vocabulary's bounded foreign
    /// text, so a line is cut at the bound with the cut marked rather than
    /// carried at whatever length a backend chose.
    #[must_use]
    pub fn unread(ordinal: usize, material: &[u8]) -> Self {
        Self {
            ordinal,
            text: ForeignText::admitted(material),
        }
    }

    /// Which line of the output it was, counting from zero.
    #[must_use]
    pub const fn ordinal(&self) -> usize {
        self.ordinal
    }

    /// The line itself.
    #[must_use]
    pub const fn text(&self) -> &ForeignText {
        &self.text
    }
}

impl WrapReading {
    /// What one reading of a backend's output recovered, stated under the
    /// profile it was read through.
    ///
    /// # Authority
    ///
    /// The profile rides the reading, so there is no road to a wrap reading
    /// that does not say which grammar it stands on and what it may claim.
    ///
    /// # Errors
    ///
    /// Refuses a run carrying a record whose verdict the profile's ceiling does
    /// not admit, naming the record, its verdict, and the ceiling — so a
    /// reading can never state more than its source affords.
    pub fn read(
        profile: AdapterProfile,
        run: MutationRun,
        announced: AnnouncedRoster,
        unparsed: Vec<UnparsedLine>,
    ) -> Result<Self, WrapRefusal> {
        let ceiling = profile.ceiling();
        for (at, report) in run.reports().iter().enumerate() {
            let verdict = report.verdict();
            if !ceiling.admits(verdict) {
                return Err(WrapRefusal::VerdictPastCeiling {
                    at,
                    verdict,
                    ceiling,
                });
            }
        }
        Ok(Self {
            profile,
            run,
            announced,
            unparsed,
        })
    }

    /// What the reading is stated under.
    #[must_use]
    pub const fn profile(&self) -> &AdapterProfile {
        &self.profile
    }

    /// The run the reading recovered.
    #[must_use]
    pub const fn run(&self) -> &MutationRun {
        &self.run
    }

    /// What the backend announced about its own roster.
    #[must_use]
    pub const fn announced(&self) -> AnnouncedRoster {
        self.announced
    }

    /// Every line the parser could not read, in output order.
    #[must_use]
    pub fn unparsed(&self) -> &[UnparsedLine] {
        &self.unparsed
    }
}

// ---------------------------------------------------------------------------
// The interpreted lane's evaluation surface.
// ---------------------------------------------------------------------------

impl ActivationSite {
    /// The site, parsed from the owner that declares it and the spelling it
    /// carries.
    ///
    /// # Errors
    ///
    /// Refuses an empty namespace, then an empty stem.
    pub fn named(namespace: &'static str, stem: &'static str) -> Result<Self, NameRefusal> {
        NamespacedName::named(namespace, stem).map(Self)
    }

    /// The site, over a name already parsed.
    #[must_use]
    pub const fn over(name: NamespacedName) -> Self {
        Self(name)
    }

    /// The namespaced name this site carries.
    #[must_use]
    pub const fn name(self) -> NamespacedName {
        self.0
    }
}

impl MutationPoint {
    /// One point, as its producer states it.
    ///
    /// # Errors
    ///
    /// Refuses an empty original operation, then an alternative byte-identical
    /// to the original — which would be the no-mutation reading under another
    /// name — then an alternative the roster already carries.
    pub fn declared(
        identity: MutationPointRef,
        owner_claim: ClaimRef,
        original_operation: &'static [u8],
        admitted_alternatives: &'static [&'static [u8]],
        activation_site: ActivationSite,
    ) -> Result<Self, PointRefusal> {
        if original_operation.is_empty() {
            return Err(PointRefusal::EmptyOriginalOperation);
        }
        for (at, alternative) in admitted_alternatives.iter().enumerate() {
            if *alternative == original_operation {
                return Err(PointRefusal::AlternativeIsOriginal { at });
            }
        }
        for (at, alternative) in admitted_alternatives.iter().enumerate() {
            if admitted_alternatives
                .iter()
                .take(at)
                .any(|earlier| earlier == alternative)
            {
                return Err(PointRefusal::DuplicateAlternative { at });
            }
        }
        Ok(Self {
            identity,
            owner_claim,
            original_operation,
            admitted_alternatives,
            activation_site,
        })
    }

    /// The reference this point is known by.
    #[must_use]
    pub const fn identity(self) -> MutationPointRef {
        self.identity
    }

    /// The claim that owns the behaviour at this point.
    #[must_use]
    pub const fn owner_claim(self) -> ClaimRef {
        self.owner_claim
    }

    /// The unmutated reading — what the point reads as under no mutation.
    #[must_use]
    pub const fn original_operation(self) -> &'static [u8] {
        self.original_operation
    }

    /// The damages this point may be selected into, in declared order.
    #[must_use]
    pub const fn admitted_alternatives(self) -> &'static [&'static [u8]] {
        self.admitted_alternatives
    }

    /// Where a selected alternative fires.
    #[must_use]
    pub const fn activation_site(self) -> ActivationSite {
        self.activation_site
    }

    /// Every active mutant this point admits, in declared order.
    ///
    /// Total: the point is the authority on which alternatives it admits, so
    /// enumerating its own carries no refusal — which is what lets a planner
    /// state every intended damage without a branch that cannot happen.
    #[must_use]
    pub fn selections(self) -> Vec<ActiveSelection> {
        (0..self.admitted_alternatives.len())
            .map(|at| ActiveSelection {
                point: self.identity,
                alternative: AlternativeIndex(at),
            })
            .collect()
    }
}

impl EvaluationSurface {
    /// One evaluation copy's point table, as conforming data.
    ///
    /// # Errors
    ///
    /// Refuses an empty table — a surface nothing could ever be selected on —
    /// then two points stating one identity.
    pub fn conforming(points: Vec<MutationPoint>) -> Result<Self, SurfaceRefusal> {
        if points.is_empty() {
            return Err(SurfaceRefusal::EmptyPointTable);
        }
        let mut seen: BTreeSet<MutationPointRef> = BTreeSet::new();
        for point in &points {
            if !seen.insert(point.identity()) {
                return Err(SurfaceRefusal::DuplicatePoint(point.identity()));
            }
        }
        Ok(Self { points })
    }

    /// Every point the table carries, in declared order.
    #[must_use]
    pub fn points(&self) -> &[MutationPoint] {
        &self.points
    }

    /// The point this reference names, where the table carries one.
    #[must_use]
    pub fn point(&self, identity: MutationPointRef) -> Option<&MutationPoint> {
        self.points
            .iter()
            .find(|point| point.identity() == identity)
    }

    /// Select one point into one of the damages it admits.
    ///
    /// # Authority
    ///
    /// Runtime is SELECTION among admitted alternatives, never interpretation of
    /// arbitrary source: an index that names no admitted alternative is refused
    /// here, so no damage outside the producer's own roster is reachable.
    ///
    /// # Errors
    ///
    /// Refuses a point the table does not carry, then an index past the point's
    /// admitted roster.
    pub fn select(
        &self,
        point: MutationPointRef,
        alternative: usize,
    ) -> Result<ActiveMutant, SelectionRefusal> {
        let Some(found) = self.point(point) else {
            return Err(SelectionRefusal::NoSuchPoint(point));
        };
        let admitted = found.admitted_alternatives().len();
        if alternative >= admitted {
            return Err(SelectionRefusal::AlternativePastRoster {
                admitted,
                named: alternative,
            });
        }
        Ok(ActiveMutant::Active(ActiveSelection {
            point,
            alternative: AlternativeIndex(alternative),
        }))
    }
}

impl AlternativeIndex {
    /// The alternative's position in its point's declared roster.
    #[must_use]
    pub const fn position(self) -> usize {
        self.0
    }
}

impl ActiveSelection {
    /// The point that is damaged.
    #[must_use]
    pub const fn point(self) -> MutationPointRef {
        self.point
    }

    /// Which of its admitted alternatives is active.
    #[must_use]
    pub const fn alternative(self) -> AlternativeIndex {
        self.alternative
    }
}

impl ParityStanding {
    /// The standing one parity trial's own conclusion states.
    #[must_use]
    pub const fn of(conclusion: &TrialConclusion) -> Self {
        match conclusion {
            TrialConclusion::Passed => Self::Passed,
            TrialConclusion::Refused(_) => Self::NotPassed,
        }
    }
}

// ---------------------------------------------------------------------------
// The rewrite lane.
// ---------------------------------------------------------------------------

impl RewriteDescriptor {
    /// One rewrite-mutation descriptor, as its author states it.
    ///
    /// # Errors
    ///
    /// Refuses an empty pattern, then an empty rewrite, then a pair whose two
    /// sides are one shape — which would damage nothing.
    pub fn declared(
        family: OperatorFamilyRef,
        pattern: &'static str,
        rewrite: &'static str,
    ) -> Result<Self, RewriteRefusal> {
        if pattern.is_empty() {
            return Err(RewriteRefusal::EmptyPattern);
        }
        if rewrite.is_empty() {
            return Err(RewriteRefusal::EmptyRewrite);
        }
        if pattern == rewrite {
            return Err(RewriteRefusal::RewriteIsPattern);
        }
        Ok(Self {
            family,
            pattern,
            rewrite,
        })
    }

    /// The operator family this pair realizes.
    #[must_use]
    pub const fn family(self) -> OperatorFamilyRef {
        self.family
    }

    /// The shape a damage matches.
    #[must_use]
    pub const fn pattern(self) -> &'static str {
        self.pattern
    }

    /// The shape it rewrites to.
    #[must_use]
    pub const fn rewrite(self) -> &'static str {
        self.rewrite
    }
}

impl RewriteRoster {
    /// The lane's declared descriptors.
    ///
    /// # Errors
    ///
    /// Refuses an empty roster, then two entries stating one pattern-and-rewrite
    /// pair — refused rather than folded away, because collapsing a duplicate
    /// silently would be the harness normalizing an authoring defect out of
    /// sight.
    pub fn declared(descriptors: Vec<RewriteDescriptor>) -> Result<Self, RosterRefusal> {
        if descriptors.is_empty() {
            return Err(RosterRefusal::EmptyRoster);
        }
        for (at, descriptor) in descriptors.iter().enumerate() {
            if descriptors.iter().take(at).any(|earlier| {
                earlier.pattern() == descriptor.pattern() && earlier.rewrite() == descriptor.rewrite()
            }) {
                return Err(RosterRefusal::DuplicateDescriptor { at });
            }
        }
        Ok(Self { descriptors })
    }

    /// Every descriptor the roster carries, in declared order.
    #[must_use]
    pub fn descriptors(&self) -> &[RewriteDescriptor] {
        &self.descriptors
    }
}

impl RewriteCandidate {
    /// One descriptor planned for the harness's audit.
    #[must_use]
    pub fn planned(descriptor: RewriteDescriptor, scope: ScopeShape) -> Self {
        Self {
            descriptor,
            scope,
            trust: RewriteTrust::AuditPending,
        }
    }

    /// The descriptor.
    #[must_use]
    pub const fn descriptor(&self) -> RewriteDescriptor {
        self.descriptor
    }

    /// The scope its application was planned under.
    #[must_use]
    pub const fn scope(&self) -> &ScopeShape {
        &self.scope
    }

    /// The trust posture it stands under.
    #[must_use]
    pub const fn trust(&self) -> RewriteTrust {
        self.trust
    }
}

// ---------------------------------------------------------------------------
// Survivor explanation and the check gap.
// ---------------------------------------------------------------------------

impl SurvivorExplanation {
    /// The explanation one survivor's record hands into synthesis.
    ///
    /// # Errors
    ///
    /// Refuses a record whose verdict is not survived, then a target whose
    /// owning claim is unmapped — the explanation never invents the claim it
    /// hands on.
    pub fn of(
        report: &MutationReport,
        missing: OracleClass,
        closing: CheckRef,
    ) -> Result<Self, ExplanationRefusal> {
        let verdict = report.verdict();
        match verdict {
            MutationVerdict::Survived => {}
            MutationVerdict::Killed | MutationVerdict::Inconclusive => {
                return Err(ExplanationRefusal::NotASurvivor(verdict));
            }
        }
        let Some(claim) = report.target().owning_claim() else {
            return Err(ExplanationRefusal::OwnerUnmapped);
        };
        Ok(Self {
            target: report.target().clone(),
            claim,
            missing,
            closing,
        })
    }

    /// The target that survived.
    #[must_use]
    pub const fn target(&self) -> &MutationTarget {
        &self.target
    }

    /// The claim that owns it.
    #[must_use]
    pub const fn claim(&self) -> ClaimRef {
        self.claim
    }

    /// The oracle class no check of that claim supplies.
    #[must_use]
    pub const fn missing(&self) -> OracleClass {
        self.missing
    }

    /// The check reference that would close the opening.
    #[must_use]
    pub const fn closing(&self) -> CheckRef {
        self.closing
    }
}

impl CheckGap {
    /// The finding a synthesis raises where the closing check has no attachment.
    #[must_use]
    pub const fn found(claim: ClaimRef, check: CheckRef, missing: OracleClass) -> Self {
        Self {
            claim,
            check,
            missing,
        }
    }

    /// The claim the opening belongs to.
    #[must_use]
    pub const fn claim(self) -> ClaimRef {
        self.claim
    }

    /// The check reference nobody has written an attachment for.
    #[must_use]
    pub const fn check(self) -> CheckRef {
        self.check
    }

    /// The oracle class the missing check would supply.
    #[must_use]
    pub const fn missing(self) -> OracleClass {
        self.missing
    }
}

impl CandidateSketch {
    /// The row coordinates the caller states beside an explanation.
    #[must_use]
    pub fn stated(
        suite: ExecutionSuite,
        classification: Classification,
        subject: SubjectRoute,
        population: PopulationRef,
    ) -> Self {
        Self {
            suite,
            classification,
            subject,
            population,
        }
    }

    /// The aggregate seat the candidate would run under.
    #[must_use]
    pub const fn suite(&self) -> ExecutionSuite {
        self.suite
    }

    /// How the candidate would be classified.
    #[must_use]
    pub const fn classification(&self) -> &Classification {
        &self.classification
    }

    /// What the candidate would exercise.
    #[must_use]
    pub const fn subject(&self) -> SubjectRoute {
        self.subject
    }

    /// The population that would supply its inputs.
    #[must_use]
    pub const fn population(&self) -> PopulationRef {
        self.population
    }
}

// ---------------------------------------------------------------------------
// Scope, budget, and the proof plan.
// ---------------------------------------------------------------------------

impl DiffPath {
    /// One path a diff touched.
    ///
    /// # Errors
    ///
    /// Refuses an empty spelling, which names nothing.
    pub fn reported(spelling: &str) -> Result<Self, DiffPathRefusal> {
        if spelling.is_empty() {
            return Err(DiffPathRefusal::EmptyPath);
        }
        Ok(Self(spelling.to_owned()))
    }

    /// The spelling the caller read.
    #[must_use]
    pub fn spelling(&self) -> &str {
        &self.0
    }
}

impl PressureBudget {
    /// What one scoped run may spend.
    ///
    /// # Errors
    ///
    /// Refuses a budget admitting no mutant, because the run it bounds would
    /// press nothing.
    pub const fn declared(
        mutants: u32,
        invocation: InvocationProfile,
    ) -> Result<Self, BudgetRefusal> {
        if mutants == 0_u32 {
            return Err(BudgetRefusal::ZeroMutants);
        }
        Ok(Self {
            mutants,
            invocation,
        })
    }

    /// The greatest number of mutants the run may press.
    #[must_use]
    pub const fn mutants(self) -> u32 {
        self.mutants
    }

    /// The per-trial budgets every witness run stands under.
    #[must_use]
    pub const fn invocation(self) -> InvocationProfile {
        self.invocation
    }
}

impl ScopedInvocation {
    /// One scope shape with the budget its run may spend.
    #[must_use]
    pub fn scoped(scope: ScopeShape, budget: PressureBudget) -> Self {
        Self { scope, budget }
    }

    /// What the run is scoped to.
    #[must_use]
    pub const fn scope(&self) -> &ScopeShape {
        &self.scope
    }

    /// What it may spend.
    #[must_use]
    pub const fn budget(&self) -> PressureBudget {
        self.budget
    }
}

impl PlannedRun {
    /// One intended run.
    #[must_use]
    pub fn intended(
        lane: PressureLane,
        target: MutationIdentity,
        damage: PlannedDamage,
        selection: Selection,
        budget: PressureBudget,
    ) -> Self {
        Self {
            lane,
            target,
            damage,
            selection,
            budget,
        }
    }

    /// Which lane it belongs to.
    #[must_use]
    pub const fn lane(&self) -> PressureLane {
        self.lane
    }

    /// What it presses.
    #[must_use]
    pub const fn target(&self) -> MutationIdentity {
        self.target
    }

    /// Which damage of that target it presses.
    #[must_use]
    pub const fn damage(&self) -> PlannedDamage {
        self.damage
    }

    /// What it selects from the complete world.
    #[must_use]
    pub const fn selection(&self) -> &Selection {
        &self.selection
    }

    /// What it may spend.
    #[must_use]
    pub const fn budget(&self) -> PressureBudget {
        self.budget
    }
}

impl ProofPlan {
    /// The complete statement of an intended pressure pass.
    ///
    /// # Errors
    ///
    /// Refuses a plan stating no run, then a plan stating more runs than the
    /// scope's mutant budget admits — so a budget is weighed before it is spent
    /// rather than discovered spent.
    pub fn planned(scope: ScopedInvocation, runs: Vec<PlannedRun>) -> Result<Self, PlanRefusal> {
        if runs.is_empty() {
            return Err(PlanRefusal::NoRunPlanned);
        }
        let admitted = scope.budget().mutants();
        let planned = runs.len();
        let within = u32::try_from(planned).is_ok_and(|count| count <= admitted);
        if !within {
            return Err(PlanRefusal::BudgetOverspent { admitted, planned });
        }
        Ok(Self { scope, runs })
    }

    /// The scope and budget the pass runs under.
    #[must_use]
    pub const fn scope(&self) -> &ScopedInvocation {
        &self.scope
    }

    /// Every intended run, in planned order.
    #[must_use]
    pub fn runs(&self) -> &[PlannedRun] {
        &self.runs
    }
}

// ---------------------------------------------------------------------------
// The obligation road.
// ---------------------------------------------------------------------------

impl OwedClaim {
    /// The owed posture a claim's declaration states.
    ///
    /// # Errors
    ///
    /// Refuses a posture naming no opening condition, because an obligation that
    /// never comes due is one nobody can discharge.
    pub const fn declared(
        claim: ClaimRef,
        opening_condition: &'static str,
    ) -> Result<Self, OwedClaimRefusal> {
        if opening_condition.is_empty() {
            return Err(OwedClaimRefusal::NoOpeningCondition);
        }
        Ok(Self {
            claim,
            opening_condition,
        })
    }

    /// The claim declared owed.
    #[must_use]
    pub const fn claim(self) -> ClaimRef {
        self.claim
    }

    /// The condition its declaration named as the opening.
    #[must_use]
    pub const fn opening_condition(self) -> &'static str {
        self.opening_condition
    }
}

impl OwedDeclaration {
    /// One owed claim and the shape of proof its opening asks for.
    #[must_use]
    pub const fn stated(owed: OwedClaim, shape: ProofShape) -> Self {
        Self { owed, shape }
    }

    /// The owed claim.
    #[must_use]
    pub const fn owed(self) -> OwedClaim {
        self.owed
    }

    /// The shape of proof it asks for.
    #[must_use]
    pub const fn shape(self) -> ProofShape {
        self.shape
    }
}

impl InferredObligation {
    /// One opening a coverage reading stated.
    #[must_use]
    pub const fn inferred(owed: OwedClaim, exercise: ClaimExercise, shape: ProofShape) -> Self {
        Self {
            owed,
            exercise,
            shape,
        }
    }

    /// The owed claim.
    #[must_use]
    pub const fn owed(self) -> OwedClaim {
        self.owed
    }

    /// The counts the coverage reading recorded for it.
    #[must_use]
    pub const fn exercise(self) -> ClaimExercise {
        self.exercise
    }

    /// The shape of proof the opening asks for.
    #[must_use]
    pub const fn shape(self) -> ProofShape {
        self.shape
    }
}

impl DischargeEvidence {
    /// What discharged one owed claim.
    #[must_use]
    pub fn recorded(lane: ObligationLane, trial: TrialId, key: ExecutionKey) -> Self {
        Self { lane, trial, key }
    }

    /// The lane the obligation was routed to.
    #[must_use]
    pub const fn lane(&self) -> ObligationLane {
        self.lane
    }

    /// The trial that discharged it.
    #[must_use]
    pub const fn trial(&self) -> TrialId {
        self.trial
    }

    /// The key that trial ran under.
    #[must_use]
    pub const fn key(&self) -> &ExecutionKey {
        &self.key
    }
}

// ---------------------------------------------------------------------------
// The proposal road.
// ---------------------------------------------------------------------------

impl Demonstration {
    /// Read a demonstrated kill out of the report a staged run wrote.
    ///
    /// # Authority
    ///
    /// The report is the evidence and this reading is the demand: a claimed kill
    /// is shown on the surface with the mutant active, never asserted.
    ///
    /// # Errors
    ///
    /// Refuses, in a declared dependent order: a report standing over the
    /// authored world rather than a staged view, a census that does not carry
    /// the candidate at all, a candidate the selection passed over, a candidate
    /// that did not execute, and a candidate that executed and did not refuse.
    pub fn read(report: RunReport, candidate: TrialId) -> Result<Self, ProofRefusal> {
        match report.posture() {
            TablePosture::Staged { parent: _ } => {}
            TablePosture::Authored => return Err(ProofRefusal::NotStaged),
        }
        let rejection = {
            let Some(entry) = report
                .census()
                .iter()
                .find(|accounting| accounting.trial() == candidate)
            else {
                return Err(ProofRefusal::CandidateNotInCensus);
            };
            let Some(executed) = entry.disposition().report() else {
                return Err(ProofRefusal::CandidateNotSelected);
            };
            match executed.attempt() {
                RunAttempt::Executed(TrialConclusion::Refused(finding)) => {
                    DemonstratedRejection::demonstrated(candidate, finding.clone())
                }
                RunAttempt::Executed(TrialConclusion::Passed) => {
                    return Err(ProofRefusal::CandidateDidNotRefuse);
                }
                RunAttempt::SkippedWithReason(_)
                | RunAttempt::TimedOut(_)
                | RunAttempt::InfrastructureFailed(_) => {
                    return Err(ProofRefusal::CandidateDidNotExecute);
                }
            }
        };
        Ok(Self { report, rejection })
    }

    /// The report the staged run wrote.
    #[must_use]
    pub const fn report(&self) -> &RunReport {
        &self.report
    }

    /// The rejection read out of it.
    #[must_use]
    pub const fn rejection(&self) -> &DemonstratedRejection {
        &self.rejection
    }
}

impl ProofDelta {
    /// How much proof one candidate added to the claim it pins.
    ///
    /// # Errors
    ///
    /// Refuses a pair that does not move: a candidate that leaves the claim's
    /// exercised count where it was pins nothing.
    pub const fn between(before: usize, after: usize) -> Result<Self, ProofDeltaRefusal> {
        if after <= before {
            return Err(ProofDeltaRefusal::NoProofAdded { before, after });
        }
        Ok(Self { before, after })
    }

    /// The exercised count before the candidate ran.
    #[must_use]
    pub const fn before(self) -> usize {
        self.before
    }

    /// The exercised count after it ran.
    #[must_use]
    pub const fn after(self) -> usize {
        self.after
    }
}

impl DuplicateEvidence {
    /// The comparison a failure-bearing ground offers.
    ///
    /// # Errors
    ///
    /// Refuses a candidate whose fingerprint the known roster already carries:
    /// the comparison is performed here, so a duplicate is a refusal rather than
    /// a paragraph a reader has to check.
    pub fn failure_compared(
        candidate: Fingerprint,
        known: Vec<Fingerprint>,
    ) -> Result<Self, DuplicateRefusal> {
        if known.contains(&candidate) {
            return Err(DuplicateRefusal::FingerprintAlreadyKnown(candidate));
        }
        Ok(Self::FailureCompared { candidate, known })
    }

    /// The comparison a discharge ground offers.
    ///
    /// # Errors
    ///
    /// Refuses an owed claim that already carries a discharge, naming the first
    /// discharge already recorded for it.
    pub fn obligation_compared(
        owed: ClaimRef,
        discharges: Vec<TrialId>,
    ) -> Result<Self, DuplicateRefusal> {
        if let Some(first) = discharges.first() {
            return Err(DuplicateRefusal::ObligationAlreadyDischarged(*first));
        }
        Ok(Self::ObligationCompared { owed, discharges })
    }

    /// The statement a ground with no comparable subject makes.
    #[must_use]
    pub const fn not_applicable(reason: NoComparisonReason) -> Self {
        Self::NotApplicable { reason }
    }
}

impl ProposalDestination {
    /// Where an admitted row would land.
    #[must_use]
    pub const fn naming(suite: ExecutionSuite) -> Self {
        Self { suite }
    }

    /// The aggregate seat it would land in.
    #[must_use]
    pub const fn suite(self) -> ExecutionSuite {
        self.suite
    }

    /// The semantic owner: the suite's own namespace.
    #[must_use]
    pub const fn owner(self) -> &'static str {
        self.suite.name().namespace()
    }
}

impl Proposal {
    /// One proposal, offered.
    ///
    /// # Errors
    ///
    /// Refuses, in a declared dependent order: a row that does not carry the
    /// candidate origin arm, duplicate evidence that does not match the ground,
    /// and a survivor synthesis fact naming a different point than the ground's
    /// target names.
    pub fn offered(
        candidate: Row,
        ground: ProposalGround,
        duplicate: DuplicateEvidence,
        destination: ProposalDestination,
    ) -> Result<Self, ProposalRefusal> {
        let facts = candidate_facts(&candidate)?;
        evidence_fits_ground(&ground, &duplicate)?;
        survivor_point_agrees(facts, &ground)?;
        Ok(Self {
            candidate,
            ground,
            duplicate,
            destination,
        })
    }

    /// The candidate row.
    #[must_use]
    pub const fn candidate(&self) -> &Row {
        &self.candidate
    }

    /// The ground it stands on.
    #[must_use]
    pub const fn ground(&self) -> &ProposalGround {
        &self.ground
    }

    /// The evidence it is not a duplicate.
    #[must_use]
    pub const fn duplicate(&self) -> &DuplicateEvidence {
        &self.duplicate
    }

    /// Where it would land.
    #[must_use]
    pub const fn destination(&self) -> ProposalDestination {
        self.destination
    }

    /// The ground at summary width — the word an admission act states.
    #[must_use]
    pub fn ground_summary(&self) -> AdmissionGround {
        AdmissionGround::from(&self.ground)
    }

    /// The proposal's content identity — permanent provenance.
    ///
    /// # The specification
    ///
    /// Two primitives: `u32be(n)`, and `bytes(x)` — `u64be(len(x))` followed by
    /// the bytes of `x`.
    ///
    /// The members, in exactly this order:
    ///
    /// | # | member | encoding |
    /// | - | ------ | -------- |
    /// | 1 | encoding version | `u32be` |
    /// | 2 | candidate row | `bytes(…)` of the descriptor home's canonical row bytes |
    /// | 3 | ground | one byte, [`AdmissionGround::slot`] |
    /// | 4 | destination namespace | `bytes(utf8)` |
    /// | 5 | destination stem | `bytes(utf8)` |
    ///
    /// # Nonclaims
    ///
    /// The evidence is deliberately absent: the replay capsule, the
    /// demonstration, and the duplicate comparison are what STANDS BEHIND the
    /// proposal rather than what it proposes. Two offers of one row on one
    /// ground into one destination therefore share an identity by design, which
    /// is what makes an admitted origin's citation stable across a rerun.
    ///
    /// # Authority
    ///
    /// Total. The candidate row's canonical bytes were written where that row
    /// was born, so this road reads them rather than encoding a row a second
    /// time, and there is no shape of this call in which a proposal holds a row
    /// it cannot name.
    #[must_use]
    pub fn identity(&self) -> ProposalId {
        let mut preimage = Vec::new();
        preimage.extend_from_slice(&PROPOSAL_ENCODING_VERSION.to_be_bytes());
        encode_bytes(self.candidate.canonical_bytes().as_bytes(), &mut preimage);
        preimage.push(self.ground_summary().slot());
        let suite = self.destination.suite().name();
        encode_bytes(suite.namespace().as_bytes(), &mut preimage);
        encode_bytes(suite.stem().as_bytes(), &mut preimage);
        ProposalId::over(ContentAddress::derived(PROPOSAL_TAG, &preimage))
    }
}

/// The synthesis facts a candidate row carries, or the refusal it earns.
fn candidate_facts(candidate: &Row) -> Result<SynthesisFacts, ProposalRefusal> {
    match candidate.origin() {
        Origin::Candidate(facts) => Ok(facts),
        Origin::HandWritten
        | Origin::Generated(_)
        | Origin::AdmittedReplay(_)
        | Origin::AdmittedDischarge(_) => Err(ProposalRefusal::NotACandidate),
    }
}

/// Whether the duplicate evidence is the comparison the ground owes.
fn evidence_fits_ground(
    ground: &ProposalGround,
    duplicate: &DuplicateEvidence,
) -> Result<(), ProposalRefusal> {
    match (ground, duplicate) {
        (ProposalGround::MutantKilled { .. }, DuplicateEvidence::FailureCompared { .. })
        | (ProposalGround::ClaimPinned { .. }, DuplicateEvidence::NotApplicable { .. })
        | (
            ProposalGround::ObligationDischarged { .. },
            DuplicateEvidence::ObligationCompared { .. },
        ) => Ok(()),
        (
            ProposalGround::MutantKilled { .. },
            DuplicateEvidence::ObligationCompared { .. } | DuplicateEvidence::NotApplicable { .. },
        )
        | (
            ProposalGround::ClaimPinned { .. },
            DuplicateEvidence::FailureCompared { .. }
            | DuplicateEvidence::ObligationCompared { .. },
        )
        | (
            ProposalGround::ObligationDischarged { .. },
            DuplicateEvidence::FailureCompared { .. } | DuplicateEvidence::NotApplicable { .. },
        ) => Err(ProposalRefusal::EvidenceDoesNotMatchGround),
    }
}

/// Whether the row's survivor point and the ground's target name one point.
///
/// The check is possible only where both name a point: an external target names
/// a coordinate, and a proof-gap synthesis names no point at all.
fn survivor_point_agrees(
    facts: SynthesisFacts,
    ground: &ProposalGround,
) -> Result<(), ProposalRefusal> {
    let SynthesisFacts::Survivor(synthesis) = facts else {
        return Ok(());
    };
    let ProposalGround::MutantKilled { target, .. } = ground else {
        return Ok(());
    };
    let Some(point) = target.identity().point() else {
        return Ok(());
    };
    if synthesis == point {
        return Ok(());
    }
    Err(ProposalRefusal::SurvivorPointMismatch {
        synthesis,
        target: point,
    })
}

impl StoredProposalRef {
    /// The location a sink stored a proposal at.
    ///
    /// # Errors
    ///
    /// Refuses an empty token, which names nowhere.
    pub fn at(token: &str) -> Result<Self, SinkRefusal> {
        if token.is_empty() {
            return Err(SinkRefusal::EmptyLocation);
        }
        Ok(Self {
            token: token.to_owned(),
        })
    }

    /// The token, for a sink to read its own location back.
    #[must_use]
    pub fn token(&self) -> &str {
        &self.token
    }
}

impl ProposalGround {
    /// The replay capsule this ground carries, where it carries one.
    ///
    /// A discharge ground authors no capsule at all — the admitted row is the
    /// discharge's permanent record — which is why this is a reading over the
    /// arms rather than a field every ground would have to leave empty.
    #[must_use]
    pub const fn capsule(&self) -> Option<&ReplayCapsule> {
        match self {
            Self::MutantKilled { capsule, .. } | Self::ClaimPinned { capsule, .. } => Some(capsule),
            Self::ObligationDischarged { .. } => None,
        }
    }
}
