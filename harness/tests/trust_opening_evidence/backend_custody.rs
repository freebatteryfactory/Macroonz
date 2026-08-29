//! Outside observations of wrapped-backend qualification and source custody.

use super::support::{
    BACKEND_COMMAND, BACKEND_CONSOLE, BACKEND_NO_KILL, BACKEND_SOURCE, BACKEND_TARGET,
    BACKEND_TOOLCHAIN, BACKEND_VERSION, COMPILED_MUTANT_FILE, CURRENT_BACKEND_SOURCE,
    HISTORICAL_BACKEND_COMMAND, HISTORICAL_BACKEND_CONSOLE, HISTORICAL_BACKEND_SOURCE,
    HISTORICAL_COMPILED_MUTANT_FILE, MutationRoadFailure, backend_invocation, compiled_artifact,
    compiled_family, compiled_owner, compiled_reading, current_custody,
    historical_compiled_artifact, historical_source_revision, source_revision,
};
use macroonz_harness::descriptor::ClaimRef;
use macroonz_harness::muterprater::wrap::{read_artifact, read_output};
use macroonz_harness::muterprater::{
    AdapterQualification, AnnouncedRoster, ArtifactCustodyRefusal, ArtifactManifestRefusal,
    BackendCommand, BackendVersion, BackendVersionPosture, CompiledSuiteArtifactCustody,
    CompiledSuiteArtifactStanding, CompiledSuitePressure, GrammarStanding,
    MutationBackendInvocation, MutationSourceRevision, MutationVerdict, OperatorFamilyRef,
    QualificationRefusal, ReadingSource, SourceCoordinate, SuitePressureRefusal, WrappedBackend,
};
use macroonz_harness::report::{TargetBinding, TargetTriple, ToolchainIdentity};

const COMPILE_CONTRACT_CONSOLE: &str =
    include_str!("compile-contract-pressure-artifact/cargo-mutants-27.0.0-console.txt");
const COMPILE_CONTRACT_COMPARE_SOURCE: &[u8] =
    include_bytes!("compile-contract-pressure-artifact/compare.rs");
const COMPILE_CONTRACT_CONCLUDE_SOURCE: &[u8] =
    include_bytes!("compile-contract-pressure-artifact/conclude.rs");
const COMPILE_CONTRACT_GUARD_SOURCE: &[u8] =
    include_bytes!("compile-contract-pressure-artifact/type_guard.rs");
const CURRENT_COMPILE_CONTRACT_COMPARE_SOURCE: &[u8] =
    include_bytes!("../../src/oracle/compiled/compare.rs");
const CURRENT_COMPILE_CONTRACT_CONCLUDE_SOURCE: &[u8] =
    include_bytes!("../../src/oracle/compiled/conclude.rs");
const CURRENT_COMPILE_CONTRACT_GUARD_SOURCE: &[u8] =
    include_bytes!("../../src/oracle/compiled/type_guard.rs");
const COMPILE_CONTRACT_COMPARE_FILE: &str = "harness/src/oracle/compiled/compare.rs";
const COMPILE_CONTRACT_CONCLUDE_FILE: &str = "harness/src/oracle/compiled/conclude.rs";
const COMPILE_CONTRACT_GUARD_FILE: &str = "harness/src/oracle/compiled/type_guard.rs";
const COMPILE_CONTRACT_COMMAND: &[&str] = &[
    "+1.98.0",
    "mutants",
    "--no-config",
    "-p",
    "macroonz-harness",
    "--file",
    COMPILE_CONTRACT_COMPARE_FILE,
    "--file",
    COMPILE_CONTRACT_CONCLUDE_FILE,
    "--file",
    COMPILE_CONTRACT_GUARD_FILE,
    "--re",
    "(compared_compilation|compilation_cause|CompilationVerdict::concluded|RustcErrorCode::|RelativeSourcePath::|SourcePosition::|PrimarySourceSpan::|DiagnosticAnchor::|DeclaredCompilation::|ObservedCompilation::|has_windows_prefix)",
    "--test-tool",
    "cargo",
    "--baseline",
    "run",
    "--jobs",
    "1",
    "--jobserver-tasks",
    "1",
    "--no-shuffle",
    "--caught",
    "--unviable",
    "--no-times",
    "--colors",
    "never",
    "--annotations",
    "none",
    "--timeout",
    "300",
    "--build-timeout",
    "300",
    "--output",
    "target/qualification/0.2-a-compile-contract-mutation-20260829/final-3",
    "--",
    "--locked",
    "--offline",
    "--test",
    "vector_oracle",
    "--test",
    "compile_refusals",
    "compilation",
    "--",
    "--test-threads=1",
];

