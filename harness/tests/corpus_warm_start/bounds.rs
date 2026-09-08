//! Independent seed-pack resource ceilings and their refusal precedence.

use super::{
    CorpusRoadFailure, SEED_PACK_FORMAT_VERSION, SeedInput, SeedPackLimits, SeedPackRefusal,
    envelope_for_body, foreign_body_prefix, foreign_envelope, pack, population, read,
};
use macroonz_harness::report::archive::ArchiveLimits;

#[test]
fn envelope_members_and_seed_count_have_independent_exact_limits() -> Result<(), CorpusRoadFailure>
{
    let population = population("bounded")?;
    let payload = [7u8; 64];
    let encoded = foreign_envelope(population, &[&payload, &[9]], &[]);
    let exact = SeedPackLimits::declared(ArchiveLimits::declared(encoded.len(), 64), 2);
    let record = read(population, &encoded, exact)?;
    assert_eq!(record.encoded(), encoded);
    assert_eq!(
        record.seeds().first().map(SeedInput::bytes),
        Some(payload.as_slice())
    );
    assert_eq!(
        read(
            population,
            &encoded,
            SeedPackLimits::declared(
                ArchiveLimits::declared(encoded.len().saturating_sub(1), 64),
                2
            )
        ),
        Err(SeedPackRefusal::EnvelopeTooLarge)
    );
    assert_eq!(
        read(
            population,
            &encoded,
            SeedPackLimits::declared(ArchiveLimits::declared(encoded.len(), 63), 2)
        ),
        Err(SeedPackRefusal::FieldTooLarge)
    );
    for count in [0, 1] {
        assert_eq!(
            read(
                population,
                &encoded,
                SeedPackLimits::declared(ArchiveLimits::declared(encoded.len(), 64), count)
            ),
            Err(SeedPackRefusal::TooManySeeds)
        );
    }
    assert_eq!(
        read(
            population,
            &encoded,
            SeedPackLimits::declared(ArchiveLimits::declared(16_384, 1_024), 128)
        )?,
        record
    );
    Ok(())
}

#[test]
fn bounds_precede_hashing_and_untrusted_roster_iteration() -> Result<(), CorpusRoadFailure> {
    let population = population("bounded")?;
    let limits = SeedPackLimits::declared(ArchiveLimits::declared(128, 64), 1);
    assert_eq!(
        read(population, &[0; 129], limits),
        Err(SeedPackRefusal::EnvelopeTooLarge)
    );
    let mut body = foreign_body_prefix(SEED_PACK_FORMAT_VERSION, population);
    body.extend_from_slice(&2u64.to_be_bytes());
    assert_eq!(
        read(population, &envelope_for_body(&body), limits),
        Err(SeedPackRefusal::TooManySeeds)
    );
    let encoded = foreign_envelope(population, &[&[7]], &[]);
    assert_eq!(
        read(
            population,
            &encoded,
            SeedPackLimits::declared(ArchiveLimits::declared(encoded.len(), 0), 1)
        ),
        Err(SeedPackRefusal::FieldTooLarge)
    );
    Ok(())
}

#[test]
fn informed_writer_and_bounded_reader_keep_the_existing_bytes() -> Result<(), CorpusRoadFailure> {
    let population = population("bounded")?;
    let written = pack(
        population,
        vec![
            SeedInput::declared(vec![3])?,
            SeedInput::declared(vec![8, 13])?,
        ],
    )?;
    let expected = foreign_envelope(population, &[&[3], &[8, 13]], &[]);
    assert_eq!(written.encoded(), expected);
    let limits = SeedPackLimits::declared(ArchiveLimits::declared(expected.len(), 64), 2);
    assert_eq!(read(population, &expected, limits)?, written);
    Ok(())
}
