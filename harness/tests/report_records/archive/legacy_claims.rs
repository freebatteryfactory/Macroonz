//! Matching sparse claims do not manufacture complete historical reproduction.

use super::{fixture, legacy};
use macroonz_harness::clock::MeasurementReading;
use macroonz_harness::identity::ContentAddress;
use macroonz_harness::input::{InputProfile, pack};
use macroonz_harness::report::legacy::{LegacyField, read_record};
use macroonz_harness::report::replay::{
    HistoricalReplayRefusal, HistoricalReplayStanding, LegacyClaimRelation, LegacyJoinRefusal,
    LegacyReading, ReplayJoinRefusal, compare_legacy,
};
use macroonz_harness::report::{Fingerprint, RunAttempt, TrialConclusion};
use macroonz_harness::runner::replay_legacy;
use std::fmt::Write as _;

fn hex(address: ContentAddress) -> Result<String, ()> {
    let mut text = String::new();
    for byte in address.as_bytes() {
        write!(&mut text, "{byte:02x}").map_err(|_| ())?;
    }
    Ok(text)
}

#[test]
fn equal_and_changed_digest_claims_never_establish_complete_reproduction() -> Result<(), ()> {
    let current = fixture::report(&[1])?;
    let RunAttempt::Executed(TrialConclusion::Refused(finding)) = current.attempt() else {
        return Err(());
    };
    let execution = hex(current.standing().key().address())?;
    let fingerprint = hex(Fingerprint::of(current.trial(), finding).address())?;
    assert_ne!(execution, fingerprint);
    let witness = pack(fixture::decoder()?.profile(), &[1], legacy::INPUT).map_err(|_| ())?;
    for (key, failure, expected) in [
        (&execution, &fingerprint, LegacyClaimRelation::SameClaim),
        (&fingerprint, &execution, LegacyClaimRelation::MovedClaim),
    ] {
        let source = format!(
            r#"{{"witness":[1],"execution_digest":"{key}","fingerprint_digest":"{failure}","reported_outcome":"passed"}}"#
        );
        let historical =
            read_record(source.as_bytes(), legacy::PROFILE, legacy::LIMITS).map_err(|_| ())?;
        let reading = compare_legacy(&historical, &current, &witness).map_err(|_| ())?;
        assert_eq!(
            reading.claims().get(&LegacyField::ExecutionDigest),
            Some(&expected)
        );
        assert_eq!(
            reading.claims().get(&LegacyField::FingerprintDigest),
            Some(&expected)
        );
        assert_eq!(
            reading.claims().get(&LegacyField::ReportedOutcome),
            Some(&LegacyClaimRelation::Uninterpreted)
        );
        assert_eq!(
            LegacyReading::standing(),
            HistoricalReplayStanding::Unverifiable(HistoricalReplayRefusal::IncompleteLegacyRecord)
        );
    }
    Ok(())
}

#[test]
fn a_claimed_fingerprint_is_unavailable_when_current_execution_passes() -> Result<(), ()> {
    let source=br#"{"witness":[0],"fingerprint_digest":"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"}"#;
    let historical = read_record(source, legacy::PROFILE, legacy::LIMITS).map_err(|_| ())?;
    let result = replay_legacy(
        &historical,
        &fixture::binding(legacy::subject)?,
        &fixture::decoder()?,
        fixture::invocation(),
        legacy::INPUT,
    )
    .map_err(|_| ())?;
    assert_eq!(
        result.report().attempt(),
        &RunAttempt::Executed(TrialConclusion::Passed)
    );
    assert_eq!(
        result
            .comparison()
            .as_ref()
            .map_err(|_| ())?
            .claims()
            .get(&LegacyField::FingerprintDigest),
        Some(&LegacyClaimRelation::CurrentUnavailable)
    );
    Ok(())
}

