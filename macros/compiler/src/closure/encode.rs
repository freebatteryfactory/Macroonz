//! The canonical bytes the proof's transcript is taken over, and the bytes one closure issue is.
//!
//! Every posture byte rides ahead of the material it governs, and every variable-length member is framed through the identity home's one framing, so no two values can be cut at another boundary and produce one byte string.

use super::prove::{count_under, under};
use super::{CarriedTokens, ClosureIssue, PartitionCargo, PartitionedEmission};
use crate::identity::{encode_bytes, encode_length};
use crate::kind::{Destination, Role};
use crate::plan::Membership;
use crate::render::RenderedProjection;

/// The posture byte a delivery nothing was planned into carries.
const NOTHING_PLANNED: u8 = 0;

/// The posture byte a delivery carrying proved tokens carries.
const CARRIED: u8 = 1;

/// The posture byte a delivery that is never joined carries.
///
/// A third posture rather than the first: the publication delivery carrying artifacts and the test carrier carrying nothing are different facts about different deliveries.
const NOT_JOINED: u8 = 2;

/// The complete closure claim, as the bytes its identity is derived over.
///
/// Three members: the plan's whole declared membership in roster order, the roster's own length followed by what stood under each seat, and the joined deliveries.
///
/// A rendered unit is written as its own identity and its digest, and the rest of what it answers — the semantic key, the origin, the profile, the address — is not missing.
/// Those are the reconstruction's members, the reconstruction was proved equal to the membership written first, and one fact spelled twice in one preimage is how a preimage drifts.
pub(super) fn claim<R: Role>(
    planned: &Membership<R>,
    rendered: &RenderedProjection<R>,
    emission: &PartitionedEmission,
) -> Vec<u8> {
    let mut material = Vec::new();
    planned.encode_into(&mut material);
    encode_length(R::ALL.len(), &mut material);
    for role in R::ALL {
        material.extend_from_slice(&role.slot().to_be_bytes());
        encode_length(count_under(rendered, *role), &mut material);
        if let Some(unit) = under(rendered, *role) {
            encode_bytes(unit.identity().as_bytes(), &mut material);
            encode_bytes(unit.digest().as_bytes(), &mut material);
        }
    }
    emission.encode_into(&mut material);
    material
}

impl PartitionedEmission {
    /// Appends this emission's canonical bytes: every delivery of the roster, in roster order, each written as its declared name and then its cargo.
    ///
    /// The published artifacts are not written here and are not missing: the claim already commits to every rendered unit's identity and digest, and to the membership that names each unit's address, so an artifact written elsewhere is already a different closure.
    fn encode_into(&self, into: &mut Vec<u8>) {
        encode_length(Destination::ALL.len(), into);
        for destination in Destination::ALL {
            encode_bytes(destination.name().as_bytes(), into);
            if let Some(cargo) = self.joined(*destination) {
                cargo.encode_into(into);
            } else {
                into.push(NOT_JOINED);
                encode_bytes(&[], into);
            }
        }
    }
}

impl PartitionCargo {
    /// Appends this cargo's canonical bytes: the posture, then the digest where tokens are carried.
    ///
    /// The posture rides ahead of the material, so a delivery nothing was planned into never encodes as one that carries bytes.
    fn encode_into(&self, into: &mut Vec<u8>) {
        match self {
            Self::NothingPlanned => {
                into.push(NOTHING_PLANNED);
                encode_bytes(&[], into);
            }
            Self::Carried(carried) => {
                into.push(CARRIED);
                carried.encode_into(into);
            }
        }
    }
}

impl CarriedTokens {
    /// Appends these tokens' canonical bytes: the digest, at full width.
    ///
    /// The tokens themselves are not written and do not need to be: the digest is derived over them at full width, so a byte that changed changes the digest and therefore this encoding.
    fn encode_into(&self, into: &mut Vec<u8>) {
        encode_bytes(self.digest().as_bytes(), into);
    }
}

impl<R: Role> ClosureIssue<R> {
    /// This issue's canonical bytes on their own, for the related identity a diagnostic derives over it.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        self.encode_into(&mut bytes);
        bytes
    }

    /// Appends this issue's canonical bytes: the row's position in the declared roster, then the typed material that row carries, framed.
    ///
    /// Exhaustive over the roster on purpose: an issue added to [`ClosureIssue`] stops compiling HERE until somebody says what of it a preimage commits to.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        into.push(self.slot());
        let mut material = Vec::new();
        self.material_into(&mut material);
        encode_bytes(&material, into);
    }

    /// The typed material one issue carries, through each value's own declared spelling.
    fn material_into(&self, into: &mut Vec<u8>) {
        match self {
            Self::MemberMissing { role }
            | Self::MemberUnplanned { role }
            | Self::OriginOrphan { role }
            | Self::DigestMismatch { role }
            | Self::SemanticKeyMismatch { role }
            | Self::MaterializationMismatch { role }
            | Self::MembershipDisagreement { role }
            | Self::ArtifactAddressAbsent { role } => seat_into(*role, into),
            Self::MemberDuplicated { role, observed }
            | Self::MemberPlannedTwice { role, observed } => {
                seat_into(*role, into);
                into.extend_from_slice(&observed.to_be_bytes());
            }
            Self::ReconstructionEmpty => {}
            Self::ReconstructionUndeclarable { observed } => {
                into.extend_from_slice(&observed.to_be_bytes());
            }
            Self::JoinedTreeUnbounded { destination } => {
                encode_bytes(destination.name().as_bytes(), into);
            }
            Self::ArtifactAddressDoubled { role, address } => {
                seat_into(*role, into);
                encode_bytes(&address.citation_bytes(), into);
            }
        }
    }
}

/// Appends one seat's roster position, in two big-endian bytes.
fn seat_into<R: Role>(role: R, into: &mut Vec<u8>) {
    into.extend_from_slice(&role.slot().to_be_bytes());
}
