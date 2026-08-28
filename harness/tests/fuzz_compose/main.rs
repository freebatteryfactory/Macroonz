//! The fuzz home admits interesting bytes into Macroonz reduction and replay without owning the coverage engine.

use macroonz_harness::clock::HarnessClock;
use macroonz_harness::corpus::{SeedInput, pack, warm_start};
use macroonz_harness::descriptor::{
    Binding, CheckRef, ClaimRef, Classification, DerivedRevision, ExecutableAttachment,
    ExecutionSuite, GeneratedSupportSchemaId, NamespacedName, Origin, PopulationRef, Provenance,
    RevisionBinding, Role, Row, SubjectRoute, Tag, TrialCoordinates, TrialKey,
};
use macroonz_harness::fuzz::{
    ComposeRefusal, CoverageAdmission, CoverageAdmissionRefusal, CoverageBudgets, CoverageCampaign,
    CoverageCorpus, CoveragePoint, CoverageProfile, CoverageReadRefusal, CoverageSourceRoot,
    CoverageSourceRootRefusal, FuzzExecution, InstrumentedTarget, InterestingBytes, MutationKind,
    MutationPlan, PreflightIncomplete, RUSTC_COVERAGE_TOOLCHAIN, ReadyPreflight,
    RustcProfileRefusal, RustcProfileRequest, RustcProfileRequestRefusal, compose_reduce_replay,
    neighboring_inputs, observe_rustc_profile, preflight_ready, read_lcov,
};
use macroonz_harness::generate::{
    ByteReducerId, ByteSource, CaseWidth, FingerprintPreservation, GenerationPlan, InputOrigin,
    PreconditionVerdict, ProbeOutcome, ReductionBudget, ReductionPlan, ReductionPlanRefusal,
    ReductionProbeBinding, ReductionRefusal, RejectionAllowance, SizeProgression, drive,
};
use macroonz_harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use macroonz_harness::report::{
    ByteBudget, CaseBudget, FailureClass, FindingCause, FindingLocation, Fingerprint,
    GenerationProfile, InvocationProfile, MinimizationProfile, ReplayPosture, TargetBinding,
    TargetTriple, TimeBudget, ToolchainIdentity, TrialConclusion, TrialFinding, TrialId,
    TrialProfile, TrialSite,
};
use macroonz_harness::runner::{Invocation, TrialBinding, run_one};
use std::cell::Cell;
use std::collections::BTreeSet;
use std::ffi::OsString;
use std::fmt;
use std::fs::File;
use std::path::PathBuf;
use std::process::{Command, Stdio};

const PRESERVED_CAUSE: FindingCause = FindingCause::named("harness", "fuzz-compose-preserved");
const SCHEMA_TAG: DomainTag =
    DomainTag::declared("fuzz-compose-schema", IdentityProfileVersion::declared(1));
const SUPERVISED_MATERIALIZED_INPUT_BYTES: usize = 16_777_216;

enum FuzzRoadFailure {
    Plan(ReductionPlanRefusal),
    Compose(ComposeRefusal),
    Preflight(PreflightIncomplete),
    CoverageAdmission(CoverageAdmissionRefusal),
    CoverageRead(CoverageReadRefusal),
    Profile(RustcProfileRefusal),
    External(String),
    Fixture,
}

impl fmt::Debug for FuzzRoadFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Plan(refusal) => formatter.debug_tuple("Plan").field(refusal).finish(),
            Self::Compose(refusal) => formatter.debug_tuple("Compose").field(refusal).finish(),
            Self::Preflight(refusal) => formatter.debug_tuple("Preflight").field(refusal).finish(),
            Self::CoverageAdmission(refusal) => formatter
                .debug_tuple("CoverageAdmission")
                .field(refusal)
                .finish(),
            Self::CoverageRead(refusal) => formatter
                .debug_tuple("CoverageRead")
                .field(refusal)
                .finish(),
            Self::Profile(refusal) => formatter.debug_tuple("Profile").field(refusal).finish(),
            Self::External(refusal) => formatter.debug_tuple("External").field(refusal).finish(),
            Self::Fixture => formatter.write_str("Fixture"),
        }
    }
}

impl From<ReductionPlanRefusal> for FuzzRoadFailure {
    fn from(refusal: ReductionPlanRefusal) -> Self {
        Self::Plan(refusal)
    }
}

impl From<ComposeRefusal> for FuzzRoadFailure {
    fn from(refusal: ComposeRefusal) -> Self {
        Self::Compose(refusal)
    }
}

impl From<PreflightIncomplete> for FuzzRoadFailure {
    fn from(refusal: PreflightIncomplete) -> Self {
        Self::Preflight(refusal)
    }
}

impl From<CoverageAdmissionRefusal> for FuzzRoadFailure {
    fn from(refusal: CoverageAdmissionRefusal) -> Self {
        Self::CoverageAdmission(refusal)
    }
}

impl From<CoverageReadRefusal> for FuzzRoadFailure {
    fn from(refusal: CoverageReadRefusal) -> Self {
        Self::CoverageRead(refusal)
    }
}

impl From<RustcProfileRefusal> for FuzzRoadFailure {
    fn from(refusal: RustcProfileRefusal) -> Self {
        Self::Profile(refusal)
    }
}

fn trial_fingerprint() -> Option<Fingerprint> {
    let coordinates = TrialCoordinates::over(
        ClaimRef::named("harness", "fuzz-compose").ok()?,
        SubjectRoute::named("harness", "byte-input").ok()?,
        CheckRef::named("harness", "fingerprint-preserved").ok()?,
        PopulationRef::named("harness", "fuzz-interesting").ok()?,
    );
    let key = TrialKey::over(coordinates);
    let trial = TrialId::of_key(key, TrialProfile::Unprofiled);
    Some(Fingerprint::over(
        trial,
        PRESERVED_CAUSE,
        FailureClass::PropertyDisagreement,
    ))
}

fn probe(input: &[u8]) -> ProbeOutcome {
    let Some(preserved) = trial_fingerprint() else {
        return ProbeOutcome::NoFailure;
    };
    match input {
        [1u8, 2u8, 3u8] | [1u8, 2u8] | [1u8] => ProbeOutcome::Reproduced(preserved),
        _ => ProbeOutcome::NoFailure,
    }
}

fn refused_trial(_invocation: &Invocation) -> TrialConclusion {
    TrialConclusion::Refused(TrialFinding::established(
        FailureClass::PropertyDisagreement,
        PRESERVED_CAUSE,
        FindingLocation::at(file!(), line!()),
        None,
    ))
}

