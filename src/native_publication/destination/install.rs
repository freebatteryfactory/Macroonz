use super::types::InstallationIntent;
use super::{
    DestinationError, DestinationProblem, PublicationDestination, PublicationInstallation, files,
    inspect, journal, record, replace,
};
use crate::compiler::Kind;
use crate::native_publication::PreparedPublication;
use std::collections::BTreeMap;

fn lease(destination: &PublicationDestination) -> Result<std::fs::File, DestinationError> {
    let observed_lease = destination
        .root
        .shared_existing()
        .map_err(DestinationError::Storage)?;
    if observed_lease.is_none()
        && inspect::current(destination.root.directory(), destination.limits)?.is_some()
    {
        return Err(DestinationError::MissingLock);
    }
    drop(observed_lease);
    destination
        .root
        .exclusive()
        .map_err(DestinationError::Storage)
}

pub(super) fn begin<'root, K: Kind>(
    destination: &'root PublicationDestination,
    prepared: &PreparedPublication<K>,
) -> Result<PublicationInstallation<'root>, DestinationError> {
    let after = record::expected(prepared, destination.limits)?;
    let lease = lease(destination)?;
    let root = destination.root.directory();
    let before = inspect::current(root, destination.limits)?;
    let issues = inspect::compare(root, before.as_ref(), &after, destination.limits)?;
    if let Some(issue) = issues.iter().find(|issue| {
        matches!(
            issue.problem,
            DestinationProblem::Unowned | DestinationProblem::Tampered
        )
    }) {
        return Err(DestinationError::Conflict(issue.path.spelling().to_owned()));
    }
    let payloads = prepared
        .files()
        .map(|file| (file.path().spelling(), file.bytes().to_vec()))
        .collect::<BTreeMap<_, _>>()
        .into_values()
        .collect();
    let intent = InstallationIntent::admitted(before, after, payloads, destination.limits)?;
    let paths = inspect::paths(intent.before.as_ref(), &intent.after, destination.limits)?;
    journal::retain(root, &intent, destination.limits)?;
    Ok(PublicationInstallation {
        destination,
        lease,
        intent,
        paths,
        cursor: 0,
    })
}

pub(super) fn recover(
    destination: &PublicationDestination,
) -> Result<Option<PublicationInstallation<'_>>, DestinationError> {
    let lease = lease(destination)?;
    let root = destination.root.directory();
    if !files::directory(root, files::CONTROL)? || !files::directory(root, files::PENDING)? {
        return Ok(None);
    }
    let intent = journal::load(root, destination.limits)?;
    let Some(intent) = intent else {
        if journal::committed(root)? {
            return Err(DestinationError::Metadata(
                "committed intent payload absent".to_owned(),
            ));
        }
        journal::retire(root, None, destination.limits)?;
        return Ok(None);
    };
    let current = inspect::current(root, destination.limits)?;
    if current != intent.before && current.as_ref() != Some(&intent.after) {
        return Err(DestinationError::Conflict("ownership record".to_owned()));
    }
    if !journal::committed(root)? {
        if current.as_ref() != Some(&intent.after) {
            return Err(DestinationError::Incomplete);
        }
        replace::completed(root, &intent, destination.limits)?;
        journal::retire(root, Some(&intent), destination.limits)?;
        return Ok(None);
    }
    replace::compatible(root, &intent, destination.limits)?;
    let paths = inspect::paths(intent.before.as_ref(), &intent.after, destination.limits)?;
    Ok(Some(PublicationInstallation {
        destination,
        lease,
        intent,
        paths,
        cursor: 0,
    }))
}

pub(super) fn retained(installation: &PublicationInstallation<'_>) -> Result<(), DestinationError> {
    let root = installation.destination.root.directory();
    let limits = installation.destination.limits;
    if !journal::committed(root)?
        || journal::load(root, limits)?.as_ref() != Some(&installation.intent)
    {
        return Err(DestinationError::Conflict(
            "installation journal".to_owned(),
        ));
    }
    Ok(())
}

pub(super) fn commit(installation: &PublicationInstallation<'_>) -> Result<(), DestinationError> {
    retained(installation)?;
    let root = installation.destination.root.directory();
    let limits = installation.destination.limits;
    replace::completed(root, &installation.intent, limits)?;
    let current = inspect::current(root, limits)?;
    if current != installation.intent.before && current.as_ref() != Some(&installation.intent.after)
    {
        return Err(DestinationError::Conflict("ownership record".to_owned()));
    }
    let bytes = record::encode(&installation.intent.after)?;
    replace::file(root, files::CURRENT, &bytes)?;
    journal::retire(root, Some(&installation.intent), limits)
}
