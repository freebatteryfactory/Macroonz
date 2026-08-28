//! Outside claims over the harness-owned operator-family bank.

use macroonz_harness::depot::operator_families::OPERATOR_FAMILIES;
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