fn trial_binding() -> Option<TrialBinding> {
    let subject = SubjectRoute::named("harness", "byte-input").ok()?;
    let check = CheckRef::named("harness", "fingerprint-preserved").ok()?;
    let row = Row::declared(
        ClaimRef::named("harness", "fuzz-compose").ok()?,
        ExecutionSuite::named("harness", "fuzz").ok()?,
        Classification::authored(
            vec![Role::named("harness", "fuzz").ok()?],
            vec![Tag::named("harness", "compose").ok()?],
        )
        .ok()?,
        subject,
        check,
        PopulationRef::named("harness", "fuzz-interesting").ok()?,
        Origin::HandWritten,
    )
    .ok()?;
    let revision = RevisionBinding::derived(DerivedRevision::from_material(b"fuzz-compose-trial"));
    Binding::bound(
        row,
        ExecutableAttachment::attached(subject, check, revision, revision, refused_trial),
        Provenance::Unproduced,
    )
    .ok()
}

fn invocation() -> Invocation {
    Invocation::declared(
        InvocationProfile::declared(
            CaseBudget::declared(1),
            ByteBudget::declared(64),
            TimeBudget::declared(1_000_000),
        ),
        TargetBinding::bound(
            TargetTriple::declared("x86_64-pc-windows-msvc"),
            ToolchainIdentity::declared("1.98.0"),
        ),
        TrialSite::located(module_path!(), file!(), line!(), "fuzz-compose"),
        HarnessClock::unavailable(),
    )
}

fn probe_binding() -> Option<ReductionProbeBinding> {
    let trial = trial_binding()?;
    let report = run_one(&trial, &invocation());
    ReductionProbeBinding::bound(
        &report,
        GenerationProfile::declared("fuzz-interesting", 1),
        GeneratedSupportSchemaId::over(ContentAddress::derived(SCHEMA_TAG, b"schema")),
        RevisionBinding::derived(DerivedRevision::from_material(b"fuzz-compose-probe")),
        probe,
    )
    .ok()
}

#[test]
fn declared_execution_inputs_refuse_ambient_paths() -> Result<(), FuzzRoadFailure> {
    assert_eq!(RUSTC_COVERAGE_TOOLCHAIN, "1.98.0");
    assert_eq!(
        InstrumentedTarget::declared(PathBuf::from("target"), Vec::new()),
        Err(RustcProfileRequestRefusal::RelativeTarget)
    );
    let Some(logical) = NamespacedName::named("harness", "rustc-coverage").ok() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    assert_eq!(
        CoverageSourceRoot::declared(logical, PathBuf::from("checkout")),
        Err(CoverageSourceRootRefusal::RelativeCheckout)
    );
    let traversing = std::env::temp_dir().join("coverage-root").join("..");
    assert_eq!(
        CoverageSourceRoot::declared(logical, traversing),
        Err(CoverageSourceRootRefusal::CheckoutTraversal)
    );
    Ok(())
}

#[test]
fn hostile_surface_refuses_malformed_fuzz_road() -> Result<(), FuzzRoadFailure> {
    let interesting = interesting_bytes("hostile-compose", &[9])?;
    let plan = ReductionPlan::declared(
        MinimizationProfile::declared("fuzz-compose-hostile", 1),
        ByteReducerId::ChunkRemovalAndZeroing,
        Vec::new(),
        FingerprintPreservation::Required,
        ReductionBudget::declared(4),
    )?;
    let Some(binding) = probe_binding() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    match compose_reduce_replay(&interesting, &plan, &binding) {
        Err(ComposeRefusal::Reduction(ReductionRefusal::BaselineDidNotFail)) => Ok(()),
        Err(refusal) => Err(FuzzRoadFailure::Compose(refusal)),
        Ok(_) => Err(FuzzRoadFailure::Fixture),
    }
}

#[test]
fn lcov_points_are_canonical_across_declared_source_roots() -> Result<(), FuzzRoadFailure> {
    let Some(logical) = NamespacedName::named("harness", "rustc-coverage-source").ok() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    let first_checkout = std::env::temp_dir().join("macroonz-coverage-first");
    let second_checkout = std::env::temp_dir().join("macroonz-coverage-second");
    let first_root =
        CoverageSourceRoot::declared(logical, first_checkout.clone()).map_err(external)?;
    let second_root =
        CoverageSourceRoot::declared(logical, second_checkout.clone()).map_err(external)?;
    let first_source = first_checkout.join("src").join("subject.rs");
    let second_source = second_checkout.join("src").join("subject.rs");
    let alpha_lcov = format!(
        "TN:\nSF:{}\nDA:10,1\nDA:11,0\nBRDA:12,0,0,1\nBRDA:12,0,1,-\nDA:10,4\nend_of_record\n",
        first_source.display()
    );
    let relocated_lcov = format!(
        "TN:\nSF:{}\nDA:10,1\nDA:11,0\nBRDA:12,0,0,1\nBRDA:12,0,1,-\nDA:10,4\nend_of_record\n",
        second_source.display()
    );
    let alpha = read_lcov(&first_root, alpha_lcov.as_bytes())?;
    let relocated = read_lcov(&second_root, relocated_lcov.as_bytes())?;
    assert_eq!(alpha, relocated);
    let [line_point, branch_point] = alpha.points() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    let CoveragePoint::Line {
        source: line_source,
        line: line_number,
    } = line_point
    else {
        return Err(FuzzRoadFailure::Fixture);
    };
    assert_eq!(line_source.root(), logical);
    assert_eq!(line_source.relative(), "src/subject.rs");
    assert_eq!(*line_number, 10);
    let CoveragePoint::Branch {
        source: branch_source,
        line: branch_line,
        block,
        branch,
    } = branch_point
    else {
        return Err(FuzzRoadFailure::Fixture);
    };
    assert_eq!(branch_source.root(), logical);
    assert_eq!(branch_source.relative(), "src/subject.rs");
    assert_eq!((*branch_line, *block, *branch), (12, 0, 0));
    assert!(!format!("{alpha:?}").contains(&first_checkout.display().to_string()));

    #[cfg(windows)]
    {
        let verbatim = format!(r"\\?\{}", first_source.display());
        let verbatim_lcov = format!(
            "TN:\nSF:{verbatim}\nDA:10,1\nDA:11,0\nBRDA:12,0,0,1\nBRDA:12,0,1,-\nDA:10,4\nend_of_record\n"
        );
        assert_eq!(read_lcov(&first_root, verbatim_lcov.as_bytes())?, alpha);
    }

    Ok(())
}

