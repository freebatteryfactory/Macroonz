//! Sparse history stays sparse while independently bound current code actually runs.

use super::fixture;
use macroonz_harness::input::{BoundInput, InputLimits};
use macroonz_harness::report::legacy::{
    LegacyField, LegacyLimits, LegacyPresence, LegacyProfile, LegacyRefusal, read_record,
};
use macroonz_harness::report::replay::{
    HistoricalReplayRefusal, HistoricalReplayStanding, LegacyClaimRelation,
};
use macroonz_harness::report::{
    ByteBudget, CaseBudget, InvocationProfile, RunAttempt, SkipReason, TimeBudget, TrialConclusion,
};
use macroonz_harness::runner::{Invocation, LegacyReplayRefusal, replay_legacy};
use std::cell::Cell;

pub(super) const PROFILE: LegacyProfile<'static> = LegacyProfile::declared("neutral", 1);
pub(super) const LIMITS: LegacyLimits = LegacyLimits::declared(4096, 256, 64, 16, 2, 8192);
pub(super) const INPUT: InputLimits = InputLimits::declared(4096, 64);

std::thread_local! {
    static SUBJECT_CALLS: Cell<u32> = const { Cell::new(0) };
}

pub(super) fn subject(invocation: &Invocation<BoundInput<Vec<u8>>>) -> TrialConclusion {
    SUBJECT_CALLS.set(SUBJECT_CALLS.get().saturating_add(1));
    fixture::subject(invocation)
}

pub(super) fn reset() {
    fixture::reset_decodes();
    SUBJECT_CALLS.set(0);
}

pub(super) fn observations() -> (u8, u32) {
    (fixture::decode_count(), SUBJECT_CALLS.get())
}

#[test]
fn every_supplied_field_retains_its_actual_value_and_source() -> Result<(), ()> {
    let source = br#"{
        "kind":"neutral","schema":1,"witness":[0,255],
        "input_profile":{"name":"","revision":0},
        "trial_name":"trial","subject_name":"subject","check_name":"check",
        "subject_revision":18446744073709551615,"check_revision":0,
        "target":"target","toolchain":"toolchain",
        "execution_digest":"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        "fingerprint_digest":"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        "reported_outcome":"an unfamiliar outcome"
    }"#;
    let record = read_record(source, PROFILE, LIMITS).map_err(|_| ())?;
    assert_eq!(record.source(), source);
    assert_eq!(
        record.kind(),
        &LegacyPresence::Present("neutral".to_owned())
    );
    assert_eq!(record.schema(), &LegacyPresence::Present(1));
    assert_eq!(record.witness(), &LegacyPresence::Present(vec![0, 255]));
    let LegacyPresence::Present(profile) = record.input_profile() else {
        return Err(());
    };
    assert_eq!(profile.name(), &LegacyPresence::Present(String::new()));
    assert_eq!(profile.revision(), &LegacyPresence::Present(0));
    assert_eq!(
        record.trial_name(),
        &LegacyPresence::Present("trial".to_owned())
    );
    assert_eq!(
        record.subject_name(),
        &LegacyPresence::Present("subject".to_owned())
    );
    assert_eq!(
        record.check_name(),
        &LegacyPresence::Present("check".to_owned())
    );
    assert_eq!(
        record.subject_revision(),
        &LegacyPresence::Present(u64::MAX)
    );
    assert_eq!(record.check_revision(), &LegacyPresence::Present(0));
    assert_eq!(
        record.target(),
        &LegacyPresence::Present("target".to_owned())
    );
    assert_eq!(
        record.toolchain(),
        &LegacyPresence::Present("toolchain".to_owned())
    );
    assert_eq!(
        record.reported_outcome(),
        &LegacyPresence::Present("an unfamiliar outcome".to_owned())
    );
    let LegacyPresence::Present(execution) = record.execution_digest() else {
        return Err(());
    };
    let LegacyPresence::Present(fingerprint) = record.fingerprint_digest() else {
        return Err(());
    };
    assert_eq!(
        execution.as_bytes(),
        &[
            1, 35, 69, 103, 137, 171, 205, 239, 1, 35, 69, 103, 137, 171, 205, 239, 1, 35, 69, 103,
            137, 171, 205, 239, 1, 35, 69, 103, 137, 171, 205, 239
        ]
    );
    assert_eq!(fingerprint.as_bytes(), &[255; 32]);
    assert_eq!(
        record.source_address().as_bytes(),
        &blake3::derive_key(
            "macroonz/harness-identity/historical-json-source/v1",
            source
        )
    );
    Ok(())
}

