//! Conventional namespace and data items assembled from exact generated-token runs.
//!
//! These operations own Rust punctuation and item order only.
//! The caller owns every name, visibility, attribute, type, bound, predicate, field, variant, value, and semantic meaning.

use super::compose::{comma_many, group};
use super::{GeneratedDelimiter, GeneratedToken};
use crate::bounded::Overflow;

/// Prefix one item with its declared attributes and visibility.
#[must_use]
pub fn decorated(
    attributes: Vec<Vec<GeneratedToken>>,
    visibility: Vec<GeneratedToken>,
    item: Vec<GeneratedToken>,
) -> Vec<GeneratedToken> {
    let mut tokens = attributes.into_iter().flatten().collect::<Vec<_>>();
    tokens.extend(visibility);
    tokens.extend(item);
    tokens
}

/// One inline module `mod name { items }`.
///
/// # Errors
///
/// Returns [`Overflow`] where the item body outgrows the declared generated-token magnitude.
pub fn inline_module(
    name: GeneratedToken,
    items: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, Overflow> {
    Ok(vec![
        GeneratedToken::word("mod"),
        name,
        group(GeneratedDelimiter::Brace, items)?,
    ])
}

/// One import `use path [as alias];`.
#[must_use]
pub fn use_item(path: Vec<GeneratedToken>, alias: Option<GeneratedToken>) -> Vec<GeneratedToken> {
    let mut tokens = vec![GeneratedToken::word("use")];
    tokens.extend(path);
    if let Some(alias) = alias {
        tokens.extend([GeneratedToken::word("as"), alias]);
    }
    tokens.push(GeneratedToken::alone(';'));
    tokens
}

/// One generic-parameter list `<parameters>`, or nothing when the roster is empty.
#[must_use]
pub fn generic_parameters(parameters: Vec<Vec<GeneratedToken>>) -> Vec<GeneratedToken> {
    if parameters.is_empty() {
        Vec::new()
    } else {
        let mut tokens = vec![GeneratedToken::alone('<')];
        tokens.extend(comma_many(parameters));
        tokens.push(GeneratedToken::alone('>'));
        tokens
    }
}

/// One `where` clause over exact caller-owned predicates, or nothing when the roster is empty.
#[must_use]
pub fn where_clause(predicates: Vec<Vec<GeneratedToken>>) -> Vec<GeneratedToken> {
    if predicates.is_empty() {
        Vec::new()
    } else {
        let mut tokens = vec![GeneratedToken::word("where")];
        tokens.extend(comma_many(predicates));
        tokens
    }
}

/// One type alias `type Name<...> = value where ...;`.
#[must_use]
pub fn type_alias(
    name: GeneratedToken,
    parameters: Vec<Vec<GeneratedToken>>,
    value: Vec<GeneratedToken>,
    predicates: Vec<Vec<GeneratedToken>>,
) -> Vec<GeneratedToken> {
    let mut tokens = vec![GeneratedToken::word("type"), name];
    tokens.extend(generic_parameters(parameters));
    tokens.push(GeneratedToken::alone('='));
    tokens.extend(value);
    tokens.extend(where_clause(predicates));
    tokens.push(GeneratedToken::alone(';'));
    tokens
}

/// One unit struct `struct Name<...> where ...;`.
#[must_use]
pub fn unit_struct(
    name: GeneratedToken,
    parameters: Vec<Vec<GeneratedToken>>,
    predicates: Vec<Vec<GeneratedToken>>,
) -> Vec<GeneratedToken> {
    let mut tokens = item_head("struct", name, parameters);
    tokens.extend(where_clause(predicates));
    tokens.push(GeneratedToken::alone(';'));
    tokens
}

/// One tuple struct `struct Name<...>(fields) where ...;`.
///
/// A single field is a newtype, and a `PhantomData<T>` field is an ordinary caller-owned type fragment rather than another projector concept.
///
/// # Errors
///
/// Returns [`Overflow`] where the field group outgrows the declared generated-token magnitude.
pub fn tuple_struct(
    name: GeneratedToken,
    parameters: Vec<Vec<GeneratedToken>>,
    fields: Vec<Vec<GeneratedToken>>,
    predicates: Vec<Vec<GeneratedToken>>,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut tokens = item_head("struct", name, parameters);
    tokens.push(group(GeneratedDelimiter::Parenthesis, comma_many(fields))?);
    tokens.extend(where_clause(predicates));
    tokens.push(GeneratedToken::alone(';'));
    Ok(tokens)
}

/// One named struct `struct Name<...> where ... { fields }`.
///
/// # Errors
///
/// Returns [`Overflow`] where the field group outgrows the declared generated-token magnitude.
pub fn named_struct(
    name: GeneratedToken,
    parameters: Vec<Vec<GeneratedToken>>,
    predicates: Vec<Vec<GeneratedToken>>,
    fields: Vec<Vec<GeneratedToken>>,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut tokens = item_head("struct", name, parameters);
    tokens.extend(where_clause(predicates));
    tokens.push(group(GeneratedDelimiter::Brace, comma_many(fields))?);
    Ok(tokens)
}

/// One named field `name: kind`.
#[must_use]
pub fn named_field(name: GeneratedToken, kind: Vec<GeneratedToken>) -> Vec<GeneratedToken> {
    let mut tokens = vec![name, GeneratedToken::alone(':')];
    tokens.extend(kind);
    tokens
}

/// One enum `enum Name<...> where ... { variants }`.
///
/// # Errors
///
/// Returns [`Overflow`] where the variant group outgrows the declared generated-token magnitude.
pub fn enumeration(
    name: GeneratedToken,
    parameters: Vec<Vec<GeneratedToken>>,
    predicates: Vec<Vec<GeneratedToken>>,
    variants: Vec<Vec<GeneratedToken>>,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut tokens = item_head("enum", name, parameters);
    tokens.extend(where_clause(predicates));
    tokens.push(group(GeneratedDelimiter::Brace, comma_many(variants))?);
    Ok(tokens)
}

/// One unit variant.
#[must_use]
pub fn unit_variant(name: GeneratedToken) -> Vec<GeneratedToken> {
    vec![name]
}

/// One tuple variant.
///
/// # Errors
///
/// Returns [`Overflow`] where the field group outgrows the declared generated-token magnitude.
pub fn tuple_variant(
    name: GeneratedToken,
    fields: Vec<Vec<GeneratedToken>>,
) -> Result<Vec<GeneratedToken>, Overflow> {
    Ok(vec![
        name,
        group(GeneratedDelimiter::Parenthesis, comma_many(fields))?,
    ])
}

/// One named variant.
///
/// # Errors
///
/// Returns [`Overflow`] where the field group outgrows the declared generated-token magnitude.
pub fn named_variant(
    name: GeneratedToken,
    fields: Vec<Vec<GeneratedToken>>,
) -> Result<Vec<GeneratedToken>, Overflow> {
    Ok(vec![
        name,
        group(GeneratedDelimiter::Brace, comma_many(fields))?,
    ])
}

/// One item keyword, name, and exact generic-parameter roster.
fn item_head(
    keyword: &str,
    name: GeneratedToken,
    parameters: Vec<Vec<GeneratedToken>>,
) -> Vec<GeneratedToken> {
    let mut tokens = vec![GeneratedToken::word(keyword), name];
    tokens.extend(generic_parameters(parameters));
    tokens
}