#[test]
fn compiled_coverage_is_identical_across_two_physical_source_roots() -> Result<(), FuzzRoadFailure>
{
    let rustc = rustc_path()?;
    let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or_else(|| FuzzRoadFailure::External("harness has no repository parent".to_owned()))?
        .to_path_buf();
    let run = repository
        .join("target")
        .join("qualification")
        .join(format!("fuzz-two-source-roots-{}", std::process::id()));
    let first_root = run.join("first-root");
    let second_root = run.join("second-root");
    let first_source = first_root.join("src").join("subject.rs");
    let second_source = second_root.join("src").join("subject.rs");
    std::fs::create_dir_all(first_source.parent().ok_or(FuzzRoadFailure::Fixture)?)
        .map_err(external)?;
    std::fs::create_dir_all(second_source.parent().ok_or(FuzzRoadFailure::Fixture)?)
        .map_err(external)?;
    let source = include_bytes!("rustc_coverage_subject.rs");
    std::fs::write(&first_source, source).map_err(external)?;
    std::fs::write(&second_source, source).map_err(external)?;
    let first_subject = first_root.join(format!("subject{}", std::env::consts::EXE_SUFFIX));
    let second_subject = second_root.join(format!("subject{}", std::env::consts::EXE_SUFFIX));
    compile_instrumented_subject(&rustc, &first_source, &first_subject)?;
    compile_instrumented_subject(&rustc, &second_source, &second_subject)?;
    let campaign = coverage_campaign()?;
    let first_ready = ready_for_compiled_root(
        rustc.clone(),
        first_subject,
        &first_root,
        run.join("first-cases"),
        campaign,
    )?;
    let second_ready = ready_for_compiled_root(
        rustc,
        second_subject,
        &second_root,
        run.join("second-cases"),
        campaign,
    )?;
    let mut first_corpus = CoverageCorpus::opening(&first_ready);
    let mut second_corpus = CoverageCorpus::opening(&second_ready);
    let first = observe_rustc_profile(&first_ready, &mut first_corpus, &[1], wait_for_exit)?;
    let second = observe_rustc_profile(&second_ready, &mut second_corpus, &[1], wait_for_exit)?;
    assert_eq!(first.standing(), second.standing());
    assert_eq!(first.observation(), second.observation());
    assert!(!first.observation().points().is_empty());
    std::fs::remove_dir_all(run).map_err(external)?;
    Ok(())
}

#[test]
fn lcov_refuses_paths_that_cannot_have_root_independent_identity() -> Result<(), FuzzRoadFailure> {
    let Some(logical) = NamespacedName::named("harness", "rustc-coverage-hostile").ok() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    let checkout = std::env::temp_dir().join("macroonz-coverage-root");
    let root = CoverageSourceRoot::declared(logical, checkout.clone()).map_err(external)?;
    assert_eq!(
        read_lcov(&root, b"TN:\nSF:src/subject.rs\nDA:1,1\nend_of_record\n"),
        Err(CoverageReadRefusal::RelativeSource { record: 2 })
    );
    let traversing = format!(
        "TN:\nSF:{}\nDA:1,1\nend_of_record\n",
        checkout.join("src").join("..").join("escape.rs").display()
    );
    assert_eq!(
        read_lcov(&root, traversing.as_bytes()),
        Err(CoverageReadRefusal::SourceTraversal { record: 2 })
    );
    let outside = format!(
        "TN:\nSF:{}\nDA:1,1\nend_of_record\n",
        std::env::temp_dir().join("macroonz-outside.rs").display()
    );
    assert_eq!(
        read_lcov(&root, outside.as_bytes()),
        Err(CoverageReadRefusal::SourceOutsideRoot { record: 2 })
    );
    let root_only = format!("TN:\nSF:{}\nDA:1,1\nend_of_record\n", checkout.display());
    assert_eq!(
        read_lcov(&root, root_only.as_bytes()),
        Err(CoverageReadRefusal::EmptyRelativeSource { record: 2 })
    );
    assert_eq!(read_lcov(&root, &[0xff]), Err(CoverageReadRefusal::NonUtf8));
    Ok(())
}

#[test]
fn neighboring_frontier_is_bounded_unique_and_repeatable() -> Result<(), FuzzRoadFailure> {
    let plan = MutationPlan::declared(128, 16, vec![b"token".to_vec()]).map_err(external)?;
    let first = neighboring_inputs(&[11, 200], Some(b"peer"), &plan).map_err(external)?;
    let second = neighboring_inputs(&[11, 200], Some(b"peer"), &plan).map_err(external)?;
    assert_eq!(first, second);
    assert!(!first.is_empty());
    assert!(first.len() <= 128);
    assert!(
        first
            .iter()
            .all(|candidate| { !candidate.bytes().is_empty() && candidate.bytes().len() <= 16 })
    );
    let unique = first
        .iter()
        .map(|candidate| candidate.bytes().to_vec())
        .collect::<BTreeSet<_>>();
    assert_eq!(unique.len(), first.len());
    for kind in [
        MutationKind::BitFlip,
        MutationKind::BoundarySubstitution,
        MutationKind::Increment,
        MutationKind::Decrement,
        MutationKind::Delete,
        MutationKind::InsertBoundary,
        MutationKind::Duplicate,
        MutationKind::Splice,
        MutationKind::DictionaryInsert,
    ] {
        assert!(first.iter().any(|candidate| candidate.kind() == kind));
    }
    Ok(())
}

#[test]
fn neighboring_frontier_budget_is_an_exact_priority_prefix() -> Result<(), FuzzRoadFailure> {
    let exhaustive = MutationPlan::declared(512, 16, vec![b"token".to_vec()]).map_err(external)?;
    let full = neighboring_inputs(&[11, 200], Some(b"peer"), &exhaustive).map_err(external)?;
    for limit in 1..=full.len() {
        let budget = u32::try_from(limit).map_err(external)?;
        let bounded =
            MutationPlan::declared(budget, 16, vec![b"token".to_vec()]).map_err(external)?;
        let observed = neighboring_inputs(&[11, 200], Some(b"peer"), &bounded).map_err(external)?;
        let expected = full.iter().take(limit).cloned().collect::<Vec<_>>();
        assert_eq!(observed, expected);
    }
    let over_budget = u32::try_from(full.len().saturating_add(1)).map_err(external)?;
    let exhausted =
        MutationPlan::declared(over_budget, 16, vec![b"token".to_vec()]).map_err(external)?;
    assert_eq!(
        neighboring_inputs(&[11, 200], Some(b"peer"), &exhausted).map_err(external)?,
        full
    );

    let eight = MutationPlan::declared(8, 4, Vec::new()).map_err(external)?;
    let bit_prefix = neighboring_inputs(&[0], None, &eight).map_err(external)?;
    assert_eq!(bit_prefix.len(), 8);
    assert!(
        bit_prefix
            .iter()
            .all(|candidate| candidate.kind() == MutationKind::BitFlip)
    );
    Ok(())
}

