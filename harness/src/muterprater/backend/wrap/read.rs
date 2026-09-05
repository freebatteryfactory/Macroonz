//! The read role: the console profile, the two readings of a backend's output, and the record and target each mutant line composes into.

use super::parse::{LineReading, read_line};
use crate::muterprater::backend::roster::{collected, matched};
use crate::muterprater::backend::types::{
    AdapterProfile, AnnouncedRoster, ArtifactManifestRefusal, BackendOutputId,
    BackendVersionPosture, CompiledSuiteArtifactManifest, FamilyLookup, GrammarVersion,
    MutationBackendInvocation, MutationSourceRevision, OwnerLookup, ReadingSource, UnparsedLine,
    WrapOutcomeWord, WrapReading, WrapRefusal, WrappedBackend,
};
use crate::muterprater::types::{
    ActivationDisposition, BaselineAxis, BaselineQualification, EquivalenceAxis, ExecutionAxis,
    FamilyAttribution, InconclusiveCause, IntendedRejection, MappingPosture, MaterializationAxis,
    MutantId, MutationIdentity, MutationReport, MutationRun, MutationSite, MutationTarget,
    SourceCoordinate,
};
use crate::report::ForeignText;
use std::collections::BTreeSet;

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
    let supplied = collected(sources, ArtifactManifestRefusal::DuplicateSource)?;
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
    matched(
        &supplied,
        &reported,
        ArtifactManifestRefusal::ReportedSourceMissing,
        ArtifactManifestRefusal::SourceNotReported,
    )?;
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
