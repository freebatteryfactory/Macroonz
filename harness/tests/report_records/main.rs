//! Report identities, source-site separation, and foreign material are observed from outside the crate.

use macroonz_harness::descriptor::{
    CheckRef, ClaimRef, Classification, DerivedRevision, ExecutionSuite, Origin, PopulationRef,
    RevisionBinding, Role, Row, SubjectRoute, Tag,
};
use macroonz_harness::identity::{ContentAddress, encode_bytes};
use macroonz_harness::report::{
    ByteBudget, CaseBudget, CheckRevisionId, EXECUTION_KEY_TAG, ExecutionKey, FINGERPRINT_TAG,
    FOREIGN_TEXT_MAX_BYTES, FailureClass, FindingCause, FindingLocation, Fingerprint, ForeignText,
    InvocationProfile, ProfiledTrial, SubjectRevisionId, TRIAL_IDENTITY_TAG, TargetBinding,
    TargetTriple, TextFidelity, TimeBudget, ToolchainIdentity, TrialFinding, TrialId, TrialProfile,
    TrialSite, Truncation, execution_key_preimage, fingerprint_preimage, trial_preimage,
};

const OWNER: &str = "report-records";
const CAUSE: FindingCause = FindingCause::named(OWNER, "identity-reading");

fn row() -> Result<Row, ()> {
    let claim = ClaimRef::named(OWNER, "records-retain-authority").map_err(|_| ())?;
    let suite = ExecutionSuite::named(OWNER, "outside-reading").map_err(|_| ())?;
    let role = Role::named(OWNER, "report").map_err(|_| ())?;
    let tag = Tag::named(OWNER, "identity").map_err(|_| ())?;
    let classification = Classification::authored(vec![role], vec![tag]).map_err(|_| ())?;
    let subject = SubjectRoute::named(OWNER, "subject").map_err(|_| ())?;
    let check = CheckRef::named(OWNER, "check").map_err(|_| ())?;
    let population = PopulationRef::named(OWNER, "one-case").map_err(|_| ())?;
    Row::declared(
        claim,
        suite,
        classification,
        subject,
        check,
        population,
        Origin::HandWritten,
    )
    .map_err(|_| ())
}

fn independently_framed_trial(profiled: ProfiledTrial) -> Vec<u8> {
    let mut bytes = Vec::new();
    encode_bytes(profiled.key().address().as_bytes(), &mut bytes);
    bytes.push(0u8);
    bytes
}

fn independently_framed_execution(key: &ExecutionKey) -> Vec<u8> {
    let mut bytes = Vec::new();
    encode_bytes(key.trial().address().as_bytes(), &mut bytes);
    encode_bytes(key.subject().address().as_bytes(), &mut bytes);
    encode_bytes(key.check().address().as_bytes(), &mut bytes);
    bytes.extend_from_slice(&key.invocation().cases().cases().to_be_bytes());
    bytes.extend_from_slice(&key.invocation().bytes().bytes().to_be_bytes());
    bytes.extend_from_slice(&key.invocation().time().nanoseconds().to_be_bytes());
    encode_bytes(key.target().target().spelling().as_bytes(), &mut bytes);
    encode_bytes(key.target().toolchain().spelling().as_bytes(), &mut bytes);
    bytes
}

fn independently_framed_fingerprint(fingerprint: Fingerprint) -> Vec<u8> {
    let mut bytes = Vec::new();
    encode_bytes(fingerprint.trial().address().as_bytes(), &mut bytes);
    encode_bytes(fingerprint.cause().family().as_bytes(), &mut bytes);
    encode_bytes(fingerprint.cause().local().as_bytes(), &mut bytes);
    bytes.push(fingerprint.class().slot());
    bytes
}

/// A trial identity is the independently framed semantic key and profile, and moving its source site cannot rename it.
#[test]
fn trial_identity_and_source_site_remain_separate_rails() -> Result<(), ()> {
    let row = row()?;
    let profiled = ProfiledTrial::of_key(row.trial_key(), TrialProfile::Unprofiled);
    let trial = TrialId::over(profiled);
    let expected = independently_framed_trial(profiled);
    assert_eq!(trial_preimage(profiled), expected);
    assert_eq!(
        *trial.address(),
        ContentAddress::derived(TRIAL_IDENTITY_TAG, &expected)
    );

    let first = TrialSite::located("report_records", "first.rs", 11u32, "outside-reading");
    let moved = TrialSite::located("report_records::moved", "moved.rs", 97u32, "renamed");
    assert_ne!(first, moved);
    assert_eq!(TrialId::over(profiled), trial);
    Ok(())
}

