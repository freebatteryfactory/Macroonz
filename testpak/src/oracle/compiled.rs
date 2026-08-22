//! The compiled read-back: what an artifact actually DOES once a compiler has
//! accepted it, compared against what a caller declared it would do.
//!
//! # The claim, and why no other lane can make it
//!
//! A compiler parses the artifact by its own rules, with no anchor of ours
//! anywhere in the path, and hands back typed VALUES rather than substrings. So
//! this lane is where a claim about resolution lives: that the trait the
//! artifact names exists, that the target type exists, that the paths it spells
//! resolve, that the implementation is coherent, and that a constant evaluates
//! to the value its spelling suggested. The structural read reaches none of
//! that, because syntax is not meaning; the byte scan reaches less still.
//!
//! It is also where a refusal is evidence: an artifact a compiler REJECTS is a
//! reading nothing else in the annex can produce, and a declaration that says
//! the compiler must refuse is held to exactly that.
//!
//! # The execution is somewhere else
//!
//! Running a compiler is an effect, and this comparison module performs no such effect. What
//! stands here is the vocabulary and the comparison: a caller on the challenge
//! side compiles the artifact, reads the constants back as values, states the
//! observation as a [`CompiledObservation`], and hands it here beside the
//! declaration it wrote independently. Nothing in this file invokes anything.
//!
//! # No participant grades itself
//!
//! The artifact under judgement is one the producer rendered, and any damage in
//! it is the harness's own — so when a damaged artifact is handed to a
//! compiler, the compiler is not being asked to agree with the thing that
//! rendered it. That is what keeps this lane's agreement uncorrelated with the
//! renderer.

use super::types::{
    CompiledDisagreement, CompiledObservation, CompiledVerdict, DeclaredBehaviour,
    DeclaredReadBack, ObservedMember,
};

/// Compare one caller-stated compiled observation against its declared behavior.
///
/// **The claim this supports** is this lane's and only this lane's: the supplied observation agrees with the declared acceptance posture and member values. The effectful challenge owns evidence that a compiler actually produced the observation. This comparison says nothing about how the artifact is WRITTEN — two artifacts spelled differently can read back identically, and naming the difference is the structural read's.
pub fn compared(
    observed: &CompiledObservation,
    declared: &DeclaredBehaviour<'_>,
) -> CompiledVerdict {
    match (observed, declared) {
        (CompiledObservation::RefusedByCompiler, DeclaredBehaviour::RefusedByCompiler) => {
            CompiledVerdict::Conforms
        }
        (CompiledObservation::RefusedByCompiler, DeclaredBehaviour::ReadsBack(_)) => {
            CompiledVerdict::Deviates(CompiledDisagreement::RefusedWhereAcceptanceDeclared)
        }
        (CompiledObservation::ReadBack(_), DeclaredBehaviour::RefusedByCompiler) => {
            CompiledVerdict::Deviates(CompiledDisagreement::AcceptedWhereRefusalDeclared)
        }
        (CompiledObservation::ReadBack(read), DeclaredBehaviour::ReadsBack(expected)) => {
            match member_disagreement(read, expected) {
                Some(found) => CompiledVerdict::Deviates(found),
                None => CompiledVerdict::Conforms,
            }
        }
    }
}

/// The first disagreement among the members a compiled artifact handed back.
///
/// Three passes, coarse to fine: a member nobody declared, then a member handed
/// back twice, then what the declared members read back as.
fn member_disagreement(
    read: &[ObservedMember],
    declared: &[DeclaredReadBack<'_>],
) -> Option<CompiledDisagreement> {
    let undeclared = read
        .iter()
        .find(|member| !declared.iter().any(|expected| expected.name == member.name));
    if let Some(member) = undeclared {
        return Some(CompiledDisagreement::UnexpectedMember {
            member: member.name.clone(),
        });
    }
    if let Some(member) = restated_member(read) {
        return Some(CompiledDisagreement::DuplicateMember { member });
    }
    declared
        .iter()
        .find_map(|expected| member_value_disagreement(read, expected))
}

/// The first member handed back more than once.
///
/// A reader that filed each name into one seat would write the second value
/// over the first and report nothing at all, so the copy is a finding of its
/// own.
fn restated_member(read: &[ObservedMember]) -> Option<String> {
    for (position, member) in read.iter().enumerate() {
        let restated = read
            .iter()
            .take(position)
            .any(|earlier| earlier.name == member.name);
        if restated {
            return Some(member.name.clone());
        }
    }
    None
}

/// The disagreement about one declared member: absent, or reading back as
/// something else.
fn member_value_disagreement(
    read: &[ObservedMember],
    expected: &DeclaredReadBack<'_>,
) -> Option<CompiledDisagreement> {
    let Some(member) = read.iter().find(|member| member.name == expected.name) else {
        return Some(CompiledDisagreement::MissingMember {
            member: expected.name.to_owned(),
        });
    };
    if member.value == expected.value {
        return None;
    }
    Some(CompiledDisagreement::MemberValue {
        member: expected.name.to_owned(),
    })
}