#[test]
fn missing_null_empty_and_parent_presence_are_separate() -> Result<(), ()> {
    let missing = read_record(b"{}", PROFILE, LIMITS).map_err(|_| ())?;
    assert_eq!(missing.kind(), &LegacyPresence::Missing);
    assert_eq!(missing.schema(), &LegacyPresence::Missing);
    assert_eq!(missing.witness(), &LegacyPresence::Missing);
    assert_eq!(missing.input_profile(), &LegacyPresence::Missing);
    let null = read_record(
        br#"{"witness":null,"input_profile":null,"subject_revision":null}"#,
        PROFILE,
        LIMITS,
    )
    .map_err(|_| ())?;
    assert_eq!(null.witness(), &LegacyPresence::Null);
    assert_eq!(null.input_profile(), &LegacyPresence::Null);
    assert_eq!(null.subject_revision(), &LegacyPresence::Null);
    let empty = read_record(
        br#"{"witness":[],"input_profile":{},"reported_outcome":""}"#,
        PROFILE,
        LIMITS,
    )
    .map_err(|_| ())?;
    assert_eq!(empty.witness(), &LegacyPresence::Present(Vec::new()));
    let LegacyPresence::Present(profile) = empty.input_profile() else {
        return Err(());
    };
    assert_eq!(profile.name(), &LegacyPresence::Missing);
    assert_eq!(profile.revision(), &LegacyPresence::Missing);
    let leaves = read_record(
        br#"{"input_profile":{"name":null,"revision":null}}"#,
        PROFILE,
        LIMITS,
    )
    .map_err(|_| ())?;
    let LegacyPresence::Present(null_profile) = leaves.input_profile() else {
        return Err(());
    };
    assert_eq!(null_profile.name(), &LegacyPresence::Null);
    assert_eq!(null_profile.revision(), &LegacyPresence::Null);
    Ok(())
}

#[test]
fn every_resource_ceiling_has_an_exact_and_exceeded_observation() -> Result<(), ()> {
    let source = br#"{"schema":1,"witness":[0,255],"target":"abc","input_profile":{"revision":0}}"#;
    let length = source.len();
    let retained = length.checked_add(21).ok_or(())?;
    let exact = LegacyLimits::declared(length, 3, 2, 5, 2, retained);
    assert!(read_record(source, PROFILE, exact).is_ok());
    for (limits, expected) in [
        (
            LegacyLimits::declared(length.saturating_sub(1), 3, 2, 5, 2, retained),
            LegacyRefusal::SourceTooLarge,
        ),
        (
            LegacyLimits::declared(length, 2, 2, 5, 2, retained),
            LegacyRefusal::TextTooLarge,
        ),
        (
            LegacyLimits::declared(length, 3, 1, 5, 2, retained),
            LegacyRefusal::WitnessTooLarge,
        ),
        (
            LegacyLimits::declared(length, 3, 2, 4, 2, retained),
            LegacyRefusal::TooManyMembers,
        ),
        (
            LegacyLimits::declared(length, 3, 2, 5, 1, retained),
            LegacyRefusal::TooDeep,
        ),
        (
            LegacyLimits::declared(length, 3, 2, 5, 2, retained.saturating_sub(1)),
            LegacyRefusal::RetainedTooLarge,
        ),
    ] {
        assert_eq!(read_record(source, PROFILE, limits), Err(expected));
    }
    assert_eq!(
        read_record(b"{}", PROFILE, LegacyLimits::declared(2, 0, 0, 0, 0, 2)),
        Err(LegacyRefusal::TooDeep)
    );
    assert!(read_record(b"{}", PROFILE, LegacyLimits::declared(2, 0, 0, 0, 1, 2)).is_ok());
    Ok(())
}

