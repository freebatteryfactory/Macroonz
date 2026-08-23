//! The canonical bytes one planning issue is, and the bytes one planning
//! refusal body is.
//!
//! # Why this home writes them at all
//!
//! A planning refusal is carried by [`ProjectionDisposition::Refused`], and a
//! disposition enters an explanation's preimage. Without a spelling for the body
//! that arm could write nothing but its own posture, so two explanations
//! differing only in WHICH planning refusal they carry derived one identity —
//! two distinct accounts under one name. The seat that closes it is a canonical
//! encoding declared HERE, beside the roster, because the roster is this home's
//! and a spelling written at the disposition would be one home legislating
//! another's encoding.
//!
//! # What enters
//!
//! Every member is a TYPED value: an issue kind's roster slot, a closed
//! roster's own slot, an identity at its full thirty-two bytes, a declared
//! magnitude, a citation through its owner's own spelling, or a typed roster
//! length-framed. No human prose enters, and no projection of one: an issue's
//! rendered sentence is derived from the issue at the moment it is asked for,
//! and a preimage carrying one would commit to a rendering.
//!
//! # Ordering
//!
//! An issue's slot rides AHEAD of its material and the material is
//! length-framed, so two issue kinds never encode alike and no two materials can
//! be cut at another boundary and produce one byte string. The framing is the
//! plane's one framing ([`encode_bytes`]), never a second length spelling
//! invented here.
//!
//! [`ProjectionDisposition::Refused`]: crate::planning::ProjectionDisposition::Refused

use super::{ContradictionPair, ProjectionPlanning, ProjectionPlanningIssue};
use crate::plane::{encode_bytes, encode_length};

impl ContradictionPair {
    /// Append this pair's canonical bytes: the left citation, then the right,
    /// each through the owner-fact spelling its own home declares.
    ///
    /// # Ordering
    ///
    /// The pair is written in the order it is held and is NOT canonicalized as a
    /// set. The two seats are named seats rather than members of a collection —
    /// neither is elected as the offender and neither is interchangeable with
    /// the other — so a spelling that sorted them would be answering a question
    /// this type deliberately does not ask.
    pub(super) fn encode_into(&self, into: &mut Vec<u8>) {
        encode_bytes(&self.left.citation_bytes(), into);
        encode_bytes(&self.right.citation_bytes(), into);
    }
}

impl ProjectionPlanningIssue {
    /// Append this issue's canonical bytes: the kind's roster slot, then the
    /// typed material that kind carries, length-framed.
    ///
    /// Every arm frames its material behind the slot, so an issue carrying one
    /// identity never encodes as one carrying a count that happens to spell the
    /// same bytes.
    ///
    /// Exhaustive over the roster on purpose: an issue added to
    /// [`ProjectionPlanningIssue`] stops compiling HERE until somebody says what
    /// of it a preimage commits to, so no issue can be admitted and left out of
    /// every identity derived over a refusal that carries it.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        into.push(self.slot());
        let mut material = Vec::new();
        self.material_into(&mut material);
        encode_bytes(&material, into);
    }

    /// The typed material one issue carries, written through each value's own
    /// declared spelling.
    fn material_into(&self, into: &mut Vec<u8>) {
        match self {
            Self::ContradictoryOwnerFacts { between } => between.encode_into(into),
            Self::UnknownProjectionKind { named } => encode_bytes(named.as_bytes(), into),
            Self::ProfileUnsupported { profile, version } => {
                encode_bytes(profile.as_bytes(), into);
                into.extend_from_slice(&version.position().to_be_bytes());
            }
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
        }
    }
}

impl ProjectionPlanning {
    /// Append this refusal body's canonical bytes: how many issues it carries,
    /// then each one in the order the body holds them.
    ///
    /// # Ordering
    ///
    /// The carry's own order IS meaning and is written as held: a pass reports
    /// its findings in the order it established them, the first issue in
    /// particular survives at the front of a truncated body by law, and a
    /// spelling that sorted them would make two different reports one preimage.
    ///
    /// # Nonclaims
    ///
    /// The COVERAGE POSTURE is not written, and its absence is stated rather
    /// than folded. A [`CompletionPosture`](macroonz::CompletionPosture)
    /// is band 00's value and band 00 declares no canonical encoding for one, so
    /// a spelling here would be these services legislating inside a vocabulary
    /// they do not own — and a partial one, a discriminant without the count a
    /// truncation carries, would be a fold, which is exactly what a preimage may
    /// never carry. So two bodies carrying the same issues under two coverage
    /// postures encode alike, and what separates them reaches a reader through
    /// the refusal itself rather than through these bytes.
    ///
    /// The seat that closes it is a canonical encoding declared by band 00 on
    /// its own posture, beside the posture.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        let carried = self.body().carried();
        encode_length(carried.len(), into);
        for issue in carried.iter() {
            issue.encode_into(into);
        }
    }
}
