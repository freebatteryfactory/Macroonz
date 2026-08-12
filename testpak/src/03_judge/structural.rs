//! Lane B — the structural read: what the artifact DECLARES, recovered from an
//! independent parse of the rendered text.
//!
//! # The question lane A cannot be asked
//!
//! Lane A's claim is exactly *the rendered text contains this exact declared
//! textual form*. Every structural question is outside it, and no number of
//! anchors moves it inside: whether the artifact declares an implementation at
//! all, what that implementation targets, which contract it realizes, whether
//! the anchored constant is a MEMBER of it or merely bytes sitting nearby,
//! whether an item nobody planned came along, and whether the same item was
//! emitted twice. A byte scan that answered any of those would have to decide
//! what the text MEANS, and deciding that means implementing the reader's own
//! understanding of Rust — at which point the scan stops being dumb, which was
//! the property that made it worth trusting.
//!
//! # So the parse is somebody else's
//!
//! This lane answers those questions by handing the text to `syn` and reading
//! the tree it hands back. `syn` shares nothing with the producer: not the
//! capture, not the plan, not the renderer, not the token type the renderer
//! writes, not the projection that turns that token tree into text. It is a
//! parser written by people who never saw this machine, and it decides where an
//! item begins, what a path is, and which constants are members of which
//! implementation, by Rust's rules rather than by ours.
//!
//! That is what makes this lane's agreement worth something. Two readers written
//! by the same hands against the same document agree because they share the
//! challenged understanding; agreement with a parser nobody here wrote is not
//! correlated with the renderer at all.
//!
//! # Everything inside an implementation is read, and nothing is stepped over
//!
//! A reader that recovers three named constants and ignores whatever else the
//! implementation carries has a blind spot exactly the size of "everything the
//! declaration did not name". A method, an associated type, a macro invocation,
//! a second copy of a constant it already read, a `cfg` deciding whether the
//! implementation exists at all — every one of those changes what the artifact
//! declares, and every one of them used to pass this lane silently. So each is a
//! finding of its own now:
//!
//! - a member that is not one of the expected constants is
//!   [`StructuralDisagreement::UnexpectedImplMember`];
//! - a constant read twice is [`StructuralDisagreement::DuplicateMember`], never
//!   an overwrite of the earlier reading;
//! - the exact path a cause row is CONSTRUCTED through is read and compared, so
//!   rows carrying the declared values through some other constructor are
//!   [`StructuralDisagreement::ConstructorPath`];
//! - how the implementation is written — `unsafe`, negative, `default`, generic
//!   — is [`StructuralDisagreement::ImplPosture`];
//! - any attribute that is not a doc comment is
//!   [`StructuralDisagreement::MeaningBearingAttribute`], because a doc comment
//!   decides nothing and a `cfg` decides everything.
//!
//! # What this lane does NOT claim
//!
//! It reads syntax, and syntax is not meaning. This lane never claims that the
//! artifact TYPECHECKS, that the paths it spells resolve to anything, that the
//! trait it names exists, that the target type exists, that the implementation
//! is coherent, or that any constant evaluates to the value its spelling
//! suggests. `::threadpak::refusal::SomethingElse` is read here as *a different
//! path than the one declared* and never as *no such contract*. Every one of
//! those is lane C's, where `rustc` compiles the artifact and the test reads the
//! constants back AS VALUES.
//!
//! [`StructuralVerdict::Unparsable`] carries the same discipline as lane A's
//! `Unreadable`: it is a failure class, never a skip, and never a softer
//! [`StructuralVerdict::Deviates`]. A caller that folded it into `Conforms`
//! would be asserting over a reading that never happened.
//!
//! Everything this lane recovers and everything it may conclude is declared in
//! `types.rs`. What stands here is the reading itself — the walk over the tree
//! `syn` handed back, and the coarse-to-fine comparison against the caller's
//! declaration.

use super::types::{
    ArtifactStructure, CauseRow, DeclaredStructure, ImplPosture, ImplementationStructure,
    StructuralDisagreement, StructuralVerdict,
};

/// Read one rendered artifact's structure, or `None` where the text is not
/// parseable Rust.
///
/// `None` is a real answer and never "nothing was wrong": a caller that treated
/// it as conformance would be reporting a verdict about a tree that was never
/// built.
#[must_use]
pub fn structure_of(rendered: &str) -> Option<ArtifactStructure> {
    let file = syn::parse_file(rendered).ok()?;
    let mut implementations = Vec::new();
    let mut other_items = 0usize;
    for item in &file.items {
        #[expect(
            clippy::wildcard_enum_match_arm,
            reason = "`syn::Item` is non_exhaustive, so no crate outside syn can enumerate its variants; a wildcard is the only arm that closes this match, and every item it catches is one this reading counts rather than reads"
        )]
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

