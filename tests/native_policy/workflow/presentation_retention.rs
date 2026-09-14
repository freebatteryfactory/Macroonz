//! Stored workflow projections preserve every witness and separate persistence failures.

use super::{
    fixture::{self, mapped},
    retention::{LIMITS, observe},
};
use crate::presentation_formats::{decoded_hex, field, parsed};
use macroonz::harness::input::InputLimits;
use macroonz::harness::report::archive::{ArchiveLimits, RunArchiveLimits};
use macroonz::harness::report::{RunAttempt, TrialConclusion};
use macroonz::native_storage::{StorageLimits, StorageName};
use macroonz::presentation;
use macroonz::workflow::{RetentionLimits, StoredRun};
use serde_json::json;

#[test]
fn presentation_stored_run_preserves_original_input_and_all_census_witness_positions()
-> Result<(), String> {
    observe(|root, _path| {
        let original = super::retention_fixture::run(&[7, 1, 9])?;
        let first = super::retention_fixture::capsule(&original, 1)?;
        let second = super::retention_fixture::capsule(&original, 2)?;
        let name = mapped(StorageName::informed("joined"))?;
        mapped(original.retain(root, &name, &[second, first], LIMITS))?;
        let loaded = mapped(StoredRun::load(
            root,
            &name,
            fixture::decoder()?.profile(),
            LIMITS,
        ))?;
        fixture::reset();
        let shown = parsed(&presentation::stored_run(&loaded))?;
        assert_eq!(field(&shown, "/standing")?, "historical-unauthenticated");
        assert_eq!(
            decoded_hex(field(&shown, "/record/input/payload")?)?,
            [7, 1, 9]
        );
        assert_eq!(
            decoded_hex(field(&shown, "/record/input/encoded")?)?,
            original.input().encoded()
        );
        assert_eq!(field(&shown, "/record/report/denominator")?, 3_usize);
        let capsules = field(&shown, "/record/capsules")?
            .as_array()
            .ok_or("missing capsule roster")?;
        assert_eq!(capsules.len(), 2);
        assert_eq!(field(&shown, "/record/capsules/0/row")?, 1_usize);
        assert_eq!(field(&shown, "/record/capsules/1/row")?, 2_usize);
        for (entry, row) in capsules.iter().zip([1_usize, 2]) {
            let expected = parsed(&presentation::archived_capsule(
                loaded.capsule(row).ok_or("missing earned capsule")?,
            ))?;
            assert_eq!(field(entry, "/capsule")?, field(&expected, "/record")?);
            assert_eq!(decoded_hex(field(entry, "/capsule/input")?)?, [1_u8]);
        }
        let report = parsed(&presentation::archived_run(loaded.report()))?;
        assert_eq!(field(&shown, "/record/report")?, field(&report, "/record")?);
        assert!(loaded.capsule(0).is_none());
        assert_eq!(fixture::observations(), (0, 0, 0));
        Ok(())
    })
}

#[test]
fn presentation_retention_failure_preserves_passed_subject_and_responsible_owner()
-> Result<(), String> {
    observe(|root, path| {
        let run = fixture::run(&[7, 9])?;
        assert!(matches!(
            fixture::selected(&run)?.attempt(),
            RunAttempt::Executed(TrialConclusion::Passed)
        ));
        let before = run.clone();
        let calls = fixture::observations();
        let name = mapped(StorageName::informed("failed-write"))?;
        for (limits, expected) in [
            (
                RetentionLimits {
                    storage: StorageLimits {
                        bytes: 0,
                        ..LIMITS.storage
                    },
                    ..LIMITS
                },
                json!({"kind":"storage","value":{"kind":"byte-bound","value":null}}),
            ),
            (
                RetentionLimits {
                    archive: RunArchiveLimits::declared(ArchiveLimits::declared(0, 8192), 16),
                    ..LIMITS
                },
                json!({"kind":"archive","value":{"kind":"envelope-too-large","value":null}}),
            ),
            (
                RetentionLimits {
                    input: InputLimits::declared(4096, 0),
                    ..LIMITS
                },
                json!({"kind":"input","value":{"kind":"payload-too-large","value":null}}),
            ),
        ] {
            let refusal = run
                .retain(root, &name, &[], limits)
                .err()
                .ok_or("undersized retention succeeded")?;
            let shown = parsed(&presentation::retention_refusal(&refusal))?;
            assert_eq!(field(&shown, "/record")?, &expected);
            assert!(!path.join("item-failed-write").exists());
            assert_eq!(run, before);
            assert_eq!(fixture::observations(), calls);
        }
        mapped(run.retain(root, &name, &[], LIMITS))?;
        let saved = mapped(StoredRun::load(
            root,
            &name,
            fixture::decoder()?.profile(),
            LIMITS,
        ))?;
        let no_capsule = saved
            .replay(
                0,
                &fixture::binding("selected", fixture::revision(), fixture::defective)?,
                &fixture::decoder()?,
                fixture::invocation(1),
                fixture::INPUT_LIMITS,
            )
            .err()
            .ok_or("unearned witness replayed")?;
        let absent_display = parsed(&presentation::retention_refusal(&no_capsule))?;
        assert_eq!(field(&absent_display, "/record/kind")?, "capsule-absent");
        assert_eq!(fixture::observations(), calls);
        let stored = parsed(&presentation::stored_run(&saved))?;
        assert_eq!(field(&stored, "/record/capsules")?, &json!([]));
        Ok(())
    })
}