fn compile_contract_owner(_: &SourceCoordinate) -> Option<ClaimRef> {
    None
}

fn compile_contract_family(_: &SourceCoordinate, _: &[u8]) -> Option<OperatorFamilyRef> {
    None
}

fn compile_contract_source(
    file: &str,
    bytes: &[u8],
) -> Result<MutationSourceRevision, MutationRoadFailure> {
    MutationSourceRevision::from_content(file, bytes).map_err(|_| MutationRoadFailure::Name)
}

fn compile_contract_sources(
    compare: &[u8],
    conclude: &[u8],
    guard: &[u8],
) -> Result<Vec<MutationSourceRevision>, MutationRoadFailure> {
    Ok(vec![
        compile_contract_source(COMPILE_CONTRACT_COMPARE_FILE, compare)?,
        compile_contract_source(COMPILE_CONTRACT_CONCLUDE_FILE, conclude)?,
        compile_contract_source(COMPILE_CONTRACT_GUARD_FILE, guard)?,
    ])
}

fn compile_contract_invocation(
    version: BackendVersion,
) -> Result<MutationBackendInvocation, MutationRoadFailure> {
    let command = BackendCommand::declared("cargo", COMPILE_CONTRACT_COMMAND)
        .map_err(|_| MutationRoadFailure::Name)?;
    Ok(MutationBackendInvocation::declared(
        WrappedBackend::CargoMutants,
        version,
        command,
        TargetBinding::bound(
            TargetTriple::declared(BACKEND_TARGET),
            ToolchainIdentity::declared(BACKEND_TOOLCHAIN),
        ),
    ))
}

