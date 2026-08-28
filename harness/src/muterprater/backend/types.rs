//! Wrapped-backend readings, artifact custody, adapter qualification, and generic suite pressure.

use crate::descriptor::ClaimRef;
use crate::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use crate::muterprater::{
    BaselinePrecondition, KillRefusal, MutationReport, MutationRun, MutationVerdict,
    OperatorFamilyRef, SourceCoordinate,
};
use crate::report::{ForeignText, TargetBinding};
#[path = "type_guard.rs"]
mod guard;

// ---------------------------------------------------------------------------
// What a wrapped backend's output is read into.
// ---------------------------------------------------------------------------

/// Which external mutation backend one reading was taken from.
///
/// One backend because one line grammar: a second backend is a second grammar and a second arm beside the line laws that read it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WrappedBackend {
    /// The `cargo-mutants` backend, which mutates real source and invokes the test command itself.
    CargoMutants,
}

/// One backend's version, as the party that ran it states it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BackendVersion(String);

/// Why one backend version was refused.
#[must_use = "a refusal is the reason a backend version was not read"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BackendVersionRefusal {
    /// The spelling is empty, so it states no version.
    EmptySpelling,
}

/// Whether the party that ran a backend stated which version produced the text a reading was taken from.
///
/// The version is declared and never observed: this lane reads text a caller already holds and invokes nothing.
/// [`BackendVersionPosture::Stated`] records that party's word, and is not a verification that the text matches that version's rendering.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BackendVersionPosture {
    /// The party that ran the backend stated this version.
    Stated(BackendVersion),
    /// No party has stated a version, so the grammar assumption stands unbound.
    Unstated,
}

/// Which of a backend's outputs one reading was taken from.
///
/// A console stream is a rendering a tool writes for a person, so the shapes it carries are the ones the adapter's own page states.
/// A machine-readable output is a second arm beside the grammar that reads it, carrying whatever ceiling that output affords.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReadingSource {
    /// The line-oriented console stream the backend writes as it runs.
    ConsoleStream,
}

/// Which version of an adapter's stated line grammar one reading was taken under.
///
/// It moves when and only when those line shapes move, and it is neither the backend's version nor an encoding version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GrammarVersion(u32);

/// The most one reading's evidence can establish, in the verdict vocabulary.
///
/// A ceiling follows from what the reading's source carries, and a run carrying a verdict outside it is refused rather than believed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClaimCeiling {
    /// The strongest verdict is a kill that asserts witness rejection and states nothing about activation.
    ///
    /// The source carries no channel that could observe a damage firing, so no mutant read under it earns [`MutationVerdict::Survived`].
    WitnessRejection,
}

/// What one reading is stated under: the backend, its version posture, the output read, and the adapter grammar that read it.
///
/// A [`WrapReading`] cannot be built without one, so "which grammar was this read under, and what may it claim" is answered at the reading rather than remembered around it.
/// [`AdapterProfile::ceiling`] reads the ceiling from the source, so no profile grants its reading more than the source affords.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AdapterProfile {
    backend: WrappedBackend,
    version: BackendVersionPosture,
    source: ReadingSource,
    grammar: GrammarVersion,
}

/// The exact command tokens a party states it used to invoke one mutation backend.
///
/// Tokens are retained separately rather than flattened into shell text, so an argument boundary cannot be reconstructed differently by a later reader.
/// This is execution custody and not proof that a process ran.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BackendCommand {
    executable: String,
    arguments: Vec<String>,
}

/// Why one backend command was refused.
#[must_use = "a refusal is the reason a backend command was not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BackendCommandRefusal {
    /// An empty executable states no program to invoke.
    EmptyExecutable,
}

/// The backend, version, command, target, and toolchain one imported suite-pressure artifact states it ran under.
///
/// The adapter profile is derived from the backend and version on the reading road rather than supplied beside this value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MutationBackendInvocation {
    backend: WrappedBackend,
    version: BackendVersion,
    command: BackendCommand,
    target: TargetBinding,
}

/// The caller-supplied reading from one source coordinate to the claim that owns it.
///
/// A function pointer rather than a closure, so the seam carries no captured state.
/// Answering `None` says no mapping was available and produces [`MappingPosture::OwnerUnmapped`](crate::muterprater::MappingPosture::OwnerUnmapped), never a claim this lane picked.
pub type OwnerLookup = fn(&SourceCoordinate) -> Option<ClaimRef>;

/// The caller-supplied reading from one backend's damage text to the operator family it realizes.
///
/// Answering `None` produces [`FamilyAttribution::OutsideTheBank`](crate::muterprater::FamilyAttribution::OutsideTheBank), never a family this lane picked.
pub type FamilyLookup = fn(&SourceCoordinate, &[u8]) -> Option<OperatorFamilyRef>;

