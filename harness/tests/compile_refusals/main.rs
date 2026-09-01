//! The compile-refusal fixtures: one per reversal a compiler refusal proves.
//!
//! trybuild covers compile refusals only.
//! It is one challenge kind, never the universal one, and each fixture in this lane's `compile-fail/` states exactly which reversal it proves.

mod compile_contracts;
mod swap_pairs;

/// Every declared compile-fail fixture refuses to compile, with the message it is recorded as producing.
#[test]
fn declared_compile_refusals_do_not_compile() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/compile_refusals/compile-fail/*.rs");
}
