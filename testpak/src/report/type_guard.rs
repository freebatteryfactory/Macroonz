//! The record vocabulary's invariant nucleus: every road that reaches a private
//! field.
//!
//! This file is declared inside `types.rs` as its own child, so it sees the
//! fields the declarations keep private and no sibling module does. An identity
//! that skipped its preimage, a key that skipped its target binding, or a
//! foreign-text field that skipped its bound would have to be written here, and
//! none is.

use super::{
    ByteBudget, CaseBudget, CensusDelta, CensusDirection, CheckRevisionId, ClaimCoverage,
    ClaimExercise, ConclusionFlip, EXECUTION_KEY_TAG, ExecutionKey, Exercise, FINGERPRINT_TAG,
    FOREIGN_TEXT_MAX_BYTES, FailureClass, FindingCause, FindingLocation, Fingerprint, ForeignText,
    GenerationProfile, InvocationProfile, MinimizationProfile, NotSelectedReason, OutcomeClass,
    ProfiledTrial, REPLAY_CAPSULE_TAG, ROW_REVISION_TAG, RecordedDuration, ReplayCapsule,
    ReplayPosture, ReportDiff, RowRevisionChange, RowRevisionId, RunAttempt, RunReport,
    SelectionDisposition, SelectionExpectation, SelectionOutcome, SubjectRevisionId,
    TRIAL_IDENTITY_TAG, TargetBinding, TargetTriple, TextFidelity, TimeBudget, ToolchainIdentity,
    TrialAccounting, TrialConclusion, TrialFinding, TrialId, TrialProfile, TrialReport, TrialSite,
    Truncation,
};
use crate::descriptor::{
    CanonicalRowBytes, ClaimRef, GeneratedSupportSchemaId, RevisionBinding, TablePosture, TrialKey,
};
use crate::identity::ContentAddress;
use crate::report::encode::{
    execution_key_preimage, fingerprint_preimage, replay_capsule_preimage, trial_preimage,
};
use core::cmp::Ordering;

// ---------------------------------------------------------------------------
// The semantic rail.
// ---------------------------------------------------------------------------

impl ProfiledTrial {
    /// One trial's key under one profile.
    ///
    /// The key is the descriptor home's own derivation over the four semantic
    /// coordinates — the structural fact that home establishes where a row is
    /// born — and the profile is the coordinate this home adds. Standing on the
    /// key rather than on the row is what keeps the execution suite, the roles,
    /// and the tags out of a trial's identity.
    #[must_use]
    pub const fn of_key(key: TrialKey, profile: TrialProfile) -> Self {
        Self { key, profile }
    }

    /// The trial's compact key.
    #[must_use]
    pub const fn key(self) -> TrialKey {
        self.key
    }

    /// The profile coordinate.
    #[must_use]
    pub const fn profile(self) -> TrialProfile {
        self.profile
    }
}

impl TrialId {
    /// Derive one trial's semantic identity from its complete preimage.
    ///
    /// Deterministic and total: every key under every profile names a trial.
    #[must_use]
    pub fn over(profiled: ProfiledTrial) -> Self {
        Self(ContentAddress::derived(
            TRIAL_IDENTITY_TAG,
            &trial_preimage(profiled),
        ))
    }

    /// Derive the identity of the trial one descriptor row declares.
    #[must_use]
    pub fn of_key(key: TrialKey, profile: TrialProfile) -> Self {
        Self::over(ProfiledTrial::of_key(key, profile))
    }

    /// The identity's address, for comparison and for rendering.
    #[must_use]
    pub const fn address(&self) -> &ContentAddress {
        &self.0
    }
}

impl TrialSite {
    /// Where one trial is written.
    #[must_use]
    pub const fn located(
        module_path: &'static str,
        file: &'static str,
        line: u32,
        name: &'static str,
    ) -> Self {
        Self {
            module_path,
            file,
            line,
            name,
        }
    }

