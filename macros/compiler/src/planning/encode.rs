//! The canonical bytes a plan's transcript is taken over.
//!
//! Every posture is written as a discriminant ahead of the material it governs.
//!
//! Declared SETS are canonicalized here too: each member is encoded, the
//! ENCODINGS are sorted, and the sorted sequence is written.

use super::{
    CauseAnchoring, ContentAddressing, DigestContract, InvalidationTrigger, MemberDestination,
    OwnerContentAccount, PlannedMember, PlannedOutput, ProjectionContext, ProjectionDisposition,
    ProjectionKind,
};
use crate::plane::{
    CapturedDeclarationSubject, ProjectionIdentity, RenderedRole, encode_bytes, encode_length,
};

impl CauseAnchoring {
    /// Append this commitment's canonical bytes: the posture's discriminant,
    /// then the one address it names, at full width.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        match self {
            Self::CapturedDeclaration(captured) => {
                into.push(1);
                encode_bytes(captured.as_bytes(), into);
            }
        }
    }
}

/// Append one captured commitment's canonical bytes, on the same terms.
fn encode_capture(captured: &ProjectionIdentity<CapturedDeclarationSubject>, into: &mut Vec<u8>) {
    encode_bytes(captured.as_bytes(), into);
}

impl ContentAddressing {
    /// Append this addressing's canonical bytes: the posture's discriminant, the
    /// content's own commitment at full width, then the dependency set,
    /// canonicalized.
    ///
    /// # Ordering
    ///
    /// The dependency set is a SET: the members are encoded, the ENCODINGS are
    /// sorted, and the sorted sequence is written, so the same commitments
    /// declared in another order produce the same bytes and therefore the same
    /// plan identity.
    /// The posture rides ahead of both seats, so a linked account and a captured
    /// one never encode alike even where their thirty-two bytes coincide.
    ///
    /// The posture and the commitment are written by the COMMITMENT's own road
    /// rather than spelled again here, and the dependency set by the road below,
    /// so the account's own encoding reaches both halves through exactly these
    /// spellings instead of restating either.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        self.commitment().encode_into(into);
        self.encode_dependencies_into(into);
    }

    /// Append the dependency set alone, canonicalized — everything this
    /// addressing writes AFTER the commitment.
    ///
    /// Split out rather than spelled twice, because an account's own encoding
    /// opens with the INTENT PREIMAGE — which writes the same commitment through
    /// the same road — and continues here.
    /// One spelling of the dependency set, whichever road reaches it, so the
    /// account's bytes and the addressing's own cannot part company.
    fn encode_dependencies_into(&self, into: &mut Vec<u8>) {
        match self {
            Self::Captured { dependencies, .. } => {
                encode_set(dependencies.iter(), encode_capture, into);
            }
        }
    }
}

impl<K: ProjectionKind> OwnerContentAccount<K> {
    /// Append this account's canonical bytes: the intent preimage — the kind's
    /// declared name and the content's own commitment — then the dependency set
    /// the addressing declares.
    ///
    /// The account's bytes therefore BEGIN with exactly the intent's preimage,
    /// through the same road [`OwnerContentAccount::intent_bytes`] hands back,
    /// rather than through a second spelling that happens to agree today.
    /// A plan transcript's first member is the intent, widened by the dependency
    /// set; it is not a second spelling of either.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        self.intent_preimage_into(into);
        self.addressing().encode_dependencies_into(into);
    }

    /// Append the intent layer's canonical preimage: the kind's declared name,
    /// then the owner content commitment at full width.
    ///
    /// The ONE spelling of the pair, and both readers take it — the account's
    /// own encoding opens with it, and [`OwnerContentAccount::intent_bytes`] is
    /// it.
    fn intent_preimage_into(&self, into: &mut Vec<u8>) {
        encode_bytes(K::KIND_NAME.as_bytes(), into);
        self.commitment().encode_into(into);
    }

    /// The intent's canonical bytes on their own — the exact preimage
    /// [`OwnerContentAccount::intent`] derives the intent identity over.
    ///
    /// Written from the PAIR and never read back off the identity.
    /// A preimage road that spelled the digest would hand back thirty-two bytes
    /// nobody can re-derive anything from, and the derivation it feeds would
    /// then be defined in terms of its own output.
    #[must_use]
    pub fn intent_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        self.intent_preimage_into(&mut bytes);
        bytes
    }
}

