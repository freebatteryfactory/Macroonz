//! Public batch custody observed independently of its filesystem implementation.

use macroonz::native_storage::{
    StorageArtifact, StorageBatch, StorageError, StorageLimits, StorageName, StorageRoot,
    StorageTransaction,
};
use std::io::{BufRead, Write};
use std::path::Path;
use std::process::{Command, Stdio};

const LIMITS: StorageLimits = StorageLimits {
    artifacts: 2,
    bytes: 11,
};

pub(super) fn storage<T>(result: Result<T, StorageError>) -> Result<T, String> {
    result.map_err(|error| format!("{error:?}"))
}

pub(super) fn observe(
    action: impl FnOnce(&StorageRoot, &Path) -> Result<(), String>,
) -> Result<(), String> {
    let scratch = super::check::scratch()?;
    let path = scratch.join("root");
    std::fs::create_dir(&path).map_err(|error| error.to_string())?;
    let root = storage(StorageRoot::open(&path))?;
    let result = action(&root, &path);
    drop(root);
    let cleanup = std::fs::remove_dir_all(&scratch).map_err(|error| error.to_string());
    result.and(cleanup)
}

pub(super) fn declared(
    action: impl FnOnce(&StorageName, StorageBatch<'_>) -> Result<(), String>,
) -> Result<(), String> {
    let name = storage(StorageName::informed("run"))?;
    let zeta = storage(StorageName::informed("zeta"))?;
    let alpha = storage(StorageName::informed("alpha"))?;
    let artifacts = [
        StorageArtifact {
            name: &zeta,
            bytes: b"second",
        },
        StorageArtifact {
            name: &alpha,
            bytes: b"first",
        },
    ];
    action(&name, storage(StorageBatch::informed(&artifacts, LIMITS))?)
}

pub(super) fn finish(mut transaction: StorageTransaction<'_>) -> Result<(), String> {
    while storage(transaction.write_next())?.is_some() {}
    storage(transaction.commit())
}

fn advance(transaction: &mut StorageTransaction<'_>, count: usize) -> Result<(), String> {
    for _position in 0usize..count {
        assert!(storage(transaction.write_next())?.is_some());
    }
    Ok(())
}

fn await_ready(reader: impl BufRead) -> Result<bool, String> {
    for line in reader.lines().take(16) {
        if line
            .map_err(|error| error.to_string())?
            .contains("storage-ready")
        {
            return Ok(true);
        }
    }
    Ok(false)
}

fn retained(root: &StorageRoot, name: &StorageName) -> Result<(), String> {
    let loaded = storage(root.load(name, LIMITS))?;
    let observed = loaded
        .iter()
        .map(|artifact| (artifact.name.spelling(), artifact.bytes.as_slice()))
        .collect::<Vec<_>>();
    assert_eq!(
        observed,
        [
            ("alpha", b"first".as_slice()),
            ("zeta", b"second".as_slice())
        ]
    );
    Ok(())
}

#[test]
fn names_and_batches_refuse_before_storage_effects() -> Result<(), String> {
    for name in [
        "",
        "../escape",
        "a/b",
        "a\\b",
        "/root",
        "C:drive",
        "X",
        "a.",
        "a ",
        "\0",
        "é",
    ] {
        assert!(matches!(
            StorageName::informed(name),
            Err(StorageError::InvalidName)
        ));
    }
    assert!(StorageName::informed(&"a".repeat(96)).is_ok());
    assert!(StorageName::informed(&"a".repeat(97)).is_err());
    let name = storage(StorageName::informed("con"))?;
    let artifact = StorageArtifact {
        name: &name,
        bytes: b"bytes",
    };
    assert!(matches!(
        StorageBatch::informed(&[], LIMITS),
        Err(StorageError::EmptyBatch)
    ));
    assert!(matches!(
        StorageBatch::informed(&[artifact, artifact], LIMITS),
        Err(StorageError::DuplicateName)
    ));
    assert!(matches!(
        StorageBatch::informed(
            &[artifact],
            StorageLimits {
                artifacts: 0,
                bytes: 5
            }
        ),
        Err(StorageError::ArtifactBound)
    ));
    assert!(matches!(
        StorageBatch::informed(
            &[artifact],
            StorageLimits {
                artifacts: 1,
                bytes: 4
            }
        ),
        Err(StorageError::ByteBound)
    ));
    Ok(())
}

#[test]
fn publication_retains_all_bytes_and_refuses_overwrite_or_undersized_reads() -> Result<(), String> {
    observe(|root, path| {
        declared(|name, batch| {
            finish(storage(StorageTransaction::begin(root, name, batch))?)?;
            retained(root, name)?;
            assert!(matches!(
                StorageTransaction::begin(root, name, batch),
                Err(StorageError::Collision)
            ));
            assert!(matches!(
                StorageTransaction::recover(root, name, batch),
                Err(StorageError::Collision)
            ));
            assert!(matches!(
                root.load(
                    name,
                    StorageLimits {
                        artifacts: 1,
                        bytes: 11
                    }
                ),
                Err(StorageError::ArtifactBound)
            ));
            assert!(matches!(
                root.load(
                    name,
                    StorageLimits {
                        artifacts: 2,
                        bytes: 10
                    }
                ),
                Err(StorageError::ByteBound)
            ));
            let reopened = storage(StorageRoot::open(path))?;
            retained(&reopened, name)
        })
    })
}

#[test]
fn every_unpublished_payload_boundary_can_be_recovered() -> Result<(), String> {
    for completed in 0usize..=2 {
        observe(|root, _path| {
            declared(|name, batch| {
                let mut transaction = storage(StorageTransaction::begin(root, name, batch))?;
                advance(&mut transaction, completed)?;
                assert!(matches!(root.load(name, LIMITS), Err(StorageError::Busy)));
                drop(transaction);
                assert!(matches!(
                    root.load(name, LIMITS),
                    Err(StorageError::Incomplete)
                ));
                finish(storage(StorageTransaction::recover(root, name, batch))?)?;
                retained(root, name)
            })
        })?;
    }
    Ok(())
}

#[test]
fn premature_commit_and_partial_payload_remain_unpublished() -> Result<(), String> {
    observe(|root, path| {
        declared(|name, batch| {
            let transaction = storage(StorageTransaction::begin(root, name, batch))?;
            assert!(matches!(
                transaction.commit(),
                Err(StorageError::Incomplete)
            ));
            std::fs::write(path.join("item-run/item-zeta"), b"sec")
                .map_err(|error| error.to_string())?;
            std::fs::write(path.join("item-run/.prepared"), b"macroonz-stor")
                .map_err(|error| error.to_string())?;
            assert!(matches!(
                root.load(name, LIMITS),
                Err(StorageError::Incomplete)
            ));
            finish(storage(StorageTransaction::recover(root, name, batch))?)?;
            retained(root, name)
        })
    })
}

#[test]
fn undeclared_recovery_entries_refuse_before_any_payload_removal() -> Result<(), String> {
    observe(|root, path| {
        declared(|name, batch| {
            let mut transaction = storage(StorageTransaction::begin(root, name, batch))?;
            assert!(storage(transaction.write_next())?.is_some());
            drop(transaction);
            let unknown = path.join("item-run/item-unexpected");
            std::fs::write(&unknown, b"authored").map_err(|error| error.to_string())?;
            assert!(matches!(
                StorageTransaction::recover(root, name, batch),
                Err(StorageError::UnexpectedEntry)
            ));
            assert_eq!(
                std::fs::read(&unknown).map_err(|error| error.to_string())?,
                b"authored"
            );
            assert_eq!(
                std::fs::read(path.join("item-run/item-zeta")).map_err(|error| error.to_string())?,
                b"second"
            );
            Ok(())
        })
    })
}

#[test]
fn redirected_batch_cannot_read_write_or_recover_outside_the_root() -> Result<(), String> {
    observe(|root, path| {
        declared(|name, batch| {
            let outside = path.parent().ok_or("root has no parent")?.join("outside");
            std::fs::create_dir(&outside).map_err(|error| error.to_string())?;
            std::fs::write(outside.join("item-zeta"), b"external")
                .map_err(|error| error.to_string())?;
            std::fs::write(outside.join(".committed"), b"").map_err(|error| error.to_string())?;
            #[cfg(unix)]
            std::os::unix::fs::symlink(&outside, path.join("item-run"))
                .map_err(|error| error.to_string())?;
            #[cfg(windows)]
            std::os::windows::fs::symlink_dir(&outside, path.join("item-run"))
                .map_err(|error| error.to_string())?;
            assert!(root.load(name, LIMITS).is_err());
            assert!(StorageTransaction::begin(root, name, batch).is_err());
            assert!(StorageTransaction::recover(root, name, batch).is_err());
            assert_eq!(
                std::fs::read(outside.join("item-zeta")).map_err(|error| error.to_string())?,
                b"external"
            );
            Ok(())
        })
    })
}

#[test]
fn non_file_payloads_and_malformed_publication_markers_refuse() -> Result<(), String> {
    observe(|root, path| {
        declared(|name, batch| {
            finish(storage(StorageTransaction::begin(root, name, batch))?)?;
            let payload = path.join("item-run/item-alpha");
            std::fs::remove_file(&payload).map_err(|error| error.to_string())?;
            std::fs::create_dir(&payload).map_err(|error| error.to_string())?;
            assert!(matches!(
                root.load(name, LIMITS),
                Err(StorageError::NotRegular)
            ));
            std::fs::remove_dir(&payload).map_err(|error| error.to_string())?;
            std::fs::write(&payload, b"first").map_err(|error| error.to_string())?;
            std::fs::write(path.join("item-run/.committed"), b"forged")
                .map_err(|error| error.to_string())?;
            assert!(matches!(
                root.load(name, LIMITS),
                Err(StorageError::InvalidMarker)
            ));
            Ok(())
        })
    })
}

#[test]
#[ignore = "Invoked as a real child by the interruption control, with the declared root supplied on stdin."]
fn interrupted_writer() -> Result<(), String> {
    let mut input = std::io::stdin().lock();
    let mut line = String::new();
    input
        .read_line(&mut line)
        .map_err(|error| error.to_string())?;
    let root = storage(StorageRoot::open(Path::new(line.trim_end())))?;
    declared(|name, batch| {
        let mut transaction = storage(StorageTransaction::begin(&root, name, batch))?;
        assert!(storage(transaction.write_next())?.is_some());
        let mut output = std::io::stdout().lock();
        writeln!(output, "storage-ready").map_err(|error| error.to_string())?;
        output.flush().map_err(|error| error.to_string())?;
        line.clear();
        input
            .read_line(&mut line)
            .map_err(|error| error.to_string())?;
        drop(transaction);
        Err("the parent must terminate this writer before normal return".to_owned())
    })
}

#[test]
fn killed_writer_releases_custody_and_requires_explicit_recovery() -> Result<(), String> {
    observe(|root, path| {
        declared(|name, batch| {
            let executable = std::env::current_exe().map_err(|error| error.to_string())?;
            let mut child = Command::new(executable)
                .args([
                    "--exact",
                    "transaction::interrupted_writer",
                    "--ignored",
                    "--nocapture",
                    "--test-threads=1",
                ])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::inherit())
                .spawn()
                .map_err(|error| error.to_string())?;
            let mut input = child.stdin.take().ok_or("child stdin missing")?;
            writeln!(input, "{}", path.display()).map_err(|error| error.to_string())?;
            input.flush().map_err(|error| error.to_string())?;
            let output = child.stdout.take().ok_or("child stdout missing")?;
            let reader = std::io::BufReader::new(output);
            let ready = await_ready(reader)?;
            let held = root.load(name, LIMITS);
            child.kill().map_err(|error| error.to_string())?;
            let status = child.wait().map_err(|error| error.to_string())?;
            drop(input);
            assert!(ready);
            assert!(!status.success());
            assert!(matches!(held, Err(StorageError::Busy)));
            assert!(matches!(
                root.load(name, LIMITS),
                Err(StorageError::Incomplete)
            ));
            finish(storage(StorageTransaction::recover(root, name, batch))?)?;
            retained(root, name)
        })
    })
}
