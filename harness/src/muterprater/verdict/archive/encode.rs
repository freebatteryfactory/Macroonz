//! Target and activation member encoding after independent size admission.

use crate::identity::encode_bytes;
use crate::muterprater::{
    ActivationDisposition, FamilyAttribution, MappingPosture, MutationIdentity, MutationSite,
    MutationTarget,
};
use crate::report::archive::{ArchiveLimits, ArchiveRefusal, bounded, name_size, sum};

pub(crate) fn target_size(
    target: &MutationTarget,
    limits: ArchiveLimits,
) -> Result<usize, ArchiveRefusal> {
    bounded(32, limits)?;
    let identity = match target.identity() {
        MutationIdentity::External(_) => 41,
        MutationIdentity::Interpreted {
            point,
            alternative: _,
        }
        | MutationIdentity::CompiledProjection {
            point,
            alternative: _,
        } => sum(&[41, name_size(point.name(), limits)?])?,
    };
    let family = match target.family() {
        FamilyAttribution::OutsideTheBank => 1,
        FamilyAttribution::Declared(family) => sum(&[9, bounded(family.slug().len(), limits)?])?,
    };
    let site = match target.site() {
        MutationSite::Reported(coordinate) => {
            sum(&[17, bounded(coordinate.file().len(), limits)?])?
        }
        MutationSite::Declared(site) => sum(&[1, name_size(site.name(), limits)?])?,
    };
    let owner = match target.owner() {
        MappingPosture::OwnerUnmapped => 1,
        MappingPosture::Mapped(claim) => sum(&[1, name_size(claim.name(), limits)?])?,
    };
    bounded(sum(&[identity, family, site, owner])?, limits)
}

pub(crate) fn activation_size(
    activation: ActivationDisposition,
    limits: ArchiveLimits,
) -> Result<usize, ArchiveRefusal> {
    let size = match activation {
        ActivationDisposition::Observed(reading) => {
            bounded(32, limits)?;
            sum(&[125, name_size(reading.point().name(), limits)?])?
        }
        ActivationDisposition::NotObserved | ActivationDisposition::UnobservableUnderBackend => 1,
    };
    bounded(size, limits)
}

pub(crate) fn write_target(target: &MutationTarget, body: &mut Vec<u8>) {
    match target.identity() {
        MutationIdentity::External(identity) => {
            body.push(0);
            encode_bytes(identity.address().as_bytes(), body);
        }
        MutationIdentity::Interpreted { point, alternative } => {
            body.push(1);
            point.name().encode_into(body);
            encode_bytes(alternative.address().as_bytes(), body);
        }
        MutationIdentity::CompiledProjection { point, alternative } => {
            body.push(2);
            point.name().encode_into(body);
            encode_bytes(alternative.address().as_bytes(), body);
        }
    }
    match target.family() {
        FamilyAttribution::OutsideTheBank => body.push(0),
        FamilyAttribution::Declared(family) => {
            body.push(1);
            encode_bytes(family.slug().as_bytes(), body);
        }
    }
    match target.site() {
        MutationSite::Reported(coordinate) => {
            body.push(0);
            encode_bytes(coordinate.file().as_bytes(), body);
            body.extend_from_slice(&coordinate.line().to_be_bytes());
            body.extend_from_slice(&coordinate.column().to_be_bytes());
        }
        MutationSite::Declared(site) => {
            body.push(1);
            site.name().encode_into(body);
        }
    }
    match target.owner() {
        MappingPosture::OwnerUnmapped => body.push(0),
        MappingPosture::Mapped(claim) => {
            body.push(1);
            claim.name().encode_into(body);
        }
    }
}

pub(crate) fn write_activation(activation: ActivationDisposition, body: &mut Vec<u8>) {
    match activation {
        ActivationDisposition::Observed(reading) => {
            body.push(0);
            encode_bytes(reading.selection().surface().address().as_bytes(), body);
            reading.point().name().encode_into(body);
            encode_bytes(reading.selection().alternative().address().as_bytes(), body);
            encode_bytes(reading.witness().address().as_bytes(), body);
            body.extend_from_slice(&reading.firings().to_be_bytes());
        }
        ActivationDisposition::NotObserved => body.push(1),
        ActivationDisposition::UnobservableUnderBackend => body.push(2),
    }
}
