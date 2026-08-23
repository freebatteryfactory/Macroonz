//! The structural comparison: what an artifact was read to declare, compared
//! against a declaration the caller states independently.
//!
//! # The question a byte scan cannot be asked
//!
//! A scan over bytes supports exactly one claim — *the rendered text contains
//! this exact declared textual form* — and no number of anchors moves a
//! structural question inside it: whether the artifact declares an
//! implementation at all, what that implementation targets, which contract it
//! realizes, whether an anchored constant is a MEMBER of it or merely bytes
//! sitting nearby, whether an item nobody planned came along, and whether the
//! same item was emitted twice. A scan that answered any of those would have to
//! decide what the text MEANS, and deciding that means implementing the
//! reader's own understanding of Rust — at which point the scan stops being
//! dumb, which was the property that made it worth trusting.
//!
//! # The parse is somebody else's
//!
//! The challenge-side host parses rendered Rust with `syn`, maps the result into
//! [`ArtifactStructure`], and hands that typed reading here. The parser shares
//! no capture, plan, renderer, token representation, or projection with the
//! producer. This production module therefore owns only the comparison and
//! names no parser dependency.
//!
//! # What this lane does NOT claim
//!
//! It reads syntax, and syntax is not meaning. It never claims that the
//! artifact TYPECHECKS, that the paths it spells resolve to anything, that the
//! trait it names exists, that the target type exists, that the implementation
//! is coherent, or that any constant evaluates to the value its spelling
//! suggests. A path the declaration did not name is read here as *a different
//! path* and never as *no such contract*. Every one of those is the compiled
//! read-back's, where a compiler parses by its own rules and hands back values.

use super::types::{
    ArtifactStructure, ConstantReading, DeclaredArtifact, DeclaredImplementation, DeclaredMember,
    ImplementationMember, ImplementationStructure, StructuralDisagreement, StructuralVerdict,
};

// ---------------------------------------------------------------------------
// The comparison.
// ---------------------------------------------------------------------------

/// Compare one reading against one declaration.
///
/// The pure half of the lane: typed values on both sides, no text, no parser.
///
/// # Bounds
///
/// It never states [`StructuralVerdict::Unparsable`]. That arm belongs to the
/// read, and a caller holding a reading is holding the proof a parse happened.
pub fn compared(
    structure: &ArtifactStructure,
    declared: &DeclaredArtifact<'_>,
) -> StructuralVerdict {
    match disagreement(structure, declared) {
        Some(found) => StructuralVerdict::Deviates(found),
        None => StructuralVerdict::Conforms,
    }
}

/// The first disagreement between one reading and one declaration.
///
/// The order is deliberate and coarse-to-fine: an artifact carrying an item
/// nobody planned is reported as that, not as whichever member the extra item
/// happened to disturb. Inside an implementation the same principle holds — how
/// the implementation is written, and whether it exists at all under some
/// `cfg`, are read before what its members say, because a member's value is
/// only interesting once the item carrying it is the declared item.
fn disagreement(
    structure: &ArtifactStructure,
    declared: &DeclaredArtifact<'_>,
) -> Option<StructuralDisagreement> {
    if structure.other_items > 0 {
        return Some(StructuralDisagreement::UnexpectedItem);
    }
    if let Some(at) = duplicated(&structure.implementations) {
        return Some(StructuralDisagreement::DuplicateImplementation { at });
    }
    if structure.implementations.len() != declared.implementations.len() {
        return Some(StructuralDisagreement::OutputCardinality {
            declared: declared.implementations.len(),
            read: structure.implementations.len(),
        });
    }
    structure
        .implementations
        .iter()
        .zip(declared.implementations.iter())
        .enumerate()
        .find_map(|(at, (read, expected))| implementation_disagreement(at, read, expected))
}

