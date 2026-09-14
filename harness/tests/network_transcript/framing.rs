//! Readdressed hostile envelopes exercise framing without borrowing a valid content claim.

use super::{support::*, vector};

fn changed_word<const WIDTH: usize>(
    encoded: &[u8],
    at: usize,
    word: [u8; WIDTH],
) -> Result<Vec<u8>, LaneFailure> {
    let mut changed = encoded.to_vec();
    changed
        .get_mut(at..at.saturating_add(WIDTH))
        .ok_or(LaneFailure::Standing)?
        .copy_from_slice(&word);
    readdress(&mut changed)?;
    Ok(changed)
}

#[test]
fn both_readers_refuse_every_truncated_body_and_trailing_material() -> Result<(), LaneFailure> {
    let topology = pair_topology()?;
    let schedule = duplicate_schedule()?;
    for simulated in [false, true] {
        let encoded = if simulated {
            vector::simulated(b"pay", 2, 2)
        } else {
            vector::live(b"pay", 1)
        };
        let read = |bytes: &[u8]| {
            if simulated {
                read_simulated(&topology, &schedule, bytes, READ_LIMITS)
            } else {
                read_recorded_live(&topology, bytes, READ_LIMITS)
            }
        };
        assert!(read(&encoded).is_ok());
        for end in 0..encoded.len() {
            let prefix = encoded.get(..end).ok_or(LaneFailure::Standing)?;
            assert!(read(prefix).is_err());
            if let Some(body) = encoded.get(32..end) {
                assert!(read(&vector::envelope(body)).is_err());
            }
        }
        let mut trailing = encoded.clone();
        trailing.push(0);
        readdress(&mut trailing)?;
        assert_eq!(
            read(&trailing),
            Err(TranscriptRefusal::TrailingBytes { count: 1 })
        );
    }
    Ok(())
}

#[test]
fn unknown_slots_refuse_under_independently_addressed_bodies() -> Result<(), LaneFailure> {
    let topology = pair_topology()?;
    let schedule = duplicate_schedule()?;
    let encoded = vector::simulated(b"pay", 2, 2);
    assert_eq!(encoded.len(), 590);
    for (at, refusal) in [
        (36, TranscriptRefusal::UnknownSourceClaim { found: 99 }),
        (321, TranscriptRefusal::UnknownFault { found: 99 }),
        (337, TranscriptRefusal::UnknownAction { found: 99 }),
        (499, TranscriptRefusal::UnknownCopy { found: 99 }),
    ] {
        let changed = changed_word(&encoded, at, 99u32.to_be_bytes())?;
        assert_eq!(
            read_simulated(&topology, &schedule, &changed, READ_LIMITS),
            Err(refusal)
        );
    }
    Ok(())
}

#[test]
fn declared_topology_and_schedule_bound_their_own_rosters() -> Result<(), LaneFailure> {
    let topology = pair_topology()?;
    let schedule = duplicate_schedule()?;
    let encoded = vector::simulated(b"pay", 2, 2);
    for (at, count, refusal) in [
        (40, 3u64, TranscriptRefusal::TopologyMismatch),
        (100, 3, TranscriptRefusal::TopologyMismatch),
        (253, 2, TranscriptRefusal::ScheduleMismatch),
        (313, 2, TranscriptRefusal::ScheduleMismatch),
    ] {
        let changed = changed_word(&encoded, at, count.to_be_bytes())?;
        assert_eq!(
            read_simulated(&topology, &schedule, &changed, READ_LIMITS),
            Err(refusal)
        );
    }
    for (at, refusal) in [
        (329, TranscriptRefusal::TooManyActions),
        (408, TranscriptRefusal::TooManyEntries),
    ] {
        let changed = changed_word(&encoded, at, u64::MAX.to_be_bytes())?;
        let result = read_simulated(&topology, &schedule, &changed, READ_LIMITS);
        assert!(
            result == Err(refusal)
                || result == Err(TranscriptRefusal::LengthOutsidePlatform { declared: u64::MAX })
        );
    }
    let changed = changed_word(&encoded, 48, u64::MAX.to_be_bytes())?;
    assert!(matches!(
        read_simulated(&topology, &schedule, &changed, READ_LIMITS),
        Err(TranscriptRefusal::Truncated
            | TranscriptRefusal::LengthOutsidePlatform { declared: u64::MAX })
    ));
    Ok(())
}
