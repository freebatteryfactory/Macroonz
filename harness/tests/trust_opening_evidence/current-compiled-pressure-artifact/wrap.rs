//! The compiled-mutation lane: the console grammar of one wrapped backend, the defensive parser that reads it, and the witness runs a reading is planned into.
//!
//! The backend is external and it runs outside the wall — it mutates real source and invokes the test command itself.
//! Nothing here executes anything: this file reads text a caller already holds, and plans runs the one report engine performs.
//!
//! # The line grammar this parser reads
//!
//! The grammar is line-oriented and the reading is defensive: a line is read only when it matches a shape stated here, and every other line becomes an [`UnparsedLine`] that travels with the reading rather than being dropped.
//!
//! - A roster line is `Found <count> mutant…`: the word `Found`, a decimal count, and a third word beginning `mutant`.
//! - A baseline line is an outcome word followed by `Unmutated baseline…`, and only `ok` reads as a qualified baseline.
//! - A mutant line is an outcome word, then `<file>:<line>:<column>:`, then the backend's own damage text.
//!
//! The coordinate's file part is everything before the last two colon-separated fields, so a drive-lettered path stays whole.
//! The outcome words are `caught`, `missed`, `unviable`, `timeout`, and `failed`, matched without regard to case.
//!
//! # What a reading may claim
//!
//! Every reading is stated under an [`AdapterProfile`] naming the backend, the version posture the running party states, the output it was taken from, and this adapter's own grammar version.
//! Three of those four are this file's own facts, so [`console_profile`] states them and only the backend version is the caller's word.
//!
//! The backend says which of its own mutants its command rejected and says nothing about whether a damaged expression was ever reached, so every mutant read here carries [`ActivationDisposition::UnobservableUnderBackend`].
//! Two consequences follow, both structural: a kill under this lane asserts witness rejection and never observed activation, and a non-kill can never earn survived — it is inconclusive, and [`MutationRun::non_kills`](crate::muterprater::MutationRun::non_kills) is the roster a reader means by "what got through".
//! The same fact at run width is the profile's [`ClaimCeiling::WitnessRejection`](crate::muterprater::ClaimCeiling::WitnessRejection), and a reading whose run carries a survivor is refused rather than believed.
//!
//! A kill's rejection is the backend's word ([`IntendedRejection::ReportedByBackend`]), because it named neither a trial nor a cause, so no fingerprint exists for it.
//!
//! # What stands behind the grammar
//!
//! The grammar is an inspectable assumption about one tool's rendering, and it qualifies nothing until a party states that these shapes were checked against real output of the exact backend version a reading names.
//! That statement is what [`AdapterQualification`](crate::muterprater::AdapterQualification) carries, and [`CompiledSuitePressure`](crate::muterprater::CompiledSuitePressure) is a lawful kill read out of a current-source-qualified artifact carrying that exact profile.
//! A different machine-readable backend surface, if adopted, earns its own profile and its own qualification rather than inheriting this grammar's standing.
//!
//! # The caller-supplied seams
//!
//! External mutants arrive as source coordinates rather than as claims, so the reading from a coordinate to its owning claim is the caller's ([`OwnerLookup`]), and so is the reading from damage text to operator family ([`FamilyLookup`]).
//! Neither answer is invented here: an unanswered lookup produces [`MappingPosture::OwnerUnmapped`] and [`FamilyAttribution::OutsideTheBank`], and the witness selection widens accordingly.

use super::super::types::{
    ActivationDisposition, BaselineAxis, BaselineQualification, EquivalenceAxis, ExecutionAxis,
    FamilyAttribution, InconclusiveCause, IntendedRejection, MappingPosture, MaterializationAxis,
    MutantId, MutationIdentity, MutationReport, MutationRun, MutationSite, MutationTarget,
    PlanRefusal, PlannedDamage, PlannedRun, PressureLane, ProofPlan, ScopedInvocation,
    SourceCoordinate,
};
use super::types::{
    AdapterProfile, AnnouncedRoster, ArtifactManifestRefusal, BackendOutputId,
    BackendVersionPosture, CompiledSuiteArtifactManifest, FamilyLookup, GrammarVersion,
    MutationBackendInvocation, MutationSourceRevision, OwnerLookup, ReadingSource, UnparsedLine,
    WrapOutcomeWord, WrapReading, WrapRefusal, WrappedBackend,
};
use crate::report::ForeignText;
use crate::runner::Selection;
use std::collections::{BTreeMap, BTreeSet};

/// The version of the console line grammar this file's page states.
///
/// It moves when and only when those line shapes move; the backend's version is the running party's word, and the mutant encoding's version is the identity's.
const CONSOLE_GRAMMAR_VERSION: u32 = 1;

