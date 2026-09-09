//! Independent physical-inventory corruption and write-failure controls.

use super::transaction::{declared, finish, observe, storage};
use macroonz::native_storage::{StorageError, StorageLimits, StorageTransaction};

#[test]
fn missing_and_extra_payloads_cannot_be_returned_as_a_complete_batch() -> Result<(), String> {
    observe(|root, path| {
        declared(|name, batch| {
            finish(storage(StorageTransaction::begin(root, name, batch))?)?;
            let missing = path.join("item-run/item-alpha");
            std::fs::remove_file(&missing).map_err(|error| error.to_string())?;
            let limits = StorageLimits {
                artifacts: 3,
                bytes: 20,
            };
            assert!(matches!(
                root.load(name, limits),
                Err(StorageError::InventoryMismatch)
            ));
            std::fs::write(&missing, b"first").map_err(|error| error.to_string())?;
            std::fs::write(path.join("item-run/item-extra"), b"extra")
                .map_err(|error| error.to_string())?;
            assert!(matches!(
                root.load(name, limits),
                Err(StorageError::InventoryMismatch)
            ));
            Ok(())
        })
    })
}

fn refused_marker(bytes: &[u8]) -> Result<(), String> {
    observe(|root, path| {
        declared(|name, batch| {
            finish(storage(StorageTransaction::begin(root, name, batch))?)?;
            std::fs::write(path.join("item-run/.committed"), bytes)
                .map_err(|error| error.to_string())?;
            assert!(matches!(
                root.load(name, batch.limits()),
                Err(StorageError::InvalidMarker)
            ));
            Ok(())
        })
    })
}

#[test]
fn publication_roster_refuses_wrong_version_order_duplicates_and_framing() -> Result<(), String> {
    for bytes in [
        b"macroonz-storage/2\nalpha\nzeta\n".as_slice(),
        b"macroonz-storage/1\nzeta\nalpha\n",
        b"macroonz-storage/1\nalpha\nalpha\n",
        b"macroonz-storage/1\nalpha\nzeta",
        b"macroonz-storage/1\n../escape\n",
        b"macroonz-storage/1\n",
        b"macroonz-storage/1\nalpha\r\nzeta\n",
        b"macroonz-storage/1\n\xff\n",
    ] {
        refused_marker(bytes)?;
    }
    Ok(())
}

#[test]
fn oversized_publication_roster_refuses_bounded_read() -> Result<(), String> {
    observe(|root, path| {
        declared(|name, batch| {
            finish(storage(StorageTransaction::begin(root, name, batch))?)?;
            let maximum = format!(
                "macroonz-storage/1\n{}\n{}\n",
                "a".repeat(96),
                "z".repeat(96)
            );
            let marker = path.join("item-run/.committed");
            std::fs::write(&marker, &maximum).map_err(|error| error.to_string())?;
            assert!(matches!(
                root.load(name, batch.limits()),
                Err(StorageError::InventoryMismatch)
            ));
            std::fs::write(&marker, format!("{maximum}!")).map_err(|error| error.to_string())?;
            assert!(matches!(
                root.load(name, batch.limits()),
                Err(StorageError::ByteBound)
            ));
            Ok(())
        })
    })
}

#[test]
fn payload_creation_failure_does_not_overwrite_or_publish() -> Result<(), String> {
    observe(|root, path| {
        declared(|name, batch| {
            let mut transaction = storage(StorageTransaction::begin(root, name, batch))?;
            let obstructed = path.join("item-run/item-zeta");
            std::fs::write(&obstructed, b"prior").map_err(|error| error.to_string())?;
            assert!(matches!(transaction.write_next(), Err(StorageError::Io(_))));
            assert!(matches!(
                transaction.commit(),
                Err(StorageError::Incomplete)
            ));
            assert_eq!(
                std::fs::read(&obstructed).map_err(|error| error.to_string())?,
                b"prior"
            );
            assert!(matches!(
                root.load(name, batch.limits()),
                Err(StorageError::Incomplete)
            ));
            finish(storage(StorageTransaction::recover(root, name, batch))?)
        })
    })
}

#[test]
fn payload_link_is_refused_during_read_and_recovery() -> Result<(), String> {
    observe(|root, path| {
        declared(|name, batch| {
            let mut transaction = storage(StorageTransaction::begin(root, name, batch))?;
            assert!(storage(transaction.write_next())?.is_some());
            drop(transaction);
            let outside = path
                .parent()
                .ok_or("root has no parent")?
                .join("outside-bytes");
            std::fs::write(&outside, b"external").map_err(|error| error.to_string())?;
            let link = path.join("item-run/item-alpha");
            #[cfg(unix)]
            std::os::unix::fs::symlink(&outside, &link).map_err(|error| error.to_string())?;
            #[cfg(windows)]
            std::os::windows::fs::symlink_file(&outside, &link)
                .map_err(|error| error.to_string())?;
            assert!(matches!(
                StorageTransaction::recover(root, name, batch),
                Err(StorageError::NotRegular)
            ));
            std::fs::write(
                path.join("item-run/.committed"),
                b"macroonz-storage/1\nalpha\nzeta\n",
            )
            .map_err(|error| error.to_string())?;
            assert!(matches!(
                root.load(name, batch.limits()),
                Err(StorageError::NotRegular)
            ));
            assert_eq!(
                std::fs::read(&outside).map_err(|error| error.to_string())?,
                b"external"
            );
            assert_eq!(
                std::fs::read(path.join("item-run/item-zeta")).map_err(|error| error.to_string())?,
                b"second"
            );
            Ok(())
        })
    })
}
