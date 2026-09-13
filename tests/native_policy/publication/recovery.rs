use super::destination_fixture::{selected_snapshot, snapshot};
use super::install_fixture::{destination, finish, installed, subject};
use crate::compiler::configure::root;
use macroonz::native_publication::DestinationError;

#[test]
fn committed_ownership_and_each_journal_retirement_boundary_recover() -> Result<(), String> {
    let source = root()?;
    let compiled = subject(&source, "subject", [7, 9, 42], "generated/other.rs")?;
    for removed in 0usize..=3 {
        let output = root()?;
        let destination = destination(&output)?;
        let installation = destination
            .begin(&compiled)
            .map_err(|error| error.to_string())?;
        let pending = output.join(".macroonz-publication/item-pending");
        let retained = [".committed", ".prepared", "item-intent"]
            .into_iter()
            .map(|name| {
                Ok((
                    name,
                    std::fs::read(pending.join(name)).map_err(|error| error.to_string())?,
                ))
            })
            .collect::<Result<Vec<_>, String>>()?;
        finish(installation)?;
        std::fs::create_dir(&pending).map_err(|error| error.to_string())?;
        for (name, bytes) in retained.iter().skip(removed) {
            std::fs::write(pending.join(name), bytes).map_err(|error| error.to_string())?;
        }
        if let Some(recovered) = destination.recover().map_err(|error| error.to_string())? {
            finish(recovered)?;
        }
        installed(&output, compiled.prepared())?;
    }
    Ok(())
}

#[test]
fn damaged_intent_markers_unknown_entries_and_changed_outputs_refuse_without_writes()
-> Result<(), String> {
    let source = root()?;
    let compiled = subject(&source, "subject", [7, 9, 42], "generated/other.rs")?;
    for damage in ["intent", "marker", "unknown", "output", "missing-intent"] {
        let output = root()?;
        let destination = destination(&output)?;
        let mut installation = destination
            .begin(&compiled)
            .map_err(|error| error.to_string())?;
        assert!(
            installation
                .write_next()
                .map_err(|error| error.to_string())?
                .is_some()
        );
        drop(installation);
        let pending = output.join(".macroonz-publication/item-pending");
        match damage {
            "intent" => std::fs::write(pending.join("item-intent"), b"[]\n"),
            "marker" => std::fs::write(pending.join(".committed"), b"forged\n"),
            "unknown" => std::fs::write(pending.join("item-authored"), b"authored"),
            "output" => std::fs::write(output.join("generated/alpha.rs"), b"changed by owner"),
            "missing-intent" => std::fs::remove_file(pending.join("item-intent")),
            _ => return Err("unknown damage".to_owned()),
        }
        .map_err(|error| error.to_string())?;
        let before = snapshot(&output)?;
        assert!(destination.recover().is_err(), "{damage}");
        assert_eq!(snapshot(&output)?, before, "{damage}");
    }
    Ok(())
}

#[test]
fn changed_ownership_or_completed_output_prevents_commit_and_retains_intent() -> Result<(), String>
{
    let source = root()?;
    let compiled = subject(&source, "subject", [7, 9, 42], "generated/other.rs")?;
    for damage in ["output", "ownership", "intent"] {
        let output = root()?;
        let destination = destination(&output)?;
        let mut installation = destination
            .begin(&compiled)
            .map_err(|error| error.to_string())?;
        while installation
            .write_next()
            .map_err(|error| error.to_string())?
            .is_some()
        {}
        match damage {
            "output" => std::fs::write(output.join("generated/other.rs"), b"authored edit"),
            "ownership" => std::fs::write(
                output.join(".macroonz-publication/current"),
                b"[\"macroonz-publication/1\",[]]\n",
            ),
            "intent" => std::fs::write(
                output.join(".macroonz-publication/item-pending/item-intent"),
                b"[]\n",
            ),
            _ => return Err("unknown damage".to_owned()),
        }
        .map_err(|error| error.to_string())?;
        let watched = [
            "generated/alpha.rs",
            "generated/beta.rs",
            "generated/definition.rs",
            "generated/other.rs",
            ".macroonz-publication/current",
            ".macroonz-publication/replacement",
            ".macroonz-publication/item-pending/item-intent",
            ".macroonz-publication/item-pending/.committed",
            ".macroonz-publication/item-pending/.prepared",
        ];
        let before = selected_snapshot(&output, &watched)?;
        assert!(installation.commit().is_err());
        assert_eq!(selected_snapshot(&output, &watched)?, before);
    }
    Ok(())
}

#[test]
fn missing_cooperating_lock_cannot_be_silently_recreated_for_owned_output() -> Result<(), String> {
    let source = root()?;
    let compiled = subject(&source, "subject", [7, 9, 42], "generated/other.rs")?;
    let output = root()?;
    let destination = destination(&output)?;
    finish(
        destination
            .begin(&compiled)
            .map_err(|error| error.to_string())?,
    )?;
    std::fs::remove_file(output.join(".macroonz-storage-lock"))
        .map_err(|error| error.to_string())?;
    let before = snapshot(&output)?;
    assert!(matches!(
        destination.begin(&compiled),
        Err(DestinationError::MissingLock)
    ));
    assert!(matches!(
        destination.recover(),
        Err(DestinationError::MissingLock)
    ));
    assert_eq!(snapshot(&output)?, before);
    Ok(())
}
