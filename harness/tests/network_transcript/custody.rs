//! Claim: a complete simulated action manifest reproduces its addressed rows and joins only with exhaustive playback of the same address.
//! Subject: simulated writing, reproduction, replay, exhaustion, and reproduced-replay joining.
//! Population: duplicating and dropping two-node simulations, including a tick-zero live playback.
//! Reversal: an extra empty advance moves identity, incomplete playback refuses, and a second address cannot join.
//! Denominator: every simulation-manifest, reproduction, replay, exhaustion, and join road.
//! Evidence ceiling: handing rows out does not prove an adopter processed them.

use super::support::*;

/// A complete simulated manifest packs, reads as declaration material, reproduces, and replays under exact address custody.
#[test]
fn a_simulated_run_reproduces_and_replays_under_one_address() -> Result<(), LaneFailure> {
    let (deliveries, schedule, pack, written_reproduction) = packed_run(0usize)?;
    assert_eq!(deliveries.len(), 2usize);
    assert_eq!(pack.source_claim(), TranscriptSourceClaim::Simulated);
    let manifest = pack.simulation_manifest().ok_or(LaneFailure::Standing)?;
    assert_eq!(manifest.schedule(), &schedule);
    assert_eq!(manifest.actions().len(), 2usize);
    assert_eq!(written_reproduction.address(), pack.address());
    assert_eq!(written_reproduction.rows(), 2usize);

    let reread = read_simulated(&pair_topology()?, &schedule, pack.encoded())?;
    assert_eq!(reread, pack);
    let decoded_reproduction = reproduce(&reread)?;
    assert_eq!(decoded_reproduction, written_reproduction);

    let (mut replay, opening) = Replay::opened(&reread);
    assert!(opening.is_empty());
    let mut played = Vec::new();
    while replay.remaining() > 0usize {
        played.extend(replay.advance());
    }
    assert_eq!(played, deliveries);
    let exhaustion = replay.exhaust()?;
    assert_eq!(exhaustion.address(), pack.address());
    assert_eq!(exhaustion.total(), 2usize);
    let joined = ReproducedReplay::joined(decoded_reproduction, exhaustion)?;
    assert_eq!(joined.reproduction(), decoded_reproduction);
    assert_eq!(joined.exhaustion(), exhaustion);
    Ok(())
}

/// Identical drives derive one address, while an extra empty advance moves the manifest identity without changing the delivery rows.
#[test]
fn the_complete_action_manifest_moves_the_address() -> Result<(), LaneFailure> {
    let (_first_rows, _first_schedule, first, _first_reproduction) = packed_run(0usize)?;
    let (_second_rows, _second_schedule, second, _second_reproduction) = packed_run(0usize)?;
    assert_eq!(first.address(), second.address());
    assert_eq!(first.encoded(), second.encoded());

    let (_extended_rows, _extended_schedule, extended, standing) = packed_run(1usize)?;
    assert_eq!(first.entries(), extended.entries());
    assert_ne!(first.address(), extended.address());
    assert_ne!(first.encoded(), extended.encoded());
    assert_eq!(standing.actions(), 3usize);
    assert_eq!(standing.final_tick(), Tick::at(2u64));
    Ok(())
}

/// A dropped send remains in the complete action denominator even though it produces no delivery row.
#[test]
fn dropped_inputs_remain_in_the_reproduced_manifest() -> Result<(), LaneFailure> {
    let schedule = NetworkSchedule::declared(
        name("drop-first")?,
        vec![LinkDiscipline::declared(
            forward()?,
            vec![LinkFault::DropAt {
                position: SendOrdinal::at(0u32),
            }],
        )],
    )?;
    let campaign = NetworkCampaign::declared(vec![schedule.clone()])?;
    let mut sim = SimNet::declared(pair_topology()?, campaign.select(name("drop-first")?)?)?;
    sim.send(forward()?, b"lost".to_vec())?;
    sim.send(forward()?, b"kept".to_vec())?;
    let delivered = sim.advance();
    assert_eq!(delivered.len(), 1usize);
    let (pack, standing) = simulated(&sim, Vec::clone)?;
    let manifest = pack.simulation_manifest().ok_or(LaneFailure::Standing)?;
    assert_eq!(manifest.actions().len(), 3usize);
    assert_eq!(pack.entries().len(), 1usize);
    assert_eq!(standing.actions(), 3usize);
    assert_eq!(standing.rows(), 1usize);
    Ok(())
}

/// Tick-zero delivery is handed out by opening and counts toward exact replay exhaustion.
#[test]
fn tick_zero_is_part_of_the_exhaustion_denominator() -> Result<(), LaneFailure> {
    let topology = pair_topology()?;
    let entry = TranscriptEntry::witnessed(
        forward()?,
        SendOrdinal::at(0u32),
        b"epoch".to_vec(),
        Tick::at(0u64),
        Tick::at(0u64),
        DeliveryCopy::Original,
    );
    let pack = recorded_live(&topology, vec![entry])?;
    let reread = read_recorded_live(&topology, pack.encoded())?;
    let (replay, opening) = Replay::opened(&reread);
    assert_eq!(opening.len(), 1usize);
    assert_eq!(replay.remaining(), 0usize);
    let exhaustion = replay.exhaust()?;
    assert_eq!(exhaustion.total(), 1usize);
    assert_eq!(exhaustion.final_tick(), Tick::at(0u64));
    Ok(())
}

/// Exhaustion refuses with the exact remaining-row count and cannot join reproduction from another address.
#[test]
fn incomplete_or_foreign_playback_cannot_open_the_join() -> Result<(), LaneFailure> {
    let (_first_rows, _first_schedule, first, first_reproduction) = packed_run(0usize)?;
    let (incomplete, first_opening) = Replay::opened(&first);
    assert!(first_opening.is_empty());
    let refusal = incomplete.exhaust().err().ok_or(LaneFailure::Standing)?;
    assert_eq!(refusal.address(), first.address());
    assert_eq!(refusal.remaining(), 2usize);

    let (_second_rows, _second_schedule, second, _second_reproduction) = packed_run(1usize)?;
    let (mut replay, second_opening) = Replay::opened(&second);
    assert!(second_opening.is_empty());
    while replay.remaining() > 0usize {
        let _handed_out = replay.advance();
    }
    let exhaustion = replay.exhaust()?;
    assert_eq!(
        ReproducedReplay::joined(first_reproduction, exhaustion),
        Err(ReproducedReplayRefusal::AddressMismatch {
            reproduction: first.address(),
            replay: second.address(),
        })
    );
    Ok(())
}
