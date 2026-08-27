//! LibAFL interesting bytes → Macroonz `fuzz::compose_reduce_replay`.

use std::{
    fs,
    io::{self, Write},
    path::Path,
};

use macroonz_f0_target::{observe, CaptureOutcome};
use macroonz_harness::clock::HarnessClock;
use macroonz_harness::descriptor::{
    Binding, CheckRef, ClaimRef, Classification, DerivedRevision, ExecutableAttachment,
    ExecutionSuite, GeneratedSupportSchemaId, Origin, PopulationRef, Provenance, RevisionBinding,
    Role, Row, SubjectRoute, Tag, TrialCoordinates, TrialKey,
};
use macroonz_harness::fuzz::{InterestingBytes, compose_reduce_replay};
use macroonz_harness::generate::{
    ByteReducerId, FingerprintPreservation, ProbeOutcome, ReductionBudget, ReductionPlan,
    ReductionProbeBinding,
};
use macroonz_harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use macroonz_harness::report::{
    ByteBudget, CaseBudget, FailureClass, FindingCause, FindingLocation, Fingerprint,
    GenerationProfile, InvocationProfile, MinimizationProfile, TargetBinding, TargetTriple,
    TimeBudget, ToolchainIdentity, TrialConclusion, TrialFinding, TrialId, TrialProfile, TrialSite,
};
use macroonz_harness::runner::{Invocation, TrialBinding, run_one};

const REFUSAL_CAUSE: FindingCause = FindingCause::named("macroonz-f0", "textcapture-typed-refusal");
const SCHEMA_TAG: DomainTag =
    DomainTag::declared("f0-compose-schema", IdentityProfileVersion::declared(1));

fn probe(input: &[u8]) -> ProbeOutcome {
    match observe(input) {
        CaptureOutcome::Refused { .. } => match trial_fingerprint() {
            Some(fp) => ProbeOutcome::Reproduced(fp),
            None => ProbeOutcome::NoFailure,
        },
        CaptureOutcome::Read { .. } | CaptureOutcome::NotUtf8 => ProbeOutcome::NoFailure,
    }
}

fn trial_fingerprint() -> Option<Fingerprint> {
    let coordinates = TrialCoordinates::over(
        ClaimRef::named("macroonz-f0", "textcapture-reduction").ok()?,
        SubjectRoute::named("macroonz-f0", "compiler-text").ok()?,
        CheckRef::named("macroonz-f0", "typed-refusal-preserved").ok()?,
        PopulationRef::named("macroonz-f0", "libafl-interesting").ok()?,
    );
    let key = TrialKey::over(coordinates);
    let trial = TrialId::of_key(key, TrialProfile::Unprofiled);
    Some(Fingerprint::over(
        trial,
        REFUSAL_CAUSE,
        FailureClass::PropertyDisagreement,
    ))
}

fn refused_trial(_invocation: &Invocation) -> TrialConclusion {
    TrialConclusion::Refused(TrialFinding::established(
        FailureClass::PropertyDisagreement,
        REFUSAL_CAUSE,
        FindingLocation::at(file!(), line!()),
        None,
    ))
}

fn trial_binding() -> Option<TrialBinding> {
    let subject = SubjectRoute::named("macroonz-f0", "compiler-text").ok()?;
    let check = CheckRef::named("macroonz-f0", "typed-refusal-preserved").ok()?;
    let row = Row::declared(
        ClaimRef::named("macroonz-f0", "textcapture-reduction").ok()?,
        ExecutionSuite::named("macroonz-f0", "compose").ok()?,
        Classification::authored(
            vec![Role::named("macroonz-f0", "reduction").ok()?],
            vec![Tag::named("macroonz-f0", "libafl-handoff").ok()?],
        )
        .ok()?,
        subject,
        check,
        PopulationRef::named("macroonz-f0", "libafl-interesting").ok()?,
        Origin::HandWritten,
    )
    .ok()?;
    let revision = RevisionBinding::derived(DerivedRevision::from_material(b"f0-compose"));
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
        TrialSite::located(module_path!(), file!(), line!(), "f0-compose"),
        HarnessClock::unavailable(),
    )
}

/// Reduce + replay one LibAFL-admitted byte string through `macroonz_harness::fuzz`.
pub(crate) fn prove_libafl_to_macroonz(
    evidence_dir: &Path,
    interesting: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(evidence_dir)?;
    let mut out = fs::File::create(evidence_dir.join("libafl-macroonz-compose.tsv"))?;
    writeln!(out, "phase\tclaim\tstatus\tfact")?;
    fs::write(evidence_dir.join("libafl-interesting.bin"), interesting)?;

    let seed = if matches!(observe(interesting), CaptureOutcome::Refused { .. }) {
        interesting.to_vec()
    } else {
        b"struct Refused {".to_vec()
    };
    if !matches!(observe(&seed), CaptureOutcome::Refused { .. }) {
        return Err(io::Error::other("compose seed did not produce typed refusal").into());
    }
    writeln!(
        out,
        "compose\tinput\tavailable\tbytes={}; path=libafl-interesting.bin; reduce-seed-len={}",
        interesting.len(),
        seed.len()
    )?;

    let trial = trial_binding().ok_or_else(|| io::Error::other("trial binding refused"))?;
    let report = run_one(&trial, &invocation());
    let revision = RevisionBinding::derived(DerivedRevision::from_material(b"f0-compose"));
    let binding = ReductionProbeBinding::bound(
        &report,
        GenerationProfile::declared("f0-libafl-input", 1),
        GeneratedSupportSchemaId::over(ContentAddress::derived(SCHEMA_TAG, b"schema")),
        revision,
        probe,
    )
    .map_err(|e| io::Error::other(format!("ReductionProbeBinding refused: {e:?}")))?;

    let plan = ReductionPlan::declared(
        MinimizationProfile::declared("f0-min", 1),
        ByteReducerId::ChunkRemovalAndZeroing,
        Vec::new(),
        FingerprintPreservation::Required,
        ReductionBudget::declared(10_000),
    )
    .map_err(|e| io::Error::other(format!("ReductionPlan refused: {e:?}")))?;

    let admitted = InterestingBytes::admitted(seed)
        .map_err(|e| io::Error::other(format!("InterestingBytes refused: {e:?}")))?;
    let capsule = compose_reduce_replay(&admitted, &plan, &binding)
        .map_err(|e| io::Error::other(format!("fuzz compose refused: {e:?}")))?;
    writeln!(
        out,
        "compose\treduce-replay\tavailable\tminimized-len={}; fingerprint-preserved=true; posture={:?}; road=macroonz_harness::fuzz::compose_reduce_replay",
        capsule.input().len(),
        capsule.posture()
    )?;
    fs::write(evidence_dir.join("minimized-replay.bin"), capsule.input())?;
    Ok(())
}