/// The first disagreement about one implementation.
fn implementation_disagreement(
    at: usize,
    read: &ImplementationStructure,
    expected: &DeclaredImplementation<'_>,
) -> Option<StructuralDisagreement> {
    if read.target != expected.target {
        return Some(StructuralDisagreement::ImplementationTarget { at });
    }
    if read.trait_path.as_deref() != expected.trait_path {
        return Some(StructuralDisagreement::TraitPath { at });
    }
    if read.postures.as_slice() != expected.postures {
        return Some(StructuralDisagreement::ImplPosture { at });
    }
    let carried = read
        .meaning_bearing_attributes
        .iter()
        .find(|spelled| !expected.attributes.contains(&spelled.as_str()));
    if let Some(attribute) = carried {
        return Some(StructuralDisagreement::MeaningBearingAttribute {
            at,
            attribute: attribute.clone(),
        });
    }
    member_disagreement(at, &read.members, expected.members)
}

/// The first disagreement among the members one implementation carries.
///
/// Three passes, coarse to fine: a member nobody declared, then a member
/// declared once and stated twice, then what the declared members say.
fn member_disagreement(
    at: usize,
    read: &[ImplementationMember],
    declared: &[DeclaredMember<'_>],
) -> Option<StructuralDisagreement> {
    if let Some(member) = undeclared_member(read, declared) {
        return Some(StructuralDisagreement::UnexpectedImplMember { at, member });
    }
    if let Some(member) = restated_member(read) {
        return Some(StructuralDisagreement::DuplicateMember { at, member });
    }
    declared
        .iter()
        .find_map(|expected| member_value_disagreement(at, read, expected))
}

/// The first member the declaration did not name, by its name or by what it is.
///
/// A member that is not an associated constant is one of these whatever it is
/// called: nothing an artifact renders lawfully carries a method, an associated
/// type, or a macro invocation, and a reader that stepped over them would have
/// a blind spot exactly the size of everything the declaration did not name.
fn undeclared_member(
    read: &[ImplementationMember],
    declared: &[DeclaredMember<'_>],
) -> Option<String> {
    for member in read {
        match member {
            ImplementationMember::Other { described } => return Some((*described).to_owned()),
            ImplementationMember::Constant { name, .. } => {
                if !declared
                    .iter()
                    .any(|expected| expected.name == name.as_str())
                {
                    return Some(name.clone());
                }
            }
        }
    }
    None
}

/// The first member stated more than once.
///
/// The second reading is a finding and never an overwrite of the first: a
/// reader that filed each named constant into one seat would write the copy
/// over the original and report nothing at all.
fn restated_member(read: &[ImplementationMember]) -> Option<String> {
    for (position, member) in read.iter().enumerate() {
        let ImplementationMember::Constant { name, .. } = member else {
            continue;
        };
        let restated = read
            .iter()
            .take(position)
            .any(|earlier| named_constant(earlier, name.as_str()).is_some());
        if restated {
            return Some(name.clone());
        }
    }
    None
}

/// The disagreement about one declared member: absent, unread, or wrong.
fn member_value_disagreement(
    at: usize,
    read: &[ImplementationMember],
    expected: &DeclaredMember<'_>,
) -> Option<StructuralDisagreement> {
    let stated = read
        .iter()
        .find_map(|member| named_constant(member, expected.name));
    let Some(member_reading) = stated else {
        return Some(StructuralDisagreement::MissingImplMember {
            at,
            member: expected.name.to_owned(),
        });
    };
    let Some(value) = member_reading.as_ref() else {
        return Some(StructuralDisagreement::MemberValueUnread {
            at,
            member: expected.name.to_owned(),
        });
    };
    if *value == expected.reading {
        return None;
    }
    Some(StructuralDisagreement::MemberValue {
        at,
        member: expected.name.to_owned(),
    })
}

/// The reading of one member, where that member is the associated constant of
/// this name.
fn named_constant<'read>(
    member: &'read ImplementationMember,
    name: &str,
) -> Option<&'read Option<ConstantReading>> {
    let ImplementationMember::Constant {
        name: stated,
        reading,
    } = member
    else {
        return None;
    };
    if stated.as_str() == name {
        Some(reading)
    } else {
        None
    }
}

/// Where one trait-and-target pair is implemented a second time.
fn duplicated(implementations: &[ImplementationStructure]) -> Option<usize> {
    for (at, found) in implementations.iter().enumerate() {
        let earlier = found.trait_path.is_some()
            && implementations
                .iter()
                .take(at)
                .any(|other| other.target == found.target && other.trait_path == found.trait_path);
        if earlier {
            return Some(at);
        }
    }
    None
}