    /// The module path the trial is declared under.
    #[must_use]
    pub const fn module_path(&self) -> &'static str {
        self.module_path
    }

    /// The file the trial is declared in.
    #[must_use]
    pub const fn file(&self) -> &'static str {
        self.file
    }

    /// The line the trial is declared on.
    #[must_use]
    pub const fn line(&self) -> u32 {
        self.line
    }

    /// The trial's display name — what a person filters on.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }
}

// ---------------------------------------------------------------------------
// The revision rail.
// ---------------------------------------------------------------------------

impl RowRevisionId {
    /// Derive one row revision identity from the row's canonical bytes.
    ///
    /// The bytes are the descriptor home's — the complete row, as that home
    /// encoded it when the row was born — and this seat derives the identity
    /// from them rather than encoding a row it does not own.
    ///
    /// Total, and typed on the bytes rather than on a slice: a caller cannot
    /// offer material that is not a row's own encoding, and holding those bytes
    /// is holding everything the derivation needs, so there is nothing left here
    /// to refuse.
    #[must_use]
    pub fn over(canonical_row: &CanonicalRowBytes) -> Self {
        Self(ContentAddress::derived(
            ROW_REVISION_TAG,
            canonical_row.as_bytes(),
        ))
    }

    /// The identity's address.
    #[must_use]
    pub const fn address(&self) -> &ContentAddress {
        &self.0
    }
}

impl SubjectRevisionId {
    /// The subject revision one attachment bound, under the record
    /// vocabulary's name for it.
    ///
    /// The binding is the authority: the address crosses unchanged, and nothing
    /// is derived a second time here.
    #[must_use]
    pub const fn of_binding(binding: RevisionBinding) -> Self {
        Self(binding.revision())
    }

    /// The identity's address.
    #[must_use]
    pub const fn address(&self) -> &ContentAddress {
        &self.0
    }
}

impl CheckRevisionId {
    /// The check revision one attachment bound, under the record vocabulary's
    /// name for it.
    #[must_use]
    pub const fn of_binding(binding: RevisionBinding) -> Self {
        Self(binding.revision())
    }

    /// The identity's address.
    #[must_use]
    pub const fn address(&self) -> &ContentAddress {
        &self.0
    }
}

// ---------------------------------------------------------------------------
// The execution rail.
// ---------------------------------------------------------------------------

impl TargetTriple {
    /// The target triple the run declared.
    #[must_use]
    pub fn declared(spelling: &str) -> Self {
        Self(spelling.to_owned())
    }

    /// The declared spelling.
    #[must_use]
    pub fn spelling(&self) -> &str {
        &self.0
    }
}

impl ToolchainIdentity {
    /// The toolchain identity the run declared.
    #[must_use]
    pub fn declared(spelling: &str) -> Self {
        Self(spelling.to_owned())
    }

    /// The declared spelling.
    #[must_use]
    pub fn spelling(&self) -> &str {
        &self.0
    }
}

impl TargetBinding {
    /// The target and toolchain one execution ran under.
    #[must_use]
    pub fn bound(target: TargetTriple, toolchain: ToolchainIdentity) -> Self {
        Self { target, toolchain }
    }

    /// The compilation target.
    #[must_use]
    pub const fn target(&self) -> &TargetTriple {
        &self.target
    }

    /// The toolchain.
    #[must_use]
    pub const fn toolchain(&self) -> &ToolchainIdentity {
        &self.toolchain
    }
}

impl CaseBudget {
    /// The case budget the invocation declared.
    #[must_use]
    pub const fn declared(cases: u32) -> Self {
        Self(cases)
    }

    /// The declared number of cases.
    #[must_use]
    pub const fn cases(self) -> u32 {
        self.0
    }
}

impl ByteBudget {
    /// The input-byte budget the invocation declared.
    #[must_use]
    pub const fn declared(bytes: u64) -> Self {
        Self(bytes)
    }

    /// The declared number of bytes.
    #[must_use]
    pub const fn bytes(self) -> u64 {
        self.0
    }
}

impl TimeBudget {
    /// The time budget the invocation declared, in nanoseconds.
    #[must_use]
    pub const fn declared(nanoseconds: u64) -> Self {
        Self(nanoseconds)
    }