/// Claim: the exact compile-contract campaign carries complete current-source custody and demonstrates generic compiled-suite pressure without inventing mutation ownership.
///
/// Subject: the persisted Cargo Mutants console, invocation, three mutated source files, adapter profile, and current-source join.
/// Population: all 49 selected mutants, divided into 28 backend-reported kills and 21 unviable rows.
/// Hostile control: every target remains owner-unmapped and outside the operator bank because this campaign tests compiler-contract sensitivity rather than Muterprater policy.
/// Denominator: the complete announced roster, parsed report roster, source roster, command, target, toolchain, grammar, and current source bytes.
/// Evidence ceiling: this establishes stable Windows compiled-suite pressure under backend-reported witness rejection; it does not establish activation, equivalence, another host, or general mutation adequacy.
/// Retained regression: a surviving selected mutant, stale source copy, incomplete roster, changed invocation, or widened ownership claim remains a permanent regression.
#[test]
fn compile_contract_pressure_is_complete_current_and_unmapped() -> Result<(), MutationRoadFailure> {
    assert_eq!(
        COMPILE_CONTRACT_COMPARE_SOURCE,
        CURRENT_COMPILE_CONTRACT_COMPARE_SOURCE
    );
    assert_eq!(
        COMPILE_CONTRACT_CONCLUDE_SOURCE,
        CURRENT_COMPILE_CONTRACT_CONCLUDE_SOURCE
    );
    assert_eq!(
        COMPILE_CONTRACT_GUARD_SOURCE,
        CURRENT_COMPILE_CONTRACT_GUARD_SOURCE
    );

    let version = BackendVersion::stated(BACKEND_VERSION).map_err(|_| MutationRoadFailure::Name)?;
    let manifest = read_artifact(
        COMPILE_CONTRACT_CONSOLE,
        compile_contract_invocation(version.clone())?,
        compile_contract_sources(
            COMPILE_CONTRACT_COMPARE_SOURCE,
            COMPILE_CONTRACT_CONCLUDE_SOURCE,
            COMPILE_CONTRACT_GUARD_SOURCE,
        )?,
        compile_contract_owner,
        compile_contract_family,
    )?;
    assert_eq!(manifest.invocation().command().executable(), "cargo");
    assert_eq!(
        manifest.invocation().command().arguments(),
        COMPILE_CONTRACT_COMMAND
    );
    assert_eq!(
        manifest.invocation().target().target().spelling(),
        BACKEND_TARGET
    );
    assert_eq!(
        manifest.invocation().target().toolchain().spelling(),
        BACKEND_TOOLCHAIN
    );
    assert_eq!(manifest.reading().announced(), AnnouncedRoster::Stated(49));
    assert_eq!(manifest.reading().run().reports().len(), 49usize);
    assert_eq!(manifest.reading().run().kills().count(), 28usize);
    assert_eq!(manifest.reading().run().non_kills().count(), 21usize);
    assert_eq!(manifest.reading().run().survivors().count(), 0usize);
    assert!(
        manifest
            .reading()
            .run()
            .reports()
            .iter()
            .all(|report| report.target().owning_claim().is_none())
    );
    let [summary] = manifest.reading().unparsed() else {
        return Err(MutationRoadFailure::MissingAlternative);
    };
    assert_eq!(summary.ordinal(), 51usize);
    assert_eq!(
        summary.text().bytes(),
        b"49 mutants tested: 28 caught, 21 unviable"
    );

    let current_sources = compile_contract_sources(
        CURRENT_COMPILE_CONTRACT_COMPARE_SOURCE,
        CURRENT_COMPILE_CONTRACT_CONCLUDE_SOURCE,
        CURRENT_COMPILE_CONTRACT_GUARD_SOURCE,
    )?;
    assert_eq!(manifest.sources(), current_sources);
    let qualification =
        AdapterQualification::of(manifest.reading(), GrammarStanding::Checked(version))?;
    let custody = CompiledSuiteArtifactCustody::current(manifest, current_sources)?;
    let pressure = CompiledSuitePressure::demonstrated(
        CompiledSuiteArtifactStanding::Reported(&custody),
        &qualification,
    )?;
    assert_eq!(pressure.qualification(), &qualification);
    assert_eq!(pressure.custody(), &custody);
    assert_eq!(pressure.kill().verdict(), MutationVerdict::Killed);
    Ok(())
}

