//! The complete public namespaced-reference roster through its one shared construction law.

use macroonz_harness::bench::{
    BenchTableName, ComplexityClaimRef, PlantedWorseRef, PreflightRef, WorkObservationRef,
    WorkloadRef,
};
use macroonz_harness::descriptor::{
    AuthoredTableName, CheckRef, ClaimRef, DoorRef, ExecutionSuite, MutationPointRef, NameRefusal,
    NamespacedName, PopulationRef, ProducerName, ProjectionRef, Role, SubjectRoute, Tag,
};
use macroonz_harness::generate::SemanticReducerId;
use macroonz_harness::muterprater::{ActivationSite, EvaluationFamilyRef};
use macroonz_harness::properties::SubstrateRef;

const OWNER: &str = "harness.reference.contract";

macro_rules! const_reference {
    ($name:ident: $reference:ty = $stem:literal) => {
        const $name: $reference = match <$reference>::named(OWNER, $stem) {
            Ok(reference) => reference,
            Err(_refusal) => panic!("the fixed reference must be valid"),
        };
    };
}

const_reference!(WORKLOAD: WorkloadRef = "workload");
const_reference!(PREFLIGHT: PreflightRef = "preflight");
const_reference!(PLANTED_WORSE: PlantedWorseRef = "planted-worse");
const_reference!(COMPLEXITY: ComplexityClaimRef = "complexity");
const_reference!(OBSERVATION: WorkObservationRef = "observation");
const_reference!(BENCH_TABLE: BenchTableName = "bench-table");

macro_rules! assert_reference {
    ($reference:ty, $stem:literal) => {{
        let name = NamespacedName::named(OWNER, $stem)?;
        let declared = <$reference>::named(OWNER, $stem)?;
        assert_eq!(declared, <$reference>::over(name));
        assert_eq!(declared.name(), name);
    }};
}

#[test]
fn every_namespaced_reference_keeps_the_shared_public_contract() -> Result<(), NameRefusal> {
    assert_reference!(AuthoredTableName, "authored-table");
    assert_reference!(CheckRef, "check");
    assert_reference!(ClaimRef, "claim");
    assert_reference!(DoorRef, "door");
    assert_reference!(ExecutionSuite, "execution-suite");
    assert_reference!(MutationPointRef, "mutation-point");
    assert_reference!(PopulationRef, "population");
    assert_reference!(ProducerName, "producer");
    assert_reference!(ProjectionRef, "projection");
    assert_reference!(Role, "role");
    assert_reference!(SubjectRoute, "subject-route");
    assert_reference!(Tag, "tag");
    assert_reference!(SemanticReducerId, "semantic-reducer");
    assert_reference!(EvaluationFamilyRef, "evaluation-family");
    assert_reference!(ActivationSite, "activation-site");
    assert_reference!(SubstrateRef, "substrate");
    assert_reference!(WorkloadRef, "workload");
    assert_reference!(PreflightRef, "preflight");
    assert_reference!(PlantedWorseRef, "planted-worse");
    assert_reference!(ComplexityClaimRef, "complexity");
    assert_reference!(WorkObservationRef, "observation");
    assert_reference!(BenchTableName, "bench-table");

    assert_eq!(WORKLOAD.name().stem().written(), "workload");
    assert_eq!(PREFLIGHT.name().stem().written(), "preflight");
    assert_eq!(PLANTED_WORSE.name().stem().written(), "planted-worse");
    assert_eq!(COMPLEXITY.name().stem().written(), "complexity");
    assert_eq!(OBSERVATION.name().stem().written(), "observation");
    assert_eq!(BENCH_TABLE.name().stem().written(), "bench-table");
    Ok(())
}

#[test]
fn every_reference_keeps_name_refusal_precedence() {
    assert_eq!(
        SemanticReducerId::named("", ""),
        Err(NameRefusal::EmptyNamespace)
    );
    assert_eq!(
        BenchTableName::named(OWNER, ""),
        Err(NameRefusal::EmptyStem)
    );
}
