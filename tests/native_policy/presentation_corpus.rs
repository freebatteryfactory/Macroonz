//! Seed packs remain ordered input material when displayed through the root facade.

use crate::presentation_formats::{decoded_hex, field, parsed};
use macroonz::harness::corpus::{self, SeedInput, SeedPackLimits, SeedPackRefusal};
use macroonz::harness::descriptor::PopulationRef;
use macroonz::harness::report::archive::ArchiveLimits;
use macroonz::presentation;
use serde_json::json;

const LIMITS: SeedPackLimits = SeedPackLimits::declared(ArchiveLimits::declared(4096, 128), 8);

fn debug(error: impl std::fmt::Debug) -> String {
    format!("{error:?}")
}

#[test]
fn presentation_seed_pack_preserves_order_exact_envelope_and_no_execution_claim()
-> Result<(), String> {
    let population = PopulationRef::named("display", "seeds").map_err(debug)?;
    let raw = [vec![0_u8, 255, 60, 38, 62], vec![7_u8], vec![0_u8]];
    let seeds = raw
        .iter()
        .cloned()
        .map(SeedInput::declared)
        .collect::<Result<Vec<_>, _>>()
        .map_err(debug)?;
    let written = corpus::pack(population, seeds).map_err(debug)?;
    let reread = corpus::read(population, written.encoded(), LIMITS).map_err(debug)?;
    let shown = parsed(&presentation::seed_pack(&reread))?;
    assert_eq!(field(&shown, "/owner")?, "macroonz-harness/corpus");
    assert_eq!(
        field(&shown, "/record/population")?,
        &json!({"namespace":"display","stem":"seeds"})
    );
    assert_eq!(
        decoded_hex(field(&shown, "/record/encoded")?)?,
        written.encoded()
    );
    assert_eq!(
        decoded_hex(field(&shown, "/record/address")?)?,
        written.address().address().as_bytes()
    );
    let retained = field(&shown, "/record/seeds")?
        .as_array()
        .ok_or("missing seeds")?;
    assert_eq!(retained.len(), raw.len());
    for (entry, expected) in retained.iter().zip(&raw) {
        assert_eq!(decoded_hex(entry)?, *expected);
    }
    assert!(shown.pointer("/record/verdict").is_none());
    assert!(shown.pointer("/record/reproduction").is_none());
    let reversed = raw
        .iter()
        .rev()
        .cloned()
        .map(SeedInput::declared)
        .collect::<Result<Vec<_>, _>>()
        .map_err(debug)?;
    let reordered = corpus::pack(population, reversed).map_err(debug)?;
    let changed = parsed(&presentation::seed_pack(&reordered))?;
    assert_ne!(
        field(&shown, "/record/address")?,
        field(&changed, "/record/address")?
    );
    assert_eq!(
        decoded_hex(field(&changed, "/record/seeds/0")?)?,
        vec![0_u8]
    );
    Ok(())
}

#[test]
fn presentation_seed_refusals_keep_duplicate_positions_address_and_reader_limits()
-> Result<(), String> {
    let population = PopulationRef::named("display", "seeds").map_err(debug)?;
    let seed = SeedInput::declared(vec![9_u8]).map_err(debug)?;
    let duplicate = corpus::pack(
        population,
        vec![
            seed.clone(),
            SeedInput::declared(vec![2_u8]).map_err(debug)?,
            seed.clone(),
        ],
    )
    .err()
    .ok_or("duplicate accepted")?;
    let duplicate_display = parsed(&presentation::seed_pack_refusal(&duplicate))?;
    assert_eq!(
        field(&duplicate_display, "/record")?,
        &json!({"kind":"duplicate-seed","value":{"first":0_usize,"duplicate":2_usize}})
    );
    let pack = corpus::pack(population, vec![seed]).map_err(debug)?;
    let mut corrupted = pack.encoded().to_vec();
    *corrupted.first_mut().ok_or("missing address")? ^= 1_u8;
    let mismatch = corpus::read(population, &corrupted, LIMITS)
        .err()
        .ok_or("corrupt claim admitted")?;
    assert_eq!(
        mismatch,
        SeedPackRefusal::AddressMismatch {
            derived: pack.address()
        }
    );
    let mismatch_display = parsed(&presentation::seed_pack_refusal(&mismatch))?;
    assert_eq!(
        decoded_hex(field(&mismatch_display, "/record/value/derived")?)?,
        pack.address().address().as_bytes()
    );
    assert!(mismatch_display.pointer("/record/value/claimed").is_none());
    for (limits, kind) in [
        (
            SeedPackLimits::declared(ArchiveLimits::declared(0, 128), 8),
            "envelope-too-large",
        ),
        (
            SeedPackLimits::declared(ArchiveLimits::declared(4096, 0), 8),
            "field-too-large",
        ),
        (
            SeedPackLimits::declared(ArchiveLimits::declared(4096, 128), 0),
            "too-many-seeds",
        ),
    ] {
        let refusal = corpus::read(population, pack.encoded(), limits)
            .err()
            .ok_or("reader ceiling ignored")?;
        let bounded_display = parsed(&presentation::seed_pack_refusal(&refusal))?;
        assert_eq!(field(&bounded_display, "/record/kind")?, kind);
    }
    Ok(())
}