#[test]
fn stable_rustc_profiles_cross_generation_novelty_and_corpus() -> Result<(), FuzzRoadFailure> {
    let (ready, run) = rustc_profile_request("feedback")?;
    let Some(population) = PopulationRef::named("harness", "rustc-profile-seeds").ok() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    let seeds = vec![SeedInput::declared(vec![0]).map_err(external)?];
    let supplied = pack(population, seeds).map_err(external)?;
    let mut coverage = CoverageCorpus::opening(&ready);

    for origin in warm_start(&supplied) {
        let InputOrigin::Supplied(material) = origin else {
            return Err(FuzzRoadFailure::Fixture);
        };
        let width = CaseWidth::declared(material.len()).map_err(external)?;
        let bytes = u64::try_from(material.len()).map_err(external)?;
        let plan = GenerationPlan::declared(
            population,
            GenerationProfile::declared("rustc-profile-candidate", 1),
            InputOrigin::Supplied(material.clone()),
            CaseBudget::declared(1),
            ByteBudget::declared(bytes),
            RejectionAllowance::NoRejections,
            SizeProgression::Constant { width },
        )
        .map_err(external)?;
        let source = ByteSource::of_plan(&plan);
        let generated = drive::<u8>(
            &plan,
            &source,
            macroonz_harness::generate::decode_arbitrary::<u8>,
            admit_byte_sequences,
        );
        let [candidate] = generated.sequences() else {
            return Err(FuzzRoadFailure::Fixture);
        };
        assert_eq!(candidate.input(), material.as_slice());
        let result =
            observe_rustc_profile(&ready, &mut coverage, candidate.input(), wait_for_exit)?;
        assert_eq!(result.execution(), FuzzExecution::Success);
        match coverage.admit(result)? {
            CoverageAdmission::Interesting(_) => {}
            CoverageAdmission::Known => return Err(FuzzRoadFailure::Fixture),
        }
    }

    let mutation = MutationPlan::declared(8, 4, Vec::new()).map_err(external)?;
    let neighbors = neighboring_inputs(&[0], None, &mutation).map_err(external)?;
    let mut known = 0usize;
    for candidate in &neighbors {
        let result =
            observe_rustc_profile(&ready, &mut coverage, candidate.bytes(), wait_for_exit)?;
        assert_eq!(result.execution(), FuzzExecution::Success);
        match coverage.admit(result)? {
            CoverageAdmission::Interesting(_) => {}
            CoverageAdmission::Known => known = known.saturating_add(1),
        }
    }

    assert_eq!(neighbors.len(), 8);
    assert!(known > 0);
    assert_eq!(coverage.interesting().len(), 4);
    let evolved = coverage
        .interesting()
        .iter()
        .map(|interesting| SeedInput::declared(interesting.as_bytes().to_vec()).map_err(external))
        .collect::<Result<Vec<_>, _>>()?;
    let retained = pack(population, evolved).map_err(external)?;
    assert_eq!(retained.seeds().len(), 4);
    assert_eq!(
        retained
            .seeds()
            .iter()
            .map(SeedInput::bytes)
            .collect::<Vec<_>>(),
        vec![&[0][..], &[1][..], &[2][..], &[0x80][..]]
    );
    std::fs::remove_dir_all(run).map_err(external)?;
    Ok(())
}

#[test]
fn campaign_join_and_execution_budgets_refuse_before_process_start() -> Result<(), FuzzRoadFailure>
{
    let (first_ready, first_run) = rustc_profile_request("campaign-join-first")?;
    let (other_ready, other_run) = rustc_profile_request_with_campaign(
        "campaign-join-other",
        Vec::new(),
        alternate_coverage_campaign()?,
    )?;
    let mut first = CoverageCorpus::opening(&first_ready);
    assert_eq!(
        observe_rustc_profile(&other_ready, &mut first, &[0], wait_for_exit),
        Err(RustcProfileRefusal::CampaignMismatch)
    );
    assert_eq!(first.attempted_cases(), 0);

    let case_campaign =
        coverage_campaign_with_budgets(coverage_budgets(1, 8, 33_554_432, 1_000_000, 1, 8)?)?;
    let (case_ready, case_run) =
        rustc_profile_request_with_campaign("case-budget", Vec::new(), case_campaign)?;
    let mut cases = CoverageCorpus::opening(&case_ready);
    let first_result = observe_rustc_profile(&case_ready, &mut cases, &[0], wait_for_exit)?;
    assert_eq!(first_result.execution(), FuzzExecution::Success);
    assert_eq!(
        observe_rustc_profile(&case_ready, &mut cases, &[1], wait_for_exit),
        Err(RustcProfileRefusal::CaseBudgetExhausted { bound: 1 })
    );

    let input_campaign =
        coverage_campaign_with_budgets(coverage_budgets(2, 1, 33_554_432, 1_000_000, 1, 8)?)?;
    let (input_ready, input_run) =
        rustc_profile_request_with_campaign("input-budget", Vec::new(), input_campaign)?;
    let mut inputs = CoverageCorpus::opening(&input_ready);
    assert_eq!(
        observe_rustc_profile(&input_ready, &mut inputs, &[0, 1], wait_for_exit),
        Err(RustcProfileRefusal::InputBudgetExhausted {
            bound: 1,
            attempted: 2,
        })
    );
    assert_eq!(inputs.attempted_cases(), 0);

    for run in [first_run, other_run, case_run, input_run] {
        std::fs::remove_dir_all(run).map_err(external)?;
    }
    Ok(())
}

#[test]
fn an_existing_case_directory_keeps_its_specific_refusal() -> Result<(), FuzzRoadFailure> {
    let (ready, run) = rustc_profile_request("existing-case-directory")?;
    let case = run.join("cases").join("case-00000000000000000000");
    std::fs::create_dir_all(&case).map_err(external)?;
    let mut coverage = CoverageCorpus::opening(&ready);
    assert_eq!(
        observe_rustc_profile(&ready, &mut coverage, &[0], wait_for_exit),
        Err(RustcProfileRefusal::CaseAlreadyExists(case))
    );
    assert_eq!(coverage.attempted_cases(), 1);
    std::fs::remove_dir_all(run).map_err(external)?;
    Ok(())
}

#[test]
fn coverage_export_and_point_budgets_refuse_atomically() -> Result<(), FuzzRoadFailure> {
    let export_campaign =
        coverage_campaign_with_budgets(coverage_budgets(1, 1, 1, 1_000_000, 1, 1)?)?;
    let (export_ready, export_run) =
        rustc_profile_request_with_campaign("export-budget", Vec::new(), export_campaign)?;
    let mut exports = CoverageCorpus::opening(&export_ready);
    let Err(RustcProfileRefusal::CovOutputBudgetExhausted {
        bound,
        observed_at_least,
    }) = observe_rustc_profile(&export_ready, &mut exports, &[0], wait_for_exit)
    else {
        return Err(FuzzRoadFailure::Fixture);
    };
    assert_eq!(bound, 1);
    assert_eq!(observed_at_least, 2);
    assert!(
        std::fs::read_dir(export_run.join("cases"))
            .map_err(external)?
            .next()
            .is_none()
    );

    let point_campaign =
        coverage_campaign_with_budgets(coverage_budgets(1, 1, 33_554_432, 1, 1, 1)?)?;
    let (point_ready, point_run) =
        rustc_profile_request_with_campaign("point-budget", Vec::new(), point_campaign)?;
    let mut points = CoverageCorpus::opening(&point_ready);
    let result = observe_rustc_profile(&point_ready, &mut points, &[0], wait_for_exit)?;
    let attempted = u64::try_from(result.observation().points().len()).map_err(external)?;
    assert!(attempted > 1);
    assert_eq!(
        points.admit(result),
        Err(CoverageAdmissionRefusal::PointBudgetExhausted {
            bound: 1,
            attempted,
        })
    );
    assert!(points.observed().is_empty());
    assert!(points.interesting().is_empty());

    for run in [export_run, point_run] {
        std::fs::remove_dir_all(run).map_err(external)?;
    }
    Ok(())
}

