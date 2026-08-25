//! The canonical bytes a plan's transcript is taken over, and the bytes one planning issue is.
//!
//! Every row's discriminant rides ahead of the material it governs, and every variable-length member is framed through the identity home's one framing, so no two values can be cut at another boundary and produce one byte string.
//! Declared SETS are canonicalized here: each member is encoded, the ENCODINGS are sorted, and the sorted sequence is written.

use super::{
    Account, Context, ContradictionPair, DigestContract, InvalidationTrigger, Membership,
    PlanIssue, PlannedMember, PlannedOutput,
};
use crate::identity::{self, Identity, encode_bytes, encode_length};
use crate::kind::{Kind, Role};

/// Appends one captured commitment's canonical bytes, at full width.
fn capture_into(captured: &Identity<identity::CapturedDeclaration>, into: &mut Vec<u8>) {
    encode_bytes(captured.as_bytes(), into);
}

impl<K: Kind> Account<K> {
    /// The intent's canonical bytes on their own — the exact preimage [`Account::intent`](super::Account::intent) is derived over.
    ///
    /// Written from the pair and never read back off the identity: a preimage road that spelled the digest would hand back thirty-two bytes nobody can re-derive anything from, and the derivation it feeds would be defined in terms of its own output.
    #[must_use]
    pub fn intent_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        self.intent_into(&mut bytes);
        bytes
    }

    /// Appends this account's canonical bytes: the intent preimage, then the dependency set, canonicalized.
    ///
    /// The account's bytes therefore BEGIN with exactly the intent's preimage, through the same road [`Account::intent_bytes`] hands back, rather than through a second spelling that happens to agree today.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        self.intent_into(into);
        encode_set(self.dependencies().iter(), capture_into, into);
    }

    /// Appends the intent preimage: the owner-qualified kind, then the kind-specific content commitment at full width.
    fn intent_into(&self, into: &mut Vec<u8>) {
        encode_bytes(self.kind().as_bytes(), into);
        encode_bytes(self.content_commitment().as_bytes(), into);
    }
}

impl Context {
    /// Appends this context's canonical bytes: the profile, then the generator identity at full width.
    ///
    /// What a plan was planned OVER is not written here and is not missing: it is the account's fact, written ahead of this by the account's own road, so no byte of a plan transcript states the content twice.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        self.profile().encode_into(into);
        encode_bytes(self.generator().as_bytes(), into);
    }
}

impl InvalidationTrigger {
    /// Appends this trigger's canonical bytes: the row's discriminant, then what it watches.
    ///
    /// Two rows watching one thing never encode alike, because the discriminant rides ahead of the material.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        into.push(self.slot());
        match self {
            Self::CapturedDeclaration { watched } => encode_bytes(watched.as_bytes(), into),
            Self::Profile { watched } => watched.encode_into(into),
            Self::Generator { watched } => encode_bytes(watched.as_bytes(), into),
            Self::ProjectionContent { watched } => encode_bytes(watched.as_bytes(), into),
            Self::Declared { name, watched } => {
                encode_bytes(name.as_bytes(), into);
                encode_bytes(&watched.citation_bytes(), into);
            }
        }
    }
}

impl DigestContract {
    /// Appends this contract's canonical bytes: the member identity the digest must be anchored to.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        encode_bytes(self.anchored_to.as_bytes(), into);
    }
}

impl PlannedOutput {
    /// Appends this output's canonical bytes: the semantic key, the origin trail in walk order, the expected profile, the publication address where one is named, and the digest contract.
    ///
    /// Everything a plan states about one member, and no rendered byte, because a plan has none.
    /// The member's delivery is not written and is not missing: it is the seat's own answer, and the seat is written by the member's own road.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        encode_bytes(self.semantic_key.as_bytes(), into);
        self.origin.encode_into(into);
        self.expected_profile.encode_into(into);
        match self.address {
            None => {
                into.push(0);
                encode_bytes(&[], into);
            }
            Some(address) => {
                into.push(1);
                encode_bytes(&address.citation_bytes(), into);
            }
        }
        self.digest_contract.encode_into(into);
    }
}

impl<R: Role> PlannedMember<R> {
    /// Appends this member's canonical bytes: the seat's roster position in two big-endian bytes, then the output planned there.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        into.extend_from_slice(&self.role.slot().to_be_bytes());
        self.output.encode_into(into);
    }
}

