//! The challenge-side `syn` decoder for one rendered Rust artifact.
//!
//! Parsing is an effect owned by the test host. This module maps the foreign
//! syntax tree into `TestPak`'s public structural-reading vocabulary; the
//! production library owns only the typed comparison.

use threadpak_testpak::oracle::{
    ArtifactStructure, ConstantReading, ImplPosture, ImplementationMember, ImplementationStructure,
};

/// Read what one rendered artifact declares, or `None` where the text is not parseable Rust.
#[must_use]
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "`syn::Item` is non_exhaustive, so no crate outside syn can enumerate its variants; a wildcard is the only arm that closes this match, and every item it catches is one this reading counts rather than reads"
)]
pub(crate) fn declarations_in(rendered: &str) -> Option<ArtifactStructure> {
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

/// One trait or inherent implementation recovered from the syntax tree.
fn implementation_structure(declared: &syn::ItemImpl) -> Option<ImplementationStructure> {
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
        trait_path: declared
            .trait_
            .as_ref()
            .map(|(path, _)| path_spelling(path)),
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

/// One member's attributes and structural reading.
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

/// Every non-documentation attribute that decides something, by path.
fn meaning_bearing(attributes: &[syn::Attribute]) -> Vec<String> {
    attributes
        .iter()
        .map(|attribute| path_spelling(attribute.path()))
        .filter(|spelled| spelled != "doc")
        .collect()
}

/// What one associated constant's value expression says.
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

/// What one supported literal says.
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

/// What one borrowed array expression says.
fn borrowed_reading(inner: &syn::Expr) -> Option<ConstantReading> {
    let syn::Expr::Array(array) = inner else {
        return None;
    };
    elements(array.elems.iter()).map(ConstantReading::BorrowedArray)
}

/// What one call says: the path called and its arguments.
fn call_reading(call: &syn::ExprCall) -> Option<ConstantReading> {
    let syn::Expr::Path(spelled) = call.func.as_ref() else {
        return None;
    };
    Some(ConstantReading::Call {
        path: path_spelling(&spelled.path),
        arguments: elements(call.args.iter())?,
    })
}

/// The readings of several expressions, in order.
fn elements<'read>(
    expressions: impl Iterator<Item = &'read syn::Expr>,
) -> Option<Vec<ConstantReading>> {
    expressions.map(constant_reading).collect()
}

/// One path, spelled with its segments and leading separator.
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

/// The path one type spells, where the type is a plain path.
fn type_path(declared: &syn::Type) -> Option<String> {
    let syn::Type::Path(typed) = declared else {
        return None;
    };
    Some(path_spelling(&typed.path))
}