#[test]
fn exact_coverage_export_byte_ceiling_is_inclusive() -> Result<(), FuzzRoadFailure> {
    let (ready, run) = rustc_profile_request("exact-export-bound")?;
    let export_bytes = coverage_export_size(&ready, &run, &[0])?;
    if export_bytes == 0 {
        return Err(FuzzRoadFailure::Fixture);
    }
    let campaign =
        coverage_campaign_with_budgets(coverage_budgets(1, 1, export_bytes, 1_000_000, 1, 1)?)?;
    let exact = rebound_ready(&ready, &run, "exact-export-cases", campaign)?;
    let mut coverage = CoverageCorpus::opening(&exact);
    let result = observe_rustc_profile(&exact, &mut coverage, &[0], wait_for_exit)?;
    assert_eq!(result.execution(), FuzzExecution::Success);
    assert!(!result.observation().points().is_empty());
    std::fs::remove_dir_all(run).map_err(external)?;
    Ok(())
}

#[test]
fn exact_coverage_point_ceiling_is_inclusive() -> Result<(), FuzzRoadFailure> {
    let (ready, run) = rustc_profile_request("exact-point-bound")?;
    let mut discovery = CoverageCorpus::opening(&ready);
    let observed = observe_rustc_profile(&ready, &mut discovery, &[0], wait_for_exit)?;
    let points = u64::try_from(observed.observation().points().len()).map_err(external)?;
    if points == 0 {
        return Err(FuzzRoadFailure::Fixture);
    }
    let campaign =
        coverage_campaign_with_budgets(coverage_budgets(1, 1, 33_554_432, points, 1, 1)?)?;
    let exact = rebound_ready(&ready, &run, "exact-point-cases", campaign)?;
    let mut coverage = CoverageCorpus::opening(&exact);
    let result = observe_rustc_profile(&exact, &mut coverage, &[0], wait_for_exit)?;
    assert!(matches!(
        coverage.admit(result)?,
        CoverageAdmission::Interesting(_)
    ));
    assert_eq!(
        u64::try_from(coverage.observed().len()).map_err(external)?,
        points
    );
    std::fs::remove_dir_all(run).map_err(external)?;
    Ok(())
}

#[test]
fn coverage_retention_budgets_refuse_without_advancing_the_frontier() -> Result<(), FuzzRoadFailure>
{
    let case_campaign =
        coverage_campaign_with_budgets(coverage_budgets(2, 2, 33_554_432, 1_000_000, 1, 8)?)?;
    let (case_ready, case_run) =
        rustc_profile_request_with_campaign("retained-case-budget", Vec::new(), case_campaign)?;
    let mut cases = CoverageCorpus::opening(&case_ready);
    let case_first = observe_rustc_profile(&case_ready, &mut cases, &[0], wait_for_exit)?;
    assert!(matches!(
        cases.admit(case_first)?,
        CoverageAdmission::Interesting(_)
    ));
    let case_points_before = cases.observed().clone();
    let case_second = observe_rustc_profile(&case_ready, &mut cases, &[1], wait_for_exit)?;
    assert_eq!(
        cases.admit(case_second),
        Err(CoverageAdmissionRefusal::RetainedCaseBudgetExhausted { bound: 1 })
    );
    assert_eq!(cases.observed(), &case_points_before);
    assert_eq!(cases.interesting().len(), 1);

    let byte_campaign =
        coverage_campaign_with_budgets(coverage_budgets(2, 3, 33_554_432, 1_000_000, 2, 1)?)?;
    let (byte_ready, byte_run) =
        rustc_profile_request_with_campaign("retained-byte-budget", Vec::new(), byte_campaign)?;
    let mut bytes = CoverageCorpus::opening(&byte_ready);
    let byte_first = observe_rustc_profile(&byte_ready, &mut bytes, &[0], wait_for_exit)?;
    assert!(matches!(
        bytes.admit(byte_first)?,
        CoverageAdmission::Interesting(_)
    ));
    let byte_points_before = bytes.observed().clone();
    let byte_second = observe_rustc_profile(&byte_ready, &mut bytes, &[1, 0], wait_for_exit)?;
    assert_eq!(
        bytes.admit(byte_second),
        Err(CoverageAdmissionRefusal::RetainedByteBudgetExhausted {
            bound: 1,
            attempted: 3,
        })
    );
    assert_eq!(bytes.observed(), &byte_points_before);
    assert_eq!(bytes.interesting().len(), 1);
    assert_eq!(bytes.retained_bytes(), 1);

    for run in [case_run, byte_run] {
        std::fs::remove_dir_all(run).map_err(external)?;
    }
    Ok(())
}

#[test]
fn active_preflight_refuses_wrong_release_and_mismatched_llvm() -> Result<(), FuzzRoadFailure> {
    let rustc = rustc_path()?;
    let host = rustc_field(&rustc, "host: ")?;
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repository = manifest
        .parent()
        .ok_or_else(|| FuzzRoadFailure::External("harness has no repository parent".to_owned()))?;
    let run = repository
        .join("target")
        .join("qualification")
        .join(format!(
            "fuzz-preflight-refusal-test-{}",
            std::process::id()
        ));

    let wrong_release = preflight_double(
        &rustc,
        &manifest,
        &run.join("wrong-release"),
        &host,
        ["1.97.0", "22.1.8", "22.1.8", "22.1.8"],
    )?;
    let wrong_request = preflight_double_request(wrong_release, repository, &run, "wrong-release")?;
    let Err(wrong_refusal) = preflight_ready(wrong_request) else {
        return Err(FuzzRoadFailure::Fixture);
    };
    assert_eq!(
        wrong_refusal,
        PreflightIncomplete::RustcRelease {
            required: RUSTC_COVERAGE_TOOLCHAIN,
            observed: "1.97.0".to_owned(),
        }
    );

    let mismatched_tools = preflight_double(
        &rustc,
        &manifest,
        &run.join("mismatched-tools"),
        &host,
        [RUSTC_COVERAGE_TOOLCHAIN, "22.1.8", "22.1.8", "22.1.9"],
    )?;
    let mismatch_request =
        preflight_double_request(mismatched_tools, repository, &run, "mismatched-tools")?;
    let Err(mismatch_refusal) = preflight_ready(mismatch_request) else {
        return Err(FuzzRoadFailure::Fixture);
    };
    assert_eq!(
        mismatch_refusal,
        PreflightIncomplete::LlvmToolVersionsDiffer {
            profdata: "22.1.8".to_owned(),
            cov: "22.1.9".to_owned(),
        }
    );
    std::fs::remove_dir_all(run).map_err(external)?;
    Ok(())
}