impl<R: Role> Membership<R> {
    /// Appends this membership's canonical bytes, in the kind's declared ROSTER order.
    ///
    /// Roster order and never declaration order: a declared output set is order-insensitive, so the same members declared in another order must encode identically.
    /// Every member standing under a seat is written rather than only the first, so a membership that doubled a seat encodes differently from one that did not — that is a defect closure reports, and the encoding must not hide it before the check runs.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        encode_length(R::ALL.len(), into);
        for role in R::ALL {
            into.extend_from_slice(&role.slot().to_be_bytes());
            let under: Vec<&PlannedMember<R>> = self.members_under(*role).collect();
            encode_length(under.len(), into);
            for member in under {
                member.encode_into(into);
            }
        }
    }
}

impl ContradictionPair {
    /// Appends this pair's canonical bytes: the left citation, then the right.
    ///
    /// # Ordering
    ///
    /// Written in the order it is held and NOT canonicalized as a set: the two seats are named seats rather than members of a collection, so a spelling that sorted them would answer a question this type deliberately does not ask.
    fn encode_into(&self, into: &mut Vec<u8>) {
        encode_bytes(&self.left.citation_bytes(), into);
        encode_bytes(&self.right.citation_bytes(), into);
    }
}

impl PlanIssue {
    /// This issue's canonical bytes on their own, for the related identity a diagnostic derives over it.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        self.encode_into(&mut bytes);
        bytes
    }

    /// Appends this issue's canonical bytes: the row's position in the declared roster, then the typed material that row carries, framed.
    ///
    /// Exhaustive over the roster on purpose: an issue added to [`PlanIssue`] stops compiling HERE until somebody says what of it a preimage commits to, so no issue can be admitted and left out of every identity derived over a refusal that carries it.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        into.push(self.slot());
        let mut material = Vec::new();
        self.material_into(&mut material);
        encode_bytes(&material, into);
    }

    /// The typed material one issue carries, through each value's own declared spelling.
    fn material_into(&self, into: &mut Vec<u8>) {
        match self {
            Self::ContradictoryFacts { between } => between.encode_into(into),
            Self::UnknownKind { named } => encode_bytes(named.as_bytes(), into),
            Self::ProfileUnsupported { profile } => profile.encode_into(into),
            Self::BoundExceeded {
                axis,
                bound,
                observed,
            } => {
                into.push(axis.slot());
                into.extend_from_slice(&bound.to_be_bytes());
                into.extend_from_slice(&observed.to_be_bytes());
            }
            Self::MembershipIncomplete { absent } => encode_bytes(absent.as_bytes(), into),
            Self::OrphanGeneratedNode { node } => encode_bytes(node.as_bytes(), into),
            Self::MembershipDoubled {
                role_slot,
                observed,
            } => {
                into.extend_from_slice(&role_slot.to_be_bytes());
                into.extend_from_slice(&observed.to_be_bytes());
            }
            Self::TrailDiscontinuous { at } => into.extend_from_slice(&at.to_be_bytes()),
            Self::CauseSetUnwatchable { named, watchable } => {
                into.extend_from_slice(&named.to_be_bytes());
                into.extend_from_slice(&watchable.to_be_bytes());
            }
            Self::MembershipForeign { seat } | Self::AddressInert { seat } => {
                encode_bytes(seat.as_bytes(), into);
            }
        }
    }
}

/// Appends one declared SET's canonical bytes: every member encoded, the encodings sorted, the sorted sequence written behind its count.
///
/// # Ordering
///
/// Sorting the ENCODINGS rather than the members is what canonicalizes a set without an `Ord` this compiler declares for nobody: a byte order over finished encodings is a spelling rule for a collection whose order carries no meaning, not a ranking of the values.
pub(super) fn encode_set<'member, T: 'member, Encode>(
    members: impl Iterator<Item = &'member T>,
    encode: Encode,
    into: &mut Vec<u8>,
) where
    Encode: Fn(&T, &mut Vec<u8>),
{
    let mut encoded: Vec<Vec<u8>> = members
        .map(|member| {
            let mut bytes = Vec::new();
            encode(member, &mut bytes);
            bytes
        })
        .collect();
    encoded.sort_unstable();
    encode_length(encoded.len(), into);
    for member in &encoded {
        encode_bytes(member, into);
    }
}
