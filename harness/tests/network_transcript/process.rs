//! Fresh-process admission and actual manifest reproduction keep their separate evidence.

use super::{support::*, vector};
use std::error::Error;
use std::io::Read as _;

fn source() -> Result<Vec<u8>, std::io::Error> {
    let mut encoded = Vec::new();
    std::io::stdin()
        .lock()
        .take(4_097)
        .read_to_end(&mut encoded)?;
    Ok(encoded)
}

fn refused(error: impl core::fmt::Debug) -> std::io::Error {
    std::io::Error::other(format!("{error:?}"))
}

#[test]
#[ignore = "driven by the transcript process parent"]
fn child_reads_live() -> Result<(), Box<dyn Error>> {
    let encoded = source()?;
    let pack = read_recorded_live(
        &pair_topology().map_err(refused)?,
        &encoded,
        TranscriptLimits::declared(
            macroonz_harness::report::archive::ArchiveLimits::declared(4_096, 128),
            2,
            2,
        ),
    )
    .map_err(refused)?;
    drop(encoded);
    assert_eq!(pack.source_claim(), TranscriptSourceClaim::RecordedLive);
    assert_eq!(
        reproduce(&pack),
        Err(TranscriptRefusal::RecordedLiveCannotReproduce)
    );
    let (mut replay, opening) = Replay::opened(&pack);
    assert!(opening.is_empty());
    assert_eq!(replay.advance().len(), 1);
    assert_eq!(replay.exhaust().map_err(refused)?.total(), 1);
    super::archive_process::publish(pack.encoded())
}

#[test]
#[ignore = "driven by the transcript process parent"]
fn child_reads_and_reproduces_simulation() -> Result<(), Box<dyn Error>> {
    let encoded = source()?;
    let pack = read_simulated(
        &pair_topology().map_err(refused)?,
        &duplicate_schedule().map_err(refused)?,
        &encoded,
        TranscriptLimits::declared(
            macroonz_harness::report::archive::ArchiveLimits::declared(4_096, 128),
            2,
            2,
        ),
    )
    .map_err(refused)?;
    drop(encoded);
    assert_eq!(pack.source_claim(), TranscriptSourceClaim::Simulated);
    let reproduced = reproduce(&pack).map_err(refused)?;
    assert_eq!(reproduced.actions(), 2);
    assert_eq!(reproduced.rows(), 2);
    let (mut replay, opening) = Replay::opened(&pack);
    assert!(opening.is_empty());
    let deliveries = replay.advance();
    assert_eq!(
        deliveries
            .iter()
            .map(|delivery| delivery.payload().as_slice())
            .collect::<Vec<_>>(),
        vec![b"pay".as_slice(); 2]
    );
    let joined = ReproducedReplay::joined(reproduced, replay.exhaust().map_err(refused)?)
        .map_err(refused)?;
    assert_eq!(joined.exhaustion().total(), 2);
    super::archive_process::publish(pack.encoded())
}

#[test]
fn actual_and_independent_transcripts_cross_a_fresh_process() -> Result<(), Box<dyn Error>> {
    let topology = pair_topology().map_err(refused)?;
    let live = recorded_live(&topology, vec![live_entry(b"pay", 0, 1).map_err(refused)?])
        .map_err(refused)?;
    let (_rows, _schedule, simulation, _standing) = packed_run(0).map_err(refused)?;
    for (pack, expected, child) in [
        (&live, vector::live(b"pay", 1), "process::child_reads_live"),
        (
            &simulation,
            vector::simulated(b"pay", 2, 2),
            "process::child_reads_and_reproduces_simulation",
        ),
    ] {
        assert_eq!(pack.encoded(), expected);
        for encoded in [pack.encoded(), expected.as_slice()] {
            assert_eq!(super::archive_process::round_trip(encoded, child)?, encoded);
        }
    }
    Ok(())
}
