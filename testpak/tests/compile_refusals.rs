//! The compile-refusal fixtures: one per owed red twin.
//!
//! trybuild covers compile refusals only. It is one challenge kind, never the
//! universal one, and the fixtures below say exactly which reversal each of
//! them proves.

/// Every declared compile-fail fixture refuses to compile, with the message it
/// is recorded as producing.
#[test]
fn declared_compile_refusals_do_not_compile() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/compile-fail/*.rs");
}
