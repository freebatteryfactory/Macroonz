//! The invariant nucleus of wrapped-backend readings and imported artifact custody.

use super::{
    AdapterProfile, AdapterQualification, AnnouncedRoster, ArtifactCustodyRefusal,
    BACKEND_OUTPUT_TAG, BackendCommand, BackendCommandRefusal, BackendOutputId, BackendVersion,
    BackendVersionPosture, BackendVersionRefusal, ClaimCeiling, CompiledSuiteArtifactCustody,
    CompiledSuiteArtifactManifest, CompiledSuiteArtifactStanding, CompiledSuitePressure,
    GrammarStanding, GrammarVersion, MUTATION_SOURCE_REVISION_TAG, MutationBackendInvocation,
    MutationSourceRevision, MutationSourceRevisionId, QualificationRefusal, ReadingSource,
    SuitePressureRefusal, UnparsedLine, WrapReading, WrapRefusal, WrappedBackend,
};
use crate::identity::ContentAddress;
use crate::muterprater::{CoordinateRefusal, MutationReport, MutationRun, MutationVerdict};
use crate::report::{ForeignText, TargetBinding};
use std::collections::BTreeMap;
impl ClaimCeiling {
    /// The strongest verdict this ceiling grants.
    #[must_use]
    pub const fn strongest(self) -> MutationVerdict {
        match self {
            Self::WitnessRejection => MutationVerdict::Killed,
        }
    }

    /// Whether one verdict stands inside this ceiling.
    ///
    /// A kill and an inconclusive both stand inside witness rejection; survived stands outside it, because earning that word takes an activation the source offers no channel to observe.
    #[must_use]
    pub const fn admits(self, verdict: MutationVerdict) -> bool {
        match (self, verdict) {
            (Self::WitnessRejection, MutationVerdict::Killed | MutationVerdict::Inconclusive) => {
                true
            }
            (Self::WitnessRejection, MutationVerdict::Survived) => false,
        }
    }
}
impl BackendCommand {
    /// Retain one backend command as an executable followed by its exact argument tokens.
    ///
    /// # Errors
    ///
    /// Refuses an empty executable, which states no program to invoke.
    pub fn declared(executable: &str, arguments: &[&str]) -> Result<Self, BackendCommandRefusal> {
        if executable.is_empty() {
            return Err(BackendCommandRefusal::EmptyExecutable);
        }
        Ok(Self {
            executable: executable.to_owned(),
            arguments: arguments
                .iter()
                .map(|argument| (*argument).to_owned())
                .collect(),
        })
    }

    /// The executable token.
    #[must_use]
    pub fn executable(&self) -> &str {
        &self.executable
    }

    /// The argument tokens, in invocation order.
    #[must_use]
    pub fn arguments(&self) -> &[String] {
        &self.arguments
    }
}

impl MutationBackendInvocation {
    /// State the exact backend execution context one imported artifact records.
    #[must_use]
    pub fn declared(
        backend: WrappedBackend,
        version: BackendVersion,
        command: BackendCommand,
        target: TargetBinding,
    ) -> Self {
        Self {
            backend,
            version,
            command,
            target,
        }
    }

    /// The backend the command invokes.
    #[must_use]
    pub const fn backend(&self) -> WrappedBackend {
        self.backend
    }

    /// The backend version the artifact states produced its output.
    #[must_use]
    pub const fn version(&self) -> &BackendVersion {
        &self.version
    }

    /// The exact command tokens the artifact states were invoked.
    #[must_use]
    pub const fn command(&self) -> &BackendCommand {
        &self.command
    }

    /// The target and toolchain the artifact states it ran under.
    #[must_use]
    pub const fn target(&self) -> &TargetBinding {
        &self.target
    }
}

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

impl BackendOutputId {
    /// Derive the content identity of exact imported backend-output bytes.
    pub(in crate::muterprater) fn derived(bytes: &[u8]) -> Self {
        Self(ContentAddress::derived(BACKEND_OUTPUT_TAG, bytes))
    }
}

crate::identity::content_address_reference! {
    /// The underlying content address.
    value BackendOutputId;
}

impl MutationSourceRevisionId {
    /// Derive one exact mutation-source revision from its bytes.
    fn derived(bytes: &[u8]) -> Self {
        Self(ContentAddress::derived(MUTATION_SOURCE_REVISION_TAG, bytes))
    }
}