impl ProjectionContext {
    /// Append this context's canonical bytes: the profile, its version, and the generator identity, each at full width and in that order.
    ///
    /// What the plan was planned OVER is not written here and is not missing: it
    /// is the entry account's fact, written ahead of this by the account's own
    /// road, so no byte of a plan transcript states the content twice.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        encode_bytes(self.profile.as_bytes(), into);
        into.extend_from_slice(&self.profile_version.position().to_be_bytes());
        encode_bytes(self.generator.as_bytes(), into);
    }
}

impl MemberDestination {
    /// Append this destination's canonical bytes: the discriminant, then the
    /// byte role where one is named.
    ///
    /// # Ordering
    ///
    /// A discriminant is APPENDED and never reused. The two deliveries written
    /// first keep the positions they were written under, so every plan spellable
    /// before the carrier arms encodes byte for byte as it did, and a carrier
    /// can never encode as the artifact it was numbered after. Reusing a
    /// position would make two different deliveries one preimage, which is the
    /// one thing a discriminant ahead of its material exists to prevent.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        match self {
            Self::AtDeclarationSite => {
                into.push(0);
                encode_bytes(&[], into);
            }
            Self::AsArtifact { byte_role } => {
                into.push(1);
                encode_bytes(byte_role.as_bytes(), into);
            }
            Self::IntoTestCarrier => {
                into.push(2);
                encode_bytes(&[], into);
            }
            Self::IntoBenchCarrier => {
                into.push(3);
                encode_bytes(&[], into);
            }
        }
    }
}

impl DigestContract {
    /// Append this contract's canonical bytes: the role slot the digest will
    /// carry, then the member identity it must be anchored to.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        into.push(self.role.slot());
        encode_bytes(self.anchored_to.as_bytes(), into);
    }
}

impl PlannedOutput {
    /// Append this output's canonical bytes: the semantic key, the destination,
    /// the origin trail in walk order, the expected profile and its version, and
    /// the digest contract.
    /// Everything a plan states about one member, and no rendered byte, because a
    /// plan has none.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        encode_bytes(self.semantic_key.as_bytes(), into);
        self.destination.encode_into(into);
        self.origin.encode_into(into);
        encode_bytes(self.expected_profile.as_bytes(), into);
        into.extend_from_slice(&self.expected_profile_version.position().to_be_bytes());
        self.digest_contract.encode_into(into);
    }
}

impl<R: RenderedRole> PlannedMember<R> {
    /// Append this member's canonical bytes: the rendered role's slot, then the
    /// logical output.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        into.extend_from_slice(&self.role.slot().to_be_bytes());
        self.output.encode_into(into);
    }
}

impl ProjectionDisposition {
    /// The posture's discriminant byte, written ahead of whatever it carries so
    /// two postures naming the same identity never encode alike.
    ///
    /// A position is APPENDED and never renumbered: renumbering an occupied
    /// posture re-encodes values that were already encoded, which renames every
    /// identity derived over them.
    #[must_use]
    pub const fn slot(&self) -> u8 {
        match self {
            Self::Generated { .. } => 0,
            Self::NotApplicable { .. } => 1,
            Self::Refused { .. } => 2,
            Self::UnavailableUnderProfile { .. } => 3,
            Self::NotRequested => 4,
        }
    }

