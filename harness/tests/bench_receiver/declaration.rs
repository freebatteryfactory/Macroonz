//! Claim: declaration guards and the independent row transcript preserve one exact benchmark identity algebra.
//! Subject: public row, attachment, binding, and table constructors.
//! Population: lawful fixture declarations and each real vacuity, duplicate, or relationship mismatch.
//! Reversal: every constructor is challenged with the invalid state its type is meant to withhold.
//! Denominator: the complete declaration and binding boundary exposed by this receiver.
//! Evidence ceiling: public construction and identity bytes only, not host execution or performance.
//! Retained regression: changed refusal priority, accepted vacuity, duplicate identity, or row-key preimage drift.

use super::{fixture, support::*};

#[test]
fn row_key_matches_an_independent_eight_fact_transcript() -> Result<(), BenchRoadFailure> {
    assert_eq!(
        fixture::lawful_row()?.key().address(),
        independent_lawful_row_address()?
    );
    Ok(())
}

#[test]
fn declaration_boundaries_refuse_vacuity_and_duplicates() -> Result<(), BenchRoadFailure> {
    assert!(matches!(
        InputSizeAxis::declared(Vec::new()),
        Err(InputSizeAxisRefusal::TooShort { found: 0 })
    ));
    assert!(matches!(
        InputSizeAxis::declared(vec![4u64, 4u64]),
        Err(InputSizeAxisRefusal::DuplicateSize {
            size: 4,
            first: 0,
            duplicate: 1,
        })
    ));
    assert!(matches!(
        DeclaredBudgets::declared(0u32, 0u32, 1u64, 1u64),
        Err(DeclaredBudgetsRefusal::NoSamples)
    ));
    assert!(matches!(
        DeclaredBudgets::declared(1u32, 0u32, 0u64, 1u64),
        Err(DeclaredBudgetsRefusal::ZeroRatioNumerator)
    ));
    assert!(matches!(
        DeclaredBudgets::declared(1u32, 0u32, 1u64, 0u64),
        Err(DeclaredBudgetsRefusal::ZeroRatioDenominator)
    ));
    assert!(matches!(
        WorkFormula::encoded(Vec::new()),
        Err(WorkFormulaRefusal::Empty)
    ));

    let observation = WorkObservationRef::named(OWNER, "one-observation")?;
    let workload = WorkloadRef::named(OWNER, "workload")?;
    let worse = PlantedWorseRef::named(OWNER, "worse")?;
    let complexity = ComplexityClaimRef::named(OWNER, "complexity")?;
    assert!(matches!(
        BenchAttachment::attached(
            workload,
            fixture::measured,
            worse,
            fixture::planted_worse,
            WorkJudgeBinding::bound(complexity, fixture::lawful_judge),
            Vec::new(),
        ),
        Err(BenchAttachmentRefusal::NoObservation)
    ));
    assert!(matches!(
        BenchAttachment::attached(
            workload,
            fixture::measured,
            worse,
            fixture::planted_worse,
            WorkJudgeBinding::bound(complexity, fixture::lawful_judge),
            vec![observation, observation],
        ),
        Err(BenchAttachmentRefusal::DuplicateObservation {
            observation: repeated,
            first: 0,
            duplicate: 1,
        }) if repeated == observation
    ));
    Ok(())
}

#[test]
fn binding_refusals_name_each_mismatched_relationship() -> Result<(), BenchRoadFailure> {
    let row = fixture::lawful_row()?;
    let preflight = fixture::lawful_preflight(fixture::preflight_passes)?;
    let observation = WorkObservationRef::named(OWNER, "unit-work")?;
    let foreign_workload = WorkloadRef::named(OWNER, "foreign-workload")?;
    let foreign_worse = PlantedWorseRef::named(OWNER, "foreign-control")?;
    let foreign_complexity = ComplexityClaimRef::named(OWNER, "foreign-complexity")?;
    let foreign_preflight = PreflightRef::named(OWNER, "foreign-preflight")?;

    let workload_attachment = fixture::attachment_with_refs(
        foreign_workload,
        row.planted_worse(),
        row.complexity(),
        fixture::measured,
        fixture::planted_worse,
        fixture::lawful_judge,
        vec![observation],
    )?;
    assert!(matches!(
        BenchBinding::bound(row.clone(), workload_attachment, preflight.clone()),
        Err(BenchBindingRefusal::Workload { .. })
    ));

    let planted_worse_attachment = fixture::attachment_with_refs(
        row.workload(),
        foreign_worse,
        row.complexity(),
        fixture::measured,
        fixture::planted_worse,
        fixture::lawful_judge,
        vec![observation],
    )?;
    assert!(matches!(
        BenchBinding::bound(row.clone(), planted_worse_attachment, preflight.clone()),
        Err(BenchBindingRefusal::PlantedWorse { .. })
    ));

    let complexity_attachment = fixture::attachment_with_refs(
        row.workload(),
        row.planted_worse(),
        foreign_complexity,
        fixture::measured,
        fixture::planted_worse,
        fixture::lawful_judge,
        vec![observation],
    )?;
    assert!(matches!(
        BenchBinding::bound(row.clone(), complexity_attachment, preflight.clone()),
        Err(BenchBindingRefusal::Complexity { .. })
    ));

    let foreign_preflight_trial = fixture::preflight_with(
        foreign_preflight,
        fixture::preflight_passes,
        fixture::target(),
    )?;
    assert!(matches!(
        BenchBinding::bound(
            row,
            fixture::lawful_attachment(
                fixture::measured,
                fixture::planted_worse,
                fixture::lawful_judge
            )?,
            foreign_preflight_trial
        ),
        Err(BenchBindingRefusal::Preflight { .. })
    ));
    Ok(())
}

#[test]
fn table_refuses_vacuity_and_exact_duplicate_identity() -> Result<(), BenchRoadFailure> {
    let name = BenchTableName::named(OWNER, "table-refusals")?;
    assert!(matches!(
        BenchTable::authored(name, Provenance::Unproduced, Vec::new()),
        Err(BenchTableRefusal::Empty)
    ));
    let binding = fixture::lawful_binding()?;
    let expected_row = binding.row().key();
    assert!(matches!(
        BenchTable::authored(name, Provenance::Unproduced, vec![binding.clone(), binding]),
        Err(BenchTableRefusal::DuplicateRow {
            row,
            first: 0,
            duplicate: 1,
        }) if row == expected_row
    ));
    Ok(())
}