#[test]
fn strict_decoded_keys_and_values_refuse_ambiguous_or_malformed_history() {
    for (source, expected) in [
        (
            br#"{"witness":null,"\u0077itness":[]}"#.as_slice(),
            LegacyRefusal::DuplicateField(LegacyField::Witness),
        ),
        (
            br#"{"input_profile":{"name":null,"\u006eame":""}}"#,
            LegacyRefusal::DuplicateField(LegacyField::ProfileName),
        ),
        (
            br#"{"input_profile":{"schema":1}}"#,
            LegacyRefusal::UnknownField,
        ),
        (br#"{"unknown":[]}"#, LegacyRefusal::UnknownField),
        (br#"{"kind":"other"}"#, LegacyRefusal::KindMismatch),
        (br#"{"schema":2}"#, LegacyRefusal::SchemaMismatch),
        (
            br#"{"execution_digest":"FF"}"#,
            LegacyRefusal::InvalidDigest,
        ),
    ] {
        assert_eq!(read_record(source, PROFILE, LIMITS), Err(expected));
    }
    for source in [
        br#"{"kind":null}"#.as_slice(),
        br#"{"schema":null}"#,
        br#"{"schema":1.0}"#,
        br#"{"schema":-0}"#,
        br#"{"schema":1e0}"#,
        br#"{"schema":18446744073709551616}"#,
        br#"{"witness":[-1]}"#,
        br#"{"witness":[256]}"#,
        br#"{"witness":[1.0]}"#,
        br#"{"witness":[[]]}"#,
        br#"{"input_profile":[]}"#,
        br#"{"target":"\ud800"}"#,
        br#"{"target":"\udc00"}"#,
        b"{\"target\":\"\xff\"}",
        br#"{"witness":[]}false"#,
    ] {
        assert!(
            matches!(
                read_record(source, PROFILE, LIMITS),
                Err(LegacyRefusal::InvalidJson { .. })
            ),
            "{source:?}"
        );
    }
    let source = br#"{"witness":[0,255],"target":"exact"}"#;
    for end in 0..source.len() {
        assert!(
            source
                .get(..end)
                .is_some_and(|prefix| read_record(prefix, PROFILE, LIMITS).is_err())
        );
    }
}

#[test]
fn unicode_and_exact_source_survive_detachment() -> Result<(), ()> {
    let source = br#" { "reported_outcome":"\u0000\u00e9\ud83d\ude00" } "#.to_vec();
    let record = read_record(&source, PROFILE, LIMITS).map_err(|_| ())?;
    let original = source.clone();
    drop(source);
    assert_eq!(record.source(), original);
    assert_eq!(
        record.reported_outcome(),
        &LegacyPresence::Present("\0é😀".to_owned())
    );
    Ok(())
}

#[test]
fn current_execution_occurs_once_without_promoting_partial_history() -> Result<(), ()> {
    let historical=read_record(br#"{"witness":[1],"target":"declared-archive-fixture","toolchain":"moved","subject_revision":4,"input_profile":null}"#,PROFILE,LIMITS).map_err(|_| ())?;
    fixture::reset_decodes();
    SUBJECT_CALLS.set(0);
    let result = replay_legacy(
        &historical,
        &fixture::binding(subject)?,
        &fixture::decoder()?,
        fixture::invocation(),
        INPUT,
    )
    .map_err(|_| ())?;
    assert_eq!(fixture::decode_count(), 1);
    assert_eq!(SUBJECT_CALLS.get(), 1);
    assert_eq!(result.historical(), historical.source_address());
    assert_eq!(result.witness().payload(), &[1]);
    assert!(matches!(
        result.report().attempt(),
        RunAttempt::Executed(TrialConclusion::Refused(_))
    ));
    let comparison = result.comparison().as_ref().map_err(|_| ())?;
    assert_eq!(
        macroonz_harness::report::replay::LegacyReading::standing(),
        HistoricalReplayStanding::Unverifiable(HistoricalReplayRefusal::IncompleteLegacyRecord)
    );
    assert_eq!(comparison.claims().len(), 16);
    for (field, expected) in [
        (LegacyField::Target, LegacyClaimRelation::SameClaim),
        (LegacyField::Toolchain, LegacyClaimRelation::MovedClaim),
        (
            LegacyField::SubjectRevision,
            LegacyClaimRelation::Uninterpreted,
        ),
        (LegacyField::CheckRevision, LegacyClaimRelation::Missing),
        (LegacyField::InputProfile, LegacyClaimRelation::Null),
        (LegacyField::ProfileName, LegacyClaimRelation::ParentNull),
    ] {
        assert_eq!(comparison.claims().get(&field), Some(&expected));
    }
    Ok(())
}

#[test]
fn absent_witnesses_and_execution_budget_skips_do_not_run_a_subject() -> Result<(), ()> {
    for (source, expected) in [
        (b"{}".as_slice(), LegacyReplayRefusal::MissingWitness),
        (br#"{"witness":null}"#, LegacyReplayRefusal::NullWitness),
    ] {
        let historical = read_record(source, PROFILE, LIMITS).map_err(|_| ())?;
        fixture::reset_decodes();
        SUBJECT_CALLS.set(0);
        assert_eq!(
            replay_legacy(
                &historical,
                &fixture::binding(subject)?,
                &fixture::decoder()?,
                fixture::invocation(),
                INPUT
            ),
            Err(expected)
        );
        assert_eq!(fixture::decode_count(), 0);
        assert_eq!(SUBJECT_CALLS.get(), 0);
    }
    let historical = read_record(br#"{"witness":[1]}"#, PROFILE, LIMITS).map_err(|_| ())?;
    let ordinary = fixture::invocation();
    let no_cases = Invocation::declared(
        InvocationProfile::declared(
            CaseBudget::declared(0),
            ByteBudget::declared(64),
            TimeBudget::declared(1000),
        ),
        ordinary.target().clone(),
        ordinary.site(),
        ordinary.clock(),
    );
    fixture::reset_decodes();
    SUBJECT_CALLS.set(0);
    let result = replay_legacy(
        &historical,
        &fixture::binding(subject)?,
        &fixture::decoder()?,
        no_cases,
        INPUT,
    )
    .map_err(|_| ())?;
    assert_eq!(fixture::decode_count(), 1);
    assert_eq!(SUBJECT_CALLS.get(), 0);
    assert_eq!(
        result.report().attempt(),
        &RunAttempt::SkippedWithReason(SkipReason::BudgetExhausted)
    );
    Ok(())
}