#[test]
fn null_missing_and_partial_profile_claims_keep_their_actual_parent() -> Result<(), ()> {
    let current = fixture::report(&[0])?;
    let witness = pack(fixture::decoder()?.profile(), &[0], legacy::INPUT).map_err(|_| ())?;
    for (members, parent, name, revision) in [
        (
            "",
            LegacyClaimRelation::Missing,
            LegacyClaimRelation::ParentMissing,
            LegacyClaimRelation::ParentMissing,
        ),
        (
            r#","input_profile":null"#,
            LegacyClaimRelation::Null,
            LegacyClaimRelation::ParentNull,
            LegacyClaimRelation::ParentNull,
        ),
        (
            r#","input_profile":{}"#,
            LegacyClaimRelation::Uninterpreted,
            LegacyClaimRelation::Missing,
            LegacyClaimRelation::Missing,
        ),
        (
            r#","input_profile":{"name":null}"#,
            LegacyClaimRelation::Uninterpreted,
            LegacyClaimRelation::Null,
            LegacyClaimRelation::Missing,
        ),
        (
            r#","input_profile":{"revision":null}"#,
            LegacyClaimRelation::Uninterpreted,
            LegacyClaimRelation::Missing,
            LegacyClaimRelation::Null,
        ),
        (
            r#","input_profile":{"name":"archive::bytes","revision":1}"#,
            LegacyClaimRelation::Uninterpreted,
            LegacyClaimRelation::Uninterpreted,
            LegacyClaimRelation::Uninterpreted,
        ),
    ] {
        let source = format!(r#"{{"witness":[0]{members}}}"#);
        let historical =
            read_record(source.as_bytes(), legacy::PROFILE, legacy::LIMITS).map_err(|_| ())?;
        let reading = compare_legacy(&historical, &current, &witness).map_err(|_| ())?;
        for (field, expected) in [
            (LegacyField::InputProfile, parent),
            (LegacyField::ProfileName, name),
            (LegacyField::ProfileRevision, revision),
        ] {
            assert_eq!(reading.claims().get(&field), Some(&expected));
        }
    }
    for (members, expected) in [
        ("", LegacyClaimRelation::Missing),
        (
            r#","target":null,"toolchain":null,"execution_digest":null,"fingerprint_digest":null"#,
            LegacyClaimRelation::Null,
        ),
    ] {
        let source = format!(r#"{{"witness":[0]{members}}}"#);
        let historical =
            read_record(source.as_bytes(), legacy::PROFILE, legacy::LIMITS).map_err(|_| ())?;
        let reading = compare_legacy(&historical, &current, &witness).map_err(|_| ())?;
        for field in [
            LegacyField::Target,
            LegacyField::Toolchain,
            LegacyField::ExecutionDigest,
            LegacyField::FingerprintDigest,
        ] {
            assert_eq!(reading.claims().get(&field), Some(&expected));
        }
    }
    Ok(())
}

#[test]
fn every_legacy_witness_join_refuses_the_wrong_current_coordinate() -> Result<(), ()> {
    let historical =
        read_record(br#"{"witness":[1]}"#, legacy::PROFILE, legacy::LIMITS).map_err(|_| ())?;
    let current = fixture::report(&[1])?;
    let decoder = fixture::decoder()?;
    let witness = pack(decoder.profile(), &[1], legacy::INPUT).map_err(|_| ())?;
    for (source, expected) in [
        (b"{}".as_slice(), LegacyJoinRefusal::MissingWitness),
        (br#"{"witness":null}"#, LegacyJoinRefusal::NullWitness),
    ] {
        let absent = read_record(source, legacy::PROFILE, legacy::LIMITS).map_err(|_| ())?;
        assert_eq!(compare_legacy(&absent, &current, &witness), Err(expected));
    }
    let different = pack(decoder.profile(), &[2], legacy::INPUT).map_err(|_| ())?;
    assert_eq!(
        compare_legacy(&historical, &current, &different),
        Err(LegacyJoinRefusal::Current(
            ReplayJoinRefusal::WitnessBytesDiffer
        ))
    );
    let unit = fixture::host_report(
        RunAttempt::Executed(TrialConclusion::Passed),
        MeasurementReading::Unavailable,
    )?;
    assert_eq!(
        compare_legacy(&historical, &unit, &witness),
        Err(LegacyJoinRefusal::Current(
            ReplayJoinRefusal::CurrentInputUnrecorded
        ))
    );
    let profile = InputProfile::declared(decoder.profile().name(), 9, decoder.profile().schema());
    let moved_profile = pack(profile, &[1], legacy::INPUT).map_err(|_| ())?;
    assert_eq!(
        compare_legacy(&historical, &current, &moved_profile),
        Err(LegacyJoinRefusal::Current(
            ReplayJoinRefusal::CurrentProfileDiffers
        ))
    );
    assert_eq!(
        compare_legacy(&historical, &fixture::report(&[2])?, &witness),
        Err(LegacyJoinRefusal::Current(
            ReplayJoinRefusal::CurrentCaseDiffers
        ))
    );
    Ok(())
}

#[test]
fn source_reformatting_moves_source_custody_without_moving_current_execution() -> Result<(), ()> {
    let first =
        read_record(br#"{"witness":[1]}"#, legacy::PROFILE, legacy::LIMITS).map_err(|_| ())?;
    let spaced = read_record(
        br#" { "witness" : [ 1 ] } "#,
        legacy::PROFILE,
        legacy::LIMITS,
    )
    .map_err(|_| ())?;
    assert_ne!(first.source_address(), spaced.source_address());
    let binding = fixture::binding(legacy::subject)?;
    let decoder = fixture::decoder()?;
    let run_first = replay_legacy(
        &first,
        &binding,
        &decoder,
        fixture::invocation(),
        legacy::INPUT,
    )
    .map_err(|_| ())?;
    let run_spaced = replay_legacy(
        &spaced,
        &binding,
        &decoder,
        fixture::invocation(),
        legacy::INPUT,
    )
    .map_err(|_| ())?;
    assert_ne!(run_first.historical(), run_spaced.historical());
    assert_eq!(
        run_first.report().standing().key(),
        run_spaced.report().standing().key()
    );
    assert_eq!(run_first.witness(), run_spaced.witness());
    Ok(())
}
