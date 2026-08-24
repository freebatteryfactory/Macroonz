//! The compiled read-back: what an artifact does once a compiler has accepted it, against what a caller declared it would do.
//!
//! # The claim no other lane can make
//!
//! A compiler parses the artifact by its own rules, with no anchor of ours anywhere in the path, and hands back typed values rather than substrings.
//! So this is where a claim about resolution lives: that the trait the artifact names exists, that the target type exists, that the paths it spells resolve, that the implementation is coherent, and that a constant evaluates to the value its spelling suggested.
//! It is also where a refusal is evidence — an artifact a compiler rejects is a reading nothing else here can produce, and a declaration that says the compiler must refuse is held to exactly that.
//!
//! # The execution is somewhere else
//!
//! Running a compiler is an effect, and nothing in this file invokes anything.
//! A caller compiles the artifact, reads the constants back as values, states what it saw as a [`CompiledObservation`], and hands that here beside the declaration it wrote independently.
//!
//! # No participant grades itself
//!
//! The artifact under judgement is one a producer rendered, and any damage in it was applied by the harness, so a compiler handed a damaged artifact is not being asked to agree with the thing that rendered it.

use super::types::{
    CompiledDisagreement, CompiledObservation, CompiledVerdict, DeclaredBehavior, DeclaredReadBack,
    ObservedMember,
};

/// Compare one caller-stated compiled observation against its declared behavior.
///
/// The claim it supports is this lane's and only this lane's: the supplied observation agrees with the declared acceptance posture and member values.
/// Evidence that a compiler actually produced the observation belongs to the caller that ran one.
/// It says nothing about how the artifact is written — two artifacts spelled differently can read back identically, and naming that difference is the structural read's.
pub fn compared(
    observed: &CompiledObservation,
    declared: &DeclaredBehavior<'_>,
) -> CompiledVerdict {
    match (observed, declared) {
        (CompiledObservation::RefusedByCompiler, DeclaredBehavior::RefusedByCompiler) => {
            CompiledVerdict::Conforms
        }
        (CompiledObservation::RefusedByCompiler, DeclaredBehavior::ReadsBack(_)) => {
            CompiledVerdict::Deviates(CompiledDisagreement::RefusedWhereAcceptanceDeclared)
        }
        (CompiledObservation::ReadBack(_), DeclaredBehavior::RefusedByCompiler) => {
            CompiledVerdict::Deviates(CompiledDisagreement::AcceptedWhereRefusalDeclared)
        }
        (CompiledObservation::ReadBack(read), DeclaredBehavior::ReadsBack(expected)) => {
            match member_disagreement(read, expected) {
                Some(found) => CompiledVerdict::Deviates(found),
                None => CompiledVerdict::Conforms,
            }
        }
    }
}

/// The first disagreement among the members a compiled artifact handed back.
///
/// Three passes, coarse to fine: a member nobody declared, then a member handed back twice, then what the declared members read back as.
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
/// A reader that filed each name into one seat would write the second value over the first and report nothing at all, so the copy is a finding of its own.
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

/// The disagreement about one declared member: absent, or reading back as something else.
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