/// The outcome word one line of a backend's output states about one mutant.
///
/// A line whose leading word is none of these becomes an [`UnparsedLine`] and is never guessed at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WrapOutcomeWord {
    /// The backend's own command rejected the mutant.
    Caught,
    /// The backend's own command accepted the mutant.
    Missed,
    /// The mutant did not build.
    Unviable,
    /// The mutant's run passed the backend's time bound.
    TimedOut,
    /// The backend itself failed around the mutant.
    ToolFailed,
}

/// One line of a backend's output this parser could not read.
///
/// Never dropped: a parser that discarded what it did not understand would shrink the denominator with nobody able to read that it had.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnparsedLine {
    ordinal: usize,
    text: ForeignText,
}

/// What the backend announced about its own roster.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnnouncedRoster {
    /// The backend stated how many mutants it found.
    Stated(u32),
    /// The output states no roster count.
    Unstated,
}

/// What one reading recovered: the profile it was read under, the run, the roster the backend announced, and every line left unread.
///
/// The announced roster and the run's census answer different questions, so a difference between them is a finding for a reader and never a number this value reconciles on its own.
/// A reading claims exactly what its profile's ceiling affords, and no road anywhere widens it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WrapReading {
    profile: AdapterProfile,
    run: MutationRun,
    announced: AnnouncedRoster,
    unparsed: Vec<UnparsedLine>,
}

/// Why one reading of a backend's output was refused.
///
/// Dependent checks in a declared order: the baseline is read before any mutant line, and the run is weighed against the profile's ceiling before a reading stands over it.
#[must_use = "a refusal is the reason a wrap reading was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WrapRefusal {
    /// The output states no unmutated-baseline line, so the kill precondition was never established.
    BaselineNotStated,
    /// The baseline the output states does not qualify.
    BaselineNotQualified(BaselinePrecondition),
    /// One mutant line's record was refused by the lawful-kill constructor.
    KillNotLawful {
        /// Which line of the output, counting from zero.
        ordinal: usize,
        /// What the constructor refused.
        cause: KillRefusal,
    },
    /// One record carries a verdict the profile's ceiling does not admit.
    VerdictPastCeiling {
        /// The record's position in the run, counting from zero.
        at: usize,
        /// The verdict the record carries.
        verdict: MutationVerdict,
        /// What the profile's source affords.
        ceiling: ClaimCeiling,
    },
}

/// The domain tag of exact imported backend-output bytes.
pub const BACKEND_OUTPUT_TAG: DomainTag = DomainTag::declared(
    "mutation-backend-output",
    IdentityProfileVersion::declared(1),
);

/// The domain tag of one exact mutation-source revision.
pub const MUTATION_SOURCE_REVISION_TAG: DomainTag = DomainTag::declared(
    "mutation-source-revision",
    IdentityProfileVersion::declared(1),
);

/// The content identity of exact output bytes imported from a mutation backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BackendOutputId(ContentAddress);

/// The content identity of exact source bytes one imported mutation report stood over.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MutationSourceRevisionId(ContentAddress);

/// One reported source file joined to the exact bytes the artifact run stood over.
///
/// The path remains a coordinate relationship while the revision is bytes-only, so identical bytes may lawfully occur at two different paths without becoming one source seat.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MutationSourceRevision {
    file: String,
    revision: MutationSourceRevisionId,
}

/// One imported mutation run's typed custody manifest.
///
/// The reading owns the parser profile; the invocation owns backend execution context; the output identity owns exact imported text; and the source roster owns the exact revision of every file named by a parsed mutation report.
/// The constructor is the wrapped-backend reader, so none of those seats can be attached to an independently parsed run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledSuiteArtifactManifest {
    invocation: MutationBackendInvocation,
    output: BackendOutputId,
    sources: Vec<MutationSourceRevision>,
    reading: WrapReading,
}

/// Why imported output and source snapshots did not become one artifact manifest.
#[must_use = "a refusal is the reason no compiled-suite artifact manifest was admitted"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtifactManifestRefusal {
    /// The output did not read under the adapter grammar.
    Reading(WrapRefusal),
    /// A parser-produced record carried a non-external source site, which this artifact road cannot bind.
    MutationSiteNotReported,
    /// Two supplied source revisions name one file.
    DuplicateSource(String),
    /// A parsed mutation report names a file absent from the artifact's source snapshots.
    ReportedSourceMissing(String),
    /// A supplied source snapshot names no parsed mutation report.
    SourceNotReported(String),
}

/// One imported artifact whose retained source revisions exactly match a caller-supplied current source roster.
///
/// This establishes currency only against the bytes the caller supplied for comparison; it does not inspect a checkout or authenticate the backend process.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledSuiteArtifactCustody {
    manifest: CompiledSuiteArtifactManifest,
}

