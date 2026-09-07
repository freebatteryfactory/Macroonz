//! Recomputed integrity cannot conceal contradictory reduction claims.

use super::{
    reduction_vector::{ReductionVector, Semantic, envelope},
    reductions::LIMITS,
    vector::InputKind,
};
use macroonz_harness::generate::reduce::archive::{ReductionArchiveRefusal, read_reduction};
use macroonz_harness::report::archive::ArchiveRefusal;

fn vector() -> ReductionVector {
    ReductionVector::declared(InputKind::Bound)
}

#[test]
fn hostile_counts_and_phase_claims_cannot_turn_into_a_completed_account() {
    let mut cases = Vec::new();
    let mut v = vector();
    v.budget = 0;
    cases.push((v, ReductionArchiveRefusal::ZeroBudget));
    v = vector();
    v.budget = 11;
    cases.push((v, ReductionArchiveRefusal::AccountingMismatch));
    v = vector();
    v.census = [u32::MAX, 1, 0];
    cases.push((v, ReductionArchiveRefusal::AccountingMismatch));
    v = vector();
    v.census = [1, 0, 0];
    cases.push((v, ReductionArchiveRefusal::AccountingMismatch));
    v = vector();
    v.halt = 1;
    cases.push((v, ReductionArchiveRefusal::AccountingMismatch));
    v = vector();
    v.byte = 1;
    cases.push((v, ReductionArchiveRefusal::AccountingMismatch));
    v = vector();
    v.semantic = vec![Semantic::offered(b"first", 3, 2)];
    cases.push((v, ReductionArchiveRefusal::AccountingMismatch));
    v = vector();
    v.semantic = vec![Semantic::offered(b"first", 3, 4)];
    cases.push((v, ReductionArchiveRefusal::AccountingMismatch));
    v = vector();
    v.semantic = vec![Semantic::offered(b"first", u64::MAX, u64::MAX)];
    cases.push((v, ReductionArchiveRefusal::AccountingMismatch));
    v = vector();
    v.semantic.push(Semantic::offered(b"first", 0, 0));
    cases.push((v, ReductionArchiveRefusal::DuplicateReducer));
    v = vector();
    v.budget = 5;
    v.census = [5, 0, 0];
    v.halt = 1;
    cases.push((v, ReductionArchiveRefusal::AccountingMismatch));
    v = vector();
    v.budget = 5;
    v.census = [5, 0, 0];
    v.byte = 1;
    cases.push((v, ReductionArchiveRefusal::AccountingMismatch));
    v = vector();
    v.budget = 3;
    v.census = [3, 0, 0];
    v.byte = 1;
    v.halt = 1;
    cases.push((v, ReductionArchiveRefusal::AccountingMismatch));
    for (case, expected) in cases {
        assert_eq!(read_reduction(&case.encoded(), LIMITS), Err(expected));
    }
}

#[test]
fn portable_offered_counts_and_exactly_spent_fixed_points_remain_lawful() -> Result<(), ()> {
    let mut v = vector();
    v.semantic = vec![Semantic::offered(b"partial", u64::MAX, 2)];
    v.budget = 2;
    v.census = [2, 0, 0];
    v.byte = 1;
    v.halt = 1;
    let record = read_reduction(&v.encoded(), LIMITS).map_err(|_| ())?;
    assert_eq!(
        record.semantic_reducers().first().ok_or(())?.candidates(),
        u64::MAX
    );
    v = vector();
    v.budget = 12;
    assert!(read_reduction(&v.encoded(), LIMITS).is_ok());
    Ok(())
}

#[test]
fn participant_ceilings_meet_without_upgrading_decoder_or_capsule_claims() -> Result<(), ()> {
    let mut cases = Vec::new();
    let mut v = vector();
    v.probe_posture = 0;
    cases.push(v);
    v = vector();
    v.report_posture = 2;
    cases.push(v);
    v = vector();
    v.probe_posture = 2;
    cases.push(v);
    v = vector();
    v.semantic.first_mut().ok_or(())?.posture = 2;
    cases.push(v);
    v = vector();
    *v.capsule.key.last_mut().ok_or(())? = 1;
    let key = super::vector::hash("input-execution-key/v1", &v.capsule.key);
    v.capsule
        .capsule
        .get_mut(8..40)
        .ok_or(())?
        .copy_from_slice(&key);
    cases.push(v);
    for case in cases {
        assert_eq!(
            read_reduction(&case.encoded(), LIMITS),
            Err(ReductionArchiveRefusal::PostureMismatch)
        );
    }
    Ok(())
}