    /// The declared bound, in nanoseconds.
    #[must_use]
    pub const fn nanoseconds(self) -> u64 {
        self.0
    }
}

impl InvocationProfile {
    /// The invocation's conclusion-relevant facts.
    #[must_use]
    pub const fn declared(cases: CaseBudget, bytes: ByteBudget, time: TimeBudget) -> Self {
        Self { cases, bytes, time }
    }

    /// The case budget.
    #[must_use]
    pub const fn cases(self) -> CaseBudget {
        self.cases
    }

    /// The input-byte budget.
    #[must_use]
    pub const fn bytes(self) -> ByteBudget {
        self.bytes
    }

    /// The time budget.
    #[must_use]
    pub const fn time(self) -> TimeBudget {
        self.time
    }
}

impl ExecutionKey {
    /// The key one execution of one trial is looked up under.
    ///
    /// The target binding is taken unconditionally: there is no road here that
    /// builds a key without it, so a cross-target hit is not a policy the
    /// harness applies but a value it cannot construct.
    #[must_use]
    pub fn over(
        trial: TrialId,
        subject: SubjectRevisionId,
        check: CheckRevisionId,
        invocation: InvocationProfile,
        target: TargetBinding,
    ) -> Self {
        Self {
            trial,
            subject,
            check,
            invocation,
            target,
        }
    }

    /// The trial this key executes.
    #[must_use]
    pub const fn trial(&self) -> TrialId {
        self.trial
    }

    /// The subject revision it stood on.
    #[must_use]
    pub const fn subject(&self) -> SubjectRevisionId {
        self.subject
    }

    /// The check revision it stood on.
    #[must_use]
    pub const fn check(&self) -> CheckRevisionId {
        self.check
    }

    /// The invocation profile it ran under.
    #[must_use]
    pub const fn invocation(&self) -> InvocationProfile {
        self.invocation
    }

    /// The target and toolchain it ran on.
    #[must_use]
    pub const fn target(&self) -> &TargetBinding {
        &self.target
    }

    /// The key's address, derived from its parts.
    #[must_use]
    pub fn address(&self) -> ContentAddress {
        ContentAddress::derived(
            EXECUTION_KEY_TAG,
            &execution_key_preimage(
                self.trial,
                self.subject,
                self.check,
                self.invocation,
                &self.target,
            ),
        )
    }
}

// ---------------------------------------------------------------------------
// The replay account.
// ---------------------------------------------------------------------------

impl GenerationProfile {
    /// The generation profile under its declared name and version.
    #[must_use]
    pub const fn declared(name: &'static str, version: u32) -> Self {
        Self { name, version }
    }

    /// The declared name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        self.name
    }

    /// The declared version.
    #[must_use]
    pub const fn version(self) -> u32 {
        self.version
    }
}

impl MinimizationProfile {
    /// The minimization profile under its declared name and version.
    #[must_use]
    pub const fn declared(name: &'static str, version: u32) -> Self {
        Self { name, version }
    }

    /// The declared name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        self.name
    }

    /// The declared version.
    #[must_use]
    pub const fn version(self) -> u32 {
        self.version
    }
}

impl ReplayCapsule {
    /// The reproduction account of one execution.
    ///
    /// The posture is taken rather than computed here: it is the attachment's
    /// meet image, and the reading that produces it is the one owning statement
    /// about postures.
    #[must_use]
    pub fn recorded(
        key: ExecutionKey,
        input: Vec<u8>,
        generation: GenerationProfile,
        minimization: MinimizationProfile,
        schema: GeneratedSupportSchemaId,
        posture: ReplayPosture,
    ) -> Self {
        Self {
            key,
            input,
            generation,
            minimization,
            schema,
            posture,
        }
    }

    /// The execution this capsule reproduces.
    #[must_use]
    pub const fn key(&self) -> &ExecutionKey {
        &self.key
    }

    /// The exact input bytes the execution was handed.
    #[must_use]
    pub fn input(&self) -> &[u8] {
        &self.input
    }

    /// The generation profile that produced the input.
    #[must_use]
    pub const fn generation(&self) -> GenerationProfile {
        self.generation
    }

