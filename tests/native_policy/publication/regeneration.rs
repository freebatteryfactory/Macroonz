use super::install_fixture::{destination, finish, installed, subject};
use crate::compiler::configure::root;
use macroonz::native_publication::DestinationState;

#[test]
fn interrupted_regeneration_recovers_every_mixed_old_new_and_retired_file_boundary()
-> Result<(), String> {
    let source = root()?;
    let original = subject(&source, "original", [7, 9, 200], "generated/other.rs")?;
    let changed = subject(&source, "changed", [1, 2, 3], "generated/moved.rs")?;
    for completed in 0usize..=5 {
        let output = root()?;
        let destination = destination(&output)?;
        finish(
            destination
                .begin(&original)
                .map_err(|error| error.to_string())?,
        )?;
        let mut installation = destination
            .begin(&changed)
            .map_err(|error| error.to_string())?;
        for _step in 0usize..completed {
            assert!(
                installation
                    .write_next()
                    .map_err(|error| error.to_string())?
                    .is_some()
            );
        }
        drop(installation);
        assert_eq!(
            destination
                .check(changed.prepared())
                .map_err(|error| error.to_string())?
                .state(),
            DestinationState::Updating
        );
        drop(destination);
        let fresh = super::install_fixture::destination(&output)?;
        finish(
            fresh
                .recover()
                .map_err(|error| error.to_string())?
                .ok_or("intent absent")?,
        )?;
        installed(&output, changed.prepared())?;
        assert!(!output.join("generated/other.rs").exists());
    }
    Ok(())
}
