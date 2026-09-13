use super::destination_fixture::snapshot;
use super::install_fixture::{destination, finish, installed, subject};
use super::stage_refusal::{link, unlink};
use crate::compiler::configure::root;
use macroonz::native_publication::DestinationError;
use std::path::Path;

fn file_link(destination: &Path, source: &Path) -> Result<(), String> {
    #[cfg(windows)]
    {
        std::os::windows::fs::symlink_file(destination, source).map_err(|error| error.to_string())
    }
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(destination, source).map_err(|error| error.to_string())
    }
}

#[test]
fn directory_and_file_links_refuse_checks_and_installation_without_touching_their_targets()
-> Result<(), String> {
    let source = root()?;
    let compiled = subject(&source, "subject", [7, 9, 42], "generated/other.rs")?;
    let outside = root()?;
    std::fs::write(outside.join("sentinel"), b"external authored bytes")
        .map_err(|error| error.to_string())?;
    let original = snapshot(&outside)?;
    for path in ["generated", ".macroonz-publication"] {
        let output = root()?;
        link(&outside, &output.join(path))?;
        let destination = destination(&output)?;
        assert!(matches!(
            destination.check(compiled.prepared()),
            Err(DestinationError::Entry(_))
        ));
        assert!(destination.begin(&compiled).is_err());
        assert_eq!(snapshot(&outside)?, original);
        unlink(&output.join(path))?;
    }
    for path in ["generated/alpha.rs", ".macroonz-publication/current"] {
        let output = root()?;
        let destination = destination(&output)?;
        finish(
            destination
                .begin(&compiled)
                .map_err(|error| error.to_string())?,
        )?;
        std::fs::remove_file(output.join(path)).map_err(|error| error.to_string())?;
        file_link(&outside.join("sentinel"), &output.join(path))?;
        assert!(matches!(
            destination.check(compiled.prepared()),
            Err(DestinationError::Entry(_))
        ));
        assert!(destination.begin(&compiled).is_err());
        assert_eq!(snapshot(&outside)?, original);
        std::fs::remove_file(output.join(path)).map_err(|error| error.to_string())?;
    }
    Ok(())
}

#[test]
fn redirected_journal_payload_or_temporary_entry_refuses_before_the_next_output_change()
-> Result<(), String> {
    let source = root()?;
    let compiled = subject(&source, "subject", [7, 9, 42], "generated/other.rs")?;
    let outside = root()?;
    std::fs::write(outside.join("sentinel"), b"external authored bytes")
        .map_err(|error| error.to_string())?;
    let original = snapshot(&outside)?;
    for path in [
        ".macroonz-publication/item-pending/item-intent",
        ".macroonz-publication/item-pending/.committed",
        ".macroonz-publication/replacement",
    ] {
        let output = root()?;
        let destination = destination(&output)?;
        let mut installation = destination
            .begin(&compiled)
            .map_err(|error| error.to_string())?;
        let visible = output.join(path);
        if visible.exists() {
            std::fs::rename(&visible, output.join("saved-entry"))
                .map_err(|error| error.to_string())?;
        }
        file_link(&outside.join("sentinel"), &visible)?;
        assert!(installation.write_next().is_err(), "{path}");
        drop(installation);
        assert!(!output.join("generated/alpha.rs").exists());
        assert_eq!(snapshot(&outside)?, original);
        std::fs::remove_file(&visible).map_err(|error| error.to_string())?;
    }
    Ok(())
}

#[test]
fn changed_directory_kinds_and_output_links_refuse_during_recovery() -> Result<(), String> {
    let source = root()?;
    let compiled = subject(&source, "subject", [7, 9, 42], "generated/other.rs")?;
    for path in [
        "generated",
        "generated/alpha.rs",
        ".macroonz-publication/current",
        ".macroonz-publication/item-pending/item-intent",
    ] {
        let output = root()?;
        let destination = destination(&output)?;
        drop(
            destination
                .begin(&compiled)
                .map_err(|error| error.to_string())?,
        );
        let visible = output.join(path);
        if visible.exists() {
            std::fs::rename(&visible, output.join("saved-entry"))
                .map_err(|error| error.to_string())?;
        }
        if path == "generated" {
            std::fs::write(&visible, b"authored file").map_err(|error| error.to_string())?;
        } else {
            std::fs::create_dir_all(&visible).map_err(|error| error.to_string())?;
        }
        let before = snapshot(&output)?;
        assert!(destination.recover().is_err(), "{path}");
        assert_eq!(snapshot(&output)?, before);
    }
    let output = root()?;
    let destination = destination(&output)?;
    drop(
        destination
            .begin(&compiled)
            .map_err(|error| error.to_string())?,
    );
    let outside = root()?;
    let before = snapshot(&outside)?;
    link(&outside, &output.join("generated"))?;
    assert!(destination.recover().is_err());
    assert_eq!(snapshot(&outside)?, before);
    unlink(&output.join("generated"))?;
    finish(
        destination
            .recover()
            .map_err(|error| error.to_string())?
            .ok_or("intent absent")?,
    )?;
    installed(&output, compiled.prepared())
}