/// Claim: the prior compiled-pressure receipt remains exact historical evidence and cannot join to the moved current source coordinate.
///
/// Subject: the retained prior artifact, the current artifact, and the public source-custody constructor.
/// Population: both exact source copies, both exact console outputs, both manifests, and the attempted old-manifest/new-coordinate join.
/// Hostile control: the source coordinate and its ownership-relative import bytes differ, so source, output, and external-mutant identities differ and the crossed custody join refuses.
/// Denominator: every coordinate-sensitive and bytes-sensitive identity carried by the two receipts.
/// Evidence ceiling: this preserves what the prior run stated and does not present it as a rerun against the moved source.
/// Retained regression: fixture drift, receipt relabeling, or acceptance of the crossed source join remains a permanent regression.
#[test]
fn prior_compiled_pressure_receipt_remains_historical() -> Result<(), MutationRoadFailure> {
    let version = BackendVersion::stated(BACKEND_VERSION).map_err(|_| MutationRoadFailure::Name)?;
    let historical = historical_compiled_artifact(
        HISTORICAL_BACKEND_CONSOLE,
        version.clone(),
        HISTORICAL_BACKEND_SOURCE,
    )?;
    let current = compiled_artifact(BACKEND_CONSOLE, version, BACKEND_SOURCE)?;

    assert_ne!(HISTORICAL_BACKEND_SOURCE, BACKEND_SOURCE);
    assert_eq!(BACKEND_SOURCE, CURRENT_BACKEND_SOURCE);
    assert_eq!(
        historical.invocation().command().arguments(),
        HISTORICAL_BACKEND_COMMAND
    );
    assert_eq!(current.invocation().command().arguments(), BACKEND_COMMAND);
    let [historical_source] = historical.sources() else {
        return Err(MutationRoadFailure::MissingAlternative);
    };
    let [current_source] = current.sources() else {
        return Err(MutationRoadFailure::MissingAlternative);
    };
    assert_eq!(historical_source.file(), HISTORICAL_COMPILED_MUTANT_FILE);
    assert_eq!(current_source.file(), COMPILED_MUTANT_FILE);
    assert_eq!(
        historical_source,
        &historical_source_revision(HISTORICAL_BACKEND_SOURCE)?
    );
    assert_eq!(current_source, &source_revision(BACKEND_SOURCE)?);
    assert_ne!(historical_source.revision(), current_source.revision());
    assert_ne!(historical.output(), current.output());
    let [historical_report] = historical.reading().run().reports() else {
        return Err(MutationRoadFailure::MissingAlternative);
    };
    let [current_report] = current.reading().run().reports() else {
        return Err(MutationRoadFailure::MissingAlternative);
    };
    assert_ne!(
        historical_report.target().identity(),
        current_report.target().identity()
    );
    assert!(matches!(
        CompiledSuiteArtifactCustody::current(
            historical,
            vec![source_revision(CURRENT_BACKEND_SOURCE)?],
        ),
        Err(ArtifactCustodyRefusal::CurrentSourceMissing(file))
            if file == HISTORICAL_COMPILED_MUTANT_FILE
    ));
    Ok(())
}

/// Adapter qualification remains bound to the exact backend profile whose reading earned it.
#[test]
fn a_compiled_witness_refuses_another_profile() -> Result<(), MutationRoadFailure> {
    let here_version =
        BackendVersion::stated(BACKEND_VERSION).map_err(|_| MutationRoadFailure::Name)?;
    let here = compiled_artifact(BACKEND_CONSOLE, here_version, BACKEND_SOURCE)?;
    let other_version = BackendVersion::stated("24.0.0").map_err(|_| MutationRoadFailure::Name)?;
    let elsewhere = read_output(
        BACKEND_CONSOLE,
        BackendVersionPosture::Stated(other_version.clone()),
        compiled_owner,
        compiled_family,
    )?;
    let borrowed = AdapterQualification::of(&elsewhere, GrammarStanding::Checked(other_version))?;
    let custody = current_custody(here, CURRENT_BACKEND_SOURCE)?;
    assert_eq!(
        CompiledSuitePressure::demonstrated(
            CompiledSuiteArtifactStanding::Reported(&custody),
            &borrowed,
        ),
        Err(SuitePressureRefusal::QualificationUnderAnotherProfile)
    );
    Ok(())
}

