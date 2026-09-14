//! Rendering storage failures never releases custody or manufactures an OS code.

use crate::presentation_formats::{field, parsed};
use crate::transaction::{declared, finish, observe, storage};
use macroonz::native_storage::{StorageError, StorageRoot, StorageTransaction};
use macroonz::presentation;
use serde_json::Value;

#[test]
fn presentation_storage_retains_busy_incomplete_and_collision_without_advancing_custody()
-> Result<(), String> {
    observe(|root, _path| {
        declared(|name, batch| {
            let mut transaction = storage(StorageTransaction::begin(root, name, batch))?;
            let busy = root
                .load(name, batch.limits())
                .err()
                .ok_or("live transaction readable")?;
            let busy_display = presentation::storage_error(&busy);
            let busy_json = parsed(&busy_display)?;
            assert_eq!(field(&busy_json, "/record/kind")?, "busy");
            assert!(matches!(
                root.load(name, batch.limits()),
                Err(StorageError::Busy)
            ));
            assert!(storage(transaction.write_next())?.is_some());
            drop(transaction);
            let incomplete = root
                .load(name, batch.limits())
                .err()
                .ok_or("partial batch published")?;
            let incomplete_json = parsed(&presentation::storage_error(&incomplete))?;
            assert_eq!(field(&incomplete_json, "/record/kind")?, "incomplete");
            finish(storage(StorageTransaction::recover(root, name, batch))?)?;
            let collision = StorageTransaction::begin(root, name, batch)
                .err()
                .ok_or("published batch overwritten")?;
            let collision_json = parsed(&presentation::storage_error(&collision))?;
            assert_eq!(field(&collision_json, "/record/kind")?, "collision");
            assert_eq!(parsed(&busy_display)?, busy_json);
            assert!(busy_json.pointer("/record/phase").is_none());
            Ok(())
        })
    })
}

#[test]
fn presentation_storage_keeps_actual_os_failure_and_message_only_error_distinct()
-> Result<(), String> {
    observe(|_root, path| {
        let missing = StorageRoot::open(&path.join("missing"))
            .err()
            .ok_or("missing root opened")?;
        let StorageError::Io(ref os_error) = missing else {
            return Err("wrong missing-root cause".to_owned());
        };
        let shown = parsed(&presentation::storage_error(&missing))?;
        assert_eq!(
            field(&shown, "/record/value/os_code")?,
            &serde_json::to_value(os_error.raw_os_error()).map_err(|e| e.to_string())?
        );
        assert_eq!(field(&shown, "/record/value/kind")?, "NotFound");
        let material = "Passed | <script>subject passed</script> & still denied";
        let supplied = StorageError::Io(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            material,
        ));
        let supplied_display = parsed(&presentation::storage_error(&supplied))?;
        assert_eq!(
            field(&supplied_display, "/record/value/os_code")?,
            &Value::Null
        );
        assert_eq!(field(&supplied_display, "/record/value/detail")?, material);
        assert_eq!(
            field(&supplied_display, "/record/value/kind")?,
            "PermissionDenied"
        );
        assert!(supplied_display.pointer("/record/verdict").is_none());
        Ok(())
    })
}
