//! Actual current admission and execution boundaries remain visible through sparse history.

use super::{fixture, legacy};
use arbitrary::Unstructured;
use macroonz_harness::clock::{ClockAttribution, HarnessClock, MeasurementReading};
use macroonz_harness::input::{InputBinding, InputLimits, InputRefusal};
use macroonz_harness::report::legacy::{LegacyField, read_record};
use macroonz_harness::report::replay::LegacyClaimRelation;
use macroonz_harness::report::{
    ByteBudget, CaseBudget, InvocationProfile, RunAttempt, SkipReason, TimeBudget, TrialConclusion,
};
use macroonz_harness::runner::{Invocation, LegacyReplayRefusal, replay_legacy};
use std::cell::Cell;

std::thread_local! { static CLOCK_READS: Cell<u32> = const { Cell::new(0) }; }

fn refuses(_: &mut Unstructured<'_>) -> arbitrary::Result<Vec<u8>> {
    fixture::record_decode();
    Err(arbitrary::Error::IncorrectFormat)
}

#[test]
fn an_explicitly_empty_witness_is_decoded_and_executed_once() -> Result<(), ()> {
    let historical =
        read_record(br#"{"witness":[]}"#, legacy::PROFILE, legacy::LIMITS).map_err(|_| ())?;
    legacy::reset();
    let result = replay_legacy(
        &historical,
        &fixture::binding(legacy::subject)?,
        &fixture::decoder()?,
        fixture::invocation(),
        legacy::INPUT,
    )
    .map_err(|_| ())?;
    assert_eq!(legacy::observations(), (1, 1));
    assert!(result.witness().payload().is_empty());
    assert_eq!(
        result.report().attempt(),
        &RunAttempt::Executed(TrialConclusion::Passed)
    );
    Ok(())
}

#[test]
fn envelope_and_payload_refusal_happen_before_decoder_and_subject() -> Result<(), ()> {
    let historical =
        read_record(br#"{"witness":[1]}"#, legacy::PROFILE, legacy::LIMITS).map_err(|_| ())?;
    for (limits, expected) in [
        (InputLimits::declared(0, 64), InputRefusal::EnvelopeTooLarge),
        (
            InputLimits::declared(4096, 0),
            InputRefusal::PayloadTooLarge,
        ),
    ] {
        legacy::reset();
        assert_eq!(
            replay_legacy(
                &historical,
                &fixture::binding(legacy::subject)?,
                &fixture::decoder()?,
                fixture::invocation(),
                limits
            ),
            Err(LegacyReplayRefusal::Input(expected))
        );
        assert_eq!(legacy::observations(), (0, 0));
    }
    Ok(())
}

#[test]
fn decoding_failure_and_incomplete_consumption_never_invent_a_report() -> Result<(), ()> {
    let historical =
        read_record(br#"{"witness":[1]}"#, legacy::PROFILE, legacy::LIMITS).map_err(|_| ())?;
    let ordinary = fixture::decoder()?;
    let cases = [
        (
            InputBinding::declared(ordinary.profile(), ordinary.revision(), refuses),
            InputRefusal::DecoderRefused(arbitrary::Error::IncorrectFormat),
        ),
        (
            InputBinding::declared(ordinary.profile(), ordinary.revision(), |_| {
                fixture::record_decode();
                Ok(Vec::new())
            }),
            InputRefusal::TrailingInputBytes { count: 1 },
        ),
    ];
    for (decoder, expected) in cases {
        legacy::reset();
        assert_eq!(
            replay_legacy(
                &historical,
                &fixture::binding(legacy::subject)?,
                &decoder,
                fixture::invocation(),
                legacy::INPUT
            ),
            Err(LegacyReplayRefusal::Input(expected))
        );
        assert_eq!(legacy::observations(), (1, 0));
    }
    Ok(())
}

#[test]
fn both_execution_budget_axes_skip_subject_and_clock_with_the_current_report_intact()
-> Result<(), ()> {
    let historical=read_record(br#"{"witness":[1],"fingerprint_digest":"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"}"#,legacy::PROFILE,legacy::LIMITS).map_err(|_| ())?;
    let ordinary = fixture::invocation();
    for (cases, bytes) in [(0, 64), (1, 0)] {
        legacy::reset();
        CLOCK_READS.set(0);
        let clock = HarnessClock::reading_as(
            || {
                CLOCK_READS.set(CLOCK_READS.get().saturating_add(1));
                0
            },
            ClockAttribution::Synthetic,
        );
        let invocation = Invocation::declared(
            InvocationProfile::declared(
                CaseBudget::declared(cases),
                ByteBudget::declared(bytes),
                TimeBudget::declared(1000),
            ),
            ordinary.target().clone(),
            ordinary.site(),
            clock,
        );
        let result = replay_legacy(
            &historical,
            &fixture::binding(legacy::subject)?,
            &fixture::decoder()?,
            invocation,
            legacy::INPUT,
        )
        .map_err(|_| ())?;
        assert_eq!(legacy::observations(), (1, 0));
        assert_eq!(CLOCK_READS.get(), 0);
        assert_eq!(
            result.report().attempt(),
            &RunAttempt::SkippedWithReason(SkipReason::BudgetExhausted)
        );
        assert_eq!(
            result.report().measurement(),
            MeasurementReading::Unavailable
        );
        assert_eq!(
            result.report().clock_attribution(),
            ClockAttribution::Unspecified
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
    }
    Ok(())
}
