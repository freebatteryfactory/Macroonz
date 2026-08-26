//! Actual proc-entry refusals through the token-span host.
//!
//! The callable compiler roads have their own observers.
//! This target crosses the proc host so token-span custody and invocation placement are observed at the public procedural entries.

/// Every hostile declaration must refuse through the actual proc crate at its observation's compiler span.
#[test]
fn every_proc_refusal_is_placed_where_its_observation_sits() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/compile_refusals/compile-fail/*.rs");
}
