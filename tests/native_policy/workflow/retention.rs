//! Complete canonical records cross actual storage without execution or historical promotion.

use super::fixture::{self, mapped};
use macroonz::harness::report::archive::{
    ArchiveLimits, ArchiveRefusal, RunArchiveLimits, retain_run,
};
use macroonz::harness::report::replay::ReplayOutcome;
use macroonz::native_storage::{StorageLimits, StorageName, StorageRoot};
use macroonz::workflow::{RetentionLimits, RetentionRefusal, StoredRun};
use std::path::Path;

pub(super) const LIMITS: RetentionLimits = RetentionLimits {
    input: fixture::INPUT_LIMITS,
    archive: RunArchiveLimits::declared(ArchiveLimits::declared(16_384, 8192), 16),
    storage: StorageLimits {
        artifacts: 18,
        bytes: 65_536,
    },
};

pub(super) fn observe(
    action: impl FnOnce(&StorageRoot, &Path) -> Result<(), String>,
) -> Result<(), String> {
    let path = crate::check::scratch()?;
    let root = mapped(StorageRoot::open(&path))?;
    let result = action(&root, &path);
    drop(root);
    let cleanup = std::fs::remove_dir_all(&path).map_err(|error| error.to_string());
    result.and(cleanup)
}

#[test]
fn retained_run_loads_without_execution_and_replays_the_reached_witness() -> Result<(), String> {
    observe(|root, _path| {
        let original = fixture::run(&[7, 1, 9])?;
        let capsule = fixture::capsule(&original)?;
        let name = mapped(StorageName::informed("run"))?;
        mapped(original.retain(root, &name, &[capsule], LIMITS))?;
        assert!(matches!(
            original.retain(root, &name, &[], LIMITS),
            Err(RetentionRefusal::Storage(
                macroonz::native_storage::StorageError::Collision
            ))
        ));
        fixture::reset();
        let loaded = mapped(StoredRun::load(
            root,
            &name,
            fixture::decoder()?.profile(),
            LIMITS,
        ))?;
        assert_eq!(fixture::observations(), (0, 0, 0));
        assert_eq!(
            loaded.report(),
            &mapped(retain_run(original.report(), LIMITS.archive))?
        );
        assert_eq!(loaded.input(), original.input());
        assert_eq!(loaded.capsule(0).ok_or("capsule missing")?.input(), &[1]);
        assert!(loaded.capsule(1).is_none());
        let current = mapped(loaded.replay(
            0,
            &fixture::binding("selected", fixture::fixed_revision(), fixture::fixed)?,
            &fixture::decoder()?,
            fixture::invocation(1),
            fixture::INPUT_LIMITS,
        ))?;
        assert_eq!(fixture::observations(), (1, 1, 0));
        assert_eq!(
            mapped(current.comparison().as_ref())?.outcome(),
            ReplayOutcome::FixedOnWitness
        );
        Ok(())
    })
}

#[test]
fn unrelated_and_duplicate_capsules_refuse_before_reserving_storage() -> Result<(), String> {
    observe(|root, path| {
        let original = fixture::run(&[7, 1, 9])?;
        let unrelated = fixture::run(&[1])?;
        let name = mapped(StorageName::informed("run"))?;
        assert!(matches!(
            original.retain(root, &name, &[fixture::capsule(&unrelated)?], LIMITS),
            Err(RetentionRefusal::CapsuleJoin)
        ));
        assert!(!path.join("item-run").exists());
        let capsule = fixture::capsule(&original)?;
        assert!(matches!(
            original.retain(root, &name, &[capsule.clone(), capsule], LIMITS),
            Err(RetentionRefusal::CapsuleJoin)
        ));
        assert!(!path.join("item-run").exists());
        Ok(())
    })
}

#[test]
fn changed_archive_bytes_and_individually_valid_unrelated_inputs_refuse_loading()
-> Result<(), String> {
    observe(|root, path| {
        let original = fixture::run(&[7, 1, 9])?;
        let name = mapped(StorageName::informed("run"))?;
        mapped(original.retain(root, &name, &[], LIMITS))?;
        let input = fixture::run(&[9])?;
        std::fs::write(path.join("item-run/item-input"), input.input().encoded())
            .map_err(|error| error.to_string())?;
        assert!(matches!(
            StoredRun::load(root, &name, fixture::decoder()?.profile(), LIMITS),
            Err(RetentionRefusal::InputJoin)
        ));
        std::fs::write(path.join("item-run/item-input"), original.input().encoded())
            .map_err(|error| error.to_string())?;
        let run_path = path.join("item-run/item-run");
        let mut bytes = std::fs::read(&run_path).map_err(|error| error.to_string())?;
        let last = bytes.last_mut().ok_or("empty report envelope")?;
        *last ^= 1;
        std::fs::write(&run_path, bytes).map_err(|error| error.to_string())?;
        fixture::reset();
        assert!(matches!(
            StoredRun::load(root, &name, fixture::decoder()?.profile(), LIMITS),
            Err(RetentionRefusal::Archive(ArchiveRefusal::AddressMismatch))
        ));
        assert_eq!(fixture::observations(), (0, 0, 0));
        Ok(())
    })
}