/// Imported suite pressure retains backend, version, command, target, output, parser, and exact current source revision without turning any of them into pair authority.
#[test]
fn compiled_suite_artifact_custody_is_complete_and_current() -> Result<(), MutationRoadFailure> {
    let version = BackendVersion::stated(BACKEND_VERSION).map_err(|_| MutationRoadFailure::Name)?;
    let manifest = compiled_artifact(BACKEND_CONSOLE, version.clone(), BACKEND_SOURCE)?;
    assert_eq!(
        manifest.invocation().backend(),
        WrappedBackend::CargoMutants
    );
    assert_eq!(manifest.invocation().version(), &version);
    assert_eq!(manifest.invocation().command().executable(), "cargo");
    assert_eq!(manifest.invocation().command().arguments(), BACKEND_COMMAND);
    assert_eq!(
        manifest.invocation().target().target().spelling(),
        BACKEND_TARGET
    );
    assert_eq!(
        manifest.invocation().target().toolchain().spelling(),
        BACKEND_TOOLCHAIN
    );
    assert_eq!(
        manifest.reading().profile().backend(),
        manifest.invocation().backend()
    );
    assert_eq!(
        manifest.reading().profile().version(),
        &BackendVersionPosture::Stated(version)
    );
    assert_eq!(
        manifest.reading().profile().source(),
        ReadingSource::ConsoleStream
    );
    assert_eq!(manifest.reading().profile().grammar().number(), 1u32);
    let [source] = manifest.sources() else {
        return Err(MutationRoadFailure::MissingAlternative);
    };
    assert_eq!(source.file(), COMPILED_MUTANT_FILE);
    assert_eq!(source, &source_revision(BACKEND_SOURCE)?);

    let same = compiled_artifact(
        BACKEND_CONSOLE,
        manifest.invocation().version().clone(),
        BACKEND_SOURCE,
    )?;
    assert_eq!(manifest.output(), same.output());
    let changed_console = format!("{BACKEND_CONSOLE}artifact-note\n");
    let changed = compiled_artifact(
        &changed_console,
        manifest.invocation().version().clone(),
        BACKEND_SOURCE,
    )?;
    assert_ne!(manifest.output(), changed.output());

    let custody = current_custody(manifest.clone(), CURRENT_BACKEND_SOURCE)?;
    assert_eq!(custody.manifest(), &manifest);
    let moved = source_revision(b"moved-source")?;
    assert!(matches!(
        CompiledSuiteArtifactCustody::current(manifest, vec![moved]),
        Err(ArtifactCustodyRefusal::CurrentSourceMoved { file, expected, found })
            if file == COMPILED_MUTANT_FILE && expected != found
    ));
    Ok(())
}

/// Source custody closes both the artifact-time and current-source rosters instead of accepting a convenient subset.
#[test]
fn compiled_suite_source_rosters_refuse_missing_extra_and_duplicate_files()
-> Result<(), MutationRoadFailure> {
    let version = BackendVersion::stated(BACKEND_VERSION).map_err(|_| MutationRoadFailure::Name)?;
    let invocation = || backend_invocation(version.clone());
    let source = source_revision(BACKEND_SOURCE)?;
    assert_eq!(
        read_artifact(
            BACKEND_CONSOLE,
            invocation()?,
            Vec::new(),
            compiled_owner,
            compiled_family,
        ),
        Err(ArtifactManifestRefusal::ReportedSourceMissing(
            COMPILED_MUTANT_FILE.to_owned(),
        ))
    );
    let extra = MutationSourceRevision::from_content("elsewhere.rs", b"elsewhere")
        .map_err(|_| MutationRoadFailure::Name)?;
    assert_eq!(
        read_artifact(
            BACKEND_CONSOLE,
            invocation()?,
            vec![source.clone(), extra.clone()],
            compiled_owner,
            compiled_family,
        ),
        Err(ArtifactManifestRefusal::SourceNotReported(
            "elsewhere.rs".to_owned(),
        ))
    );
    assert_eq!(
        read_artifact(
            BACKEND_CONSOLE,
            invocation()?,
            vec![source.clone(), source.clone()],
            compiled_owner,
            compiled_family,
        ),
        Err(ArtifactManifestRefusal::DuplicateSource(
            COMPILED_MUTANT_FILE.to_owned(),
        ))
    );

    let manifest = compiled_artifact(BACKEND_CONSOLE, version, BACKEND_SOURCE)?;
    assert_eq!(
        CompiledSuiteArtifactCustody::current(manifest.clone(), Vec::new()),
        Err(ArtifactCustodyRefusal::CurrentSourceMissing(
            COMPILED_MUTANT_FILE.to_owned(),
        ))
    );
    assert_eq!(
        CompiledSuiteArtifactCustody::current(manifest.clone(), vec![source.clone(), extra],),
        Err(ArtifactCustodyRefusal::CurrentSourceUnexpected(
            "elsewhere.rs".to_owned(),
        ))
    );
    assert_eq!(
        CompiledSuiteArtifactCustody::current(manifest, vec![source.clone(), source]),
        Err(ArtifactCustodyRefusal::DuplicateCurrentSource(
            COMPILED_MUTANT_FILE.to_owned(),
        ))
    );
    Ok(())
}