/// The activation every mutant this lane reads carries.
///
/// A fact about the backend: it mutates source and runs a command, and nothing in its output states whether a damaged expression was reached.
const WRAP_ACTIVATION: ActivationDisposition = ActivationDisposition::UnobservableUnderBackend;

/// The equivalence every mutant this lane reads carries.
///
/// The backend puts no equivalence question, so nothing was assessed.
const WRAP_EQUIVALENCE: EquivalenceAxis = EquivalenceAxis::NotAssessed;

/// The prefix a baseline line's remainder begins with.
const BASELINE_MARKER: &str = "Unmutated baseline";

/// The word a roster line opens with.
const ROSTER_MARKER: &str = "Found";

/// The prefix a roster line's third word begins with.
const ROSTER_SUBJECT: &str = "mutant";

/// The outcome word a qualified baseline is stated under.
const BASELINE_QUALIFIED_WORD: &str = "ok";

/// The backend's outcome words, and what each one states about one mutant.
const OUTCOME_WORDS: [(&str, WrapOutcomeWord); 5] = [
    ("caught", WrapOutcomeWord::Caught),
    ("missed", WrapOutcomeWord::Missed),
    ("unviable", WrapOutcomeWord::Unviable),
    ("timeout", WrapOutcomeWord::TimedOut),
    ("failed", WrapOutcomeWord::ToolFailed),
];

impl WrappedBackend {
    /// The backend's own name.
    ///
    /// A projection: a reader of a profile names the tool through it, and no decision anywhere consults it.
    #[must_use]
    pub const fn spelling(self) -> &'static str {
        match self {
            Self::CargoMutants => "cargo-mutants",
        }
    }
}

/// What one line of the backend's output states.
///
/// Private to this reading: the public record is [`WrapReading`], and a shape the parser uses on the way there is not vocabulary anybody else writes against.
enum LineReading<'line> {
    /// The backend announced its roster count.
    Roster(u32),
    /// The backend reported the unmutated baseline.
    Baseline(BaselineAxis),
    /// The backend reported one mutant.
    Mutant {
        /// The outcome word the line opens with.
        word: WrapOutcomeWord,
        /// Where the damage was placed.
        coordinate: SourceCoordinate,
        /// The backend's own damage text.
        damage: String,
        /// The whole line, for the rejection a kill carries.
        line: &'line str,
    },
    /// The parser does not read this line.
    Unread,
}

/// What a console reading of the wrapped backend is stated under.
///
/// The backend, the output, and the grammar are this file's own facts, so the only thing left for a caller to state is which version of the backend produced the text.
#[must_use]
pub fn console_profile(version: BackendVersionPosture) -> AdapterProfile {
    AdapterProfile::stated(
        WrappedBackend::CargoMutants,
        version,
        ReadingSource::ConsoleStream,
        GrammarVersion::adapter(CONSOLE_GRAMMAR_VERSION),
    )
}

/// Read one backend's console output into this lane's record.
///
/// Two passes, and the order is the law: the baseline is established before any mutant line is read, so a kill can never be minted under a baseline the output did not qualify.
/// The second pass is total over the remaining lines — every one lands in a report or in the unparsed roster.
///
/// # Errors
///
/// Refuses an output stating no baseline, then a baseline that does not qualify, then a mutant line whose record the lawful-kill constructor refused, naming which line and what was refused.
pub fn read_output(
    text: &str,
    version: BackendVersionPosture,
    owner: OwnerLookup,
    family_lookup: FamilyLookup,
) -> Result<WrapReading, WrapRefusal> {
    let profile = console_profile(version);
    let baseline = read_baseline(text)?;
    let mut reports: Vec<MutationReport> = Vec::new();
    let mut unparsed: Vec<UnparsedLine> = Vec::new();
    let mut announced = AnnouncedRoster::Unstated;
    for (ordinal, line) in text.lines().enumerate() {
        match read_line(line) {
            LineReading::Roster(found) => announced = AnnouncedRoster::Stated(found),
            LineReading::Baseline(_) => {}
            LineReading::Mutant {
                word,
                coordinate,
                damage,
                line: whole,
            } => {
                let target = targeted(&coordinate, damage.as_bytes(), owner, family_lookup);
                reports.push(recorded(ordinal, word, target, whole, baseline)?);
            }
            LineReading::Unread => unparsed.push(UnparsedLine::unread(ordinal, line.as_bytes())),
        }
    }
    WrapReading::read(
        profile,
        MutationRun::recorded(baseline, reports),
        announced,
        unparsed,
    )
}

