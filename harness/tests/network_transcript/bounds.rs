//! Independent decoder ceilings preserve canonical bytes and source-specific authority.

use super::{support::*, vector};
use macroonz_harness::report::archive::ArchiveLimits;

#[test]
fn independent_envelopes_match_the_existing_live_and_simulated_writers() -> Result<(), LaneFailure>
{
    let topology = pair_topology()?;
    let live = recorded_live(&topology, vec![live_entry(b"pay", 0, 1)?])?;
    let (_rows, schedule, simulated, standing) = packed_run(0)?;
    assert_eq!(live.encoded(), vector::live(b"pay", 1));
    assert_eq!(simulated.encoded(), vector::simulated(b"pay", 2, 2));
    assert_eq!(
        read_recorded_live(&topology, live.encoded(), READ_LIMITS)?,
        live
    );
    let loaded = read_simulated(&topology, &schedule, simulated.encoded(), READ_LIMITS)?;
    assert_eq!(loaded, simulated);
    assert_eq!(reproduce(&loaded)?, standing);
    Ok(())
}

#[test]
fn every_decoder_ceiling_is_independent_and_inclusive() -> Result<(), LaneFailure> {
    let topology = pair_topology()?;
    let schedule = duplicate_schedule()?;
    let payload = [7u8; 128];
    let encoded = vector::simulated(&payload, 2, 2);
    let exact = TranscriptLimits::declared(ArchiveLimits::declared(encoded.len(), 128), 2, 2);
    let record = read_simulated(&topology, &schedule, &encoded, exact)?;
    assert_eq!(record.encoded(), encoded);
    assert_eq!(reproduce(&record)?.rows(), 2);
    let smaller = [
        (
            TranscriptLimits::declared(
                ArchiveLimits::declared(encoded.len().saturating_sub(1), 128),
                2,
                2,
            ),
            TranscriptRefusal::EnvelopeTooLarge,
        ),
        (
            TranscriptLimits::declared(ArchiveLimits::declared(encoded.len(), 127), 2, 2),
            TranscriptRefusal::FieldTooLarge,
        ),
        (
            TranscriptLimits::declared(ArchiveLimits::declared(encoded.len(), 128), 1, 2),
            TranscriptRefusal::TooManyActions,
        ),
        (
            TranscriptLimits::declared(ArchiveLimits::declared(encoded.len(), 128), 2, 1),
            TranscriptRefusal::TooManyEntries,
        ),
    ];
    for (limits, refusal) in smaller {
        assert_eq!(
            read_simulated(&topology, &schedule, &encoded, limits),
            Err(refusal)
        );
    }
    let live = vector::live(&payload, 1);
    let live_limits = TranscriptLimits::declared(ArchiveLimits::declared(live.len(), 128), 0, 1);
    assert_eq!(
        read_recorded_live(&topology, &live, live_limits)?.encoded(),
        live
    );
    assert_eq!(
        read_recorded_live(
            &topology,
            &live,
            TranscriptLimits::declared(ArchiveLimits::declared(live.len(), 127), 0, 1)
        ),
        Err(TranscriptRefusal::FieldTooLarge)
    );
    assert_eq!(
        read_recorded_live(
            &topology,
            &live,
            TranscriptLimits::declared(ArchiveLimits::declared(live.len(), 128), 0, 0)
        ),
        Err(TranscriptRefusal::TooManyEntries)
    );
    assert_eq!(
        read_simulated(&topology, &schedule, &encoded, READ_LIMITS)?,
        record
    );
    Ok(())
}

#[test]
fn oversized_rosters_refuse_before_following_their_declared_members() -> Result<(), LaneFailure> {
    let topology = pair_topology()?;
    let schedule = duplicate_schedule()?;
    let limits = TranscriptLimits::declared(ArchiveLimits::declared(4_096, 128), 2, 2);
    assert_eq!(
        read_simulated(&topology, &schedule, &[0; 4_097], limits),
        Err(TranscriptRefusal::EnvelopeTooLarge)
    );
    assert_eq!(
        read_recorded_live(&topology, &[0; 4_097], limits),
        Err(TranscriptRefusal::EnvelopeTooLarge)
    );
    assert_eq!(
        read_simulated(
            &topology,
            &schedule,
            &vector::simulated(b"pay", 3, 2),
            limits
        ),
        Err(TranscriptRefusal::TooManyActions)
    );
    assert_eq!(
        read_simulated(
            &topology,
            &schedule,
            &vector::simulated(b"pay", 2, 3),
            limits
        ),
        Err(TranscriptRefusal::TooManyEntries)
    );
    assert_eq!(
        read_recorded_live(&topology, &vector::live(b"pay", 3), limits),
        Err(TranscriptRefusal::TooManyEntries)
    );
    let no_names = TranscriptLimits::declared(ArchiveLimits::declared(4_096, 0), 2, 2);
    assert_eq!(
        read_recorded_live(&topology, &vector::live(b"pay", 1), no_names),
        Err(TranscriptRefusal::FieldTooLarge)
    );
    Ok(())
}

#[test]
fn bounds_do_not_normalize_or_promote_live_adapter_claims() -> Result<(), LaneFailure> {
    let topology = pair_topology()?;
    let row = live_entry(b"pay", 0, 1)?;
    let written = recorded_live(&topology, vec![row.clone(), row])?;
    let read = read_recorded_live(&topology, written.encoded(), READ_LIMITS)?;
    assert_eq!(read, written);
    assert_eq!(read.entries().len(), 2);
    assert_eq!(
        reproduce(&read),
        Err(TranscriptRefusal::RecordedLiveCannotReproduce)
    );
    Ok(())
}
