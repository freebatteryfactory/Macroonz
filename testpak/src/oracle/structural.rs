//! The structural read: what an artifact DECLARES, recovered from a parse
//! nobody here wrote, and compared against a declaration the caller states
//! independently.
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
//! # So the parse is somebody else's
//!
//! This lane hands the text to `syn` and reads the tree it hands back. `syn`
//! shares nothing with the producer: not the capture, not the plan, not the
//! renderer, not the token type the renderer writes, not the projection that
//! turns that token tree into text. It decides where an item begins, what a
//! path is, and which constants are members of which implementation, by Rust's
//! rules rather than by ours. Two readers written by the same hands against the
//! same document agree because they share the challenged understanding;
//! agreement with a parser nobody here wrote is not correlated with the
//! renderer at all.
//!
//! # The decoder is a lodger, and the comparison is the resident
//!
//! [`declarations_in`] is the only road in this package that names `syn`, and
//! its home is the challenge side: structural decoding of this repository's own
//! rendered artifacts is qualification-era work, while the annex owns
//! vocabulary and comparison. It stands here, isolated in this one file, until
//! a challenge-side caller exists.
//!
//! The opening condition is exact, and the manifest carries the same sentence:
//! the move happens when a challenge-side caller exists to call it, and that
//! same move retires `syn` from the library's dependency table. Relocating the
//! decoder ahead of its caller would be unreachable code wearing the shape of
//! a move, so the manifest states what the code does today rather than what it
//! is owed.
//!
//! [`compared`] is what survives that move: it takes typed values on both
//! sides, reads no text, and names no parser.
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
    ImplPosture, ImplementationMember, ImplementationStructure, StructuralDisagreement,
    StructuralVerdict,
};

// ---------------------------------------------------------------------------
// The verdicts.
// ---------------------------------------------------------------------------

/// Read one rendered artifact and state a verdict over what it declares.
///
/// **The claim this supports** is this lane's and only this lane's: the
/// artifact DECLARES these implementations, of these traits, for these targets,
/// written this way, carrying these members and no others. It says nothing
/// about whether any of it compiles.
pub fn verdict(rendered: &str, declared: &DeclaredArtifact<'_>) -> StructuralVerdict {
    let Some(structure) = declarations_in(rendered) else {
        return StructuralVerdict::Unparsable;
    };
    compared(&structure, declared)
}

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
    if read.trait_path != expected.trait_path {
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
        let earlier = implementations
            .iter()
            .take(at)
            .any(|other| other.target == found.target && other.trait_path == found.trait_path);
        if earlier {
            return Some(at);
        }
    }
    None
}

// ---------------------------------------------------------------------------
// The decode. Everything below this line names `syn`, and nothing above it
// does.
// ---------------------------------------------------------------------------

/// Read what one rendered artifact declares, or `None` where the text is not
/// parseable Rust.
///
/// `None` is a real answer and never "nothing was wrong": a caller that treated
/// it as conformance would be reporting a verdict about a tree that was never
/// built. [`verdict`] turns it into [`StructuralVerdict::Unparsable`], which is
/// a failure class of its own.
#[must_use]
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "`syn::Item` is non_exhaustive, so no crate outside syn can enumerate its variants; a wildcard is the only arm that closes this match, and every item it catches is one this reading counts rather than reads"
)]
pub fn declarations_in(rendered: &str) -> Option<ArtifactStructure> {
    let file = syn::parse_file(rendered).ok()?;
    let mut implementations: Vec<ImplementationStructure> = Vec::new();
    let mut other_items = 0usize;
    for item in &file.items {
        match item {
            syn::Item::Impl(declared) => match implementation_structure(declared) {
                Some(structure) => implementations.push(structure),
                None => other_items = other_items.saturating_add(1),
            },
            _ => other_items = other_items.saturating_add(1),
        }
    }
    Some(ArtifactStructure {
        implementations,
        other_items,
    })
}

/// One trait implementation, read out of the tree, or `None` where the item is
/// an implementation of something this lane cannot name — an inherent
/// implementation, or one whose target is not a plain path.
fn implementation_structure(declared: &syn::ItemImpl) -> Option<ImplementationStructure> {
    let (path, _) = declared.trait_.as_ref()?;
    let target = type_path(&declared.self_ty)?;
    let mut meaning_bearing_attributes = meaning_bearing(&declared.attrs);
    let mut members: Vec<ImplementationMember> = Vec::new();
    for member in &declared.items {
        let (attributes, read) = member_reading(member);
        meaning_bearing_attributes.extend(meaning_bearing(attributes));
        members.push(read);
    }
    Some(ImplementationStructure {
        target,
        trait_path: path_spelling(path),
        postures: postures_of(declared),
        meaning_bearing_attributes,
        members,
    })
}

/// The postures one implementation is written under, in roster order.
fn postures_of(declared: &syn::ItemImpl) -> Vec<ImplPosture> {
    let mut carried: Vec<ImplPosture> = Vec::new();
    if declared.unsafety.is_some() {
        carried.push(ImplPosture::Unsafely);
    }
    if declared.modifiers.polarity.is_some() {
        carried.push(ImplPosture::Negative);
    }
    if declared.modifiers.defaultness.is_some() {
        carried.push(ImplPosture::Defaulted);
    }
    if !declared.generics.params.is_empty() || declared.generics.where_clause.is_some() {
        carried.push(ImplPosture::Generic);
    }
    carried
}

