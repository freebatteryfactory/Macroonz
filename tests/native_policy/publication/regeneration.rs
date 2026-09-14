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
        let destination = destination(&output)
            .map_err(|error| format!("boundary {completed}, open original: {error}"))?;
        finish(
            destination
                .begin(&original)
                .map_err(|error| format!("boundary {completed}, begin original: {error}"))?,
        )
        .map_err(|error| format!("boundary {completed}, finish original: {error}"))?;
        let mut installation = destination
            .begin(&changed)
            .map_err(|error| format!("boundary {completed}, begin changed: {error}"))?;
        for step in 0usize..completed {
            assert!(
                installation
                    .write_next()
                    .map_err(|error| {
                        format!("boundary {completed}, changed write {step}: {error}")
                    })?
                    .is_some()
            );
        }
        drop(installation);
        assert_eq!(
            destination
                .check(changed.prepared())
                .map_err(|error| format!("boundary {completed}, check pending: {error}"))?
                .state(),
            DestinationState::Updating
        );
        drop(destination);
        let fresh = super::install_fixture::destination(&output)
            .map_err(|error| format!("boundary {completed}, open recovery: {error}"))?;
        finish(
            fresh
                .recover()
                .map_err(|error| format!("boundary {completed}, recover: {error}"))?
                .ok_or("intent absent")?,
        )
        .map_err(|error| format!("boundary {completed}, finish recovery: {error}"))?;
        installed(&output, changed.prepared())
            .map_err(|error| format!("boundary {completed}, inspect installed: {error}"))?;
        assert!(!output.join("generated/other.rs").exists());
    }
    Ok(())
}
