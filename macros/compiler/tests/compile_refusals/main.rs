//! The compile-refusal driver: common fixtures and the selected feature posture must refuse with the message recorded beside each fixture.
//!
//! A lane in `tests/` observes behavior; a fixture here observes what no behavior can reach.
//! Each one names a seat that is not a caller's to write, and the recorded `.stderr` is the evidence that the refusal is the compiler's own rather than a convention this crate follows.

/// Common boundaries and the active host posture refuse with their recorded messages.
#[test]
fn every_compile_fail_fixture_refuses() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/compile_refusals/compile-fail/*.rs");
    #[cfg(feature = "host")]
    cases.compile_fail(
        "tests/compile_refusals/host/host-emission-without-span-custody-is-the-old-contract.rs",
    );
    #[cfg(not(feature = "host"))]
    cases.compile_fail("tests/compile_refusals/no-host/host-is-unavailable-without-its-feature.rs");
    drop(cases);
    #[cfg(feature = "host")]
    trybuild::TestCases::new()
        .pass("tests/compile_refusals/no-host/host-is-unavailable-without-its-feature.rs");
}