/// Adapter qualification preserves its complete refusal order over unchecked, unstated, and differently versioned profiles.
#[test]
fn adapter_qualification_requires_one_checked_profile_version() -> Result<(), MutationRoadFailure> {
    let stated = compiled_reading()?;
    assert_eq!(
        AdapterQualification::of(&stated, GrammarStanding::Unchecked),
        Err(QualificationRefusal::GrammarUnchecked)
    );

    let checked = BackendVersion::stated(BACKEND_VERSION).map_err(|_| MutationRoadFailure::Name)?;
    let unstated = read_output(
        BACKEND_CONSOLE,
        BackendVersionPosture::Unstated,
        compiled_owner,
        compiled_family,
    )?;
    assert_eq!(
        AdapterQualification::of(&unstated, GrammarStanding::Checked(checked.clone())),
        Err(QualificationRefusal::BackendVersionUnstated)
    );

    let another = BackendVersion::stated("24.0.0").map_err(|_| MutationRoadFailure::Name)?;
    assert_eq!(
        AdapterQualification::of(&stated, GrammarStanding::Checked(another.clone())),
        Err(QualificationRefusal::CheckedAgainstAnotherVersion {
            stated: checked,
            checked: another,
        })
    );
    Ok(())
}

/// Generic compiled suite pressure requires both a reported reading and a lawful backend-reported kill from that reading.
#[test]
fn generic_suite_pressure_requires_a_reported_kill() -> Result<(), MutationRoadFailure> {
    let version = BackendVersion::stated(BACKEND_VERSION).map_err(|_| MutationRoadFailure::Name)?;
    let killed = compiled_artifact(BACKEND_CONSOLE, version.clone(), BACKEND_SOURCE)?;
    assert_eq!(killed.reading().announced(), AnnouncedRoster::Stated(1));
    assert!(matches!(
        killed.reading().unparsed(),
        [summary]
            if summary.ordinal() == 3
                && summary.text().bytes() == b"1 mutant tested: 1 caught"
    ));
    let killed_qualification =
        AdapterQualification::of(killed.reading(), GrammarStanding::Checked(version.clone()))?;
    assert_eq!(
        CompiledSuitePressure::demonstrated(
            CompiledSuiteArtifactStanding::NotReported,
            &killed_qualification,
        ),
        Err(SuitePressureRefusal::ArtifactNotReported)
    );

    let missed_source = MutationSourceRevision::from_content("src/subject/lane.rs", b"missed")
        .map_err(|_| MutationRoadFailure::Name)?;
    let missed = read_artifact(
        BACKEND_NO_KILL,
        backend_invocation(version.clone())?,
        vec![missed_source.clone()],
        compiled_owner,
        compiled_family,
    )?;
    let missed_qualification =
        AdapterQualification::of(missed.reading(), GrammarStanding::Checked(version))?;
    let missed_custody = CompiledSuiteArtifactCustody::current(missed, vec![missed_source])?;
    assert_eq!(
        CompiledSuitePressure::demonstrated(
            CompiledSuiteArtifactStanding::Reported(&missed_custody),
            &missed_qualification,
        ),
        Err(SuitePressureRefusal::NoKillDemonstrated)
    );
    Ok(())
}