#[test]
fn declared_supervisor_transports_crash_timeout_and_resource_classes() -> Result<(), FuzzRoadFailure>
{
    let (ready, run) = rustc_profile_request("classifications")?;
    let mut coverage = CoverageCorpus::opening(&ready);
    assert_eq!(ready.release(), RUSTC_COVERAGE_TOOLCHAIN);
    assert!(!ready.host().is_empty());
    assert!(!ready.llvm_version().is_empty());
    assert!(ready.sysroot().is_absolute());
    assert_eq!(ready.standing().target().target().spelling(), ready.host());
    assert_eq!(
        ready.standing().target().toolchain().spelling(),
        format!("rustc {} LLVM {}", ready.release(), ready.llvm_version())
    );
    let crash = observe_rustc_profile(&ready, &mut coverage, &[0xff], wait_for_crash)?;
    assert!(matches!(crash.execution(), FuzzExecution::Crash(_)));
    assert!(crash.observation().points().is_empty());
    let timeout = observe_rustc_profile(&ready, &mut coverage, &[0xfe], |child| {
        stop_as(child, FuzzExecution::Timeout)
    })?;
    assert_eq!(timeout.execution(), FuzzExecution::Timeout);
    assert!(timeout.observation().points().is_empty());
    let resource = observe_rustc_profile(&ready, &mut coverage, &[0xfe], |child| {
        stop_as(child, FuzzExecution::ResourceExhaustion)
    })?;
    assert_eq!(resource.execution(), FuzzExecution::ResourceExhaustion);
    assert!(resource.observation().points().is_empty());

    let early_process = Cell::new(None);
    assert_eq!(
        observe_rustc_profile(&ready, &mut coverage, &[0xfe], |child| {
            early_process.set(Some(child.id()));
            Ok(FuzzExecution::Timeout)
        }),
        Err(RustcProfileRefusal::SupervisorReturnedBeforeExit)
    );
    let Some(early_process) = early_process.get() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    assert!(!process_is_running(early_process)?);

    let refused_process = Cell::new(None);
    assert_eq!(
        observe_rustc_profile(&ready, &mut coverage, &[0xfe], |child| {
            refused_process.set(Some(child.id()));
            Err("planted supervisor refusal".to_owned())
        }),
        Err(RustcProfileRefusal::SuperviseTarget(
            "planted supervisor refusal".to_owned()
        ))
    );
    let Some(refused_process) = refused_process.get() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    assert!(!process_is_running(refused_process)?);

    let (large_input_ready, large_input_run) = rustc_profile_request_with_arguments(
        "large-input-supervision",
        vec!["--park-before-read".to_owned()],
    )?;
    let mut large_input_coverage = CoverageCorpus::opening(&large_input_ready);
    let large_input = observe_rustc_profile(
        &large_input_ready,
        &mut large_input_coverage,
        &vec![0_u8; SUPERVISED_MATERIALIZED_INPUT_BYTES],
        |child| stop_as(child, FuzzExecution::Timeout),
    )?;
    assert_eq!(large_input.execution(), FuzzExecution::Timeout);
    std::fs::remove_dir_all(large_input_run).map_err(external)?;
    std::fs::remove_dir_all(run).map_err(external)?;
    Ok(())
}

fn admit_byte_sequences(_commands: &[u8]) -> PreconditionVerdict {
    PreconditionVerdict::Admitted
}

fn coverage_campaign() -> Result<CoverageCampaign, FuzzRoadFailure> {
    let budgets = coverage_budgets(512, 33_554_432, 33_554_432, 1_000_000, 128, 1_048_576)?;
    coverage_campaign_with_budgets(budgets)
}

fn coverage_budgets(
    executions: u32,
    input_bytes: u64,
    export_bytes: u64,
    points: u64,
    retained_cases: u32,
    retained_bytes: u64,
) -> Result<CoverageBudgets, FuzzRoadFailure> {
    CoverageBudgets::declared(
        CaseBudget::declared(executions),
        ByteBudget::declared(input_bytes),
        export_bytes,
        points,
        CaseBudget::declared(retained_cases),
        ByteBudget::declared(retained_bytes),
    )
    .map_err(external)
}

fn coverage_campaign_with_budgets(
    budgets: CoverageBudgets,
) -> Result<CoverageCampaign, FuzzRoadFailure> {
    let Some(population) = PopulationRef::named("harness", "rustc-profile-candidates").ok() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    let Some(profile) = NamespacedName::named("harness", "rustc-region-coverage").ok() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    let revision = RevisionBinding::derived(DerivedRevision::from_material(include_bytes!(
        "rustc_coverage_subject.rs"
    )));
    Ok(CoverageCampaign::declared(
        population,
        revision,
        CoverageProfile::declared(profile, 1),
        budgets,
    ))
}

fn alternate_coverage_campaign() -> Result<CoverageCampaign, FuzzRoadFailure> {
    let campaign = coverage_campaign()?;
    let Some(population) = PopulationRef::named("harness", "another-rustc-profile-campaign").ok()
    else {
        return Err(FuzzRoadFailure::Fixture);
    };
    Ok(CoverageCampaign::declared(
        population,
        campaign.revision(),
        campaign.profile(),
        campaign.budgets(),
    ))
}

fn rustc_profile_request(stem: &str) -> Result<(ReadyPreflight, PathBuf), FuzzRoadFailure> {
    rustc_profile_request_with_arguments(stem, Vec::new())
}

fn rustc_profile_request_with_arguments(
    stem: &str,
    arguments: Vec<String>,
) -> Result<(ReadyPreflight, PathBuf), FuzzRoadFailure> {
    rustc_profile_request_with_campaign(stem, arguments, coverage_campaign()?)
}

fn rustc_profile_request_with_campaign(
    stem: &str,
    arguments: Vec<String>,
    campaign: CoverageCampaign,
) -> Result<(ReadyPreflight, PathBuf), FuzzRoadFailure> {
    let rustc = rustc_path()?;
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repository = manifest
        .parent()
        .ok_or_else(|| FuzzRoadFailure::External("harness has no repository parent".to_owned()))?;
    let run = repository
        .join("target")
        .join("qualification")
        .join(format!(
            "fuzz-rustc-profile-test-{}-{stem}",
            std::process::id()
        ));
    std::fs::create_dir_all(&run).map_err(external)?;
    let subject = run.join(format!(
        "rustc-coverage-subject{}",
        std::env::consts::EXE_SUFFIX
    ));
    let source = manifest
        .join("tests")
        .join("fuzz_compose")
        .join("rustc_coverage_subject.rs");
    compile_instrumented_subject(&rustc, &source, &subject)?;
    let target = InstrumentedTarget::declared(subject, arguments).map_err(external)?;
    let Some(logical) = NamespacedName::named("harness", "rustc-profile-subject").ok() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    let source_root =
        CoverageSourceRoot::declared(logical, repository.to_path_buf()).map_err(external)?;
    let request =
        RustcProfileRequest::declared(rustc, target, source_root, run.join("cases"), campaign)
            .map_err(external)?;
    let ready = preflight_ready(request)?;
    Ok((ready, run))
}

