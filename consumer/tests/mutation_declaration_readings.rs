//! The generated mutation producer retains discoveries that owner admission withholds.

use harness::muterprater::EvaluationDirective;
use harness::muterprater::{DiscoveryDisposition, MappedUnpermittedCause};

threadpak_consumer::owner_unmapped_support! {
    harness: harness,
}

threadpak_consumer::mapped_unpermitted_support! {
    harness: harness,
}

fn assert_complete_generated_reading(
    production: macroonz::DeclaredCauseOrder,
    candidate: macroonz::DeclaredCauseOrder,
    evaluation: &harness::muterprater::EvaluationObservation<macroonz::DeclaredCauseOrder>,
) {
    assert_eq!(
        production
            .iter()
            .map(macroonz::DeclaredCause::spelling)
            .collect::<Vec<_>>(),
        vec!["First", "Second"],
    );
    assert_eq!(
        candidate
            .iter()
            .map(macroonz::DeclaredCause::spelling)
            .collect::<Vec<_>>(),
        vec!["Second", "First"],
    );
    assert_eq!(evaluation.meaning(), &production);
    assert_eq!(evaluation.firings(), 0);
}

/// An owner-unmapped generated site stays in the complete reading and cannot enter the executable surface.
#[test]
fn owner_unmapped_discovery_is_retained_but_not_executable() -> Result<(), ()> {
    let lowering = generated_owner_unmapped_mutations::lowering().map_err(|_| ())?;
    let [entry] = lowering.discovery().entries() else {
        return Err(());
    };
    assert_eq!(entry.disposition(), DiscoveryDisposition::OwnerUnmapped);
    assert!(lowering.surface().points().is_empty());
    let production = generated_owner_unmapped_mutations::production(&());
    let [candidate] = generated_owner_unmapped_mutations::candidate_orders();
    let evaluation =
        generated_owner_unmapped_mutations::evaluation(&(), EvaluationDirective::no_mutation())
            .map_err(|_| ())?;
    assert_complete_generated_reading(production, candidate, &evaluation);
    Ok(())
}

/// A mapped site without owner permission stays in the complete reading and cannot enter the executable surface.
#[test]
fn mapped_unpermitted_discovery_is_retained_but_not_executable() -> Result<(), ()> {
    let lowering = generated_mapped_unpermitted_mutations::lowering().map_err(|_| ())?;
    let [entry] = lowering.discovery().entries() else {
        return Err(());
    };
    assert!(matches!(
        entry.disposition(),
        DiscoveryDisposition::MappedUnpermitted {
            cause: MappedUnpermittedCause::Claim(claim),
        } if claim.name().namespace().written() == "consumer"
            && claim.name().stem().written() == "mapped-order"
    ));
    assert!(lowering.surface().points().is_empty());
    let production = generated_mapped_unpermitted_mutations::production(&());
    let [candidate] = generated_mapped_unpermitted_mutations::candidate_orders();
    let evaluation =
        generated_mapped_unpermitted_mutations::evaluation(&(), EvaluationDirective::no_mutation())
            .map_err(|_| ())?;
    assert_complete_generated_reading(production, candidate, &evaluation);
    Ok(())
}