crate::identity::content_address_reference! {
    /// The underlying content address.
    value MutationSourceRevisionId;
}

impl MutationSourceRevision {
    /// Bind one reported source path to the exact source bytes an artifact or current comparison stood over.
    ///
    /// # Errors
    ///
    /// Refuses an empty file spelling, which identifies no source seat.
    pub fn from_content(file: &str, bytes: &[u8]) -> Result<Self, CoordinateRefusal> {
        if file.is_empty() {
            return Err(CoordinateRefusal::EmptyFile);
        }
        Ok(Self {
            file: file.to_owned(),
            revision: MutationSourceRevisionId::derived(bytes),
        })
    }

    /// The reported source-file spelling.
    #[must_use]
    pub fn file(&self) -> &str {
        &self.file
    }

    /// The exact content revision of that source file.
    #[must_use]
    pub const fn revision(&self) -> MutationSourceRevisionId {
        self.revision
    }
}

impl CompiledSuiteArtifactManifest {
    /// Retain one parser-produced reading under its exact backend invocation, output identity, and source revisions.
    pub(in crate::muterprater) fn recorded(
        invocation: MutationBackendInvocation,
        output: BackendOutputId,
        sources: Vec<MutationSourceRevision>,
        reading: WrapReading,
    ) -> Self {
        Self {
            invocation,
            output,
            sources,
            reading,
        }
    }

    /// The backend execution context the artifact states.
    #[must_use]
    pub const fn invocation(&self) -> &MutationBackendInvocation {
        &self.invocation
    }

    /// The exact imported backend-output content identity.
    #[must_use]
    pub const fn output(&self) -> BackendOutputId {
        self.output
    }

    /// The exact source revisions, ordered by reported file spelling.
    #[must_use]
    pub fn sources(&self) -> &[MutationSourceRevision] {
        &self.sources
    }

    /// The parser-produced reading retained by this manifest.
    #[must_use]
    pub const fn reading(&self) -> &WrapReading {
        &self.reading
    }
}

impl CompiledSuiteArtifactCustody {
    /// Join an imported artifact manifest to the exact current source revisions a caller supplies.
    ///
    /// The comparison is over the complete manifest roster by file and revision, so a missing, added, duplicated, or moved source refuses instead of silently narrowing currency.
    ///
    /// # Errors
    ///
    /// Refuses duplicate current files first, then a manifest file missing from the current roster, then an unexpected current file, then the first source revision that moved in file order.
    pub fn current(
        manifest: CompiledSuiteArtifactManifest,
        current_sources: Vec<MutationSourceRevision>,
    ) -> Result<Self, ArtifactCustodyRefusal> {
        let mut current = BTreeMap::new();
        for source in current_sources {
            let file = source.file().to_owned();
            if current.insert(file.clone(), source).is_some() {
                return Err(ArtifactCustodyRefusal::DuplicateCurrentSource(file));
            }
        }
        let expected: BTreeMap<&str, MutationSourceRevisionId> = manifest
            .sources()
            .iter()
            .map(|source| (source.file(), source.revision()))
            .collect();
        for file in expected.keys().copied() {
            if !current.contains_key(file) {
                return Err(ArtifactCustodyRefusal::CurrentSourceMissing(
                    file.to_owned(),
                ));
            }
        }
        for file in current.keys() {
            if !expected.contains_key(file.as_str()) {
                return Err(ArtifactCustodyRefusal::CurrentSourceUnexpected(
                    file.to_owned(),
                ));
            }
        }
        for (file, expected_revision) in expected {
            match current.get(file) {
                Some(found) if expected_revision != found.revision() => {
                    return Err(ArtifactCustodyRefusal::CurrentSourceMoved {
                        file: file.to_owned(),
                        expected: expected_revision,
                        found: found.revision(),
                    });
                }
                Some(_) => {}
                None => {
                    return Err(ArtifactCustodyRefusal::CurrentSourceMissing(
                        file.to_owned(),
                    ));
                }
            }
        }
        Ok(Self { manifest })
    }