/// Read one persisted backend output into a typed custody manifest.
///
/// The invocation supplies backend, version, command, target, and toolchain once; the parser derives its profile from that invocation and the exact output bytes derive their own identity.
/// The source roster must name exactly the files named by parsed mutation reports, once each, and is returned in file order.
/// This operation records what its caller supplies and performs no process or filesystem observation.
///
/// # Errors
///
/// Refuses output the adapter cannot read, then duplicate source files, then a reported file missing from the source roster, then a source file named by no parsed mutation report.
pub fn read_artifact(
    text: &str,
    invocation: MutationBackendInvocation,
    sources: Vec<MutationSourceRevision>,
    owner: OwnerLookup,
    family_lookup: FamilyLookup,
) -> Result<CompiledSuiteArtifactManifest, ArtifactManifestRefusal> {
    let reading = match invocation.backend() {
        WrappedBackend::CargoMutants => read_output(
            text,
            BackendVersionPosture::Stated(invocation.version().clone()),
            owner,
            family_lookup,
        ),
    }
    .map_err(ArtifactManifestRefusal::Reading)?;
    let mut supplied = BTreeMap::new();
    for source in sources {
        let file = source.file().to_owned();
        if supplied.insert(file.clone(), source).is_some() {
            return Err(ArtifactManifestRefusal::DuplicateSource(file));
        }
    }
    let mut reported = BTreeSet::new();
    for report in reading.run().reports() {
        match report.target().site() {
            MutationSite::Reported(coordinate) => {
                reported.insert(coordinate.file());
            }
            MutationSite::Declared(_) => {
                return Err(ArtifactManifestRefusal::MutationSiteNotReported);
            }
        }
    }
    for file in reported.iter().copied() {
        if !supplied.contains_key(file) {
            return Err(ArtifactManifestRefusal::ReportedSourceMissing(
                file.to_owned(),
            ));
        }
    }
    for file in supplied.keys() {
        if !reported.contains(file.as_str()) {
            return Err(ArtifactManifestRefusal::SourceNotReported(file.to_owned()));
        }
    }
    Ok(CompiledSuiteArtifactManifest::recorded(
        invocation,
        BackendOutputId::derived(text.as_bytes()),
        supplied.into_values().collect(),
        reading,
    ))
}

/// The qualified baseline the output states, if it states one that qualifies.
///
/// # Errors
///
/// Refuses an output with no baseline line, then one whose baseline did not pass.
fn read_baseline(text: &str) -> Result<BaselineQualification, WrapRefusal> {
    for line in text.lines() {
        if let LineReading::Baseline(axis) = read_line(line) {
            return BaselineQualification::read(axis).map_err(WrapRefusal::BaselineNotQualified);
        }
    }
    Err(WrapRefusal::BaselineNotStated)
}

/// One mutant's record, composed from the backend's word and the baseline the output qualified.
///
/// Only `caught` reaches the lawful-kill constructor; every other word states which link of the chain did not hold.
fn recorded(
    ordinal: usize,
    word: WrapOutcomeWord,
    target: MutationTarget,
    line: &str,
    baseline: BaselineQualification,
) -> Result<MutationReport, WrapRefusal> {
    let axis = baseline.axis();
    let cause = match word {
        WrapOutcomeWord::Caught => {
            return MutationReport::killed(
                target,
                axis,
                MaterializationAxis::from(word),
                WRAP_ACTIVATION,
                ExecutionAxis::from(word),
                IntendedRejection::ReportedByBackend {
                    stated: ForeignText::admitted(line.as_bytes()),
                },
                WRAP_EQUIVALENCE,
            )
            .map_err(|cause| WrapRefusal::KillNotLawful { ordinal, cause });
        }
        WrapOutcomeWord::Missed => InconclusiveCause::UnobservableAndUnrejected,
        WrapOutcomeWord::Unviable | WrapOutcomeWord::ToolFailed => {
            InconclusiveCause::NotMaterialized
        }
        WrapOutcomeWord::TimedOut => InconclusiveCause::WitnessIncomplete,
    };
    Ok(unlearned(target, axis, word, cause))
}

/// One record that established nothing, over the axes the backend's word already fixed.
fn unlearned(
    target: MutationTarget,
    axis: BaselineAxis,
    word: WrapOutcomeWord,
    cause: InconclusiveCause,
) -> MutationReport {
    MutationReport::inconclusive(
        target,
        axis,
        MaterializationAxis::from(word),
        WRAP_ACTIVATION,
        ExecutionAxis::from(word),
        cause,
        WRAP_EQUIVALENCE,
    )
}

/// One target, over the two caller-supplied readings.
fn targeted(
    coordinate: &SourceCoordinate,
    damage: &[u8],
    owner: OwnerLookup,
    family: FamilyLookup,
) -> MutationTarget {
    let attribution = family(coordinate, damage).map_or(
        FamilyAttribution::OutsideTheBank,
        FamilyAttribution::Declared,
    );
    let posture = owner(coordinate).map_or(MappingPosture::OwnerUnmapped, MappingPosture::Mapped);
    MutationTarget::pressed(
        MutationIdentity::External(MutantId::over(coordinate, damage)),
        attribution,
        MutationSite::Reported(coordinate.clone()),
        posture,
    )
}

