//! Outside compilation controls for storage construction and consuming custody.

use std::path::Path;

#[test]
fn storage_batches_require_admission_and_transaction_commit_consumes_custody() -> Result<(), String>
{
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let scratch = super::check::scratch()?;
    let subject = scratch.join("storage-ownership");
    std::fs::create_dir(&subject).map_err(|error| error.to_string())?;
    std::fs::write(
        subject.join("Cargo.toml"),
        super::consumer::manifest(root, "\"native-tooling\"")?,
    )
    .map_err(|error| error.to_string())?;
    std::fs::copy(root.join("Cargo.lock"), subject.join("Cargo.lock"))
        .map_err(|error| error.to_string())?;
    std::fs::write(subject.join("main.rs"), "fn main() {}\n").map_err(|error| error.to_string())?;
    let locked = super::check::cargo(
        &subject,
        root,
        &scratch,
        "storage-lock",
        &["update", "--workspace", "--offline"],
    )?;
    if !locked.status.success() {
        return Err(String::from_utf8_lossy(&locked.stderr).into_owned());
    }
    for (name, source, expected) in [
        (
            "admitted",
            "use macroonz::native_storage::*; fn once(transaction: StorageTransaction<'_>) { drop(transaction.commit()); } fn main() { let _consume = once; let _name = StorageName::informed(\"run\"); let _batch = StorageBatch::informed(&[], StorageLimits { artifacts: 0, bytes: 0 }); }",
            None,
        ),
        (
            "forged-batch",
            "use macroonz::native_storage::*; fn main() { let _batch = StorageBatch { artifacts: &[], limits: StorageLimits { artifacts: 0, bytes: 0 } }; }",
            Some("E0451"),
        ),
        (
            "forged-name",
            "use macroonz::native_storage::*; fn main() { let _name = StorageName(\"../escape\".to_owned()); }",
            Some("E0423"),
        ),
        (
            "reused-transaction",
            "use macroonz::native_storage::*; fn twice(transaction: StorageTransaction<'_>) { drop(transaction.commit()); drop(transaction.commit()); } fn main() { let _consume = twice; }",
            Some("E0382"),
        ),
        (
            "forged-input-run",
            "fn forged(report: macroonz::harness::report::RunReport, input: macroonz::harness::input::InputEnvelope) { let _run = macroonz::workflow::InputRun { report, input }; } fn main() { let _forge = forged; }",
            Some("E0451"),
        ),
        (
            "forged-stored-run",
            "fn forged(report: macroonz::harness::report::archive::ArchivedRun, input: macroonz::harness::input::InputEnvelope) { let _run = macroonz::workflow::StoredRun { report, input, capsules: Default::default() }; } fn main() { let _forge = forged; }",
            Some("E0451"),
        ),
    ] {
        std::fs::write(subject.join("main.rs"), source).map_err(|error| error.to_string())?;
        let output = super::check::cargo(
            &subject,
            root,
            &scratch,
            name,
            &["check", "-j1", "--locked", "--offline"],
        )?;
        let stderr = String::from_utf8_lossy(&output.stderr);
        match expected {
            None if output.status.success() => {}
            Some(code) if output.status.code() == Some(101_i32) && stderr.contains(code) => {}
            _ => return Err(format!("{name}: unexpected compile disposition\n{stderr}")),
        }
    }
    Ok(())
}