/// One member's attributes, and what the member is.
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "`syn::ImplItem` is non_exhaustive, so no crate outside syn can enumerate its variants; a wildcard is the only arm that closes this match, and every member it catches is by definition not an associated constant"
)]
fn member_reading(member: &syn::ImplItem) -> (&[syn::Attribute], ImplementationMember) {
    match member {
        syn::ImplItem::Const(constant) => (
            &constant.attrs,
            ImplementationMember::Constant {
                name: constant.ident.to_string(),
                reading: constant_reading(&constant.expr),
            },
        ),
        syn::ImplItem::Fn(method) => (
            &method.attrs,
            ImplementationMember::Other {
                described: "an associated function",
            },
        ),
        syn::ImplItem::Type(associated) => (
            &associated.attrs,
            ImplementationMember::Other {
                described: "an associated type",
            },
        ),
        syn::ImplItem::Macro(invocation) => (
            &invocation.attrs,
            ImplementationMember::Other {
                described: "a macro invocation in member position",
            },
        ),
        _ => (
            &[],
            ImplementationMember::Other {
                described: "a member this lane cannot name",
            },
        ),
    }
}

/// Every attribute that decides something, by path.
///
/// A doc comment reaches the tree as `#[doc = "…"]` and decides nothing, so it
/// is not carried. Everything else is: `cfg` decides whether the implementation
/// exists at all, and an attribute nobody planned is a finding whatever it
/// does.
fn meaning_bearing(attributes: &[syn::Attribute]) -> Vec<String> {
    attributes
        .iter()
        .map(|attribute| path_spelling(attribute.path()))
        .filter(|spelled| spelled != "doc")
        .collect()
}

/// What one associated constant's value expression says.
///
/// Shallow and dumb on purpose: each shape is read as the shape it is, and a
/// value of any other shape is not read at all rather than flattened into a
/// shape this lane happens to name.
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "`syn::Expr` is non_exhaustive, so no crate outside syn can enumerate its variants; a wildcard is the only arm that closes this match, and every expression it catches is one this reading deliberately does not name"
)]
fn constant_reading(expression: &syn::Expr) -> Option<ConstantReading> {
    match expression {
        syn::Expr::Path(spelled) => Some(ConstantReading::Path(path_spelling(&spelled.path))),
        syn::Expr::Lit(literal) => literal_reading(&literal.lit),
        syn::Expr::Reference(reference) => borrowed_reading(reference.expr.as_ref()),
        syn::Expr::Array(array) => elements(array.elems.iter()).map(ConstantReading::Array),
        syn::Expr::Call(call) => call_reading(call),
        _ => None,
    }
}

/// What one literal says, where it is one of the three literal kinds this lane
/// names.
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "`syn::Lit` is non_exhaustive, so no crate outside syn can enumerate its variants; a wildcard is the only arm that closes this match, and every literal it catches is one this reading deliberately does not name"
)]
fn literal_reading(literal: &syn::Lit) -> Option<ConstantReading> {
    match literal {
        syn::Lit::Str(text) => Some(ConstantReading::Text(text.value())),
        syn::Lit::Int(number) => Some(ConstantReading::Number(number.base10_digits().to_owned())),
        syn::Lit::Bool(truth) => Some(ConstantReading::Truth(truth.value)),
        _ => None,
    }
}

/// What one borrowed expression says, where it borrows an array.
///
/// A borrow of anything else is not read: `&[…]` and `[…]` are different
/// declarations, and so are `&X` and `X`.
fn borrowed_reading(inner: &syn::Expr) -> Option<ConstantReading> {
    let syn::Expr::Array(array) = inner else {
        return None;
    };
    elements(array.elems.iter()).map(ConstantReading::BorrowedArray)
}

/// What one call says: the path it calls, and the readings of its arguments.
///
/// A call whose callee is not a plain path is not read, because this lane names
/// paths and nothing else.
fn call_reading(call: &syn::ExprCall) -> Option<ConstantReading> {
    let syn::Expr::Path(spelled) = call.func.as_ref() else {
        return None;
    };
    Some(ConstantReading::Call {
        path: path_spelling(&spelled.path),
        arguments: elements(call.args.iter())?,
    })
}

/// The readings of several expressions, in order, or `None` where any one of
/// them is of a shape this lane does not name.
///
/// One unread element makes the whole reading unread. A collection that
/// silently dropped the element it could not name would compare equal to a
/// shorter declaration.
fn elements<'read>(
    expressions: impl Iterator<Item = &'read syn::Expr>,
) -> Option<Vec<ConstantReading>> {
    expressions.map(constant_reading).collect()
}

/// One path, spelled back with its segments and its leading `::`.
fn path_spelling(path: &syn::Path) -> String {
    let mut spelled = String::new();
    if path.leading_colon.is_some() {
        spelled.push_str("::");
    }
    for (position, segment) in path.segments.iter().enumerate() {
        if position > 0 {
            spelled.push_str("::");
        }
        spelled.push_str(&segment.ident.to_string());
    }
    spelled
}

/// The path one type spells, or `None` where the type is not a plain path.
fn type_path(declared: &syn::Type) -> Option<String> {
    let syn::Type::Path(typed) = declared else {
        return None;
    };
    Some(path_spelling(&typed.path))
}
