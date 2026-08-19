//! The compiled-mutation lane: the vocabulary a wrapped backend's output is read
//! into, the defensive parser that reads it, and the mutant-scoped selection a
//! witness run is executed under.
//!
//! The backend is EXTERNAL and it runs outside the wall: it mutates the real
//! source and invokes the test command itself. Nothing in this file executes
//! anything. It reads text a caller already holds, and it plans runs the one
//! pure engine performs — this lane is a reader and a planner, never a second
//! runner.
//!
//! # The output grammar this parser reads
//!
//! The grammar is line-oriented and this reading is DEFENSIVE: a line is read
//! only when it matches a shape stated here, and every other line becomes an
//! [`UnparsedLine`] that travels with the reading rather than being dropped.
//!
//! - A ROSTER line is `Found <count> mutant…`: the word `Found`, a decimal
//!   count, and a third word beginning `mutant`.
//! - A BASELINE line is an outcome word followed by `Unmutated baseline…`. The
//!   word `ok` reads as a qualified baseline and every other word reads as a
//!   failed one, because a baseline that is anything but a clean pass is not a
//!   precondition anybody can mint a kill under.
//! - A MUTANT line is an outcome word, then `<file>:<line>:<column>:`, then the
//!   backend's own damage text. The coordinate's file part is everything before
//!   the last two colon-separated fields, so a drive-lettered path stays whole.
//!
//! The outcome words are `caught`, `missed`, `unviable`, `timeout`, and
//! `failed`, matched without regard to case.
//!
//! # What this lane can and cannot establish
//!
//! The backend states which of its own mutants its command rejected. It states
//! nothing about whether a damaged expression was ever REACHED, so every mutant
//! read here carries [`ActivationDisposition::UnobservableUnderBackend`] — a
//! fact about the backend, not about the damage. Two consequences follow and
//! both are structural rather than remembered: a kill under this lane asserts
//! witness rejection and never observed activation, and a non-kill can never
//! earn survived — it is inconclusive, and
//! [`MutationRun::non_kills`](super::MutationRun::non_kills) is the roster a
//! reader means by "what got through".
//!
//! The rejection a kill carries is the backend's WORD
//! ([`IntendedRejection::ReportedByBackend`]), because the backend named neither
//! a trial nor a cause: no fingerprint exists for it, and a proposal standing on
//! a kill is grounded on a demonstration this harness ran itself.
//!
//! # The two caller-supplied seams
//!
//! External mutants arrive as source coordinates, not as claims. The origin
//! graph is read on the generator side — a reading of the one join, never a
//! second structure — and it reaches this lane as [`OwnerLookup`]. The operator
//! family is a second reading for the same reason, [`FamilyLookup`]. Neither
//! answer is invented here: an unanswered lookup produces
//! [`MappingPosture::OwnerUnmapped`] and
//! [`FamilyAttribution::OutsideTheBank`], and the witness selection widens
//! accordingly.

use super::types::{
    ActivationDisposition, AnnouncedRoster, BaselineAxis, BaselineQualification, EquivalenceAxis,
    ExecutionAxis, FamilyAttribution, FamilyLookup, InconclusiveCause, IntendedRejection,
    MappingPosture, MaterializationAxis, MutantId, MutationIdentity, MutationReport, MutationRun,
    MutationSite, MutationTarget, OwnerLookup, PlanRefusal, PlannedDamage, PlannedRun, PressureLane,
    ProofPlan, ScopedInvocation, SourceCoordinate, UnparsedLine, WrapOutcomeWord, WrapReading,
    WrapRefusal,
};
use crate::report::ForeignText;
use crate::runner::Selection;
use std::collections::BTreeSet;

/// The activation every mutant this lane reads carries.
///
/// A fact about the BACKEND: it mutates source and runs a command, and nothing
/// in its output states whether a damaged expression was reached. Declared once
/// here so the ceiling is applied uniformly rather than decided per line.
const WRAP_ACTIVATION: ActivationDisposition = ActivationDisposition::UnobservableUnderBackend;

/// The equivalence every mutant this lane reads carries.
///
/// The backend puts no equivalence question, so nothing was assessed. Recording
/// anything else would be this lane answering a question nobody asked.
const WRAP_EQUIVALENCE: EquivalenceAxis = EquivalenceAxis::NotAssessed;

/// The prefix a baseline line's remainder begins with.
const BASELINE_MARKER: &str = "Unmutated baseline";

/// The word a roster line opens with.
const ROSTER_MARKER: &str = "Found";

/// The prefix a roster line's third word begins with.
const ROSTER_SUBJECT: &str = "mutant";

/// The outcome word a qualified baseline is stated under.
const BASELINE_QUALIFIED_WORD: &str = "ok";

/// What one line of the backend's output states.
///
/// Private to this reading: the public record is [`WrapReading`], and a shape
/// this parser uses on the way there is not vocabulary anybody else writes
/// against.
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