fn compile_instrumented_subject(
    rustc: &std::path::Path,
    source: &std::path::Path,
    subject: &std::path::Path,
) -> Result<(), FuzzRoadFailure> {
    let status = Command::new(rustc)
        .args([
            "--edition=2024",
            "-C",
            "instrument-coverage",
            "-C",
            "opt-level=0",
        ])
        .arg(source)
        .arg("-o")
        .arg(subject)
        .status()
        .map_err(external)?;
    if !status.success() {
        return Err(FuzzRoadFailure::External(format!(
            "rustc coverage subject compilation failed with {status}"
        )));
    }
    Ok(())
}

fn ready_for_compiled_root(
    rustc: PathBuf,
    subject: PathBuf,
    source_root: &std::path::Path,
    scratch: PathBuf,
    campaign: CoverageCampaign,
) -> Result<ReadyPreflight, FuzzRoadFailure> {
    let target = InstrumentedTarget::declared(subject, Vec::new()).map_err(external)?;
    let Some(logical) = NamespacedName::named("harness", "rustc-profile-subject").ok() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    let root =
        CoverageSourceRoot::declared(logical, source_root.to_path_buf()).map_err(external)?;
    let request =
        RustcProfileRequest::declared(rustc, target, root, scratch, campaign).map_err(external)?;
    preflight_ready(request).map_err(FuzzRoadFailure::Preflight)
}

fn rebound_ready(
    _ready: &ReadyPreflight,
    run: &std::path::Path,
    scratch: &str,
    campaign: CoverageCampaign,
) -> Result<ReadyPreflight, FuzzRoadFailure> {
    let target =
        InstrumentedTarget::declared(profile_subject(run), Vec::new()).map_err(external)?;
    let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or_else(|| FuzzRoadFailure::External("harness has no repository parent".to_owned()))?
        .to_path_buf();
    let Some(logical) = NamespacedName::named("harness", "rustc-profile-subject").ok() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    let root = CoverageSourceRoot::declared(logical, repository).map_err(external)?;
    let request =
        RustcProfileRequest::declared(rustc_path()?, target, root, run.join(scratch), campaign)
            .map_err(external)?;
    preflight_ready(request).map_err(FuzzRoadFailure::Preflight)
}

fn coverage_export_size(
    ready: &ReadyPreflight,
    run: &std::path::Path,
    candidate: &[u8],
) -> Result<u64, FuzzRoadFailure> {
    let probe = run.join("export-size-probe");
    std::fs::create_dir(&probe).map_err(external)?;
    let input_path = probe.join("candidate.bin");
    let raw = probe.join("coverage.profraw");
    let merged = probe.join("coverage.profdata");
    std::fs::write(&input_path, candidate).map_err(external)?;
    let input = File::open(&input_path).map_err(external)?;
    let subject = profile_subject(run);
    let status = Command::new(&subject)
        .env("LLVM_PROFILE_FILE", &raw)
        .stdin(Stdio::from(input))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(external)?;
    if !status.success() || !raw.is_file() {
        return Err(FuzzRoadFailure::External(
            "coverage-size probe did not produce a successful raw profile".to_owned(),
        ));
    }
    let tools = ready
        .sysroot()
        .join("lib")
        .join("rustlib")
        .join(ready.host())
        .join("bin");
    let profdata = tools.join(format!("llvm-profdata{}", std::env::consts::EXE_SUFFIX));
    let cov = tools.join(format!("llvm-cov{}", std::env::consts::EXE_SUFFIX));
    let merge = Command::new(profdata)
        .arg("merge")
        .arg("-sparse")
        .arg(&raw)
        .arg("-o")
        .arg(&merged)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(external)?;
    if !merge.success() {
        return Err(FuzzRoadFailure::External(
            "coverage-size profile merge failed".to_owned(),
        ));
    }
    let mut profile = OsString::from("-instr-profile=");
    profile.push(merged.as_os_str());
    let output = Command::new(cov)
        .arg("export")
        .arg("-format=lcov")
        .arg(profile)
        .arg(subject)
        .output()
        .map_err(external)?;
    if !output.status.success() {
        return Err(FuzzRoadFailure::External(
            "coverage-size export failed".to_owned(),
        ));
    }
    let bytes = u64::try_from(output.stdout.len()).map_err(external)?;
    std::fs::remove_dir_all(probe).map_err(external)?;
    Ok(bytes)
}

fn profile_subject(run: &std::path::Path) -> PathBuf {
    run.join(format!(
        "rustc-coverage-subject{}",
        std::env::consts::EXE_SUFFIX
    ))
}

fn rustc_path() -> Result<PathBuf, FuzzRoadFailure> {
    let output = successful_output(
        Command::new("rustup").args(["which", "--toolchain", RUSTC_COVERAGE_TOOLCHAIN, "rustc"]),
        "rustup rustc path",
    )?;
    let path = PathBuf::from(String::from_utf8(output).map_err(external)?.trim());
    if path.is_absolute() {
        Ok(path)
    } else {
        Err(FuzzRoadFailure::External(
            "rustup returned a relative rustc path".to_owned(),
        ))
    }
}

fn rustc_field(rustc: &PathBuf, prefix: &str) -> Result<String, FuzzRoadFailure> {
    let output = successful_output(Command::new(rustc).arg("-vV"), "rustc identity")?;
    let text = String::from_utf8(output).map_err(external)?;
    text.lines()
        .find_map(|line| line.strip_prefix(prefix))
        .filter(|field| !field.is_empty())
        .map(str::to_owned)
        .ok_or(FuzzRoadFailure::Fixture)
}

