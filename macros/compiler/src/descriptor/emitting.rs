//! Destination-aware token-writing helpers for the declaration renders that emit direct Rust.
//!
//! Generic Rust spelling belongs to the token home; these operations supply descriptor destinations and compose that vocabulary without reproducing it.

use crate::bounded::Overflow;
use crate::descriptor::DirectBinding;
use crate::descriptor::vocabulary::HarnessWord;
use crate::token::{
    GeneratedDelimiter, GeneratedToken, absolute_path, associated_function, attribute, comma_many,
    documentation, function_signature, group, implementation, result_type, typed_parameter,
};

/// Append one direct dependency path and the destination segments after it.
pub(crate) fn direct_path(
    binding: &DirectBinding,
    destination: &[&str],
    into: &mut Vec<GeneratedToken>,
) {
    let segments = binding
        .segments()
        .iter()
        .map(String::as_str)
        .chain(destination.iter().copied())
        .collect::<Vec<_>>();
    into.extend(absolute_path(&segments));
}

/// Return one owned direct dependency path under its declared binding.
#[must_use]
pub(crate) fn owned_direct_path(
    binding: &DirectBinding,
    destination: &[&str],
) -> Vec<GeneratedToken> {
    let mut tokens = Vec::new();
    direct_path(binding, destination, &mut tokens);
    tokens
}

/// Compose one row lens and one declared attachment seat into their matcher metavariable.
#[must_use]
pub(crate) fn row_metavariable(lens: &str, seat: HarnessWord) -> String {
    let seat = seat.spelling();
    format!("{lens}_{seat}")
}

/// Append one `#[doc = "<text>"]` attribute.
///
/// # Errors
///
/// Returns [`Overflow`] where a composed group carries more tokens than the declared magnitude admits.
pub(crate) fn doc_attribute(text: &str, into: &mut Vec<GeneratedToken>) -> Result<(), Overflow> {
    into.extend(documentation(text)?);
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
    let listed = comma_many(
        traits
            .iter()
            .map(|name| vec![GeneratedToken::word(name)])
            .collect(),
    );
    into.extend(attribute(vec![
        GeneratedToken::word("derive"),
        group(GeneratedDelimiter::Parenthesis, listed)?,
    ])?);
    Ok(())
}

/// Append one `impl ::core::convert::From<<source>> for <target> { fn from(refusal: <source>) -> Self { Self::<arm>(refusal) } }`.
///
/// # Errors
///
/// Returns [`Overflow`] where a composed group carries more tokens than the declared magnitude admits.
pub(crate) fn from_impl(
    source: Vec<GeneratedToken>,
    target: &str,
    arm: &str,
    into: &mut Vec<GeneratedToken>,
) -> Result<(), Overflow> {
    let mut trait_path = absolute_path(&["core", "convert", "From"]);
    trait_path.push(GeneratedToken::alone('<'));
    trait_path.extend(source.iter().cloned());
    trait_path.push(GeneratedToken::alone('>'));
    let signature = function_signature(
        Vec::new(),
        GeneratedToken::word("from"),
        vec![typed_parameter(
            vec![GeneratedToken::word("refusal")],
            source,
        )],
        Vec::new(),
        Some(vec![GeneratedToken::word("Self")]),
        Vec::new(),
    )?;
    let body = vec![
        GeneratedToken::word("Self"),
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        GeneratedToken::word(arm),
        GeneratedToken::group(
            GeneratedDelimiter::Parenthesis,
            vec![GeneratedToken::word("refusal")],
        )?,
    ];
    let road = associated_function(signature, Some(body))?;
    into.extend(implementation(
        Vec::new(),
        Vec::new(),
        Some(trait_path),
        vec![GeneratedToken::word(target)],
        Vec::new(),
        road,
    )?);
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
    into.extend(result_type(ok_seat, vec![GeneratedToken::word(fault)]));
}
