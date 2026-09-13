use super::configure::publication;
use super::destination_fixture::{LIMITS, historical, record, snapshot, write_record};
use super::install_fixture::{destination, finish, subject};
use crate::compiler::configure::root;
use macroonz::native_publication::{
    DestinationError, DestinationLimits, PreparedPublication, PublicationDestination,
};
use macroonz::native_storage::StorageError;

#[test]
fn malformed_noncanonical_and_escaping_ownership_refuses_without_writes() -> Result<(), String> {
    let output = root()?;
    let prepared = PreparedPublication::unformatted(publication()?);
    historical(&output, &prepared)?;
    let destination = destination(&output)?;
    for damage in [
        "format",
        "duplicate",
        "order",
        "escape",
        "parent",
        "length",
        "digest",
        "case-alias",
        "whitespace",
    ] {
        let mut value = record(&prepared);
        if damage == "format" {
            *value.get_mut(0).ok_or("no format")? = serde_json::json!("unknown");
        } else {
            let rows = value
                .get_mut(1)
                .and_then(serde_json::Value::as_array_mut)
                .ok_or("no rows")?;
            if damage == "case-alias" {
                rows.truncate(1);
            }
            match damage {
                "duplicate" => rows.insert(0, rows.first().ok_or("no first row")?.clone()),
                "order" => rows.reverse(),
                "escape" | "parent" | "length" | "digest" | "case-alias" => {
                    let row = rows
                        .first_mut()
                        .and_then(serde_json::Value::as_array_mut)
                        .ok_or("no row")?;
                    let (index, replacement) = match damage {
                        "escape" => (0, serde_json::json!("../outside.rs")),
                        "parent" => (0, serde_json::json!("generated")),
                        "length" => (3, serde_json::json!(usize::MAX)),
                        "digest" => (2, serde_json::json!([1u8])),
                        "case-alias" => (0, serde_json::json!("Generated/alpha.rs")),
                        _ => return Err("unknown row damage".to_owned()),
                    };
                    *row.get_mut(index).ok_or("no field")? = replacement;
                }
                "whitespace" => {}
                _ => return Err("unknown damage".to_owned()),
            }
        }
        write_record(&output, &value)?;
        if damage == "whitespace" {
            std::fs::write(
                output.join(".macroonz-publication/current"),
                format!(" {value}\n"),
            )
            .map_err(|error| error.to_string())?;
        }
        let before = snapshot(&output)?;
        assert!(destination.check(&prepared).is_err(), "{damage}");
        assert_eq!(snapshot(&output)?, before, "{damage}");
    }
    Ok(())
}

#[test]
fn metadata_and_actual_physical_read_bounds_refuse_without_partial_success() -> Result<(), String> {
    let output = root()?;
    let prepared = PreparedPublication::unformatted(publication()?);
    historical(&output, &prepared)?;
    let metadata_bytes = format!("{}\n", record(&prepared)).len();
    let exact = PublicationDestination::open(
        &output,
        DestinationLimits {
            metadata: metadata_bytes,
            ..LIMITS
        },
    )
    .map_err(|error| error.to_string())?;
    assert!(
        exact
            .check(&prepared)
            .map_err(|error| error.to_string())?
            .is_current()
    );
    for limits in [
        DestinationLimits {
            metadata: metadata_bytes.checked_sub(1).ok_or("empty metadata")?,
            ..LIMITS
        },
        DestinationLimits { files: 3, ..LIMITS },
        DestinationLimits { bytes: 0, ..LIMITS },
    ] {
        let before = snapshot(&output)?;
        assert!(
            PublicationDestination::open(&output, limits)
                .map_err(|error| error.to_string())?
                .check(&prepared)
                .is_err()
        );
        assert_eq!(snapshot(&output)?, before);
    }
    std::fs::write(
        output.join("generated/other.rs"),
        vec![b'x'; LIMITS.bytes.checked_add(1).ok_or("overflow")?],
    )
    .map_err(|error| error.to_string())?;
    let before = snapshot(&output)?;
    assert!(matches!(
        destination(&output)?.check(&prepared),
        Err(DestinationError::Storage(StorageError::ByteBound))
    ));
    assert_eq!(snapshot(&output)?, before);
    Ok(())
}

#[test]
fn old_new_union_bounds_cover_every_intermediate_installation() -> Result<(), String> {
    let source = root()?;
    let original = subject(&source, "original", [7, 9, 42], "generated/other.rs")?;
    let changed = subject(&source, "changed", [7, 9, 42], "generated/moved.rs")?;
    let output = root()?;
    finish(
        destination(&output)?
            .begin(&original)
            .map_err(|error| error.to_string())?,
    )?;
    let old_bytes = original
        .prepared()
        .files()
        .map(|file| file.bytes().len())
        .sum::<usize>();
    let next_bytes = changed
        .prepared()
        .files()
        .map(|file| file.bytes().len())
        .sum::<usize>();
    for limits in [
        DestinationLimits { files: 4, ..LIMITS },
        DestinationLimits {
            bytes: old_bytes.max(next_bytes),
            ..LIMITS
        },
    ] {
        let before = snapshot(&output)?;
        let destination =
            PublicationDestination::open(&output, limits).map_err(|error| error.to_string())?;
        assert!(matches!(
            destination.begin(&changed),
            Err(DestinationError::Inventory(_))
        ));
        assert_eq!(snapshot(&output)?, before);
    }
    Ok(())
}
