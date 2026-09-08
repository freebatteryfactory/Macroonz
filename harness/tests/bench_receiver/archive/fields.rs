//! Text, nested provenance and row slots retain their owning admission rules.

use super::{LIMITS, fixture, types::DeclarationVector, wire};
use macroonz_harness::bench::archive::{BenchArchiveRefusal, read_report};
use macroonz_harness::descriptor::archive::{BindingArchiveRefusal, CandidateArchiveRefusal};
use macroonz_harness::report::archive::ArchiveRefusal;
use std::error::Error;

#[test]
fn names_and_free_text_keep_distinct_utf8_and_nonempty_rules() -> Result<(), Box<dyn Error>> {
    let base = fixture::vector()?;
    for invalid in [vec![], vec![0xff]] {
        for component in [0u8, 1] {
            let mut vector = base.clone();
            if component == 0 {
                vector.table.0.clone_from(&invalid);
            } else {
                vector.table.1.clone_from(&invalid);
            }
            assert_eq!(
                read_report(&vector.encoded(), LIMITS),
                Err(BenchArchiveRefusal::Canonical(ArchiveRefusal::InvalidText))
            );
        }
        let mut vector = base.clone();
        let row = vector.rows.first_mut().ok_or("row")?;
        let mut prefix = Vec::new();
        wire::name(b"harness.bench.consumer", b"linear-workload", &mut prefix);
        let rest = row
            .declaration
            .get(prefix.len()..)
            .ok_or("declaration suffix")?
            .to_vec();
        row.declaration.clear();
        wire::name(&invalid, b"linear-workload", &mut row.declaration);
        row.declaration.extend_from_slice(&rest);
        assert_eq!(
            read_report(&vector.encoded(), LIMITS),
            Err(BenchArchiveRefusal::Canonical(ArchiveRefusal::InvalidText))
        );
    }
    let mut invalid_target = base.clone();
    invalid_target.rows.first_mut().ok_or("row")?.target = vec![0xff];
    assert_eq!(
        read_report(&invalid_target.encoded(), LIMITS),
        Err(BenchArchiveRefusal::Canonical(ArchiveRefusal::InvalidText))
    );
    let mut invalid_cause = base.clone();
    let cause_row = invalid_cause.rows.first_mut().ok_or("row")?;
    cause_row.judgment = vec![1];
    wire::frame(&[0xff], &mut cause_row.judgment);
    wire::frame(b"", &mut cause_row.judgment);
    cause_row.judgment.extend_from_slice(&[0, 0]);
    assert_eq!(
        read_report(&invalid_cause.encoded(), LIMITS),
        Err(BenchArchiveRefusal::Canonical(ArchiveRefusal::InvalidText))
    );
    let mut invalid_observation = base;
    let row = invalid_observation.rows.first_mut().ok_or("row")?;
    row.measured = 3u64.to_be_bytes().to_vec();
    row.measured.extend_from_slice(&2u64.to_be_bytes());
    row.measured.extend_from_slice(&1u64.to_be_bytes());
    wire::name(b"harness.bench.consumer", &[0xff], &mut row.measured);
    assert_eq!(
        read_report(&invalid_observation.encoded(), LIMITS),
        Err(BenchArchiveRefusal::Canonical(ArchiveRefusal::InvalidText))
    );
    Ok(())
}

#[test]
fn provenance_width_slot_and_trailing_data_remain_descriptor_refusals() -> Result<(), Box<dyn Error>>
{
    let base = fixture::vector()?;
    for width in [0usize, 31, 33] {
        let mut vector = base.clone();
        vector.provenance = vec![1];
        wire::name(b"outside", b"producer", &mut vector.provenance);
        wire::frame(&vec![7; width], &mut vector.provenance);
        assert_eq!(
            read_report(&vector.encoded(), LIMITS),
            Err(BenchArchiveRefusal::Provenance(
                BindingArchiveRefusal::InvalidAddressWidth
            ))
        );
    }
    for (provenance, expected) in [
        (vec![3], BindingArchiveRefusal::InvalidSlot),
        (
            vec![0, 0],
            BindingArchiveRefusal::Canonical(CandidateArchiveRefusal::TrailingBytes),
        ),
    ] {
        let mut vector = base.clone();
        vector.provenance = provenance;
        assert_eq!(
            read_report(&vector.encoded(), LIMITS),
            Err(BenchArchiveRefusal::Provenance(expected))
        );
    }
    let mut invalid_name = base;
    invalid_name.provenance = vec![1];
    wire::name(b"", b"producer", &mut invalid_name.provenance);
    wire::frame(&[7; 32], &mut invalid_name.provenance);
    assert_eq!(
        read_report(&invalid_name.encoded(), LIMITS),
        Err(BenchArchiveRefusal::Provenance(
            BindingArchiveRefusal::Canonical(CandidateArchiveRefusal::InvalidName)
        ))
    );
    Ok(())
}

#[test]
fn row_formula_slot_and_overclaimed_frames_refuse() -> Result<(), Box<dyn Error>> {
    let mut vector = fixture::vector()?;
    let row = vector.rows.first_mut().ok_or("row")?;
    row.declaration = DeclarationVector {
        formula: None,
        ..DeclarationVector::lawful(&[2, 4, 8])
    }
    .encoded();
    let mut suffix = Vec::new();
    wire::name(b"harness.bench.consumer", b"linear-growth", &mut suffix);
    let slot = row
        .declaration
        .len()
        .checked_sub(suffix.len())
        .and_then(|size| size.checked_sub(1))
        .ok_or("formula slot")?;
    *row.declaration.get_mut(slot).ok_or("formula byte")? = 2;
    assert_eq!(
        read_report(&vector.encoded(), LIMITS),
        Err(BenchArchiveRefusal::Canonical(ArchiveRefusal::InvalidSlot))
    );
    let mut body = vector.body();
    body.get_mut(12..20)
        .ok_or("table namespace frame")?
        .copy_from_slice(&8193u64.to_be_bytes());
    assert_eq!(
        read_report(&wire::address(&body), LIMITS),
        Err(BenchArchiveRefusal::Canonical(ArchiveRefusal::Truncated))
    );
    vector.rows.first_mut().ok_or("row")?.declaration =
        DeclarationVector::lawful(&[2, 4, 8]).encoded();
    vector.table.0 = vec![b'x'; 8193];
    assert_eq!(
        read_report(&vector.encoded(), LIMITS),
        Err(BenchArchiveRefusal::Canonical(
            ArchiveRefusal::FieldTooLarge
        ))
    );
    Ok(())
}
