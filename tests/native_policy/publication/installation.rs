use super::destination_fixture::snapshot;
use super::install_fixture::{destination, finish, installed, subject};
use crate::compiler::configure::root;
use macroonz::native_publication::{DestinationError, DestinationState};
use macroonz::native_storage::StorageError;

#[test]
fn actual_installation_regenerates_replaces_retires_repairs_and_relocates() -> Result<(), String> {
    let source = root()?;
    let original = subject(&source, "original", [7, 9, 200], "generated/other.rs")?;
    let changed = subject(&source, "changed", [1, 2, 3], "generated/moved.rs")?;
    let output = root()?;
    std::fs::write(output.join("authored.rs"), b"authored neighbor")
        .map_err(|error| error.to_string())?;
    let destination = destination(&output)?;
    for _repeat in 0usize..2 {
        finish(
            destination
                .begin(&original)
                .map_err(|error| error.to_string())?,
        )?;
        installed(&output, original.prepared())?;
    }
    finish(
        destination
            .begin(&changed)
            .map_err(|error| error.to_string())?,
    )?;
    installed(&output, changed.prepared())?;
    assert!(!output.join("generated/other.rs").exists());
    std::fs::remove_file(output.join("generated/beta.rs")).map_err(|error| error.to_string())?;
    finish(
        destination
            .begin(&changed)
            .map_err(|error| error.to_string())?,
    )?;
    installed(&output, changed.prepared())?;
    assert_eq!(
        std::fs::read(output.join("authored.rs")).map_err(|error| error.to_string())?,
        b"authored neighbor"
    );
    drop(destination);
    let moved = root()?.join("relocated");
    std::fs::rename(&output, &moved).map_err(|error| error.to_string())?;
    installed(&moved, changed.prepared())
}

#[test]
fn complete_preflight_refuses_last_path_collision_and_preserves_tampering() -> Result<(), String> {
    let source = root()?;
    let original = subject(&source, "original", [7, 9, 200], "generated/other.rs")?;
    let changed = subject(&source, "changed", [1, 2, 3], "generated/other.rs")?;
    let output = root()?;
    std::fs::create_dir(output.join("generated")).map_err(|error| error.to_string())?;
    std::fs::write(output.join("generated/other.rs"), b"authored collision")
        .map_err(|error| error.to_string())?;
    let destination = destination(&output)?;
    assert!(
        matches!(destination.begin(&original), Err(DestinationError::Conflict(path)) if path == "generated/other.rs")
    );
    assert!(!output.join("generated/alpha.rs").exists());
    assert!(!output.join(".macroonz-publication").exists());
    assert_eq!(
        std::fs::read(output.join("generated/other.rs")).map_err(|error| error.to_string())?,
        b"authored collision"
    );
    std::fs::remove_file(output.join("generated/other.rs")).map_err(|error| error.to_string())?;
    finish(
        destination
            .begin(&original)
            .map_err(|error| error.to_string())?,
    )?;
    std::fs::write(output.join("generated/other.rs"), b"authored amendment")
        .map_err(|error| error.to_string())?;
    let before = snapshot(&output)?;
    assert!(
        matches!(destination.begin(&changed), Err(DestinationError::Conflict(path)) if path == "generated/other.rs")
    );
    assert_eq!(snapshot(&output)?, before);
    Ok(())
}

#[test]
fn every_installation_step_and_premature_commit_recovers_with_fresh_custody() -> Result<(), String>
{
    let source = root()?;
    let compiled = subject(&source, "subject", [7, 9, 42], "generated/other.rs")?;
    for completed in 0usize..=4 {
        let output = root()?;
        let writer = destination(&output)?;
        let mut installation = writer.begin(&compiled).map_err(|error| error.to_string())?;
        assert!(matches!(
            destination(&output)?.check(compiled.prepared()),
            Err(DestinationError::Storage(StorageError::Busy))
        ));
        for _step in 0usize..completed {
            assert!(
                installation
                    .write_next()
                    .map_err(|error| error.to_string())?
                    .is_some()
            );
        }
        if completed == 0 {
            assert!(matches!(
                installation.commit(),
                Err(DestinationError::Incomplete)
            ));
        } else {
            drop(installation);
        }
        drop(writer);
        let fresh = destination(&output)?;
        assert_eq!(
            fresh
                .check(compiled.prepared())
                .map_err(|error| error.to_string())?
                .state(),
            DestinationState::Preparing
        );
        assert!(fresh.begin(&compiled).is_err());
        finish(
            fresh
                .recover()
                .map_err(|error| error.to_string())?
                .ok_or("recovery absent")?,
        )?;
        installed(&output, compiled.prepared())?;
        assert!(
            fresh
                .recover()
                .map_err(|error| error.to_string())?
                .is_none()
        );
    }
    Ok(())
}

#[test]
fn leftover_temporary_hard_link_cannot_rewrite_an_already_installed_file() -> Result<(), String> {
    let source = root()?;
    let compiled = subject(&source, "subject", [7, 9, 42], "generated/other.rs")?;
    let output = root()?;
    let writer = destination(&output)?;
    let mut installation = writer.begin(&compiled).map_err(|error| error.to_string())?;
    let first = installation
        .write_next()
        .map_err(|error| error.to_string())?
        .ok_or("no first file")?
        .spelling()
        .to_owned();
    let before = std::fs::read(output.join(&first)).map_err(|error| error.to_string())?;
    assert_eq!(
        std::fs::read(output.join(".macroonz-publication/replacement"))
            .map_err(|error| error.to_string())?,
        before
    );
    std::fs::write(
        output.join(".macroonz-publication/incoming"),
        b"partial incoming bytes",
    )
    .map_err(|error| error.to_string())?;
    drop(installation);
    finish(
        writer
            .recover()
            .map_err(|error| error.to_string())?
            .ok_or("recovery absent")?,
    )?;
    assert_eq!(
        std::fs::read(output.join(first)).map_err(|error| error.to_string())?,
        before
    );
    installed(&output, compiled.prepared())
}
