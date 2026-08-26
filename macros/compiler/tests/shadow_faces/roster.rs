use macroonz_compiler::descriptor::shadow::SHADOW_ROSTER;
use std::collections::BTreeSet;

/// The stated roster holds its own shape: distinct names, and every row's two paths rooted where the row claims.
#[test]
fn the_roster_is_distinct_and_rooted() {
    let mut seen = BTreeSet::new();
    for row in SHADOW_ROSTER {
        assert!(seen.insert(row.name()), "{} is stated twice", row.name());
        assert_eq!(row.std_path().first(), Some(&"std"));
        assert!(!row.shadow_path().is_empty());
        assert_eq!(
            row.std_path().last(),
            row.shadow_path().last(),
            "{} does not end on one name",
            row.name()
        );
    }
}