/// Read one compiled-mutation backend's output into this lane's record.
///
/// # Authority
///
/// Two passes, and the order is the law: the baseline is established BEFORE any
/// mutant line is read, so a kill can never be minted under a baseline the
/// output did not qualify. The second pass is total over the remaining lines —
/// every one of them lands in a report or in the unparsed roster.
///
/// # Errors
///
/// Refuses an output stating no baseline at all, then a baseline that does not
/// qualify, then a mutant line whose record the lawful-kill constructor refused
/// — the last carrying which line and what the constructor refused.
pub fn read_output(
    text: &str,
    owner: OwnerLookup,
    family: FamilyLookup,
) -> Result<WrapReading, WrapRefusal> {
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
                let target = targeted(&coordinate, damage.as_bytes(), owner, family);
                reports.push(recorded(ordinal, word, target, whole, baseline)?);
            }
            LineReading::Unread => unparsed.push(UnparsedLine::unread(ordinal, line.as_bytes())),
        }
    }
    Ok(WrapReading::read(
        MutationRun::recorded(baseline, reports),
        announced,
        unparsed,
    ))
}

/// The qualified baseline the output states, if it states one that qualifies.
///
/// # Errors
///
/// Refuses an output with no baseline line, then one whose baseline did not
/// pass.
fn read_baseline(text: &str) -> Result<BaselineQualification, WrapRefusal> {
    for line in text.lines() {
        if let LineReading::Baseline(axis) = read_line(line) {
            return BaselineQualification::read(axis).map_err(WrapRefusal::BaselineNotQualified);
        }
    }
    Err(WrapRefusal::BaselineNotStated)
}

/// One mutant's record, composed from the backend's word and the baseline the
/// output qualified.
fn recorded(
    ordinal: usize,
    word: WrapOutcomeWord,
    target: MutationTarget,
    line: &str,
    baseline: BaselineQualification,
) -> Result<MutationReport, WrapRefusal> {
    let axis = baseline.axis();
    let materialization = MaterializationAxis::from(word);
    let execution = ExecutionAxis::from(word);
    match word {
        WrapOutcomeWord::Caught => MutationReport::killed(
            target,
            axis,
            materialization,
            WRAP_ACTIVATION,
            execution,
            IntendedRejection::ReportedByBackend {
                stated: ForeignText::admitted(line.as_bytes()),
            },
            WRAP_EQUIVALENCE,
        )
        .map_err(|cause| WrapRefusal::KillNotLawful { ordinal, cause }),
        WrapOutcomeWord::Missed => Ok(MutationReport::inconclusive(
            target,
            axis,
            materialization,
            WRAP_ACTIVATION,
            execution,
            InconclusiveCause::UnobservableAndUnrejected,
            WRAP_EQUIVALENCE,
        )),
        WrapOutcomeWord::Unviable | WrapOutcomeWord::ToolFailed => {
            Ok(MutationReport::inconclusive(
                target,
                axis,
                materialization,
                WRAP_ACTIVATION,
                execution,
                InconclusiveCause::NotMaterialized,
                WRAP_EQUIVALENCE,
            ))
        }
        WrapOutcomeWord::TimedOut => Ok(MutationReport::inconclusive(
            target,
            axis,
            materialization,
            WRAP_ACTIVATION,
            execution,
            InconclusiveCause::WitnessIncomplete,
            WRAP_EQUIVALENCE,
        )),
    }
}

/// One target, over the two caller-supplied readings.
fn targeted(
    coordinate: &SourceCoordinate,
    damage: &[u8],
    owner: OwnerLookup,
    family: FamilyLookup,
) -> MutationTarget {
    let attribution = family(coordinate, damage)
        .map_or(FamilyAttribution::OutsideTheBank, FamilyAttribution::Declared);
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
/// Only a clean pass qualifies. Every other word — a failure, a timeout, a
/// tooling fault — reads as a failed baseline, because none of them is the
/// unchanged passing suite a kill stands on.
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
    if token.eq_ignore_ascii_case("caught") {
        return Some(WrapOutcomeWord::Caught);
    }
    if token.eq_ignore_ascii_case("missed") {
        return Some(WrapOutcomeWord::Missed);
    }
    if token.eq_ignore_ascii_case("unviable") {
        return Some(WrapOutcomeWord::Unviable);
    }
    if token.eq_ignore_ascii_case("timeout") {
        return Some(WrapOutcomeWord::TimedOut);
    }
    if token.eq_ignore_ascii_case("failed") {
        return Some(WrapOutcomeWord::ToolFailed);
    }
    None
}

/// The coordinate one `<file>:<line>:<column>:` token states.
///
/// Cut from the right, so a path carrying its own colons stays whole in the file
/// part.
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
/// # Authority
///
/// The one join, over a shape the rows already carry: a mapped target names the
/// claim that owns its site, and the rows serving that claim are the ones worth
/// running. An unmapped target widens to the whole world — the CONSERVATIVE
/// selection, chosen because a narrower one would rest on a claim nobody
/// established.
///
/// # Nonclaims
///
/// A selection narrows a RUN and never the denominator. The report a narrowed
/// run writes still stands over every row of the complete table.
#[must_use]
pub fn mutant_scoped(target: &MutationTarget) -> Selection {
    match target.owning_claim() {
        Some(claim) => Selection::ByClaim(BTreeSet::from([claim])),
        None => Selection::All,
    }
}

/// Plan one compiled-mutation pass over the targets a reading recovered.
///
/// # Authority
///
/// A pure function of its arguments, and it spends nothing: the plan lists every
/// intended run with the selection it would use and the budget it would spend,
/// so a caller reads the whole pass before the first mutant is pressed.
///
/// # Errors
///
/// Refuses a pass with no target, then one stating more runs than the scope's
/// mutant budget admits.
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