    /// The minimization profile that reduced the input.
    #[must_use]
    pub const fn minimization(&self) -> MinimizationProfile {
        self.minimization
    }

    /// The generated-support schema identity the input conforms to.
    #[must_use]
    pub const fn schema(&self) -> GeneratedSupportSchemaId {
        self.schema
    }

    /// What reproducing this capsule can claim.
    #[must_use]
    pub const fn posture(&self) -> ReplayPosture {
        self.posture
    }

    /// The capsule's own identity, derived from its complete preimage.
    #[must_use]
    pub fn identity(&self) -> ContentAddress {
        ContentAddress::derived(REPLAY_CAPSULE_TAG, &replay_capsule_preimage(self))
    }
}

// ---------------------------------------------------------------------------
// Findings.
// ---------------------------------------------------------------------------

impl FindingCause {
    /// Cite one cause by the declared identity pair its home wrote down.
    #[must_use]
    pub const fn named(family: &'static str, local: &'static str) -> Self {
        Self { family, local }
    }

    /// The cause family.
    #[must_use]
    pub const fn family(self) -> &'static str {
        self.family
    }

    /// The local key inside that family.
    #[must_use]
    pub const fn local(self) -> &'static str {
        self.local
    }
}

impl FindingLocation {
    /// Where a refusal was raised.
    #[must_use]
    pub const fn at(file: &'static str, line: u32) -> Self {
        Self { file, line }
    }

    /// The file.
    #[must_use]
    pub const fn file(self) -> &'static str {
        self.file
    }

    /// The line.
    #[must_use]
    pub const fn line(self) -> u32 {
        self.line
    }
}

impl ForeignText {
    /// Admit foreign material at the bound, recording what was cut and whether
    /// rendering it loses bytes.
    ///
    /// Total: material past the bound is cut rather than refused, and the cut
    /// is a typed fact on the value, so nothing shows a reader a shortened
    /// rendering that looks whole. A cut can land mid-sequence, which is
    /// exactly what [`TextFidelity::LossyReplacement`] states.
    #[must_use]
    pub fn admitted(material: &[u8]) -> Self {
        let bytes: Vec<u8> = material
            .iter()
            .copied()
            .take(FOREIGN_TEXT_MAX_BYTES)
            .collect();
        let truncation = if material.len() > FOREIGN_TEXT_MAX_BYTES {
            Truncation::TruncatedAt {
                admitted: bytes.len(),
                offered: material.len(),
            }
        } else {
            Truncation::Complete
        };
        let fidelity = if core::str::from_utf8(&bytes).is_ok() {
            TextFidelity::Exact
        } else {
            TextFidelity::LossyReplacement
        };
        Self {
            bytes,
            truncation,
            fidelity,
        }
    }

    /// The admitted bytes.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Whether everything offered was admitted.
    #[must_use]
    pub const fn truncation(&self) -> Truncation {
        self.truncation
    }

    /// Whether rendering the admitted bytes as text loses anything.
    #[must_use]
    pub const fn fidelity(&self) -> TextFidelity {
        self.fidelity
    }

    /// The rendering, for a caller to SHOW a person.
    ///
    /// The one lawful use of the bytes, and a one-way road: nothing in the
    /// harness reads this back.
    #[must_use]
    pub fn shown(&self) -> String {
        String::from_utf8_lossy(&self.bytes).into_owned()
    }
}

impl TrialFinding {
    /// The typed refusal one check returned.
    #[must_use]
    pub fn established(
        class: FailureClass,
        cause: FindingCause,
        located: FindingLocation,
        foreign: Option<ForeignText>,
    ) -> Self {
        Self {
            class,
            cause,
            located,
            foreign,
        }
    }

    /// The normalized failure class.
    #[must_use]
    pub const fn class(&self) -> FailureClass {
        self.class
    }

    /// The typed cause.
    #[must_use]
    pub const fn cause(&self) -> FindingCause {
        self.cause
    }

    /// Where the refusal was raised.
    #[must_use]
    pub const fn located(&self) -> FindingLocation {
        self.located
    }

