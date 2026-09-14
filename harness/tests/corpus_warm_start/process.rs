//! Fresh-process seed admission preserves bytes without minting a verdict.

use super::{READ_LIMITS, SeedInput, foreign_envelope, pack, population, read};
use std::error::Error;
use std::io::Read as _;

#[test]
#[ignore = "driven by the seed-pack process parent"]
fn child_reads_seed_pack() -> Result<(), Box<dyn Error>> {
    let mut encoded = Vec::new();
    std::io::stdin()
        .lock()
        .take(16_385)
        .read_to_end(&mut encoded)?;
    let population = population("process").map_err(|e| std::io::Error::other(format!("{e:?}")))?;
    let record = read(population, &encoded, READ_LIMITS)
        .map_err(|e| std::io::Error::other(format!("{e:?}")))?;
    drop(encoded);
    let expected: &[&[u8]] = &[&[3], &[8, 13]];
    assert_eq!(
        record
            .seeds()
            .iter()
            .map(SeedInput::bytes)
            .collect::<Vec<_>>(),
        expected
    );
    super::archive_process::publish(record.encoded())
}

#[test]
fn actual_and_independent_packs_cross_a_fresh_process() -> Result<(), Box<dyn Error>> {
    let population = population("process").map_err(|e| std::io::Error::other(format!("{e:?}")))?;
    let seeds = [vec![3], vec![8, 13]]
        .into_iter()
        .map(SeedInput::declared)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| std::io::Error::other(format!("{e:?}")))?;
    let written = pack(population, seeds).map_err(|e| std::io::Error::other(format!("{e:?}")))?;
    let expected = foreign_envelope(population, &[&[3], &[8, 13]], &[]);
    assert_eq!(written.encoded(), expected);
    for encoded in [written.encoded(), expected.as_slice()] {
        assert_eq!(
            super::archive_process::round_trip(encoded, "process::child_reads_seed_pack")?,
            encoded
        );
    }
    Ok(())
}
