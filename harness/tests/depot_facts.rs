//! Outside claims over the harness-owned authored-fact banks.

use macroonz_harness::depot::operator_families::OPERATOR_FAMILIES;
use macroonz_harness::descriptor::{
    BENCH_FIELDS, FieldCardinality, FieldShape, MUTATION_DISCOVERY_FIELDS, SchemaField,
};
use macroonz_harness::muterprater::OperatorFamilyRef;
use std::collections::BTreeSet;

/// Claim: Every authored operator family reaches the mutation-pressure resolver without losing its stable slug or its account of the damage.
/// Subject: The complete public operator-family bank and the public bank-backed resolver.
/// Population: Every row the bank declares.
/// Hostile control: An unbanked slug remains outside the vocabulary, and a repeated or empty authored coordinate fails the observation.
/// Denominator: Derived by traversing the complete public bank rather than restating its width.
/// Evidence ceiling: This establishes bank reachability and lookup fidelity, not that any damage was applied, activated, killed, or survived.
/// Retained regression: Orphaned rows, duplicate slugs, empty authored coordinates, and lookup drift remain permanent owner regressions.
#[test]
fn every_operator_family_reaches_the_bank_backed_resolver() -> Result<(), &'static str> {
    let mut slugs = BTreeSet::new();
    for family in OPERATOR_FAMILIES {
        if family.slug().is_empty() || family.attacks().is_empty() || !slugs.insert(family.slug()) {
            return Err("operator-family coordinates must be nonempty and unique");
        }
        let resolved = OperatorFamilyRef::of_slug(family.slug())
            .ok_or("every bank row must resolve through mutation pressure")?;
        if resolved.family() != *family {
            return Err("operator-family resolution must preserve the authored row");
        }
    }
    if OperatorFamilyRef::of_slug("not-authored-by-the-harness").is_some() {
        return Err("an unbanked slug must remain outside the bank");
    }
    Ok(())
}

/// Claim: The producer-field banks retain every mutation-discovery and benchmark field with its exact shape, cardinality, and order.
/// Subject: The two public schema-field roster projections authored by the depot.
/// Population: Every row of both producer-facing banks.
/// Hostile control: Reversing either independently written expected roster disagrees with the public projection.
/// Denominator: Both complete public field rosters.
/// Evidence ceiling: This establishes authored vocabulary only and does not admit a producer delivery or establish schema-pin currency.
/// Retained regression: Field loss, reordering, shape drift, and cardinality drift remain permanent owner regressions.
#[test]
fn producer_field_banks_project_exact_public_rosters() {
    let mutation_discovery = [
        SchemaField::declared(
            "identity",
            FieldShape::NamespacedName,
            FieldCardinality::ExactlyOne,
        ),
        SchemaField::declared(
            "owner_claim",
            FieldShape::NamespacedName,
            FieldCardinality::ZeroOrOne,
        ),
        SchemaField::declared(
            "original_operation",
            FieldShape::Bytes,
            FieldCardinality::ExactlyOne,
        ),
        SchemaField::declared(
            "candidate_alternatives",
            FieldShape::MutationAlternative,
            FieldCardinality::OneOrMore,
        ),
        SchemaField::declared(
            "activation_site",
            FieldShape::NamespacedName,
            FieldCardinality::ExactlyOne,
        ),
    ];
    let bench = [
        SchemaField::declared(
            "workload_identity",
            FieldShape::NamespacedName,
            FieldCardinality::ExactlyOne,
        ),
        SchemaField::declared(
            "input_size_axis",
            FieldShape::Count,
            FieldCardinality::ZeroOrMore,
        ),
        SchemaField::declared(
            "correctness_preflight",
            FieldShape::NamespacedName,
            FieldCardinality::ExactlyOne,
        ),
        SchemaField::declared(
            "planted_worse_falsifier",
            FieldShape::NamespacedName,
            FieldCardinality::ExactlyOne,
        ),
        SchemaField::declared(
            "declared_budgets",
            FieldShape::Count,
            FieldCardinality::ZeroOrMore,
        ),
        SchemaField::declared(
            "contention_posture",
            FieldShape::ClosedChoice(&["no-declared-contention"]),
            FieldCardinality::ExactlyOne,
        ),
        SchemaField::declared(
            "work_formula",
            FieldShape::Bytes,
            FieldCardinality::ZeroOrOne,
        ),
        SchemaField::declared(
            "complexity_claim",
            FieldShape::NamespacedName,
            FieldCardinality::ExactlyOne,
        ),
    ];

    assert_eq!(MUTATION_DISCOVERY_FIELDS, mutation_discovery);
    assert_eq!(BENCH_FIELDS, bench);
    assert_ne!(
        MUTATION_DISCOVERY_FIELDS,
        mutation_discovery.iter().copied().rev().collect::<Vec<_>>()
    );
    assert_ne!(
        BENCH_FIELDS,
        bench.iter().copied().rev().collect::<Vec<_>>()
    );
}