    /// The foreign text the finding carried in, where there was any.
    #[must_use]
    pub fn foreign(&self) -> Option<&ForeignText> {
        self.foreign.as_ref()
    }
}

impl Fingerprint {
    /// The failure identity of one finding under one trial.
    #[must_use]
    pub const fn of(trial: TrialId, finding: &TrialFinding) -> Self {
        Self {
            trial,
            cause: finding.cause,
            class: finding.class,
        }
    }

    /// The failure identity of one cause and class under one trial.
    #[must_use]
    pub const fn over(trial: TrialId, cause: FindingCause, class: FailureClass) -> Self {
        Self {
            trial,
            cause,
            class,
        }
    }

    /// The trial the failure was found under.
    #[must_use]
    pub const fn trial(self) -> TrialId {
        self.trial
    }

    /// The typed cause.
    #[must_use]
    pub const fn cause(self) -> FindingCause {
        self.cause
    }

    /// The normalized failure class.
    #[must_use]
    pub const fn class(self) -> FailureClass {
        self.class
    }

    /// The fingerprint's address, derived from its three coordinates.
    #[must_use]
    pub fn address(self) -> ContentAddress {
        ContentAddress::derived(
            FINGERPRINT_TAG,
            &fingerprint_preimage(self.trial, self.cause, self.class),
        )
    }
}

// ---------------------------------------------------------------------------
// One execution's record.
// ---------------------------------------------------------------------------

impl RecordedDuration {
    /// The duration the caller measured, in nanoseconds.
    #[must_use]
    pub const fn recorded(nanoseconds: u64) -> Self {
        Self(nanoseconds)
    }

    /// The recorded nanoseconds.
    #[must_use]
    pub const fn nanoseconds(self) -> u64 {
        self.0
    }
}

impl TrialReport {
    /// One execution's record.
    #[must_use]
    pub fn recorded(
        trial: TrialId,
        site: TrialSite,
        attempt: RunAttempt,
        elapsed: RecordedDuration,
    ) -> Self {
        Self {
            trial,
            site,
            attempt,
            elapsed,
        }
    }

    /// The trial's semantic identity.
    #[must_use]
    pub const fn trial(&self) -> TrialId {
        self.trial
    }

    /// Where the trial is written.
    #[must_use]
    pub const fn site(&self) -> TrialSite {
        self.site
    }

    /// What became of the attempt.
    #[must_use]
    pub const fn attempt(&self) -> &RunAttempt {
        &self.attempt
    }

    /// How long it took, as recorded.
    #[must_use]
    pub const fn elapsed(&self) -> RecordedDuration {
        self.elapsed
    }
}

impl SelectionDisposition {
    /// Selected, carrying its execution record.
    #[must_use]
    pub fn selected(report: TrialReport) -> Self {
        Self::Selected(Box::new(report))
    }

    /// Not selected, for a stated reason.
    #[must_use]
    pub const fn not_selected(reason: NotSelectedReason) -> Self {
        Self::NotSelected { reason }
    }

    /// The execution record, where the invocation selected the trial.
    #[must_use]
    pub fn report(&self) -> Option<&TrialReport> {
        match self {
            Self::Selected(report) => Some(report.as_ref()),
            Self::NotSelected { reason: _ } => None,
        }
    }

    /// Whether this row of the denominator was actually exercised.
    #[must_use]
    pub fn exercise(&self) -> Exercise {
        match self {
            Self::Selected(report) => match report.attempt() {
                RunAttempt::Executed(_) => Exercise::Exercised,
                RunAttempt::SkippedWithReason(_)
                | RunAttempt::TimedOut(_)
                | RunAttempt::InfrastructureFailed(_) => Exercise::Unexercised,
            },
            Self::NotSelected { reason: _ } => Exercise::Unexercised,
        }
    }

