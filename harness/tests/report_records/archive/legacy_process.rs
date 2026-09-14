//! New processes inspect owned sparse history and independently execute its witness.

use super::{fixture, legacy, process};
use macroonz_harness::input::BoundInput;
use macroonz_harness::report::TrialConclusion;
use macroonz_harness::report::archive::{
    ArchivedAttempt, ArchivedConclusion, read_trial, retain_trial,
};
use macroonz_harness::report::legacy::{LegacyRecord, read_record};
use macroonz_harness::report::replay::{
    HistoricalReplayRefusal, HistoricalReplayStanding, LegacyReading,
};
use macroonz_harness::runner::{Invocation, replay_legacy};
use std::error::Error;
use std::io::Read as _;

fn load() -> Result<LegacyRecord, Box<dyn Error>> {
    let mut bytes = Vec::new();
    std::io::stdin().lock().take(4097).read_to_end(&mut bytes)?;
    legacy::reset();
    let record = read_record(&bytes, legacy::PROFILE, legacy::LIMITS)
        .map_err(|refusal| std::io::Error::other(format!("{refusal:?}")))?;
    drop(bytes);
    assert_eq!(legacy::observations(), (0, 0));
    Ok(record)
}

#[test]
#[ignore = "invoked by the legacy owned-readback parent"]
fn child_loads_legacy() -> Result<(), Box<dyn Error>> {
    let record = load()?;
    process::publish(record.source())
}

fn child(
    call: fn(&Invocation<BoundInput<Vec<u8>>>) -> TrialConclusion,
    marker: u8,
) -> Result<(), Box<dyn Error>> {
    let historical = load()?;
    let decoder =
        fixture::decoder().map_err(|()| std::io::Error::other("decoder fixture refused"))?;
    let binding =
        fixture::binding(call).map_err(|()| std::io::Error::other("current binding refused"))?;
    let result = replay_legacy(
        &historical,
        &binding,
        &decoder,
        fixture::invocation(),
        legacy::INPUT,
    )
    .map_err(|refusal| std::io::Error::other(format!("{refusal:?}")))?;
    assert_eq!(legacy::observations(), (1, 1));
    assert_eq!(result.witness().payload(), &[1]);
    assert_eq!(result.historical(), historical.source_address());
    assert!(result.comparison().is_ok());
    assert_eq!(
        LegacyReading::standing(),
        HistoricalReplayStanding::Unverifiable(HistoricalReplayRefusal::IncompleteLegacyRecord)
    );
    let retained = retain_trial(result.report(), super::trials::LIMITS)
        .map_err(|refusal| std::io::Error::other(format!("{refusal:?}")))?;
    let mut response = vec![1, 1, marker];
    response.extend_from_slice(retained.encoded());
    process::publish(&response)
}

#[test]
#[ignore = "invoked by the legacy current-execution parent with its defective binding"]
fn child_refuses_current_witness() -> Result<(), Box<dyn Error>> {
    child(legacy::subject, 0)
}

#[test]
#[ignore = "invoked by the legacy current-execution parent with its passing binding"]
fn child_passes_current_witness() -> Result<(), Box<dyn Error>> {
    child(
        |invocation| {
            drop(legacy::subject(invocation));
            TrialConclusion::Passed
        },
        1,
    )
}

#[test]
fn owned_legacy_fields_survive_fresh_process_loading_without_execution()
-> Result<(), Box<dyn Error>> {
    for source in [
        br#"{"witness":[1],"input_profile":{"name":null,"revision":0},"reported_outcome":"\u0000\u00e9"}"#.as_slice(),
        br#" {"kind":"neutral","schema":1,"witness":[],"subject_revision":18446744073709551615} "#,
        br#"{"witness":null,"input_profile":null,"execution_digest":"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"}"#,
        b"{}",
    ] {
        let owned=read_record(source,legacy::PROFILE,legacy::LIMITS)
            .map_err(|refusal| std::io::Error::other(format!("{refusal:?}")))?;
        assert_eq!(process::round_trip(owned.source(),"archive::legacy_process::child_loads_legacy")?,source);
    }
    Ok(())
}

#[test]
fn a_fresh_process_earns_each_current_outcome_without_claiming_historical_repair()
-> Result<(), Box<dyn Error>> {
    let source = br#"{"witness":[1],"subject_revision":4,"reported_outcome":"unknown"}"#;
    let decoder =
        fixture::decoder().map_err(|()| std::io::Error::other("decoder fixture refused"))?;
    let witness = macroonz_harness::input::pack(decoder.profile(), &[1], legacy::INPUT)
        .map_err(|refusal| std::io::Error::other(format!("{refusal:?}")))?;
    for (child_name, marker) in [
        ("archive::legacy_process::child_refuses_current_witness", 0),
        ("archive::legacy_process::child_passes_current_witness", 1),
    ] {
        let response = process::round_trip(source, child_name)?;
        assert_eq!(response.get(..3), Some([1, 1, marker].as_slice()));
        let current = read_trial(
            response.get(3..).ok_or("missing current report")?,
            super::trials::LIMITS,
        )
        .map_err(|refusal| std::io::Error::other(format!("{refusal:?}")))?;
        assert_eq!(
            current
                .key()
                .input()
                .ok_or("missing current input")?
                .case()
                .as_bytes(),
            witness.case().address().as_bytes()
        );
        match (marker, current.attempt()) {
            (0, ArchivedAttempt::Executed(ArchivedConclusion::Refused(_)))
            | (1, ArchivedAttempt::Executed(ArchivedConclusion::Passed)) => {}
            _ => return Err("current outcome did not match independently selected child".into()),
        }
    }
    Ok(())
}