/// Judge one rendered artifact's structure against an independently declared
/// one.
///
/// **The claim this function supports** is lane B's and only lane B's: the
/// artifact DECLARES these implementations, of these traits, for this target,
/// written this way, carrying these members and no others. It says nothing about
/// whether any of it compiles.
#[must_use]
pub fn judge_structure(rendered: &str, declared: &DeclaredStructure<'_>) -> StructuralVerdict {
    let Some(structure) = structure_of(rendered) else {
        return StructuralVerdict::Unparsable;
    };
    match disagreement(&structure, declared) {
        Some(found) => StructuralVerdict::Deviates(found),
        None => StructuralVerdict::Conforms,
    }
}

/// The first structural disagreement between one reading and one declaration.
///
/// The order is deliberate and coarse-to-fine: an artifact carrying an item
/// nobody planned is reported as that, not as whichever member constant the
/// extra item happened to disturb. Inside an implementation the same principle
/// holds — how the implementation is written, and whether it exists at all under
/// some `cfg`, are read before what its members say, because a member's value is
/// only interesting once the item carrying it is the declared item.
fn disagreement(
    structure: &ArtifactStructure,
    declared: &DeclaredStructure<'_>,
) -> Option<StructuralDisagreement> {
    if structure.other_items > 0 {
        return Some(StructuralDisagreement::UnexpectedItem);
    }
    if duplicated(structure) {
        return Some(StructuralDisagreement::DuplicateImplementation);
    }
    if structure.implementations.len() != declared.traits.len() {
        return Some(StructuralDisagreement::OutputCardinality);
    }
    if structure
        .implementations
        .iter()
        .any(|found| found.target != declared.target)
    {
        return Some(StructuralDisagreement::ImplementationTarget);
    }
    if structure
        .implementations
        .iter()
        .zip(declared.traits.iter())
        .any(|(found, expected)| found.trait_path != *expected)
    {
        return Some(StructuralDisagreement::TraitPath);
    }
    if structure
        .implementations
        .iter()
        .any(|found| found.postures.as_slice() != declared.postures)
    {
        return Some(StructuralDisagreement::ImplPosture);
    }
    if structure.implementations.iter().any(|found| {
        found
            .meaning_bearing_attributes
            .iter()
            .any(|carried| !declared.attributes.contains(&carried.as_str()))
    }) {
        return Some(StructuralDisagreement::MeaningBearingAttribute);
    }
    if structure
        .implementations
        .iter()
        .any(|found| !found.unexpected_members.is_empty())
    {
        return Some(StructuralDisagreement::UnexpectedImplMember);
    }
    if structure
        .implementations
        .iter()
        .any(|found| !found.duplicated_members.is_empty())
    {
        return Some(StructuralDisagreement::DuplicateMember);
    }
    member_disagreement(structure, declared)
}

/// The first disagreement among the members the implementations carry.
fn member_disagreement(
    structure: &ArtifactStructure,
    declared: &DeclaredStructure<'_>,
) -> Option<StructuralDisagreement> {
    let shapes: Vec<&String> = structure
        .implementations
        .iter()
        .filter_map(|found| found.shape.as_ref())
        .collect();
    if sole(&shapes).is_none_or(|shape| shape.as_str() != declared.shape) {
        return Some(StructuralDisagreement::FamilyShape);
    }

    let orders: Vec<&Vec<String>> = structure
        .implementations
        .iter()
        .filter_map(|found| found.selection_order.as_ref())
        .collect();
    if sole(&orders).is_none_or(|order| !same_spellings(order, declared.spellings)) {
        return Some(StructuralDisagreement::SelectionOrder);
    }

    let constructors: Vec<&String> = structure
        .implementations
        .iter()
        .filter_map(|found| found.order_constructor.as_ref())
        .collect();
    if sole(&constructors).is_none_or(|called| called.as_str() != declared.order_constructor) {
        return Some(StructuralDisagreement::ConstructorPath);
    }

    let rosters: Vec<&Vec<CauseRow>> = structure
        .implementations
        .iter()
        .filter_map(|found| found.cause_rows.as_ref())
        .collect();
    let Some(rows) = sole(&rosters) else {
        return Some(StructuralDisagreement::CauseRows);
    };
    if rows.iter().any(|row| {
        row.row_constructor != declared.row_constructor
            || row.identity_constructor != declared.identity_constructor
    }) {
        return Some(StructuralDisagreement::ConstructorPath);
    }
    if !same_rows(rows, declared) {
        return Some(StructuralDisagreement::CauseRows);
    }
    None
}

