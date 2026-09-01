//! Conventional Rust behavior shells assembled from exact generated-token runs.
//!
//! These operations own fixed function, receiver and match punctuation only.
//! The caller owns every name, qualifier, parameter, result, predicate, pattern, guard, body, effect and semantic meaning.

use super::compose::{absolute_path, comma_many, group};
use super::items::{generic_parameters, where_clause};
use super::{GeneratedDelimiter, GeneratedToken};
use crate::bounded::Overflow;

/// One typed parameter `pattern: kind`.
#[must_use]
pub fn typed_parameter(
    mut pattern: Vec<GeneratedToken>,
    kind: Vec<GeneratedToken>,
) -> Vec<GeneratedToken> {
    pattern.push(GeneratedToken::alone(':'));
    pattern.extend(kind);
    pattern
}

/// The conventional consuming receiver `self`.
#[must_use]
pub fn consuming_receiver() -> Vec<GeneratedToken> {
    vec![GeneratedToken::word("self")]
}

/// The conventional shared receiver `&'a self`, with an empty lifetime run producing `&self`.
#[must_use]
pub fn shared_receiver(lifetime: Vec<GeneratedToken>) -> Vec<GeneratedToken> {
    let mut tokens = vec![GeneratedToken::alone('&')];
    tokens.extend(lifetime);
    tokens.push(GeneratedToken::word("self"));
    tokens
}

/// The conventional exclusive receiver `&'a mut self`, with an empty lifetime run producing `&mut self`.
#[must_use]
pub fn exclusive_receiver(lifetime: Vec<GeneratedToken>) -> Vec<GeneratedToken> {
    let mut tokens = vec![GeneratedToken::alone('&')];
    tokens.extend(lifetime);
    tokens.extend([GeneratedToken::word("mut"), GeneratedToken::word("self")]);
    tokens
}

/// The conventional pinned receiver `self: ::core::pin::Pin<&'a mut Self>`.
#[must_use]
pub fn pinned_receiver(lifetime: Vec<GeneratedToken>) -> Vec<GeneratedToken> {
    let mut kind = absolute_path(&["core", "pin", "Pin"]);
    kind.extend([GeneratedToken::alone('<'), GeneratedToken::alone('&')]);
    kind.extend(lifetime);
    kind.extend([
        GeneratedToken::word("mut"),
        GeneratedToken::word("Self"),
        GeneratedToken::alone('>'),
    ]);
    typed_parameter(vec![GeneratedToken::word("self")], kind)
}

/// One exact function signature without visibility, attributes, body or terminal semicolon.
///
/// Qualifiers are emitted exactly before `fn`, so the caller retains `const`, `async`, `unsafe`, or external-ABI authority.
/// An absent result emits no thin arrow, while a present result is preserved exactly.
///
/// # Errors
///
/// Returns [`Overflow`] where the parameter group outgrows the declared generated-token magnitude.
pub fn function_signature(
    qualifiers: Vec<GeneratedToken>,
    name: GeneratedToken,
    parameters: Vec<Vec<GeneratedToken>>,
    generics: Vec<Vec<GeneratedToken>>,
    result: Option<Vec<GeneratedToken>>,
    predicates: Vec<Vec<GeneratedToken>>,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut tokens = qualifiers;
    tokens.extend([GeneratedToken::word("fn"), name]);
    tokens.extend(generic_parameters(generics));
    tokens.push(group(
        GeneratedDelimiter::Parenthesis,
        comma_many(parameters),
    )?);
    if let Some(result) = result {
        tokens.extend([GeneratedToken::joint('-'), GeneratedToken::alone('>')]);
        tokens.extend(result);
    }
    tokens.extend(where_clause(predicates));
    Ok(tokens)
}

/// One function or method item from an exact signature and exact body.
///
/// # Errors
///
/// Returns [`Overflow`] where the body outgrows the declared generated-token magnitude.
pub fn function_item(
    mut signature: Vec<GeneratedToken>,
    body: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, Overflow> {
    signature.push(group(GeneratedDelimiter::Brace, body)?);
    Ok(signature)
}

/// One conventional `fn name(parameters) -> result { body }` item.
///
/// The visibility is the caller's and is written before this, so one operation spells a private function, a `pub` one, and a `pub(crate)` one.
/// This narrow convenience accepts one flattened parameter run and delegates all function framing to [`function_signature`] and [`function_item`].
///
/// # Errors
///
/// Returns [`Overflow`] where the parameter list or body outgrows the declared generated-token magnitude.
pub fn function(
    name: &str,
    parameters: Vec<GeneratedToken>,
    result: Vec<GeneratedToken>,
    body: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let signature = function_signature(
        Vec::new(),
        GeneratedToken::word(name),
        vec![parameters],
        Vec::new(),
        Some(result),
        Vec::new(),
    )?;
    function_item(signature, body)
}

/// One match arm with an optional exact guard and a trailing comma.
#[must_use]
pub fn match_arm(
    mut pattern: Vec<GeneratedToken>,
    guard: Option<Vec<GeneratedToken>>,
    body: Vec<GeneratedToken>,
) -> Vec<GeneratedToken> {
    if let Some(guard) = guard {
        pattern.push(GeneratedToken::word("if"));
        pattern.extend(guard);
    }
    pattern.extend([GeneratedToken::joint('='), GeneratedToken::alone('>')]);
    pattern.extend(body);
    pattern.push(GeneratedToken::alone(','));
    pattern
}

/// One match expression over exact subject and arm runs.
///
/// # Errors
///
/// Returns [`Overflow`] where the arm group outgrows the declared generated-token magnitude.
pub fn match_expression(
    mut subject: Vec<GeneratedToken>,
    arms: Vec<Vec<GeneratedToken>>,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut tokens = vec![GeneratedToken::word("match")];
    tokens.append(&mut subject);
    tokens.push(group(
        GeneratedDelimiter::Brace,
        arms.into_iter().flatten().collect(),
    )?);
    Ok(tokens)
}
