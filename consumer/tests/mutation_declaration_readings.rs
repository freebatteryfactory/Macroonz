//! The generated mutation producer retains discoveries that owner admission withholds.

use harness::muterprater::{DiscoveryDisposition, MappedUnpermittedCause};

#[derive(tp_macros::RefusalFamily)]
#[refusal(
    crate = tp,
    family = "consumer.owner-unmapped-producer",
    shape = single_cause,
    order(First = "first", Second = "second")
)]
#[threadpak_mutations(
    support = owner_unmapped_support,
    module = generated_owner_unmapped_mutations,
    family = named("consumer", "owner-unmapped-evaluation"),
    permit named("consumer", "unmapped-order") = ["declared-order-permutation"],
)]
enum OwnerUnmappedFamily {
    First,
    Second,
}

#[derive(tp_macros::RefusalFamily)]
#[refusal(
    crate = tp,
    family = "consumer.mapped-unpermitted-producer",
    shape = single_cause,
    order(First = "first", Second = "second")
)]
#[threadpak_mutations(
    support = mapped_unpermitted_support,
    module = generated_mapped_unpermitted_mutations,
    family = named("consumer", "mapped-unpermitted-evaluation"),
    map declared_order = named("consumer", "mapped-order"),
)]
enum MappedUnpermittedFamily {
    First,
    Second,
}

owner_unmapped_support! {
    harness: harness,
}

mapped_unpermitted_support! {
    harness: harness,
}

/// An owner-unmapped generated site stays in the complete reading and cannot enter the executable surface.
#[test]
fn owner_unmapped_discovery_is_retained_but_not_executable(
) -> Result<(), generated_owner_unmapped_mutations::MutationLoweringRefusal> {
    let lowering = generated_owner_unmapped_mutations::lowering()?;
    let [entry] = lowering.discovery().entries() else {
        panic!("the producer must retain its one discovered site");
    };
    assert_eq!(entry.disposition(), DiscoveryDisposition::OwnerUnmapped);
    assert!(lowering.surface().points().is_empty());
    let _ = OwnerUnmappedFamily::First;
    let _ = OwnerUnmappedFamily::Second;
    Ok(())
}

/// A mapped site without owner permission stays in the complete reading and cannot enter the executable surface.
#[test]
fn mapped_unpermitted_discovery_is_retained_but_not_executable(
) -> Result<(), generated_mapped_unpermitted_mutations::MutationLoweringRefusal> {
    let lowering = generated_mapped_unpermitted_mutations::lowering()?;
    let [entry] = lowering.discovery().entries() else {
        panic!("the producer must retain its one discovered site");
    };
    assert!(matches!(
        entry.disposition(),
        DiscoveryDisposition::MappedUnpermitted {
            cause: MappedUnpermittedCause::Claim(claim),
        } if claim.name().namespace().written() == "consumer"
            && claim.name().stem().written() == "mapped-order"
    ));
    assert!(lowering.surface().points().is_empty());
    let _ = MappedUnpermittedFamily::First;
    let _ = MappedUnpermittedFamily::Second;
    Ok(())
}
