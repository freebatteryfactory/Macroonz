use super::destination_fixture::snapshot;
use super::stage_fixture::{DRIVER, plan};
use crate::compiler::configure::root;
use macroonz::native_publication::StagingError;

#[test]
fn cached_sources_are_reused_only_for_the_complete_identical_declaration() -> Result<(), String> {
    let work = root()?;
    let first = plan(DRIVER)?
        .cached(&work)
        .map_err(|error| error.to_string())?;
    let before = snapshot(&work)?;
    let identical = plan(DRIVER)?
        .cached(&work)
        .map_err(|error| error.to_string())?;
    assert_eq!(first.path(), identical.path());
    assert_eq!(snapshot(&work)?, before);
    let changed = plan(&format!("{DRIVER}\n"))?
        .cached(&work)
        .map_err(|error| error.to_string())?;
    assert_ne!(first.path(), changed.path());
    assert_eq!(
        std::fs::read(first.path().join("fixture.rs")).map_err(|error| error.to_string())?,
        DRIVER.as_bytes()
    );
    Ok(())
}

#[test]
fn damaged_cache_refuses_without_repairing_or_overwriting_material() -> Result<(), String> {
    for change in ["changed", "missing", "extra"] {
        let work = root()?;
        let staged = plan(DRIVER)?
            .cached(&work)
            .map_err(|error| error.to_string())?;
        let file = staged.path().join("fixture.rs");
        match change {
            "changed" => std::fs::write(&file, b"caller changed source"),
            "missing" => std::fs::remove_file(&file),
            _ => std::fs::write(staged.path().join("extra.rs"), b"extra"),
        }
        .map_err(|error| error.to_string())?;
        let before = snapshot(&work)?;
        assert!(matches!(
            plan(DRIVER)?.cached(&work),
            Err(StagingError::Source(_) | StagingError::Storage(_))
        ));
        assert_eq!(snapshot(&work)?, before);
    }
    Ok(())
}

#[test]
fn a_link_at_the_cache_name_cannot_supply_cached_source() -> Result<(), String> {
    let work = root()?;
    let staged = plan(DRIVER)?
        .cached(&work)
        .map_err(|error| error.to_string())?;
    let visible = staged.path().to_path_buf();
    drop(staged);
    let moved = work.join("retained-source");
    std::fs::rename(&visible, &moved).map_err(|error| error.to_string())?;
    let before = snapshot(&moved)?;
    super::stage_refusal::link(&moved, &visible)?;
    assert!(matches!(
        plan(DRIVER)?.cached(&work),
        Err(StagingError::Source(_))
    ));
    assert_eq!(snapshot(&moved)?, before);
    super::stage_refusal::unlink(&visible)
}