fn preflight_double(
    rustc: &PathBuf,
    manifest: &std::path::Path,
    directory: &std::path::Path,
    host: &str,
    versions: [&str; 4],
) -> Result<PathBuf, FuzzRoadFailure> {
    let [release, rustc_llvm, profdata_llvm, cov_llvm] = versions;
    std::fs::create_dir_all(directory).map_err(external)?;
    let executable = directory.join(format!("fake-rustc{}", std::env::consts::EXE_SUFFIX));
    let source = manifest
        .join("tests")
        .join("fuzz_compose")
        .join("rustc_preflight_subject.rs");
    let status = Command::new(rustc)
        .arg("--edition=2024")
        .arg(source)
        .arg("-o")
        .arg(&executable)
        .status()
        .map_err(external)?;
    if !status.success() {
        return Err(FuzzRoadFailure::External(format!(
            "preflight double compilation failed with {status}"
        )));
    }
    let sysroot = directory.join("sysroot");
    let tool_directory = sysroot.join("lib").join("rustlib").join(host).join("bin");
    std::fs::create_dir_all(&tool_directory).map_err(external)?;
    std::fs::write(directory.join("release.txt"), release).map_err(external)?;
    std::fs::write(directory.join("host.txt"), host).map_err(external)?;
    std::fs::write(directory.join("rustc-llvm.txt"), rustc_llvm).map_err(external)?;
    std::fs::write(
        directory.join("sysroot.txt"),
        sysroot.to_string_lossy().as_bytes(),
    )
    .map_err(external)?;
    std::fs::write(tool_directory.join("profdata-version.txt"), profdata_llvm).map_err(external)?;
    std::fs::write(tool_directory.join("cov-version.txt"), cov_llvm).map_err(external)?;
    std::fs::copy(
        &executable,
        tool_directory.join(format!("llvm-profdata{}", std::env::consts::EXE_SUFFIX)),
    )
    .map_err(external)?;
    std::fs::copy(
        &executable,
        tool_directory.join(format!("llvm-cov{}", std::env::consts::EXE_SUFFIX)),
    )
    .map_err(external)?;
    Ok(executable)
}

fn preflight_double_request(
    rustc: PathBuf,
    repository: &std::path::Path,
    run: &std::path::Path,
    stem: &str,
) -> Result<RustcProfileRequest, FuzzRoadFailure> {
    let target = InstrumentedTarget::declared(rustc.clone(), Vec::new()).map_err(external)?;
    let Some(logical) = NamespacedName::named("harness", "preflight-double").ok() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    let source_root =
        CoverageSourceRoot::declared(logical, repository.to_path_buf()).map_err(external)?;
    RustcProfileRequest::declared(
        rustc,
        target,
        source_root,
        run.join(stem).join("cases"),
        coverage_campaign()?,
    )
    .map_err(external)
}

fn wait_for_exit(child: &mut std::process::Child) -> Result<FuzzExecution, String> {
    let status = child.wait().map_err(|error| error.to_string())?;
    if status.success() {
        Ok(FuzzExecution::Success)
    } else {
        Ok(FuzzExecution::NonzeroExit(status.code()))
    }
}

fn wait_for_crash(child: &mut std::process::Child) -> Result<FuzzExecution, String> {
    let status = child.wait().map_err(|error| error.to_string())?;
    if status.success() {
        Err("crash control exited successfully".to_owned())
    } else {
        Ok(FuzzExecution::Crash(status.code()))
    }
}

fn stop_as(
    child: &mut std::process::Child,
    execution: FuzzExecution,
) -> Result<FuzzExecution, String> {
    child.kill().map_err(|error| error.to_string())?;
    child.wait().map_err(|error| error.to_string())?;
    Ok(execution)
}

#[cfg(windows)]
fn process_is_running(process: u32) -> Result<bool, FuzzRoadFailure> {
    let filter = format!("PID eq {process}");
    let output = successful_output(
        Command::new("tasklist").args(["/FI", &filter, "/FO", "CSV", "/NH"]),
        "tasklist process observation",
    )?;
    let text = String::from_utf8(output).map_err(external)?;
    Ok(text.contains(&format!("\"{process}\"")))
}

#[cfg(unix)]
fn process_is_running(process: u32) -> Result<bool, FuzzRoadFailure> {
    let output = Command::new("ps")
        .args(["-p", &process.to_string(), "-o", "pid="])
        .output()
        .map_err(external)?;
    let text = String::from_utf8(output.stdout).map_err(external)?;
    Ok(output.status.success() && text.trim() == process.to_string())
}

#[cfg(not(any(windows, unix)))]
fn process_is_running(_process: u32) -> Result<bool, FuzzRoadFailure> {
    Err(FuzzRoadFailure::External(
        "this target has no external process observer".to_owned(),
    ))
}

fn successful_output(command: &mut Command, role: &str) -> Result<Vec<u8>, FuzzRoadFailure> {
    let output = command.output().map_err(external)?;
    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(FuzzRoadFailure::External(format!(
            "{role} failed with {}",
            output.status
        )))
    }
}

fn external(error: impl fmt::Debug) -> FuzzRoadFailure {
    FuzzRoadFailure::External(format!("{error:?}"))
}

fn interesting_bytes(stem: &str, candidate: &[u8]) -> Result<InterestingBytes, FuzzRoadFailure> {
    let (ready, run) = rustc_profile_request(stem)?;
    let mut coverage = CoverageCorpus::opening(&ready);
    let result = observe_rustc_profile(&ready, &mut coverage, candidate, wait_for_exit)?;
    let admission = coverage.admit(result);
    std::fs::remove_dir_all(run).map_err(external)?;
    match admission? {
        CoverageAdmission::Interesting(interesting) => Ok(interesting),
        CoverageAdmission::Known => Err(FuzzRoadFailure::Fixture),
    }
}

#[test]
fn interesting_bytes_compose_into_exact_derived_replay() -> Result<(), FuzzRoadFailure> {
    let interesting = interesting_bytes("compose-replay", &[1, 2, 3])?;
    let plan = ReductionPlan::declared(
        MinimizationProfile::declared("fuzz-compose", 1),
        ByteReducerId::ChunkRemovalAndZeroing,
        Vec::new(),
        FingerprintPreservation::Required,
        ReductionBudget::declared(16),
    )?;
    let Some(binding) = probe_binding() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    let capsule = compose_reduce_replay(&interesting, &plan, &binding)?;
    assert_eq!(capsule.input(), &[1u8]);
    assert_eq!(capsule.posture(), ReplayPosture::ExactDerived);
    assert_eq!(
        probe(capsule.input()),
        ProbeOutcome::Reproduced(capsule.fingerprint())
    );
    Ok(())
}

#[test]
fn compose_refuses_when_seed_does_not_fail() -> Result<(), FuzzRoadFailure> {
    let interesting = interesting_bytes("compose-refusal", &[9])?;
    let plan = ReductionPlan::declared(
        MinimizationProfile::declared("fuzz-compose", 1),
        ByteReducerId::ChunkRemovalAndZeroing,
        Vec::new(),
        FingerprintPreservation::Required,
        ReductionBudget::declared(4),
    )?;
    let Some(binding) = probe_binding() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    match compose_reduce_replay(&interesting, &plan, &binding) {
        Err(ComposeRefusal::Reduction(ReductionRefusal::BaselineDidNotFail)) => Ok(()),
        Err(refusal) => Err(FuzzRoadFailure::Compose(refusal)),
        Ok(_) => Err(FuzzRoadFailure::Fixture),
    }
}