/// The one member of a one-member roster, or `None` for any other count. Two
/// implementations both stating `SHAPE` is as much a finding as neither doing
/// so, and this is where both become one.
fn sole<'a, T>(found: &'a [&'a T]) -> Option<&'a T> {
    match found.split_first() {
        Some((first, [])) => Some(first),
        _ => None,
    }
}

/// Whether one trait-and-target pair is implemented more than once.
fn duplicated(structure: &ArtifactStructure) -> bool {
    structure
        .implementations
        .iter()
        .enumerate()
        .any(|(position, found)| {
            structure
                .implementations
                .iter()
                .skip(position.saturating_add(1))
                .any(|other| other.target == found.target && other.trait_path == found.trait_path)
        })
}

/// Whether one read roster is the declared roster, spelling for spelling and
/// position for position.
fn same_spellings(read: &[String], declared: &[&str]) -> bool {
    read.len() == declared.len()
        && read
            .iter()
            .zip(declared.iter())
            .all(|(found, expected)| found == expected)
}

/// Whether the read cause rows are the declared identities and spellings, in
/// order. Both columns are held: a roster keeping every spelling and recycling
/// one identity is as wrong as a permuted one.
fn same_rows(read: &[CauseRow], declared: &DeclaredStructure<'_>) -> bool {
    read.len() == declared.identities.len()
        && read.len() == declared.spellings.len()
        && read
            .iter()
            .zip(declared.identities.iter().zip(declared.spellings.iter()))
            .all(|(row, (identity, spelling))| {
                row.identity == *identity && row.spelling == *spelling
            })
}

/// Which expected associated constant one member name spells, where it spells
/// one at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExpectedConstant {
    /// `SHAPE`.
    Shape,
    /// `SELECTION_ORDER`.
    SelectionOrder,
    /// `DECLARED_ORDER`.
    DeclaredOrder,
}

/// The expected constant one member name spells.
fn expected_constant(name: &str) -> Option<ExpectedConstant> {
    match name {
        "SHAPE" => Some(ExpectedConstant::Shape),
        "SELECTION_ORDER" => Some(ExpectedConstant::SelectionOrder),
        "DECLARED_ORDER" => Some(ExpectedConstant::DeclaredOrder),
        _ => None,
    }
}

/// One trait implementation, read out of the tree, or `None` where the item is
/// an implementation of something this lane cannot name — an inherent impl, or
/// one whose target is not a plain path.
fn implementation_structure(declared: &syn::ItemImpl) -> Option<ImplementationStructure> {
    let (path, _) = declared.trait_.as_ref()?;
    let target = type_path(&declared.self_ty)?;
    let mut shape = None;
    let mut selection_order = None;
    let mut order_constructor = None;
    let mut cause_rows = None;
    let mut read: Vec<String> = Vec::new();
    let mut unexpected_members: Vec<String> = Vec::new();
    let mut duplicated_members: Vec<String> = Vec::new();
    let mut meaning_bearing_attributes = meaning_bearing(&declared.attrs);
    for member in &declared.items {
        let (attributes, reading) = member_reading(member);
        meaning_bearing_attributes.extend(meaning_bearing(attributes));
        let constant = match reading {
            ImplMember::Constant(constant) => constant,
            ImplMember::Other(kind) => {
                unexpected_members.push(kind.to_owned());
                continue;
            }
        };
        let name = constant.ident.to_string();
        match expected_constant(&name) {
            None => unexpected_members.push(format!("the associated constant `{name}`")),
            Some(_) if read.contains(&name) => duplicated_members.push(name),
            Some(ExpectedConstant::Shape) => {
                read.push(name);
                shape = last_segment(&constant.expr);
            }
            Some(ExpectedConstant::SelectionOrder) => {
                read.push(name);
                selection_order = string_list(&constant.expr);
            }
            Some(ExpectedConstant::DeclaredOrder) => {
                read.push(name);
                order_constructor = called_path(&constant.expr);
                cause_rows = cause_row_list(&constant.expr);
            }
        }
    }
    Some(ImplementationStructure {
        target,
        trait_path: path_spelling(path),
        postures: postures_of(declared),
        meaning_bearing_attributes,
        shape,
        selection_order,
        order_constructor,
        cause_rows,
        unexpected_members,
        duplicated_members,
    })
}