/// Why an artifact manifest did not stand over the supplied current source roster.
#[must_use = "a refusal is the reason imported mutation evidence is not current for the supplied sources"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtifactCustodyRefusal {
    /// Two current source revisions name one file.
    DuplicateCurrentSource(String),
    /// The artifact names a source file absent from the current roster.
    CurrentSourceMissing(String),
    /// The current roster names a source file absent from the artifact.
    CurrentSourceUnexpected(String),
    /// One source file's exact bytes moved since the artifact was captured.
    CurrentSourceMoved {
        /// The source file whose bytes moved.
        file: String,
        /// The revision retained by the artifact.
        expected: MutationSourceRevisionId,
        /// The revision derived from the supplied current bytes.
        found: MutationSourceRevisionId,
    },
}

// ---------------------------------------------------------------------------
// Qualification, and the generic suite bite.
// ---------------------------------------------------------------------------

/// Whether current-source-qualified external suite pressure has reported, and what it reported.
///
/// The whole custody value rides here rather than a bare reading, so backend, version, command, target, output, parser, and source revision cannot fall away before pressure is minted.
/// A pass with no kill is not evidence that the properties bite, and [`CompiledSuitePressure::demonstrated`] reads it as the absence it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompiledSuiteArtifactStanding<'artifact> {
    /// The wrapped-backend pressure reported under complete current-source custody.
    Reported(&'artifact CompiledSuiteArtifactCustody),
    /// No current-source-qualified artifact has reported.
    NotReported,
}

/// Whether anybody has checked one adapter's stated line grammar against output the backend itself wrote.
///
/// The bare arm is the bootstrap posture rather than a value somebody forgot to fill in.
/// [`GrammarStanding::Checked`] is the checking party's own word: nothing here invokes a backend, and nothing here reads a backend's output to discover what that backend renders.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GrammarStanding {
    /// A party checked the adapter's line shapes against output this version of the backend wrote.
    Checked(BackendVersion),
    /// Nobody has checked them, so nothing is qualified and [`AdapterQualification::of`] refuses.
    Unchecked,
}

/// One adapter profile qualified for every reading taken under that exact profile.
///
/// [`AdapterQualification::of`] is the only road, and exactly one pairing travels it: the reading's profile states a backend version, and the standing is a check against that same version.
/// The qualification is reusable across readings carrying that profile, and it identifies no single reading instance.
///
/// # Nonclaims
///
/// Parser correctness is not suite bite: this says the adapter is fit to be read under, and nothing about whether a property rejected anything.
/// The claim ceiling rides through unchanged, because qualifying an adapter never widens what its source affords.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AdapterQualification {
    profile: AdapterProfile,
    standing: GrammarStanding,
}

/// Why one reading's profile was not qualified.
///
/// Dependent checks in a declared order: whether anybody checked the shapes at all, whether the profile states a version a check could name, then whether the check and the reading name one version.
#[must_use = "a refusal is the reason a reading's profile was not qualified"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QualificationRefusal {
    /// Nobody has checked the adapter's stated shapes against output the backend wrote.
    GrammarUnchecked,
    /// The reading's profile states no backend version, so a check names nothing it stands under.
    BackendVersionUnstated,
    /// The reading was taken under one backend version and the shapes were checked against another.
    CheckedAgainstAnotherVersion {
        /// The version the reading's own profile states wrote the text.
        stated: BackendVersion,
        /// The version the standing states the shapes were checked against.
        checked: BackendVersion,
    },
}

/// At least one lawful backend-reported kill, read out of a current-source-qualified artifact whose adapter profile stands qualified.
///
/// The qualification and complete artifact custody ride inside, so suite pressure over an unqualified profile or stale supplied source roster is not a value anybody can hold.
///
/// # Nonclaims
///
/// Suite bite is not campaign accounting: how many mutants a run pressed and how they divide is [`MutationCensus`](crate::muterprater::MutationCensus)'s question.
/// Neither is it the no-mutation parity ([`NoMutationParityQualification`](crate::muterprater::NoMutationParityQualification)), and it cannot open any pair's interpreted trust by itself.
/// Source currency is exact only against the source bytes supplied to [`CompiledSuiteArtifactCustody`]; it is not ambient checkout observation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledSuitePressure {
    qualification: AdapterQualification,
    custody: CompiledSuiteArtifactCustody,
    kill: MutationReport,
}

/// Why one current-source-qualified artifact standing demonstrated no generic compiled suite pressure.
///
/// Dependent checks in a declared order: whether the pressure reported, whether the qualification carries the reading's exact profile, then whether what it reported carries a kill.
#[must_use = "a refusal is the reason no compiled suite pressure was demonstrated"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SuitePressureRefusal {
    /// No current-source-qualified artifact has reported, so there is no reading to stand on.
    ArtifactNotReported,
    /// The qualification names another adapter profile and stands behind nothing here.
    QualificationUnderAnotherProfile,
    /// The reading's run demonstrated no lawful kill.
    NoKillDemonstrated,
}
