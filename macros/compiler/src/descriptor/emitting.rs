//! Shared token-writing helpers for the declaration renders that emit direct Rust.
//!
//! One seat per spelling: the absolute path, the doc attribute, the derive attribute, and the `From` impl are each written here and nowhere else, so two renders cannot drift apart on how a generated item is spelled.

use crate::bounded::Overflow;
use crate::token::{GeneratedDelimiter, GeneratedToken};

/// Append one absolute path: `::seg::seg…`.
pub(crate) fn absolute_path(segments: &[&str], into: &mut Vec<GeneratedToken>) {
    for segment in segments {
        into.push(GeneratedToken::joint(':'));
        into.push(GeneratedToken::alone(':'));
        into.push(GeneratedToken::word(segment));
    }
}

/// Append one `#[doc = "<text>"]` attribute.
///
/// # Errors
///
/// Returns [`Overflow`] where a composed group carries more tokens than the declared magnitude admits.
pub(crate) fn doc_attribute(text: &str, into: &mut Vec<GeneratedToken>) -> Result<(), Overflow> {
    into.push(GeneratedToken::alone('#'));
    into.push(GeneratedToken::group(
        GeneratedDelimiter::Bracket,
        vec![
            GeneratedToken::word("doc"),
            GeneratedToken::alone('='),
            GeneratedToken::text(text),
        ],
    )?);
    Ok(())
}

/// Append one `#[derive(<traits>)]` attribute.
///
/// # Errors
///
/// Returns [`Overflow`] where a composed group carries more tokens than the declared magnitude admits.
pub(crate) fn derive_attribute(
    traits: &[&str],
    into: &mut Vec<GeneratedToken>,
) -> Result<(), Overflow> {
    let mut listed = Vec::new();
    for (position, name) in traits.iter().enumerate() {
        if position > 0 {
            listed.push(GeneratedToken::alone(','));
        }
        listed.push(GeneratedToken::word(name));
    }
    into.push(GeneratedToken::alone('#'));
    into.push(GeneratedToken::group(
        GeneratedDelimiter::Bracket,
        vec![
            GeneratedToken::word("derive"),
            GeneratedToken::group(GeneratedDelimiter::Parenthesis, listed)?,
        ],
    )?);
    Ok(())
}

/// Append one `impl ::core::convert::From<<source>> for <target> { fn from(refusal: <source>) -> Self { Self::<arm>(refusal) } }`.
///
/// # Errors
///
/// Returns [`Overflow`] where a composed group carries more tokens than the declared magnitude admits.
pub(crate) fn from_impl(
    source: &[&str],
    target: &str,
    arm: &str,
    into: &mut Vec<GeneratedToken>,
) -> Result<(), Overflow> {
    into.push(GeneratedToken::word("impl"));
    absolute_path(&["core", "convert", "From"], into);
    into.push(GeneratedToken::alone('<'));
    absolute_path(source, into);
    into.push(GeneratedToken::alone('>'));
    into.push(GeneratedToken::word("for"));
    into.push(GeneratedToken::word(target));
    let mut body = vec![GeneratedToken::word("fn"), GeneratedToken::word("from")];
    let mut parameter = vec![GeneratedToken::word("refusal"), GeneratedToken::alone(':')];
    absolute_path(source, &mut parameter);
    body.push(GeneratedToken::group(
        GeneratedDelimiter::Parenthesis,
        parameter,
    )?);
    body.push(GeneratedToken::joint('-'));
    body.push(GeneratedToken::alone('>'));
    body.push(GeneratedToken::word("Self"));
    body.push(GeneratedToken::group(
        GeneratedDelimiter::Brace,
        vec![
            GeneratedToken::word("Self"),
            GeneratedToken::joint(':'),
            GeneratedToken::alone(':'),
            GeneratedToken::word(arm),
            GeneratedToken::group(
                GeneratedDelimiter::Parenthesis,
                vec![GeneratedToken::word("refusal")],
            )?,
        ],
    )?);
    into.push(GeneratedToken::group(GeneratedDelimiter::Brace, body)?);
    Ok(())
}

/// Append one `::core::result::Result<<ok path>, Fault>` return spelling after its `->`.
///
/// The ok seat arrives as tokens rather than a path, so a tuple or a generic seat can stand there.
pub(crate) fn fallible_return(
    ok_seat: Vec<GeneratedToken>,
    fault: &str,
    into: &mut Vec<GeneratedToken>,
) {
    into.push(GeneratedToken::joint('-'));
    into.push(GeneratedToken::alone('>'));
    absolute_path(&["core", "result", "Result"], into);
    into.push(GeneratedToken::alone('<'));
    into.extend(ok_seat);
    into.push(GeneratedToken::alone(','));
    into.push(GeneratedToken::word(fault));
    into.push(GeneratedToken::alone('>'));
}
