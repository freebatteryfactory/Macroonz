//! Claim: source-specific readers preserve live versus simulated authority and addressed bytes cannot impersonate reproduction.
//! Subject: live writing, source-specific reading, and explicit simulation reproduction.
//! Population: one live pack, one simulated pack, one mismatched schedule, and a self-consistently readdressed hostile row.
//! Hostile control: altered output bytes remain readable declaration material but fail reproduction.
//! Denominator: both source claims and both public reading roads.
//! Evidence ceiling: the live source claim records adapter testimony and does not prove a real network run occurred.

use super::support::*;

/// Live-recorded material preserves its honest source ceiling and cannot enter simulation reproduction.
#[test]
fn a_live_record_is_replayable_but_not_reproducible() -> Result<(), LaneFailure> {
    let topology = pair_topology()?;
    let pack = recorded_live(
        &topology,
        vec![
            live_entry(b"first", 0u32, 2u64)?,
            live_entry(b"second", 1u32, 5u64)?,
        ],
    )?;
    assert_eq!(pack.source_claim(), TranscriptSourceClaim::RecordedLive);
    assert!(pack.simulation_manifest().is_none());
    let reread = read_recorded_live(&topology, pack.encoded())?;
    assert_eq!(reread, pack);
    assert_eq!(
        reproduce(&reread).err(),
        Some(TranscriptRefusal::RecordedLiveCannotReproduce)
    );
    let (mut replay, opening) = Replay::opened(&reread);
    assert!(opening.is_empty());
    while replay.remaining() > 0usize {
        let _handed_out = replay.advance();
    }
    assert_eq!(replay.exhaust()?.total(), 2usize);
    Ok(())
}

/// Source-specific readers reject a body from the other road, and simulation reading demands the exact selected schedule.
#[test]
fn readers_do_not_upgrade_or_relabel_source_material() -> Result<(), LaneFailure> {
    let topology = pair_topology()?;
    let (_rows, schedule, simulated_pack, _standing) = packed_run(0usize)?;
    assert_eq!(
        read_recorded_live(&topology, simulated_pack.encoded()).err(),
        Some(TranscriptRefusal::SourceClaimMismatch {
            expected: TranscriptSourceClaim::RecordedLive,
            found: TranscriptSourceClaim::Simulated,
        })
    );
    let live = recorded_live(&topology, vec![live_entry(b"live", 0u32, 1u64)?])?;
    assert_eq!(
        read_simulated(&topology, &schedule, live.encoded()).err(),
        Some(TranscriptRefusal::SourceClaimMismatch {
            expected: TranscriptSourceClaim::Simulated,
            found: TranscriptSourceClaim::RecordedLive,
        })
    );
    let other = NetworkSchedule::declared(name("other")?, Vec::new())?;
    assert_eq!(
        read_simulated(&topology, &other, simulated_pack.encoded()).err(),
        Some(TranscriptRefusal::ScheduleMismatch)
    );
    Ok(())
}

/// A self-consistent address over altered output rows still cannot mint simulation reproduction.
#[test]
fn addressed_bytes_do_not_impersonate_reproduction() -> Result<(), LaneFailure> {
    let topology = pair_topology()?;
    let (_rows, schedule, pack, _standing) = packed_run(0usize)?;
    let mut altered = pack.encoded().to_vec();
    let positions: Vec<_> = altered
        .windows(3usize)
        .enumerate()
        .filter_map(|(at, bytes)| (bytes == b"pay").then_some(at))
        .collect();
    assert_eq!(positions.len(), 3usize);
    let last = positions.last().copied().ok_or(LaneFailure::Standing)?;
    let byte = altered.get_mut(last).ok_or(LaneFailure::Standing)?;
    *byte = b'x';
    readdress(&mut altered)?;
    let decoded = read_simulated(&topology, &schedule, &altered)?;
    assert_eq!(
        reproduce(&decoded).err(),
        Some(TranscriptRefusal::SimulationRowsDiverge { at: 1usize })
    );
    Ok(())
}
