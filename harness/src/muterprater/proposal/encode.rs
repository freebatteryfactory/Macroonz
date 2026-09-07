//! The canonical identity encoding shared by proposal documents.

use super::{PROPOSAL_TAG, ProposalDestination};
use crate::descriptor::archive::{ArchivedCandidate, ArchivedName};
use crate::descriptor::{AdmissionGround, ProposalId, Row};
use crate::identity::ContentAddress;
use crate::report::encode_bytes;

/// The version of the proposal identity encoding.
const PROPOSAL_ENCODING_VERSION: u32 = 1;

/// The one road every proposal's identity is derived by, over the three readings the three of them share.
///
/// The candidate row's canonical bytes were written where that row was born, so this reads them rather than encoding a row a second time.
/// Written once rather than per proposal: three copies of one preimage agree until one is edited, and the specification is stated on [`super::ProposalDocument::identity`].
pub(super) fn proposal_identity(
    candidate: &Row,
    ground: AdmissionGround,
    destination: ProposalDestination,
) -> ProposalId {
    ProposalId::over(identity(
        candidate.canonical_bytes().as_bytes(),
        ground,
        |into| destination.suite().name().encode_into(into),
    ))
}

/// Derive historical proposal identity from admitted historical components.
pub(super) fn historical_identity(
    candidate: &ArchivedCandidate,
    ground: AdmissionGround,
    destination: &ArchivedName,
) -> ContentAddress {
    identity(candidate.canonical_bytes(), ground, |into| {
        destination.encode_into(into);
    })
}

fn identity(
    candidate: &[u8],
    ground: AdmissionGround,
    destination: impl FnOnce(&mut Vec<u8>),
) -> ContentAddress {
    let mut preimage = Vec::new();
    preimage.extend_from_slice(&PROPOSAL_ENCODING_VERSION.to_be_bytes());
    encode_bytes(candidate, &mut preimage);
    preimage.push(ground.slot());
    destination(&mut preimage);
    ContentAddress::derived(PROPOSAL_TAG, &preimage)
}