    /// The complete imported artifact manifest this current-source join stands over.
    #[must_use]
    pub const fn manifest(&self) -> &CompiledSuiteArtifactManifest {
        &self.manifest
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
    /// Read from the source rather than stored, so a profile can never grant more than the output it was taken from affords.
    #[must_use]
    pub fn ceiling(&self) -> ClaimCeiling {
        ClaimCeiling::from(self.source)
    }
}

impl UnparsedLine {
    /// One line of a backend's output this parser could not read.
    ///
    /// The material is admitted through the record vocabulary's bounded foreign text, so a long line is cut at the bound with the cut marked.
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
    /// What one reading recovered, stated under the profile it was read through.
    ///
    /// # Errors
    ///
    /// Refuses a run carrying a record whose verdict the profile's ceiling does not admit, naming the record, its verdict, and the ceiling.
    pub(in crate::muterprater) fn read(
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
// Qualification, and the generic suite bite.
// ---------------------------------------------------------------------------

impl AdapterQualification {
    /// The qualification one exact adapter profile stands under.
    ///
    /// The profile is taken from the reading rather than stated beside it, and what the caller states is the grammar standing.
    /// One pairing qualifies: the reading's profile states backend version `v`, and the standing is [`GrammarStanding::Checked`] over `v`.
    ///
    /// # Errors
    ///
    /// Refuses, in a declared dependent order: a standing under which nobody has checked anything, a reading whose profile states no backend version, then a check made against a version other than the one the reading names.
    pub fn of(
        reading: &WrapReading,
        standing: GrammarStanding,
    ) -> Result<Self, QualificationRefusal> {
        let GrammarStanding::Checked(checked) = &standing else {
            return Err(QualificationRefusal::GrammarUnchecked);
        };
        let BackendVersionPosture::Stated(stated) = reading.profile().version() else {
            return Err(QualificationRefusal::BackendVersionUnstated);
        };
        if stated != checked {
            return Err(QualificationRefusal::CheckedAgainstAnotherVersion {
                stated: stated.clone(),
                checked: checked.clone(),
            });
        }
        Ok(Self {
            profile: reading.profile().clone(),
            standing,
        })
    }

    /// The profile the reading was taken under.
    #[must_use]
    pub const fn profile(&self) -> &AdapterProfile {
        &self.profile
    }

    /// Whether that adapter's grammar has been checked against real output.
    #[must_use]
    pub const fn standing(&self) -> &GrammarStanding {
        &self.standing
    }

    /// The most a reading under this qualification can establish.
    ///
    /// The profile's own ceiling, read through rather than restated: qualifying an adapter never widens what its source affords.
    #[must_use]
    pub fn ceiling(&self) -> ClaimCeiling {
        self.profile.ceiling()
    }
}

impl CompiledSuitePressure {
    /// The generic suite pressure one current-source-qualified artifact demonstrated, where it demonstrated one.
    ///
    /// The qualification arrives from [`AdapterQualification::of`] rather than being minted here, so this road weighs a standing somebody already earned against the reading in hand.
    ///
    /// # Errors
    ///
    /// Refuses, in a declared dependent order: a standing that has not reported, a qualification naming a profile other than this artifact's reading, then a reading whose run demonstrated no lawful kill.
    pub fn demonstrated(
        artifact: CompiledSuiteArtifactStanding<'_>,
        qualification: &AdapterQualification,
    ) -> Result<Self, SuitePressureRefusal> {
        let CompiledSuiteArtifactStanding::Reported(custody) = artifact else {
            return Err(SuitePressureRefusal::ArtifactNotReported);
        };
        let reading = custody.manifest().reading();
        if qualification.profile() != reading.profile() {
            return Err(SuitePressureRefusal::QualificationUnderAnotherProfile);
        }
        let Some(kill) = reading
            .run()
            .reports()
            .iter()
            .find(|report| report.verdict() == MutationVerdict::Killed)
        else {
            return Err(SuitePressureRefusal::NoKillDemonstrated);
        };
        Ok(Self {
            qualification: qualification.clone(),
            custody: custody.clone(),
            kill: kill.clone(),
        })
    }

    /// The qualification the witness was demonstrated under.
    #[must_use]
    pub const fn qualification(&self) -> &AdapterQualification {
        &self.qualification
    }

    /// The exact backend invocation, output, parser reading, and current-source custody behind this pressure.
    #[must_use]
    pub const fn custody(&self) -> &CompiledSuiteArtifactCustody {
        &self.custody
    }

    /// The kill it was demonstrated by.
    #[must_use]
    pub const fn kill(&self) -> &MutationReport {
        &self.kill
    }
}
