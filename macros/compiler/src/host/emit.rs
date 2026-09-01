//! Converting one value's proved declaration-site cargo into the compiler's tokens.

use super::types::{EmissionError, Emittable};
use crate::closure::PartitionCargo;
use crate::token::{
    GeneratedDelimiter, GeneratedLiteralForm, GeneratedSpacing, GeneratedToken, GeneratedTree,
    SpanHandle,
};
use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use std::ffi::CString;
use std::str::FromStr;

/// One value's declaration-site cargos as the compiler's tokens — this host's only act on the way out.
///
/// Each cargo is read off its own proved delivery, and a delivery that planned nothing writes nothing for it.
/// The cargos reach the compiler one after another, in the order the value delivers them, and no third generated tree is assembled out of them here — a tree this host joined would be bytes no proof committed to.
///
/// # Errors
///
/// Returns [`EmissionError`] where the stable proc-macro literal API rejects one exact literal the ordinary compiler already admitted.
pub fn emit(
    emittable: &impl Emittable,
    spans: &super::Spans,
) -> Result<TokenStream, EmissionError> {
    let mut emitted = TokenStream::new();
    for cargo in emittable.cargos() {
        emitted.extend(emit_cargo(cargo, spans)?);
    }
    Ok(emitted)
}

/// One proved delivery as the compiler's tokens.
fn emit_cargo(cargo: &PartitionCargo, spans: &super::Spans) -> Result<TokenStream, EmissionError> {
    cargo
        .tokens()
        .map_or_else(|| Ok(TokenStream::new()), |tree| emit_tree(tree, spans))
}

/// One generated tree as the compiler's tokens.
///
/// # Errors
///
/// Returns [`EmissionError`] where the stable proc-macro literal API rejects one exact literal the ordinary compiler already admitted.
pub fn emit_tree(tree: &GeneratedTree, spans: &super::Spans) -> Result<TokenStream, EmissionError> {
    let mut sources = tree.source_spans().iter();
    let emitted = tree
        .tokens()
        .iter()
        .map(|token| emit_token(token, &mut sources, spans))
        .collect::<Result<TokenStream, _>>()?;
    if sources.next().is_some() {
        return Err(EmissionError::SourceSpanRosterContradiction);
    }
    Ok(emitted)
}

/// One generated token as the compiler's token.
///
/// A renderer states a literal's value; the quoting, the escaping, and the absence of a suffix are settled here, at the one seat that writes a compiler literal.
fn emit_token(
    token: &GeneratedToken,
    sources: &mut core::slice::Iter<'_, Option<SpanHandle>>,
    spans: &super::Spans,
) -> Result<TokenTree, EmissionError> {
    let source = *sources
        .next()
        .ok_or(EmissionError::SourceSpanRosterContradiction)?;
    let mut emitted = match token {
        GeneratedToken::Word(word) => TokenTree::Ident(Ident::new(word, Span::call_site())),
        GeneratedToken::RawIdentifier(name) => {
            TokenTree::Ident(Ident::new_raw(name, Span::call_site()))
        }
        GeneratedToken::Punct { mark, spacing } => {
            TokenTree::Punct(Punct::new(*mark, written_spacing(*spacing)))
        }
        GeneratedToken::Text(text) => TokenTree::Literal(Literal::string(text)),
        GeneratedToken::Group { delimiter, tokens } => {
            let stream = tokens
                .iter()
                .map(|nested_token| emit_token(nested_token, sources, spans))
                .collect::<Result<_, _>>()?;
            TokenTree::Group(Group::new(written_delimiter(*delimiter), stream))
        }
        GeneratedToken::ByteText(material) => TokenTree::Literal(Literal::byte_string(material)),
        GeneratedToken::Number(value) => TokenTree::Literal(Literal::u64_unsuffixed(*value)),
        GeneratedToken::Literal(literal) => TokenTree::Literal(emit_literal(literal.form())?),
    };
    if let Some(handle) = source {
        let authored_span = spans
            .resolve(handle)
            .map_err(EmissionError::SourceSpanUnresolved)?;
        emitted.set_span(authored_span);
    }
    Ok(emitted)
}

/// Materialize one admitted exact literal through the proc-macro literal API.
fn emit_literal(literal: GeneratedLiteralForm<'_>) -> Result<Literal, EmissionError> {
    match literal {
        GeneratedLiteralForm::Number(spelling) => {
            Literal::from_str(spelling).map_err(|_| EmissionError::NumberRejected {
                spelling: spelling.to_owned(),
            })
        }
        GeneratedLiteralForm::Character(character) => Ok(Literal::character(character)),
        GeneratedLiteralForm::Byte(byte) => Ok(Literal::byte_character(byte)),
        GeneratedLiteralForm::NulTerminatedText(material) => CString::new(material)
            .map(|text| Literal::c_string(&text))
            .map_err(|_| EmissionError::NulTerminatedTextRejected),
    }
}

/// The compiler spacing one generated mark is written with.
const fn written_spacing(spacing: GeneratedSpacing) -> Spacing {
    match spacing {
        GeneratedSpacing::Joint => Spacing::Joint,
        GeneratedSpacing::Alone => Spacing::Alone,
    }
}

/// The compiler delimiter one generated group is written with.
const fn written_delimiter(delimiter: GeneratedDelimiter) -> Delimiter {
    match delimiter {
        GeneratedDelimiter::Parenthesis => Delimiter::Parenthesis,
        GeneratedDelimiter::Brace => Delimiter::Brace,
        GeneratedDelimiter::Bracket => Delimiter::Bracket,
        GeneratedDelimiter::Bare => Delimiter::None,
    }
}