    /// The normalized outcome, for a comparison to read.
    #[must_use]
    pub fn outcome(&self) -> OutcomeClass {
        match self {
            Self::Selected(report) => match report.attempt() {
                RunAttempt::Executed(TrialConclusion::Passed) => OutcomeClass::Passed,
                RunAttempt::Executed(TrialConclusion::Refused(finding)) => {
                    OutcomeClass::Refused(finding.class())
                }
                RunAttempt::SkippedWithReason(reason) => OutcomeClass::Skipped(*reason),
                RunAttempt::TimedOut(_) => OutcomeClass::TimedOut,
                RunAttempt::InfrastructureFailed(fault) => {
                    OutcomeClass::InfrastructureFailed(*fault)
                }
            },
            Self::NotSelected { reason } => OutcomeClass::NotSelected(*reason),
        }
    }
}

impl TrialAccounting {
    /// One row of the denominator, and what this invocation did about it.
    #[must_use]
    pub fn recorded(
        trial: TrialId,
        row: RowRevisionId,
        claim: ClaimRef,
        disposition: SelectionDisposition,
    ) -> Self {
        Self {
            trial,
            row,
            claim,
            disposition,
        }
    }

    /// The trial's semantic identity.
    #[must_use]
    pub const fn trial(&self) -> TrialId {
        self.trial
    }

    /// The authored row's revision identity.
    #[must_use]
    pub const fn row(&self) -> RowRevisionId {
        self.row
    }

    /// The claim the row serves.
    #[must_use]
    pub const fn claim(&self) -> ClaimRef {
        self.claim
    }

    /// What the invocation did about it.
    #[must_use]
    pub const fn disposition(&self) -> &SelectionDisposition {
        &self.disposition
    }
}

impl SelectionOutcome {
    /// Read one run's selection against what the run expected of it.
    ///
    /// A total map over two facts the run already holds: how many rows the
    /// selection named, and what the caller declared beforehand. Nothing else
    /// enters it, and the same pair always reads the same way.
    #[must_use]
    pub const fn read(expectation: SelectionExpectation, selected: usize) -> Self {
        if selected > 0_usize {
            return Self::Satisfied;
        }
        match expectation {
            SelectionExpectation::AtLeastOne => Self::UnsatisfiedByEmptySelection,
            SelectionExpectation::AllowEmpty(reason) => Self::EmptyAsStated(reason),
        }
    }
}

impl RunReport {
    /// One run's complete-table accounting.
    ///
    /// The census arrives complete — one entry per row of the table the run
    /// stood over — because the denominator is the table itself and a report
    /// that dropped its unselected rows would be stating a smaller world than
    /// the one it ran in.
    ///
    /// The selection outcome arrives read rather than computed here: what a run
    /// expected of its selection is the engine's parameter, and this seat
    /// records the answer instead of re-deriving it from a census that cannot
    /// state an expectation.
    #[must_use]
    pub fn recorded(
        census: Vec<TrialAccounting>,
        posture: TablePosture,
        selection: SelectionOutcome,
        invocation: InvocationProfile,
    ) -> Self {
        Self {
            census,
            posture,
            selection,
            invocation,
        }
    }

    /// What this run's selection matched, read against what it expected.
    #[must_use]
    pub const fn selection(&self) -> SelectionOutcome {
        self.selection
    }

    /// Every row of the denominator, with its disposition.
    #[must_use]
    pub fn census(&self) -> &[TrialAccounting] {
        &self.census
    }

    /// How many rows the run was stated over.
    #[must_use]
    pub fn denominator(&self) -> usize {
        self.census.len()
    }

    /// Which table the run stood over.
    #[must_use]
    pub const fn posture(&self) -> TablePosture {
        self.posture
    }

    /// The invocation profile the run ran under.
    #[must_use]
    pub const fn invocation(&self) -> InvocationProfile {
        self.invocation
    }
}

// ---------------------------------------------------------------------------
// The comparison.
// ---------------------------------------------------------------------------

impl CensusDelta {
    /// How the denominator moved between two runs.
    #[must_use]
    pub fn between(before: usize, after: usize) -> Self {
        let direction = match after.cmp(&before) {
            Ordering::Greater => CensusDirection::Grew,
            Ordering::Equal => CensusDirection::Unchanged,
            Ordering::Less => CensusDirection::Shrank,
        };
        Self {
            before,
            after,
            direction,
        }
    }