    /// Append this disposition's canonical bytes: the posture's discriminant,
    /// then the typed material that posture carries.
    ///
    /// Every arm writes its material length-framed behind the discriminant, so a
    /// posture that carries nothing never encodes as one that carries bytes.
    ///
    /// # Whose spelling the refused arm writes
    ///
    /// The REFUSED arm writes the body it carries, through the encoding the
    /// REFUSAL home declares beside its own issue roster
    /// ([`ProjectionPlanning::encode_into`]). This home spells no part of it: a
    /// planning refusal is another home's value, and a spelling written here
    /// would be a second answer to what a planning issue IS.
    ///
    /// That arm used to write its posture and no material at all, and the cost
    /// was exact: two dispositions refused with two different bodies encoded
    /// alike, so two explanations differing only in which planning refusal they
    /// carried derived one identity — two accounts under one name. What closed
    /// it is the encoding beside the roster, and this arm consuming it.
    ///
    /// # What the profile-unavailable arm writes
    ///
    /// The profile, its version, and then the CITATION naming what that profile
    /// could not furnish, through the owner-fact spelling the plane declares —
    /// the same road the not-applicable arm writes its citation through, so the
    /// two arms cannot part company about what a citation IS.
    ///
    /// The citation is APPENDED behind the version rather than written ahead of
    /// the profile. The two members that were already written keep their
    /// positions, so what moved is the arm's LENGTH rather than its layout: a
    /// reader holding the earlier grammar reads the profile and the version
    /// correctly and then runs out of grammar, which is a widening and not a
    /// re-encoding of what was already spelled.
    ///
    /// It is a widening all the same, and it is not free. This arm's bytes reach
    /// an explanation's identity through the related-projection seat, so the
    /// explanation preimage grammar widened with it — see
    /// [`EXPLANATION_IDENTITY_PROFILE`], whose version moved to 2 for exactly
    /// this member and states so at its own position.
    ///
    /// # Nonclaims
    ///
    /// What that encoding leaves out is its own statement rather than this
    /// home's: the coverage posture a collection-shaped body carries is band
    /// 00's value, band 00 declares no canonical form for one, and the refusal
    /// home writes none. So two bodies carrying the same issues under two
    /// coverage postures still encode alike.
    ///
    /// [`ProjectionPlanning::encode_into`]: crate::refusal::ProjectionPlanning::encode_into
    /// [`EXPLANATION_IDENTITY_PROFILE`]: crate::plane::EXPLANATION_IDENTITY_PROFILE
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        into.push(self.slot());
        match self {
            Self::Generated { output } => {
                let mut material = Vec::new();
                output.encode_into(&mut material);
                encode_bytes(&material, into);
            }
            Self::NotApplicable { because } => encode_bytes(&because.citation_bytes(), into),
            Self::Refused { refusal } => {
                let mut material = Vec::new();
                refusal.encode_into(&mut material);
                encode_bytes(&material, into);
            }
            Self::NotRequested => encode_bytes(&[], into),
            Self::UnavailableUnderProfile {
                profile,
                version,
                because,
            } => {
                encode_bytes(profile.as_bytes(), into);
                into.extend_from_slice(&version.position().to_be_bytes());
                encode_bytes(&because.citation_bytes(), into);
            }
        }
    }
}

impl InvalidationTrigger {
    /// The trigger kind's discriminant byte, written ahead of the identity it
    /// watches so two kinds watching the same bytes never encode alike.
    #[must_use]
    pub const fn slot(&self) -> u8 {
        match self {
            Self::CapturedDeclarationChanged { .. } => 1,
            Self::ProjectionProfileChanged { .. } => 3,
            Self::GeneratorVersionChanged { .. } => 5,
            Self::MechanismProfileChanged { .. } => 6,
            Self::WorkFormulaChanged { .. } => 7,
            Self::FixturePopulationChanged { .. } => 8,
        }
    }

    /// Append this trigger's canonical bytes: the kind, then the watched
    /// identity at full width.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        into.push(self.slot());
        let watched: &[u8; 32] = match self {
            Self::CapturedDeclarationChanged { watched } => watched.as_bytes(),
            Self::ProjectionProfileChanged { watched } => watched.as_bytes(),
            Self::GeneratorVersionChanged { watched } => watched.as_bytes(),
            Self::MechanismProfileChanged { watched } => watched.as_bytes(),
            Self::WorkFormulaChanged { watched } => watched.as_bytes(),
            Self::FixturePopulationChanged { watched } => watched.as_bytes(),
        };
        encode_bytes(watched, into);
    }
}

/// Append one declared SET's canonical bytes: every member encoded, the
/// encodings sorted, the sorted sequence written length-prefixed.
///
/// # Ordering
///
/// Sorting the ENCODINGS rather than the members is what lets a set be
/// canonicalized without an `Ord` the plane refuses to declare: the plane ranks
/// nothing, and a byte order over finished encodings is not a ranking of the
/// values — it is a spelling rule for a collection whose order carries no
/// meaning.
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
