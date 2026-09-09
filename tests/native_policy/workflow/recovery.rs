//! Root recovery and historical joins retain their independent refusal planes.

use super::{
    fixture::{self, mapped},
    retention::{LIMITS, observe},
};
use macroonz::native_storage::{
    StorageArtifact, StorageBatch, StorageError, StorageLimits, StorageName, StorageTransaction,
};
use macroonz::workflow::{RetentionLimits, RetentionRefusal, StoredRun};

#[test]
fn workflow_recovery_replaces_partial_bytes_and_write_refusal_preserves_the_report()
-> Result<(), String> {
    observe(|root, path| {
        let run = fixture::run(&[7, 1, 9])?;
        let before = run.report().clone();
        let name = mapped(StorageName::informed("run"))?;
        let bounded = RetentionLimits {
            storage: StorageLimits {
                artifacts: 18,
                bytes: 0,
            },
            ..LIMITS
        };
        assert!(matches!(
            run.retain(root, &name, &[], bounded),
            Err(RetentionRefusal::Storage(StorageError::ByteBound))
        ));
        assert!(!path.join("item-run").exists());
        assert_eq!(run.report(), &before);
        let input_name = mapped(StorageName::informed("input"))?;
        let partial = [StorageArtifact {
            name: &input_name,
            bytes: b"partial input",
        }];
        let mut transaction = mapped(StorageTransaction::begin(
            root,
            &name,
            mapped(StorageBatch::informed(&partial, LIMITS.storage))?,
        ))?;
        assert!(mapped(transaction.write_next())?.is_some());
        drop(transaction);
        assert!(matches!(
            StoredRun::load(root, &name, fixture::decoder()?.profile(), LIMITS),
            Err(RetentionRefusal::Storage(StorageError::Incomplete))
        ));
        mapped(run.recover_retention(root, &name, &[fixture::capsule(&run)?], LIMITS))?;
        let saved = mapped(StoredRun::load(
            root,
            &name,
            fixture::decoder()?.profile(),
            LIMITS,
        ))?;
        assert_eq!(saved.input(), run.input());
        assert_eq!(
            saved.capsule(0).ok_or("missing reached witness")?.input(),
            &[1]
        );
        assert_eq!(run.report(), &before);
        Ok(())
    })
}

#[test]
fn valid_capsule_from_another_original_case_refuses_the_retained_join() -> Result<(), String> {
    observe(|root, path| {
        let run = fixture::run(&[7, 1, 9])?;
        let key = mapped(StorageName::informed("run"))?;
        mapped(run.retain(root, &key, &[fixture::capsule(&run)?], LIMITS))?;
        let other = fixture::run(&[1])?;
        let capsule = mapped(macroonz::harness::report::archive::retain_capsule(
            &fixture::capsule(&other)?,
            LIMITS.archive.bytes(),
        ))?;
        std::fs::write(path.join("item-run/item-capsule-0"), capsule.encoded())
            .map_err(|error| error.to_string())?;
        fixture::reset();
        assert!(matches!(
            StoredRun::load(root, &key, fixture::decoder()?.profile(), LIMITS),
            Err(RetentionRefusal::CapsuleJoin)
        ));
        assert_eq!(fixture::observations(), (0, 0, 0));
        Ok(())
    })
}