/// The postures one implementation is written under, in roster order.
fn postures_of(declared: &syn::ItemImpl) -> Vec<ImplPosture> {
    let mut carried = Vec::new();
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

/// What one implementation member is, for the two questions this lane asks of
/// members.
enum ImplMember<'a> {
    /// An associated constant, whose name and value are both read.
    Constant(&'a syn::ImplItemConst),
    /// Anything else, described by what it is. Nothing lawful renders one, so
    /// the description is what the finding carries.
    Other(&'static str),
}

/// One member's attributes, and what the member is.
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "`syn::ImplItem` is non_exhaustive, so no crate outside syn can enumerate its variants; a wildcard is the only arm that closes this match, and every member it catches is by definition not one of the expected associated constants"
)]
fn member_reading(member: &syn::ImplItem) -> (&[syn::Attribute], ImplMember<'_>) {
    match member {
        syn::ImplItem::Const(constant) => (&constant.attrs, ImplMember::Constant(constant)),
        syn::ImplItem::Fn(method) => (&method.attrs, ImplMember::Other("an associated function")),
        syn::ImplItem::Type(associated) => {
            (&associated.attrs, ImplMember::Other("an associated type"))
        }
        syn::ImplItem::Macro(invocation) => (
            &invocation.attrs,
            ImplMember::Other("a macro invocation in member position"),
        ),
        _ => (&[], ImplMember::Other("a member this lane cannot name")),
    }
}

/// Every attribute that decides something, by path.
///
/// A doc comment reaches the tree as `#[doc = "…"]` and decides nothing, so it
/// is not carried. Everything else is: `cfg` decides whether the implementation
/// exists at all, and an attribute nobody planned is a finding whatever it does.
fn meaning_bearing(attributes: &[syn::Attribute]) -> Vec<String> {
    attributes
        .iter()
        .map(|attribute| path_spelling(attribute.path()))
        .filter(|spelled| spelled != "doc")
        .collect()
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
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "`syn::Type` is non_exhaustive, so no crate outside syn can enumerate its variants; a wildcard is the only arm that closes this match, and every type it catches is by definition not a plain path"
)]
fn type_path(declared: &syn::Type) -> Option<String> {
    match declared {
        syn::Type::Path(typed) => Some(path_spelling(&typed.path)),
        _ => None,
    }
}

/// The last segment of a path expression — the variant word of a path-spelled
/// constant.
fn last_segment(expression: &syn::Expr) -> Option<String> {
    let syn::Expr::Path(spelled) = expression else {
        return None;
    };
    spelled
        .path
        .segments
        .last()
        .map(|segment| segment.ident.to_string())
}

/// The path a call expression calls, where the callee is a plain path.
fn called_path(expression: &syn::Expr) -> Option<String> {
    let syn::Expr::Call(call) = expression else {
        return None;
    };
    let syn::Expr::Path(spelled) = call.func.as_ref() else {
        return None;
    };
    Some(path_spelling(&spelled.path))
}

/// The string literals of a `&[…]` expression, in order.
fn string_list(expression: &syn::Expr) -> Option<Vec<String>> {
    array_elements(expression)?.map(string_literal).collect()
}

/// The cause rows of a `Type::declared(&[…])` expression, in order.
fn cause_row_list(expression: &syn::Expr) -> Option<Vec<CauseRow>> {
    let syn::Expr::Call(call) = expression else {
        return None;
    };
    array_elements(call.args.first()?)?.map(cause_row).collect()
}

/// The elements of a `&[…]` expression.
fn array_elements(expression: &syn::Expr) -> Option<syn::punctuated::Iter<'_, syn::Expr>> {
    let syn::Expr::Reference(reference) = expression else {
        return None;
    };
    let syn::Expr::Array(array) = reference.expr.as_ref() else {
        return None;
    };
    Some(array.elems.iter())
}

/// One `Row::declared(Identity::declared("…"), "…")` element, as its four
/// columns: the two constructors it is built through, and the two values it
/// carries.
fn cause_row(expression: &syn::Expr) -> Option<CauseRow> {
    let syn::Expr::Call(call) = expression else {
        return None;
    };
    let row_constructor = called_path(expression)?;
    let mut arguments = call.args.iter();
    let minted = arguments.next()?;
    let spelling = string_literal(arguments.next()?)?;
    let identity_constructor = called_path(minted)?;
    let syn::Expr::Call(identity) = minted else {
        return None;
    };
    Some(CauseRow {
        row_constructor,
        identity_constructor,
        identity: string_literal(identity.args.first()?)?,
        spelling,
    })
}

/// The value of a string-literal expression.
fn string_literal(expression: &syn::Expr) -> Option<String> {
    let syn::Expr::Lit(literal) = expression else {
        return None;
    };
    let syn::Lit::Str(text) = &literal.lit else {
        return None;
    };
    Some(text.value())
}
