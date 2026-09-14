//! Canonical owner bytes enter one bounded storage transaction.

use super::{InputRun, RetentionLimits, RetentionRefusal};
use crate::harness::report::archive::{
    ArchiveLimits, ArchiveRefusal, RunArchiveLimits, retain_capsule, retain_run,
};
use crate::harness::report::{Fingerprint, ReplayCapsule, RunAttempt, TrialConclusion};
use crate::native_storage::{
    StorageArtifact, StorageBatch, StorageName, StorageRoot, StorageTransaction,
};
use std::collections::BTreeSet;

fn material(
    run: &InputRun,
    capsules: &[ReplayCapsule],
    limits: RetentionLimits,
) -> Result<Vec<(StorageName, Vec<u8>)>, RetentionRefusal> {
    if run.input().encoded().len() > limits.input.envelope() {
        return Err(RetentionRefusal::Input(
            crate::harness::input::InputRefusal::EnvelopeTooLarge,
        ));
    }
    if run.input().payload().len() > limits.input.payload() {
        return Err(RetentionRefusal::Input(
            crate::harness::input::InputRefusal::PayloadTooLarge,
        ));
    }
    let count = capsules
        .len()
        .checked_add(2)
        .ok_or(RetentionRefusal::Storage(
            crate::native_storage::StorageError::ArtifactBound,
        ))?;
    if count > limits.storage.artifacts {
        return Err(RetentionRefusal::Storage(
            crate::native_storage::StorageError::ArtifactBound,
        ));
    }
    let mut remaining = limits
        .storage
        .bytes
        .checked_sub(run.input().encoded().len())
        .ok_or(RetentionRefusal::Storage(
            crate::native_storage::StorageError::ByteBound,
        ))?;
    let report = retain_run(
        run.report(),
        RunArchiveLimits::declared(
            archive_limit(limits.archive.bytes(), remaining),
            limits.archive.rows(),
        ),
    )
    .map_err(|error| archive_error(error, limits.archive.bytes(), remaining))?
    .encoded()
    .to_vec();
    remaining = remaining
        .checked_sub(report.len())
        .ok_or(RetentionRefusal::Storage(
            crate::native_storage::StorageError::ByteBound,
        ))?;
    let mut files = vec![
        (name("input")?, run.input().encoded().to_vec()),
        (name("run")?, report),
    ];
    let mut positions = BTreeSet::new();
    for capsule in capsules {
        let index = capsule_row(run, capsule)?;
        if !positions.insert(index) {
            return Err(RetentionRefusal::CapsuleJoin);
        }
        let archived = retain_capsule(capsule, archive_limit(limits.archive.bytes(), remaining))
            .map_err(|error| archive_error(error, limits.archive.bytes(), remaining))?
            .encoded()
            .to_vec();
        remaining = remaining
            .checked_sub(archived.len())
            .ok_or(RetentionRefusal::Storage(
                crate::native_storage::StorageError::ByteBound,
            ))?;
        files.push((name(&format!("capsule-{index}"))?, archived));
    }
    Ok(files)
}

fn archive_limit(limits: ArchiveLimits, remaining: usize) -> ArchiveLimits {
    ArchiveLimits::declared(limits.envelope().min(remaining), limits.field())
}

fn archive_error(
    error: ArchiveRefusal,
    limits: ArchiveLimits,
    remaining: usize,
) -> RetentionRefusal {
    if error == ArchiveRefusal::EnvelopeTooLarge && remaining < limits.envelope() {
        RetentionRefusal::Storage(crate::native_storage::StorageError::ByteBound)
    } else {
        RetentionRefusal::Archive(error)
    }
}

fn name(spelling: &str) -> Result<StorageName, RetentionRefusal> {
    StorageName::informed(spelling).map_err(RetentionRefusal::Storage)
}

fn capsule_row(run: &InputRun, capsule: &ReplayCapsule) -> Result<usize, RetentionRefusal> {
    run.report()
        .census()
        .iter()
        .position(|row| {
            let Some(trial) = row.disposition().report() else {
                return false;
            };
            let RunAttempt::Executed(TrialConclusion::Refused(finding)) = trial.attempt() else {
                return false;
            };
            trial.standing().key() == capsule.key()
                && Fingerprint::of(trial.trial(), finding) == capsule.fingerprint()
        })
        .ok_or(RetentionRefusal::CapsuleJoin)
}

fn complete(mut transaction: StorageTransaction<'_>) -> Result<(), RetentionRefusal> {
    while transaction
        .write_next()
        .map_err(RetentionRefusal::Storage)?
        .is_some()
    {}
    transaction.commit().map_err(RetentionRefusal::Storage)
}

pub(super) fn retain(
    run: &InputRun,
    root: &StorageRoot,
    key: &StorageName,
    capsules: &[ReplayCapsule],
    limits: RetentionLimits,
) -> Result<(), RetentionRefusal> {
    store(run, capsules, limits, |batch| {
        StorageTransaction::begin(root, key, batch)
    })
}

pub(super) fn recover(
    run: &InputRun,
    root: &StorageRoot,
    key: &StorageName,
    capsules: &[ReplayCapsule],
    limits: RetentionLimits,
) -> Result<(), RetentionRefusal> {
    store(run, capsules, limits, |batch| {
        StorageTransaction::recover(root, key, batch)
    })
}

fn store(
    run: &InputRun,
    capsules: &[ReplayCapsule],
    limits: RetentionLimits,
    open: impl for<'data> FnOnce(
        StorageBatch<'data>,
    ) -> Result<
        StorageTransaction<'data>,
        crate::native_storage::StorageError,
    >,
) -> Result<(), RetentionRefusal> {
    let files = material(run, capsules, limits)?;
    let artifacts = files
        .iter()
        .map(|(name, bytes)| StorageArtifact { name, bytes })
        .collect::<Vec<_>>();
    let batch =
        StorageBatch::informed(&artifacts, limits.storage).map_err(RetentionRefusal::Storage)?;
    complete(open(batch).map_err(RetentionRefusal::Storage)?)
}
