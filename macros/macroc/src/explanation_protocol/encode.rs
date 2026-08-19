//! The canonical bytes one answered seat is, and the bytes a complete view's
//! identity is derived over.
//!
//! # What enters, and what does not
//!
//! Every member here is a TYPED value: a question's roster slot, an answer's own
//! discriminant, an identity at its full thirty-two bytes, a typed roster
//! length-framed, or a typed posture written as its own discriminant ahead of
//! whatever it carries.
//!
//! **No human prose enters.** The line a person reads is composed from the
//! answer at the moment it is asked for and is never stored — see
//! `explanation_protocol::project` — so a preimage carrying one would commit to
//! a rendering rather than to what was answered, and would rename every
//! explanation in the tree the day a sentence was reworded. The one seat where
//! prose sits beside a typed value is a repair, and the repair's CITATION is
//! written while the sentence it declares is not: the sentence is that
//! citation's own projection, and a projection is never a preimage member.
//!
//! # Whose spellings these are
//!
//! Nothing here invents a canonical form for a value another home owns. A cause
//! anchoring, a graph anchoring, a planned output, an invalidation trigger, a
//! decision trace, a projection disposition, and an owner-fact citation are all
//! written through the road their own home declares, so an explanation's
//! preimage and a plan's cannot part company about what one of those values is.
//!
//! # Ordering
//!
//! The seats are written in the KIND's declared question order, which the view
//! settles before it exists — never in the order a caller supplied them. A
//! preimage over caller order would derive two identities for one set of
//! answers.

use super::{ExplanationAnswer, ProjectionExplanation};
use crate::plane::{encode_bytes, encode_length};

impl ProjectionExplanation {
    /// Append this answered seat's canonical bytes: the question's roster slot,
    /// the answer's own discriminant, then the answer's typed material.
    ///
    /// The question rides ahead of the answer because the question is what was
    /// asked; the discriminant rides ahead of the material because two answer
    /// shapes carrying the same bytes are two answers.
    pub(super) fn encode_into(&self, into: &mut Vec<u8>) {
        into.push(self.question().slot());
        self.answer().encode_into(into);
    }
}

impl ExplanationAnswer {
    /// Append this answer's canonical bytes: its own discriminant, then the
    /// typed material it carries, length-framed.
    ///
    /// Every arm frames its material behind the discriminant, so an answer
    /// carrying nothing could never encode as one carrying bytes — and no two
    /// arms' materials can be cut at another boundary and produce one byte
    /// string.
    pub(super) fn encode_into(&self, into: &mut Vec<u8>) {
        into.push(self.slot());
        let mut material = Vec::new();
        self.material_into(&mut material);
        encode_bytes(&material, into);
    }

    /// The typed material one answer carries, written through each value's own
    /// declared spelling.
    ///
    /// Exhaustive over the roster on purpose: an answer added to
    /// [`ExplanationAnswer`] stops compiling HERE until somebody says what of it
    /// an explanation's identity commits to, so no answer can be admitted and
    /// left out of the preimage.
    fn material_into(&self, into: &mut Vec<u8>) {
        match self {
            Self::Kind { kind } => encode_bytes(kind.as_bytes(), into),
            Self::Owner { owner } => encode_bytes(&owner.citation_bytes(), into),
            Self::CausingDeclarations { sources } => sources.encode_into(into),
            Self::PatternInstance { pattern, instance } => {
                encode_bytes(pattern.as_bytes(), into);
                encode_bytes(instance.as_bytes(), into);
            }
            Self::GraphAndProfile {
                graph,
                profile,
                version,
            } => {
                graph.encode_into(into);
                encode_bytes(profile.as_bytes(), into);
                into.extend_from_slice(&version.position().to_be_bytes());
            }
            Self::SelectedWrappers { trace } => trace.encode_into(into),
            Self::AssumptionsAndSpecializations { assumptions } => {
                encode_length(assumptions.len(), into);
                for assumption in assumptions.iter() {
                    encode_bytes(&assumption.citation_bytes(), into);
                }
            }
            Self::OutputAndDigest { output, digest } => {
                output.encode_into(into);
                encode_bytes(digest.as_bytes(), into);
            }
            // The two descriptor rosters and the trace roster are written the
            // same way and are not one road: they stand under three different
            // magnitudes and answer three different questions, and the question
            // slot ahead of them is what tells two of them apart when their
            // members happen to coincide.
            Self::ChallengingTests { descriptors } | Self::MeasuringBenchmarks { descriptors } => {
                encode_length(descriptors.len(), into);
                for descriptor in descriptors.iter() {
                    encode_bytes(descriptor.as_bytes(), into);
                }
            }
            Self::CorrespondingRuntimeTraces { traces } => {
                encode_length(traces.len(), into);
                for trace in traces.iter() {
                    encode_bytes(trace.as_bytes(), into);
                }
            }
            Self::Invalidators { triggers } => {
                encode_length(triggers.len(), into);
                for trigger in triggers.iter() {
                    trigger.encode_into(into);
                }
            }
            Self::RelatedProjectionDisposition {
                related,
                disposition,
            } => {
                encode_bytes(related.as_bytes(), into);
                disposition.encode_into(into);
            }
            // The CITATION of every repair, and none of their sentences: a
            // repair's text is the projection of the fact it cites, and a
            // projection is never a preimage member.
            Self::Repairs { repairs } => {
                encode_length(repairs.len(), into);
                for repair in repairs.iter() {
                    encode_bytes(&repair.declared_by.citation_bytes(), into);
                }
            }
        }
    }
}

/// The canonical bytes one complete view's answered seats are, in the order the
/// view holds them — which is the kind's declared question order.
///
/// Private to this home, with one caller: the mint inside
/// [`ProjectionExplanationView::complete`](super::ProjectionExplanationView::complete).
/// A second road to these bytes would be a second road to an explanation
/// identity's preimage.
pub(super) fn answered_seats(answers: &[ProjectionExplanation], into: &mut Vec<u8>) {
    encode_length(answers.len(), into);
    for answer in answers {
        answer.encode_into(into);
    }
}
