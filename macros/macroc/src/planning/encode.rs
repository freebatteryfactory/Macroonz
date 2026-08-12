//! The canonical bytes a plan's transcript is taken over.
//!
//! Every posture is written as a discriminant AHEAD of the material it governs,
//! so two postures naming the same identity never encode alike — target-free is
//! a stated posture and not an absent contract, and a plan decided against a
//! captured declaration alone never reads as one decided against a closed graph.
//!
//! Declared SETS are canonicalized here too: each member is encoded, the
//! ENCODINGS are sorted, and the sorted sequence is written. Sorting finished
//! encodings rather than values is what lets a set be canonical without an `Ord`
//! the plane refuses to declare — the plane ranks nothing, and a byte order over
//! encodings is a spelling rule rather than a ranking.

use super::{
    CauseAnchoring, DigestContract, GraphAnchoring, InvalidationTrigger, MemberDestination,
    PlannedMember, PlannedOutput, ProjectionContext, TargetBinding,
};
use crate::plane::{RenderedRole, encode_bytes, encode_length};

impl TargetBinding {
    /// Append this binding's canonical bytes: the posture's discriminant, then
    /// the contract where one is named. Target-free is written as a posture and
    /// never as an absent contract, exactly as the type states it.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        match self {
            Self::HostContract(contract) => {
                into.push(0);
                encode_bytes(contract.as_bytes(), into);
            }
            Self::TargetFree => {
                into.push(1);
                encode_bytes(&[], into);
            }
        }
    }
}

impl GraphAnchoring {
    /// Append this anchoring's canonical bytes: the posture's discriminant, then
    /// the identity it names.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        match self {
            Self::ClosedGraph(graph) => {
                into.push(0);
                encode_bytes(graph.as_bytes(), into);
            }
            Self::CapturedDeclarationOnly(captured) => {
                into.push(1);
                encode_bytes(captured.as_bytes(), into);
            }
        }
    }
}

impl CauseAnchoring {
    /// Append this cause's canonical bytes: the posture's discriminant, then
    /// every declaration it names, in the order the cause set was declared.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        match self {
            Self::Declarations(sources) => {
                into.push(0);
                encode_length(sources.len(), into);
                for source in sources.iter() {
                    encode_bytes(source.as_bytes(), into);
                }
            }
            Self::CapturedDeclaration(captured) => {
                into.push(1);
                encode_length(1, into);
                encode_bytes(captured.as_bytes(), into);
            }
        }
    }
}

impl ProjectionContext {
    /// Append this context's canonical bytes: what it was decided against, the
    /// profile and its version, what caused it, the generator identity, and the
    /// target binding, each at full width and in that order.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        self.graph.encode_into(into);
        encode_bytes(self.profile.as_bytes(), into);
        into.extend_from_slice(&self.profile_version.position().to_be_bytes());
        self.sources.encode_into(into);
        encode_bytes(self.generator.as_bytes(), into);
        self.target.encode_into(into);
    }
}

impl MemberDestination {
    /// Append this destination's canonical bytes: the discriminant, then the
    /// byte role where one is named.
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
    /// the digest contract — everything a plan states about one member, and no
    /// rendered byte, because a plan has none.
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

impl InvalidationTrigger {
    /// The trigger kind's discriminant byte, written ahead of the identity it
    /// watches so two kinds watching the same bytes never encode alike.
    #[must_use]
    pub const fn slot(&self) -> u8 {
        match self {
            Self::SourceDeclarationChanged { .. } => 0,
            Self::CapturedDeclarationChanged { .. } => 1,
            Self::GraphIdentityChanged { .. } => 2,
            Self::ProjectionProfileChanged { .. } => 3,
            Self::TargetContractChanged { .. } => 4,
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
            Self::SourceDeclarationChanged { watched } => watched.as_bytes(),
            Self::CapturedDeclarationChanged { watched } => watched.as_bytes(),
            Self::GraphIdentityChanged { watched } => watched.as_bytes(),
            Self::ProjectionProfileChanged { watched } => watched.as_bytes(),
            Self::TargetContractChanged { watched } => watched.as_bytes(),
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