/// What one line states, under the grammar this file's page declares.
fn read_line(line: &str) -> LineReading<'_> {
    let mut tokens = line.split_whitespace();
    let Some(word) = tokens.next() else {
        return LineReading::Unread;
    };
    let rest = tokens.collect::<Vec<&str>>();
    if let Some(found) = roster_count(word, &rest) {
        return LineReading::Roster(found);
    }
    if rest.join(" ").starts_with(BASELINE_MARKER) {
        return LineReading::Baseline(baseline_axis(word));
    }
    mutant_reading(word, &rest, line)
}

/// The roster count a `Found <count> mutant…` line states.
fn roster_count(word: &str, rest: &[&str]) -> Option<u32> {
    if word != ROSTER_MARKER {
        return None;
    }
    let count = rest.first()?;
    let subject = rest.get(1)?;
    if !subject.starts_with(ROSTER_SUBJECT) {
        return None;
    }
    count.parse::<u32>().ok()
}

/// The baseline axis one outcome word states.
///
/// Only a clean pass qualifies, because no other word is the unchanged passing suite a kill stands on.
fn baseline_axis(word: &str) -> BaselineAxis {
    if word.eq_ignore_ascii_case(BASELINE_QUALIFIED_WORD) {
        BaselineAxis::Qualified
    } else {
        BaselineAxis::Failed
    }
}

/// The mutant reading a line carries, where it carries one.
fn mutant_reading<'line>(word: &str, rest: &[&str], line: &'line str) -> LineReading<'line> {
    let Some(outcome) = outcome_word(word) else {
        return LineReading::Unread;
    };
    let Some(coordinate_text) = rest.first() else {
        return LineReading::Unread;
    };
    let Some(coordinate) = read_coordinate(coordinate_text) else {
        return LineReading::Unread;
    };
    let damage = rest.get(1..).unwrap_or_default().join(" ");
    if damage.is_empty() {
        return LineReading::Unread;
    }
    LineReading::Mutant {
        word: outcome,
        coordinate,
        damage,
        line,
    }
}

/// The outcome word one token states, where it states one.
fn outcome_word(token: &str) -> Option<WrapOutcomeWord> {
    OUTCOME_WORDS
        .iter()
        .find(|(spelling, _)| token.eq_ignore_ascii_case(spelling))
        .map(|(_, word)| *word)
}

/// The coordinate one `<file>:<line>:<column>:` token states.
///
/// Cut from the right, so a path carrying its own colons stays whole in the file part.
fn read_coordinate(token: &str) -> Option<SourceCoordinate> {
    let body = token.strip_suffix(':').unwrap_or(token);
    let mut fields = body.rsplitn(3, ':');
    let column_text = fields.next()?;
    let line_text = fields.next()?;
    let file = fields.next()?;
    let column = column_text.parse::<u32>().ok()?;
    let line = line_text.parse::<u32>().ok()?;
    SourceCoordinate::reported(file, line, column).ok()
}

/// The selection one mutant's witness run is executed under.
///
/// A mapped target names the claim that owns its site, and the rows serving that claim are the ones worth running.
/// An unmapped target widens to the whole world — the conservative selection, because a narrower one would rest on a claim nobody established.
/// A selection narrows a run and never the denominator: the report a narrowed run writes still stands over every row of the complete table.
#[must_use]
pub fn mutant_scoped(target: &MutationTarget) -> Selection {
    match target.owning_claim() {
        Some(claim) => Selection::ByClaim(BTreeSet::from([claim])),
        None => Selection::All,
    }
}

/// Plan one compiled-mutation pass over the targets a reading recovered.
///
/// A pure function of its arguments that spends nothing: the plan lists every intended run with the selection it would use and the budget it would spend, so a caller reads the whole pass before the first mutant is pressed.
///
/// # Errors
///
/// Refuses a pass with no target, then one stating more runs than the scope's mutant budget admits.
pub fn plan_pass(
    targets: &[MutationTarget],
    scope: ScopedInvocation,
) -> Result<ProofPlan, PlanRefusal> {
    let budget = scope.budget();
    let runs: Vec<PlannedRun> = targets
        .iter()
        .map(|target| {
            PlannedRun::intended(
                PressureLane::CompiledMutation,
                target.identity(),
                PlannedDamage::BackendChosen,
                mutant_scoped(target),
                budget,
            )
        })
        .collect();
    ProofPlan::planned(scope, runs)
}
