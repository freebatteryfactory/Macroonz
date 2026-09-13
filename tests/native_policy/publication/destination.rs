use super::configure::publication;
use super::destination_fixture::{LIMITS, historical, record, snapshot, write_record};
use crate::compiler::configure::root;
use macroonz::native_publication::{
    DestinationError, DestinationProblem, DestinationState, PreparedPublication,
    PublicationDestination,
};

#[test]
fn destination_check_is_read_only_and_requires_actual_ownership_even_for_identical_files()
-> Result<(), String> {
    let root = root()?;
    std::fs::write(root.join("authored.rs"), b"authored neighbor")
        .map_err(|error| error.to_string())?;
    let prepared = PreparedPublication::unformatted(publication()?);
    let before = snapshot(&root)?;
    let destination =
        PublicationDestination::open(&root, LIMITS).map_err(|error| error.to_string())?;
    let absent = destination
        .check(&prepared)
        .map_err(|error| error.to_string())?;
    assert!(!absent.is_current());
    assert_eq!(absent.state(), DestinationState::Uninitialized);
    assert_eq!(absent.issues().len(), 4);
    assert!(
        absent
            .issues()
            .iter()
            .all(|issue| issue.problem == DestinationProblem::Missing)
    );
    assert_eq!(snapshot(&root)?, before);
    std::fs::create_dir(root.join("generated")).map_err(|error| error.to_string())?;
    let file = prepared.files().last().ok_or("no output")?;
    std::fs::write(root.join(file.path().spelling()), file.bytes())
        .map_err(|error| error.to_string())?;
    let before_existing = snapshot(&root)?;
    let unowned = destination
        .check(&prepared)
        .map_err(|error| error.to_string())?;
    assert!(unowned.issues().iter().any(|issue| issue.path == *file.path() && issue.problem == DestinationProblem::Unowned));
    assert_eq!(snapshot(&root)?, before_existing);
    historical(&root, &prepared)?;
    let before_owned = snapshot(&root)?;
    assert!(
        destination
            .check(&prepared)
            .map_err(|error| error.to_string())?
            .is_current()
    );
    assert_eq!(snapshot(&root)?, before_owned);
    Ok(())
}

#[test]
fn complete_destination_check_distinguishes_missing_stale_tampered_and_extra_owned_material()
-> Result<(), String> {
    let root = root()?;
    let prepared = PreparedPublication::unformatted(publication()?);
    historical(&root, &prepared)?;
    let mut recorded = record(&prepared);
    let empty_digest = [0u8; 32];
    let rows = recorded
        .get_mut(1)
        .and_then(serde_json::Value::as_array_mut)
        .ok_or("no rows")?;
    rows.push(serde_json::json!([
        "obsolete.rs",
        empty_digest,
        macroonz::native_publication::published_digest(b"old\n").as_bytes(),
        4usize
    ]));
    let first = rows
        .first_mut()
        .and_then(serde_json::Value::as_array_mut)
        .ok_or("no first row")?;
    *first.get_mut(1).ok_or("no canonical digest")? = serde_json::json!(empty_digest);
    write_record(&root, &recorded)?;
    std::fs::write(root.join("obsolete.rs"), b"old\n").map_err(|error| error.to_string())?;
    std::fs::write(root.join("neighbor.rs"), b"not owned").map_err(|error| error.to_string())?;
    std::fs::remove_file(root.join("generated/beta.rs")).map_err(|error| error.to_string())?;
    std::fs::write(root.join("generated/other.rs"), b"altered\n")
        .map_err(|error| error.to_string())?;
    let before = snapshot(&root)?;
    let destination =
        PublicationDestination::open(&root, LIMITS).map_err(|error| error.to_string())?;
    let report = destination
        .check(&prepared)
        .map_err(|error| error.to_string())?;
    let issues = report
        .issues()
        .iter()
        .map(|issue| (issue.path.spelling(), issue.problem))
        .collect::<Vec<_>>();
    assert_eq!(
        issues,
        [
            ("generated/alpha.rs", DestinationProblem::Stale),
            ("generated/beta.rs", DestinationProblem::Missing),
            ("generated/other.rs", DestinationProblem::Tampered),
            ("generated/other.rs", DestinationProblem::Stale),
            ("obsolete.rs", DestinationProblem::ExtraOwned),
        ]
    );
    assert!(!report.is_current());
    assert_eq!(snapshot(&root)?, before);
    Ok(())
}

#[test]
fn pending_installation_and_exclusive_writer_custody_prevent_a_current_result() -> Result<(), String>
{
    let root = root()?;
    let prepared = PreparedPublication::unformatted(publication()?);
    historical(&root, &prepared)?;
    let destination =
        PublicationDestination::open(&root, LIMITS).map_err(|error| error.to_string())?;
    let lease = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(root.join(".macroonz-storage-lock"))
        .map_err(|error| error.to_string())?;
    lease.try_lock().map_err(|error| error.to_string())?;
    assert!(matches!(
        destination.check(&prepared),
        Err(DestinationError::Storage(
            macroonz::native_storage::StorageError::Busy
        ))
    ));
    drop(lease);
    std::fs::create_dir(root.join(".macroonz-publication/item-pending"))
        .map_err(|error| error.to_string())?;
    let before = snapshot(&root)?;
    let report = destination
        .check(&prepared)
        .map_err(|error| error.to_string())?;
    assert_eq!(report.state(), DestinationState::Updating);
    assert!(report.issues().is_empty());
    assert!(!report.is_current());
    assert_eq!(snapshot(&root)?, before);
    std::fs::remove_file(root.join(".macroonz-storage-lock")).map_err(|error| error.to_string())?;
    assert!(matches!(
        destination.check(&prepared),
        Err(DestinationError::MissingLock)
    ));
    assert!(!root.join(".macroonz-storage-lock").exists());
    Ok(())
}
