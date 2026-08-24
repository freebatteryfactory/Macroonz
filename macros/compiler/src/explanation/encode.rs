//! The canonical bytes one answered seat is, one coverage issue is, and one complete view's identity is derived over.
//!
//! Every member is a typed value: a question's roster position, an answer's own discriminant, an identity at full width, or a typed roster written behind its count.
//! **No human prose enters.** A rendered line is composed from an answer at the moment it is asked for, so a preimage carrying one would commit to a rendering rather than to what was answered, and would rename every explanation in the tree the day a sentence was reworded.
//! The one seat where prose sits beside a typed value is a repair, and only the repair's CITATION is written: the sentence is that citation's own projection.

use super::{ExplanationIssue, RelatedDisposition, UniversalAnswer};
use crate::identity::{encode_bytes, encode_length};
use crate::kind::{Answer, Disposition, Question};

/// Appends one disposition's canonical bytes: the row's discriminant, then the material that row carries.
fn disposition_into(disposition: Disposition, into: &mut Vec<u8>) {
    match disposition {
        Disposition::Generated { unit } => {
            into.push(0);
            encode_bytes(unit.as_bytes(), into);
        }
        Disposition::NotApplicable { because } => {
            into.push(1);
            encode_bytes(&because.citation_bytes(), into);
        }
        Disposition::NotRequested { because } => {
            into.push(2);
            encode_bytes(&because.citation_bytes(), into);
        }
        Disposition::UnavailableUnderProfile { profile, because } => {
            into.push(3);
            profile.encode_into(into);
            encode_bytes(&because.citation_bytes(), into);
        }
    }
}

/// Appends one accounted kind's canonical bytes: its declared name, then what happened to it.
fn related_into(related: &RelatedDisposition, into: &mut Vec<u8>) {
    encode_bytes(related.kind.as_bytes(), into);
    disposition_into(related.disposition, into);
}

/// The typed material one universal answer carries, written through each value's own declared spelling.
///
/// Exhaustive on purpose: an arm added to [`UniversalAnswer`](super::UniversalAnswer) stops compiling HERE until somebody says what of it an explanation's identity commits to, so no answer can be admitted and left out of the preimage.
pub(super) fn answer_material(answer: &UniversalAnswer, into: &mut Vec<u8>) {
    match answer {
        UniversalAnswer::Kind { name } => encode_bytes(name.as_bytes(), into),
        UniversalAnswer::Owner { owner } => encode_bytes(&owner.citation_bytes(), into),
        UniversalAnswer::CausingDeclarations {
            commitment,
            dependencies,
        } => {
            encode_bytes(commitment.as_bytes(), into);
            encode_length(dependencies.len(), into);
            for dependency in dependencies.iter() {
                encode_bytes(dependency.as_bytes(), into);
            }
        }
        UniversalAnswer::Profile { profile } => profile.encode_into(into),
        UniversalAnswer::OutputAndDigest { outputs } => {
            encode_length(outputs.len(), into);
            for row in outputs.iter() {
                row.output.encode_into(into);
                encode_bytes(row.digest.as_bytes(), into);
            }
        }
        UniversalAnswer::Assumptions { assumptions } => {
            encode_length(assumptions.len(), into);
            for assumption in assumptions.iter() {
                encode_bytes(&assumption.citation_bytes(), into);
            }
        }
        UniversalAnswer::Invalidators { triggers } => {
            encode_length(triggers.count(), into);
            for trigger in triggers.iter() {
                trigger.encode_into(into);
            }
        }
        UniversalAnswer::RelatedDispositions { related } => {
            encode_length(related.len(), into);
            for accounted in related.iter() {
                related_into(accounted, into);
            }
        }
        UniversalAnswer::Repairs { repairs } => {
            encode_length(repairs.len(), into);
            for repair in repairs.iter() {
                encode_bytes(&repair.declared_by.citation_bytes(), into);
            }
        }
    }
}

/// Appends one roster's answered seats: the count, then each seat as its question's roster position in two big-endian bytes followed by the answer's own canonical bytes.
///
/// The question rides ahead of the answer because the question is what was asked.
/// Each roster is written behind its own count, so the split between the universal seats and the kind's is framed rather than inferred — a universal seat and a declared seat may share a position, and one run of seats could otherwise be cut at another boundary.
pub(super) fn seats_into<A: Answer>(answers: &[A], into: &mut Vec<u8>) {
    encode_length(answers.len(), into);
    for answer in answers {
        into.extend_from_slice(&answer.question().slot().to_be_bytes());
        answer.encode_into(into);
    }
}

impl ExplanationIssue {
    /// This issue's canonical bytes on their own, for the related identity a diagnostic derives over it.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        self.encode_into(&mut bytes);
        bytes
    }

    /// Appends this issue's canonical bytes: the row's position in the declared roster, then the typed material that row carries, framed.
    ///
    /// Two rows carrying one question encode differently, because the row's position rides ahead of the material.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        into.push(self.slot());
        let mut material = Vec::new();
        self.material_into(&mut material);
        encode_bytes(&material, into);
    }

    /// The typed material one issue carries.
    fn material_into(&self, into: &mut Vec<u8>) {
        match self {
            Self::UniversalUnanswered { question } | Self::UniversalAnsweredTwice { question } => {
                into.extend_from_slice(&question.slot().to_be_bytes());
            }
            Self::DeclaredUnanswered { question, slot }
            | Self::DeclaredAnsweredTwice { question, slot } => {
                encode_bytes(question.as_bytes(), into);
                into.extend_from_slice(&slot.to_be_bytes());
            }
            Self::QuestionOutsideRoster { question } => encode_bytes(question.as_bytes(), into),
            Self::SeatBoundExceeded { bound, observed } => {
                into.extend_from_slice(&bound.to_be_bytes());
                into.extend_from_slice(&observed.to_be_bytes());
            }
        }
    }
}
