//! The read: rendered Rust text mapped into a structural reading.
//!
//! A foreign parser does the reading, and this module maps its syntax tree into the home's own vocabulary.
//! It shares no capture, plan, renderer, token representation, or projection with whatever produced the artifact — that is the whole reason the structural comparison downstream is worth anything.
//!
//! The map is deliberately shallow.
//! It recovers what an item declares, never what the declaration means: nothing here resolves a path, checks a type, or evaluates a constant, and every value shape it cannot name reads as `None` rather than being flattened into one it can.

use super::types::{
    ArtifactStructure, ConstantReading, ImplPosture, ImplementationMember, ImplementationStructure,
};

/// Read what one rendered artifact declares, or `None` where the text is not parseable Rust.
///
/// An item that is not an implementation of a named type is counted rather than read, because a reader that stepped over one would have a blind spot exactly the size of everything nobody planned.
#[must_use]
pub fn declarations_in(rendered: &str) -> Option<ArtifactStructure> {
    let file = syn::parse_file(rendered).ok()?;
    let mut implementations: Vec<ImplementationStructure> = Vec::new();
    let mut other_items = 0usize;
    for item in &file.items {
        let structure = if let syn::Item::Impl(declared) = item {
            implementation_structure(declared)
        } else {
            None
        };
        match structure {
            Some(read) => implementations.push(read),
            None => other_items = other_items.saturating_add(1),
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
fn member_reading(member: &syn::ImplItem) -> (&[syn::Attribute], ImplementationMember) {
    if let syn::ImplItem::Const(constant) = member {
        (
            &constant.attrs,
            ImplementationMember::Constant {
                name: constant.ident.to_string(),
                reading: constant_reading(&constant.expr),
            },
        )
    } else if let syn::ImplItem::Fn(method) = member {
        (
            &method.attrs,
            ImplementationMember::Other {
                described: "an associated function",
            },
        )
    } else if let syn::ImplItem::Type(associated) = member {
        (
            &associated.attrs,
            ImplementationMember::Other {
                described: "an associated type",
            },
        )
    } else if let syn::ImplItem::Macro(invocation) = member {
        (
            &invocation.attrs,
            ImplementationMember::Other {
                described: "a macro invocation in member position",
            },
        )
    } else {
        (
            &[],
            ImplementationMember::Other {
                described: "a member this lane cannot name",
            },
        )
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
///
/// An expression this reading deliberately does not name answers `None`.
fn constant_reading(expression: &syn::Expr) -> Option<ConstantReading> {
    if let syn::Expr::Path(spelled) = expression {
        Some(ConstantReading::Path(path_spelling(&spelled.path)))
    } else if let syn::Expr::Lit(literal) = expression {
        literal_reading(&literal.lit)
    } else if let syn::Expr::Reference(reference) = expression {
        borrowed_reading(reference.expr.as_ref())
    } else if let syn::Expr::Array(array) = expression {
        elements(array.elems.iter()).map(ConstantReading::Array)
    } else if let syn::Expr::Call(call) = expression {
        call_reading(call)
    } else {
        None
    }
}

/// What one supported literal says.
///
/// A literal this reading deliberately does not name answers `None`.
fn literal_reading(literal: &syn::Lit) -> Option<ConstantReading> {
    if let syn::Lit::Str(text) = literal {
        Some(ConstantReading::Text(text.value()))
    } else if let syn::Lit::Int(number) = literal {
        Some(ConstantReading::Number(number.base10_digits().to_owned()))
    } else if let syn::Lit::Bool(truth) = literal {
        Some(ConstantReading::Truth(truth.value))
    } else {
        None
    }
}

/// What one borrowed array expression says.
fn borrowed_reading(inner: &syn::Expr) -> Option<ConstantReading> {
    let syn::Expr::Array(array) = inner else {
        return None;
    };
    elements(array.elems.iter()).map(ConstantReading::BorrowedArray)
}

/// What one call says: the path called, and its arguments.
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

/// One path, spelled with its segments and its leading separator.
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
