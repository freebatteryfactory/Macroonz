//! Values only the harness can judge remain typed harness refusals through generated functions.

use mh::descriptor::NameRefusal;
use mh::interleave::ExplorationBoundRefusal;

/// Bound admission stays owned by the harness and travels through the generated fault sum unchanged.
#[test]
fn harness_bound_refusals_cross_without_compiler_policy() -> Result<(), ()> {
    let strands = super::support::strands().ok_or(())?;
    let contract = super::support::contract().ok_or(())?;
    let interleavings = crate::refused_bounds::no_interleavings(&strands, &contract)
        .err()
        .ok_or(())?;
    assert!(matches!(
        interleavings,
        crate::refused_bounds::Fault::Bound(ExplorationBoundRefusal::ZeroInterleavings)
    ));
    let samples = crate::refused_bounds::no_samples(&strands, &contract)
        .err()
        .ok_or(())?;
    assert!(matches!(
        samples,
        crate::refused_bounds::Fault::Bound(ExplorationBoundRefusal::ZeroSamples)
    ));
    Ok(())
}

/// Name admission stays owned by the harness for both halves of the generated population reference.
#[test]
fn harness_name_refusals_cross_without_compiler_policy() -> Result<(), ()> {
    let strands = super::support::strands().ok_or(())?;
    let contract = super::support::contract().ok_or(())?;
    let namespace = crate::refused_namespace::row(&strands, &contract)
        .err()
        .ok_or(())?;
    assert!(matches!(
        namespace,
        crate::refused_namespace::Fault::Name(NameRefusal::EmptyNamespace)
    ));
    let population = crate::refused_population::row(&strands, &contract)
        .err()
        .ok_or(())?;
    assert!(matches!(
        population,
        crate::refused_population::Fault::Name(NameRefusal::EmptyStem)
    ));
    Ok(())
}
