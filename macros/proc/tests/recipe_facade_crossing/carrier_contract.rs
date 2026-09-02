//! The irreducible proc carrier's uniqueness and documentation posture.

#[test]
fn the_irreducible_proc_carrier_is_unique_and_hidden_from_generated_docs() {
    const PROC_SOURCE: &str = include_str!("../../src/lib.rs");
    const CARRIER: &str = "pub fn __macroonz_recipe_carrier";
    const HIDDEN_CARRIER: &str = "#[doc(hidden)]\n#[proc_macro]\npub fn __macroonz_recipe_carrier";

    assert_eq!(PROC_SOURCE.matches(CARRIER).count(), 1usize);
    assert_eq!(PROC_SOURCE.matches(HIDDEN_CARRIER).count(), 1usize);
}