#[test]
fn malformed_reducer_names_revisions_and_discriminants_refuse_with_valid_integrity()
-> Result<(), ()> {
    let mut cases = Vec::new();
    let mut v = vector();
    v.probe.clear();
    cases.push((v, ArchiveRefusal::InvalidAddressWidth));
    v = vector();
    v.probe_posture = 9;
    cases.push((v, ArchiveRefusal::InvalidSlot));
    v = vector();
    v.report_posture = 9;
    cases.push((v, ArchiveRefusal::InvalidSlot));
    v = vector();
    v.byte = 9;
    cases.push((v, ArchiveRefusal::InvalidSlot));
    v = vector();
    v.halt = 9;
    cases.push((v, ArchiveRefusal::InvalidSlot));
    for invalid in [Vec::new(), vec![0xff]] {
        v = vector();
        v.semantic.first_mut().ok_or(())?.namespace = invalid.clone();
        cases.push((v, ArchiveRefusal::InvalidText));
        v = vector();
        v.semantic.first_mut().ok_or(())?.stem = invalid;
        cases.push((v, ArchiveRefusal::InvalidText));
    }
    v = vector();
    v.semantic.first_mut().ok_or(())?.revision = vec![8; 31];
    cases.push((v, ArchiveRefusal::InvalidAddressWidth));
    v = vector();
    v.semantic.first_mut().ok_or(())?.posture = 9;
    cases.push((v, ArchiveRefusal::InvalidSlot));
    for (case, expected) in cases {
        assert_eq!(
            read_reduction(&case.encoded(), LIMITS),
            Err(ReductionArchiveRefusal::Archive(expected))
        );
    }
    Ok(())
}

#[test]
fn envelope_count_truncation_and_nested_integrity_refuse_before_historical_admission()
-> Result<(), ()> {
    let v = vector();
    let body = v.body();
    for end in 0..body.len() {
        assert!(read_reduction(&envelope(body.get(..end).ok_or(())?), LIMITS).is_err());
    }
    let mut changed = body.clone();
    changed.push(0);
    assert_eq!(
        read_reduction(&envelope(&changed), LIMITS),
        Err(ReductionArchiveRefusal::Archive(
            ArchiveRefusal::TrailingBytes
        ))
    );
    for (offset, value, expected) in [
        (0usize, 2u32, ArchiveRefusal::UnsupportedFormat { found: 2 }),
        (4, 3, ArchiveRefusal::WrongKind { found: 3 }),
        (8, 1, ArchiveRefusal::UnsupportedCustody { found: 1 }),
    ] {
        changed = body.clone();
        changed
            .get_mut(offset..offset.saturating_add(4))
            .ok_or(())?
            .copy_from_slice(&value.to_be_bytes());
        assert_eq!(
            read_reduction(&envelope(&changed), LIMITS),
            Err(ReductionArchiveRefusal::Archive(expected))
        );
    }
    changed = body.clone();
    *changed.get_mut(20).ok_or(())? ^= 1;
    assert_eq!(
        read_reduction(&envelope(&changed), LIMITS),
        Err(ReductionArchiveRefusal::Archive(
            ArchiveRefusal::AddressMismatch
        ))
    );
    let count_offset = 66usize.saturating_add(v.capsule.encoded().len());
    changed = body;
    changed
        .get_mut(count_offset..count_offset.saturating_add(8))
        .ok_or(())?
        .copy_from_slice(&u64::MAX.to_be_bytes());
    assert_eq!(
        read_reduction(&envelope(&changed), LIMITS),
        Err(ReductionArchiveRefusal::TooManyReducers)
    );
    Ok(())
}
