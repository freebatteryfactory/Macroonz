//! Independent filesystem crossings for the selected native dependency.

use cap_std::fs::{Dir, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

fn observe(action: impl FnOnce(&Dir, &Path) -> Result<(), String>) -> Result<(), String> {
    let scratch = super::check::scratch()?;
    let root = scratch.join("root");
    std::fs::create_dir(&root).map_err(|error| error.to_string())?;
    let result = {
        let directory = Dir::open_ambient_dir(&root, cap_std::ambient_authority())
            .map_err(|error| error.to_string())?;
        action(&directory, &scratch)
    };
    let cleanup = std::fs::remove_dir_all(&scratch).map_err(|error| error.to_string());
    result.and(cleanup)
}

fn create(directory: &Dir, name: &str, bytes: &[u8]) -> Result<(), String> {
    let mut file = directory
        .open_with(name, OpenOptions::new().write(true).create_new(true))
        .map_err(|error| error.to_string())?;
    file.write_all(bytes).map_err(|error| error.to_string())?;
    file.sync_all().map_err(|error| error.to_string())
}

#[test]
fn declared_directory_reads_exact_bytes_and_refuses_escape() -> Result<(), String> {
    observe(|directory, scratch| {
        let outside = scratch.join("outside");
        std::fs::write(&outside, b"outside witness").map_err(|error| error.to_string())?;
        create(directory, "inside", b"inside witness")?;
        assert_eq!(
            directory
                .read("inside")
                .map_err(|error| error.to_string())?,
            b"inside witness"
        );
        assert!(directory.open("../outside").is_err());
        assert!(directory.open(&outside).is_err());
        assert!(directory.create("../escaped-write").is_err());
        assert!(!scratch.join("escaped-write").exists());
        assert_eq!(
            std::fs::read(&outside).map_err(|error| error.to_string())?,
            b"outside witness"
        );
        Ok(())
    })
}

#[test]
fn creation_and_link_publication_refuse_overwrite() -> Result<(), String> {
    observe(|directory, _scratch| {
        create(directory, "prepared", b"complete bytes")?;
        create(directory, "different", b"other bytes")?;
        directory
            .hard_link("prepared", directory, "published")
            .map_err(|error| error.to_string())?;
        let collision =
            directory.open_with("published", OpenOptions::new().write(true).create_new(true));
        assert!(
            matches!(collision, Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists)
        );
        assert!(
            directory
                .hard_link("different", directory, "published")
                .is_err()
        );
        directory
            .remove_file("prepared")
            .map_err(|error| error.to_string())?;
        assert_eq!(
            directory
                .read("published")
                .map_err(|error| error.to_string())?,
            b"complete bytes"
        );
        Ok(())
    })
}

#[test]
fn opened_files_support_bounded_reads_and_exclusive_lock_custody() -> Result<(), String> {
    observe(|directory, _scratch| {
        create(directory, "bounded", b"five!")?;
        let mut reader = directory
            .open("bounded")
            .map_err(|error| error.to_string())?;
        let mut prefix = Vec::new();
        Read::by_ref(&mut reader)
            .take(4)
            .read_to_end(&mut prefix)
            .map_err(|error| error.to_string())?;
        assert_eq!(prefix, b"five");
        let mut sentinel = [0u8; 1];
        assert_eq!(
            reader
                .read(&mut sentinel)
                .map_err(|error| error.to_string())?,
            1
        );
        assert_eq!(sentinel, *b"!");
        drop(reader);
        create(directory, "custody", b"")?;
        let first = directory
            .open_with("custody", OpenOptions::new().read(true).write(true))
            .map_err(|error| error.to_string())?
            .into_std();
        let second = directory
            .open_with("custody", OpenOptions::new().read(true).write(true))
            .map_err(|error| error.to_string())?
            .into_std();
        first.try_lock().map_err(|error| error.to_string())?;
        assert!(matches!(
            second.try_lock(),
            Err(std::fs::TryLockError::WouldBlock)
        ));
        drop(first);
        second.try_lock().map_err(|error| error.to_string())?;
        Ok(())
    })
}

#[test]
fn an_existing_link_cannot_redirect_a_read_or_write_outside_the_root() -> Result<(), String> {
    observe(|directory, scratch| {
        let outside = scratch.join("outside");
        std::fs::create_dir(&outside).map_err(|error| error.to_string())?;
        std::fs::write(outside.join("witness"), b"external bytes")
            .map_err(|error| error.to_string())?;
        let link = scratch.join("root/redirect");
        #[cfg(unix)]
        std::os::unix::fs::symlink(&outside, &link).map_err(|error| error.to_string())?;
        #[cfg(windows)]
        std::os::windows::fs::symlink_dir(&outside, &link).map_err(|error| {
            format!("host cannot create the required link-escape control: {error}")
        })?;
        assert!(directory.open("redirect/witness").is_err());
        assert!(directory.create("redirect/witness").is_err());
        assert_eq!(
            std::fs::read(outside.join("witness")).map_err(|error| error.to_string())?,
            b"external bytes"
        );
        Ok(())
    })
}
