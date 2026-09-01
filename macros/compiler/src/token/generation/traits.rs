//! Conventional Rust trait and implementation shells assembled from exact generated-token runs.
//!
//! These operations own fixed trait, associated-item and implementation punctuation only.
//! The caller owns every qualifier, name, generic, bound, predicate, path, item, body, safety contract and semantic meaning.

use super::behavior::function_item;
use super::compose::group;
use super::items::{generic_parameters, where_clause};
use super::{GeneratedDelimiter, GeneratedToken};
use crate::bounded::Overflow;

/// One trait declaration with exact qualifiers, parameters, supertraits, predicates and associated items.
///
/// Qualifiers are emitted exactly before `trait`, so an unsafe trait remains explicit caller authority.
///
/// # Errors
///
/// Returns [`Overflow`] where the associated-item body outgrows the declared generated-token magnitude.
pub fn trait_declaration(
    qualifiers: Vec<GeneratedToken>,
    name: GeneratedToken,
    parameters: Vec<Vec<GeneratedToken>>,
    supertraits: Vec<Vec<GeneratedToken>>,
    predicates: Vec<Vec<GeneratedToken>>,
    items: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut tokens = qualifiers;
    tokens.extend([GeneratedToken::word("trait"), name]);
    tokens.extend(generic_parameters(parameters));
    if !supertraits.is_empty() {
        tokens.push(GeneratedToken::alone(':'));
        tokens.extend(plus_many(supertraits));
    }
    tokens.extend(where_clause(predicates));
    tokens.push(group(GeneratedDelimiter::Brace, items)?);
    Ok(tokens)
}

/// One associated type declaration or definition.
///
/// An absent value emits a declaration, while a present value emits the exact definition.
#[must_use]
pub fn associated_type(
    name: GeneratedToken,
    parameters: Vec<Vec<GeneratedToken>>,
    bounds: Vec<Vec<GeneratedToken>>,
    value: Option<Vec<GeneratedToken>>,
    predicates: Vec<Vec<GeneratedToken>>,
) -> Vec<GeneratedToken> {
    let mut tokens = vec![GeneratedToken::word("type"), name];
    tokens.extend(generic_parameters(parameters));
    if !bounds.is_empty() {
        tokens.push(GeneratedToken::alone(':'));
        tokens.extend(plus_many(bounds));
    }
    if let Some(value) = value {
        tokens.push(GeneratedToken::alone('='));
        tokens.extend(value);
    }
    tokens.extend(where_clause(predicates));
    tokens.push(GeneratedToken::alone(';'));
    tokens
}

/// One associated constant declaration or definition.
///
/// An absent value emits a declaration, while a present value emits the exact definition.
#[must_use]
pub fn associated_constant(
    name: GeneratedToken,
    kind: Vec<GeneratedToken>,
    value: Option<Vec<GeneratedToken>>,
) -> Vec<GeneratedToken> {
    let mut tokens = vec![
        GeneratedToken::word("const"),
        name,
        GeneratedToken::alone(':'),
    ];
    tokens.extend(kind);
    if let Some(value) = value {
        tokens.push(GeneratedToken::alone('='));
        tokens.extend(value);
    }
    tokens.push(GeneratedToken::alone(';'));
    tokens
}

/// One associated function declaration or definition from an exact signature.
///
/// An absent body emits the required signature with a terminal semicolon.
///
/// # Errors
///
/// Returns [`Overflow`] where a supplied body outgrows the declared generated-token magnitude.
pub fn associated_function(
    mut signature: Vec<GeneratedToken>,
    body: Option<Vec<GeneratedToken>>,
) -> Result<Vec<GeneratedToken>, Overflow> {
    if let Some(body) = body {
        function_item(signature, body)
    } else {
        signature.push(GeneratedToken::alone(';'));
        Ok(signature)
    }
}

/// One inherent or trait implementation with exact qualifiers, parameters, target, predicates and items.
///
/// An absent trait path emits an inherent implementation.
/// Qualifiers are emitted exactly before `impl`, so an unsafe implementation remains explicit caller authority.
///
/// # Errors
///
/// Returns [`Overflow`] where the implementation body outgrows the declared generated-token magnitude.
pub fn implementation(
    qualifiers: Vec<GeneratedToken>,
    parameters: Vec<Vec<GeneratedToken>>,
    trait_path: Option<Vec<GeneratedToken>>,
    target: Vec<GeneratedToken>,
    predicates: Vec<Vec<GeneratedToken>>,
    items: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut tokens = qualifiers;
    tokens.push(GeneratedToken::word("impl"));
    tokens.extend(generic_parameters(parameters));
    if let Some(trait_path) = trait_path {
        tokens.extend(trait_path);
        tokens.push(GeneratedToken::word("for"));
    }
    tokens.extend(target);
    tokens.extend(where_clause(predicates));
    tokens.push(group(GeneratedDelimiter::Brace, items)?);
    Ok(tokens)
}

/// Joins exact runs with conventional trait-bound plus punctuation.
fn plus_many(parts: Vec<Vec<GeneratedToken>>) -> Vec<GeneratedToken> {
    let mut tokens = Vec::new();
    for (position, part) in parts.into_iter().enumerate() {
        if position > 0 {
            tokens.push(GeneratedToken::alone('+'));
        }
        tokens.extend(part);
    }
    tokens
}
