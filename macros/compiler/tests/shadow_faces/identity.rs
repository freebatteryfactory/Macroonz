use super::support::canonical_content;

/// Formatting is absent from canonical content, while the physical binding and authored row order remain members.
#[test]
fn canonical_content_moves_only_with_declared_meaning() -> Result<(), ()> {
    let compact = canonical_content("loom=loom,names=[Arc,Mutex]").ok_or(())?;
    let spaced = canonical_content("loom = loom, names = [Arc, Mutex]").ok_or(())?;
    let facade = canonical_content("loom = renamed_facade::loom, names = [Arc, Mutex]").ok_or(())?;
    let reversed = canonical_content("loom = loom, names = [Mutex, Arc]").ok_or(())?;

    assert_eq!(compact, spaced);
    assert_ne!(compact, facade);
    assert_ne!(compact, reversed);
    Ok(())
}