/// An execution key independently frames both revisions, every budget, and the exact target and toolchain.
#[test]
fn execution_identity_retains_every_execution_coordinate() -> Result<(), ()> {
    let profiled = ProfiledTrial::of_key(row()?.trial_key(), TrialProfile::Unprofiled);
    let trial = TrialId::over(profiled);
    let subject = SubjectRevisionId::of_binding(RevisionBinding::derived(
        DerivedRevision::from_material(b"subject-v1"),
    ));
    let check = CheckRevisionId::of_binding(RevisionBinding::derived(
        DerivedRevision::from_material(b"check-v1"),
    ));
    let invocation = InvocationProfile::declared(
        CaseBudget::declared(3u32),
        ByteBudget::declared(55u64),
        TimeBudget::declared(89u64),
    );
    let target = TargetBinding::bound(
        TargetTriple::declared("x86_64-pc-windows-msvc"),
        ToolchainIdentity::declared("1.98.0"),
    );
    let key = ExecutionKey::over(trial, subject, check, invocation, target);
    let expected = independently_framed_execution(&key);
    assert_eq!(
        execution_key_preimage(trial, subject, check, invocation, key.target()),
        expected
    );
    assert_eq!(
        key.address(),
        ContentAddress::derived(EXECUTION_KEY_TAG, &expected)
    );

    let foreign_target = TargetBinding::bound(
        TargetTriple::declared("x86_64-unknown-linux-gnu"),
        ToolchainIdentity::declared("1.98.0"),
    );
    let foreign = ExecutionKey::over(trial, subject, check, invocation, foreign_target);
    assert_ne!(foreign.address(), key.address());
    Ok(())
}

/// A failure fingerprint is independently framed from semantic trial, caller-owned cause, and failure class.
#[test]
fn fingerprint_identity_keeps_the_callers_cause_bytes() -> Result<(), ()> {
    let trial = TrialId::over(ProfiledTrial::of_key(
        row()?.trial_key(),
        TrialProfile::Unprofiled,
    ));
    let finding = TrialFinding::established(
        FailureClass::PropertyDisagreement,
        CAUSE,
        FindingLocation::at(file!(), line!()),
        None,
    );
    let fingerprint = Fingerprint::of(trial, &finding);
    let expected = independently_framed_fingerprint(fingerprint);
    assert_eq!(
        fingerprint_preimage(trial, CAUSE, FailureClass::PropertyDisagreement),
        expected
    );
    assert_eq!(
        fingerprint.address(),
        ContentAddress::derived(FINGERPRINT_TAG, &expected)
    );
    Ok(())
}

/// Foreign material preserves admitted bytes, reports both truncation and lossy rendering, and never disguises either as complete text.
#[test]
fn foreign_text_reports_every_loss_at_its_boundary() {
    let exact = ForeignText::admitted(b"readable");
    assert_eq!(exact.bytes(), b"readable");
    assert_eq!(exact.truncation(), Truncation::Complete);
    assert_eq!(exact.fidelity(), TextFidelity::Exact);
    assert_eq!(exact.shown(), "readable");

    let lossy = ForeignText::admitted(&[b'a', 0xffu8, b'b']);
    assert_eq!(lossy.bytes(), &[b'a', 0xffu8, b'b']);
    assert_eq!(lossy.truncation(), Truncation::Complete);
    assert_eq!(lossy.fidelity(), TextFidelity::LossyReplacement);
    assert_eq!(lossy.shown(), "a\u{fffd}b");

    let oversized = vec![b'x'; FOREIGN_TEXT_MAX_BYTES.saturating_add(1usize)];
    let truncated = ForeignText::admitted(&oversized);
    assert_eq!(truncated.bytes().len(), FOREIGN_TEXT_MAX_BYTES);
    assert_eq!(
        truncated.truncation(),
        Truncation::TruncatedAt {
            admitted: FOREIGN_TEXT_MAX_BYTES,
            offered: oversized.len(),
        }
    );
    assert_eq!(truncated.fidelity(), TextFidelity::Exact);
}
