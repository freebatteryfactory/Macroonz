//! The compile-refusal driver: every fixture in this lane's `compile-fail/` must fail to compile, with the message recorded beside it.
//!
//! A lane in `tests/` observes behavior; a fixture here observes what no behavior can reach.
//! Each one names a seat that is not a caller's to write, and the recorded `.stderr` is the evidence that the refusal is the compiler's own rather than a convention this crate follows.

/// Every fixture beside this file refuses to compile, with its recorded message.
#[test]
fn every_compile_fail_fixture_refuses() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/compile_refusals/compile-fail/*.rs");
}
