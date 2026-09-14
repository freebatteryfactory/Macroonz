use super::types::Ownership;
use super::{
    DestinationCheck, DestinationError, DestinationIssue, DestinationLimits, DestinationProblem,
    DestinationState, PublicationDestination, files, record,
};
use crate::native_publication::{PublicationLimits, PublicationPath, published_digest};
use cap_std::fs::Dir;
use std::collections::BTreeMap;

pub(super) fn current(
    root: &Dir,
    limits: DestinationLimits,
) -> Result<Option<Ownership>, DestinationError> {
    files::read(root, files::CURRENT, limits.metadata)?
        .map(|bytes| record::decode(&bytes, limits))
        .transpose()
}

pub(super) fn check(
    destination: &PublicationDestination,
    expected: &Ownership,
) -> Result<DestinationCheck, DestinationError> {
    let lease = destination
        .root
        .shared_existing()
        .map_err(DestinationError::Storage)?;
    let root = destination.root.directory();
    let current = current(root, destination.limits)?;
    if current.is_some() && lease.is_none() {
        return Err(DestinationError::MissingLock);
    }
    let pending =
        files::directory(root, files::CONTROL)? && files::directory(root, files::PENDING)?;
    let issues = compare(root, current.as_ref(), expected, destination.limits)?;
    drop(lease);
    let state = match (current.is_some(), pending) {
        (false, false) => DestinationState::Uninitialized,
        (true, false) => DestinationState::Installed,
        (false, true) => DestinationState::Preparing,
        (true, true) => DestinationState::Updating,
    };
    Ok(DestinationCheck { state, issues })
}

pub(super) fn paths(
    previous: Option<&Ownership>,
    expected: &Ownership,
    limits: DestinationLimits,
) -> Result<Vec<PublicationPath>, DestinationError> {
    let paths = previous
        .into_iter()
        .flat_map(|record| &record.files)
        .chain(&expected.files)
        .map(|file| (file.path.spelling(), file.path.clone()))
        .collect::<BTreeMap<_, _>>()
        .into_values()
        .collect::<Vec<_>>();
    drop(
        crate::native_publication::inventory::bounded_paths(
            paths.iter().cloned().map(|path| (path, 0)),
            PublicationLimits {
                files: limits.files,
                bytes: 0,
            },
        )
        .map_err(DestinationError::Inventory)?,
    );
    Ok(paths)
}

pub(super) fn compare(
    root: &Dir,
    previous: Option<&Ownership>,
    expected: &Ownership,
    limits: DestinationLimits,
) -> Result<Vec<DestinationIssue>, DestinationError> {
    let mut issues = Vec::new();
    let mut remaining = limits.bytes;
    for path in paths(previous, expected, limits)? {
        let old = previous
            .into_iter()
            .flat_map(|record| &record.files)
            .find(|file| file.path == path);
        let next = expected.files.iter().find(|file| file.path == path);
        let actual = files::read(root, path.spelling(), remaining)?;
        let mut report = |problem| {
            issues.push(DestinationIssue {
                path: path.clone(),
                problem,
            });
        };
        if old.is_some() && next.is_none() {
            report(DestinationProblem::ExtraOwned);
        }
        let Some(bytes) = actual else {
            report(DestinationProblem::Missing);
            continue;
        };
        remaining = remaining
            .checked_sub(bytes.len())
            .ok_or(DestinationError::Inventory(
                crate::native_publication::InventoryError::ByteBound,
            ))?;
        let digest = *published_digest(&bytes).as_bytes();
        if old.is_none() {
            report(DestinationProblem::Unowned);
        }
        if old.is_some_and(|file| file.published != digest || file.bytes != bytes.len()) {
            report(DestinationProblem::Tampered);
        }
        if next.is_some_and(|file| {
            file.published != digest
                || file.bytes != bytes.len()
                || old.is_some_and(|recorded| recorded != file)
        }) {
            report(DestinationProblem::Stale);
        }
    }
    Ok(issues)
}