    /// The baseline's denominator.
    #[must_use]
    pub const fn before(self) -> usize {
        self.before
    }

    /// The current report's denominator.
    #[must_use]
    pub const fn after(self) -> usize {
        self.after
    }

    /// Which way it moved.
    #[must_use]
    pub const fn direction(self) -> CensusDirection {
        self.direction
    }
}

impl RowRevisionChange {
    /// One trial whose authored row was edited between the two runs.
    #[must_use]
    pub const fn between(trial: TrialId, before: RowRevisionId, after: RowRevisionId) -> Self {
        Self {
            trial,
            before,
            after,
        }
    }

    /// The trial.
    #[must_use]
    pub const fn trial(self) -> TrialId {
        self.trial
    }

    /// The row revision the baseline recorded.
    #[must_use]
    pub const fn before(self) -> RowRevisionId {
        self.before
    }

    /// The row revision the current report records.
    #[must_use]
    pub const fn after(self) -> RowRevisionId {
        self.after
    }
}

impl ConclusionFlip {
    /// One trial whose outcome differs between the two runs.
    #[must_use]
    pub const fn between(trial: TrialId, before: OutcomeClass, after: OutcomeClass) -> Self {
        Self {
            trial,
            before,
            after,
        }
    }

    /// The trial.
    #[must_use]
    pub const fn trial(self) -> TrialId {
        self.trial
    }

    /// What the baseline recorded.
    #[must_use]
    pub const fn before(self) -> OutcomeClass {
        self.before
    }

    /// What the current report records.
    #[must_use]
    pub const fn after(self) -> OutcomeClass {
        self.after
    }
}

impl ReportDiff {
    /// The difference between two reports.
    #[must_use]
    pub fn stated(
        added: Vec<TrialId>,
        removed: Vec<TrialId>,
        revised: Vec<RowRevisionChange>,
        flips: Vec<ConclusionFlip>,
        census: CensusDelta,
    ) -> Self {
        Self {
            added,
            removed,
            revised,
            flips,
            census,
        }
    }

    /// Trials the current report has and the baseline did not.
    #[must_use]
    pub fn added(&self) -> &[TrialId] {
        &self.added
    }

    /// Trials the baseline had and the current report does not.
    #[must_use]
    pub fn removed(&self) -> &[TrialId] {
        &self.removed
    }

    /// Trials in both runs whose authored row was edited.
    #[must_use]
    pub fn revised(&self) -> &[RowRevisionChange] {
        &self.revised
    }

    /// Trials in both runs whose outcome differs.
    #[must_use]
    pub fn flips(&self) -> &[ConclusionFlip] {
        &self.flips
    }

    /// How the denominator moved.
    #[must_use]
    pub const fn census(&self) -> CensusDelta {
        self.census
    }
}

// ---------------------------------------------------------------------------
// The coverage reading.
// ---------------------------------------------------------------------------

impl ClaimExercise {
    /// One claim's counts over the denominator.
    #[must_use]
    pub const fn counted(claim: ClaimRef, exercised: usize, unexercised: usize) -> Self {
        Self {
            claim,
            exercised,
            unexercised,
        }
    }

    /// The claim.
    #[must_use]
    pub const fn claim(self) -> ClaimRef {
        self.claim
    }

    /// How many of the claim's rows executed.
    #[must_use]
    pub const fn exercised(self) -> usize {
        self.exercised
    }

    /// How many of the claim's rows did not.
    #[must_use]
    pub const fn unexercised(self) -> usize {
        self.unexercised
    }

    /// How many rows the claim owns in the denominator.
    #[must_use]
    pub const fn denominator(self) -> usize {
        self.exercised.saturating_add(self.unexercised)
    }
}

impl ClaimCoverage {
    /// The reading, one entry per claim the denominator names.
    #[must_use]
    pub fn read(entries: Vec<ClaimExercise>) -> Self {
        Self { entries }
    }

    /// Every claim the denominator names, with its counts.
    #[must_use]
    pub fn entries(&self) -> &[ClaimExercise] {
        &self.entries
    }
}
