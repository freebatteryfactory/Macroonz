//! Converting one value's proved declaration-site cargo into the compiler's tokens.

use super::types::Emittable;
use crate::closure::PartitionCargo;
use crate::token::{GeneratedDelimiter, GeneratedSpacing, GeneratedToken, GeneratedTree};
use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

/// One value's declaration-site cargos as the compiler's tokens — this host's only act on the way out.
///
/// Each cargo is read off its own proved delivery, and a delivery that planned nothing writes nothing for it.
/// The cargos reach the compiler one after another, in the order the value delivers them, and no third generated tree is assembled out of them here — a tree this host joined would be bytes no proof committed to.
#[must_use]
pub fn emit(emittable: &impl Emittable) -> TokenStream {
    let mut emitted = TokenStream::new();
    for cargo in emittable.cargos() {
        emitted.extend(emit_cargo(cargo));
    }
    emitted
}

/// One proved delivery as the compiler's tokens.
fn emit_cargo(cargo: &PartitionCargo) -> TokenStream {
    cargo.tokens().map_or_else(TokenStream::new, emit_tree)
}

/// One generated tree as the compiler's tokens.
fn emit_tree(tree: &GeneratedTree) -> TokenStream {
    tree.tokens().iter().map(emit_token).collect()
}

/// One generated token as the compiler's token.
///
/// A renderer states a literal's value; the quoting, the escaping, and the absence of a suffix are settled here, at the one seat that writes a compiler literal.
fn emit_token(token: &GeneratedToken) -> TokenTree {
    match token {
        GeneratedToken::Word(word) => TokenTree::Ident(Ident::new(word, Span::call_site())),
        GeneratedToken::RawIdentifier(name) => {
            TokenTree::Ident(Ident::new_raw(name, Span::call_site()))
        }
        GeneratedToken::Punct { mark, spacing } => {
            TokenTree::Punct(Punct::new(*mark, written_spacing(*spacing)))
        }
        GeneratedToken::Text(text) => TokenTree::Literal(Literal::string(text)),
        GeneratedToken::Group { delimiter, tokens } => TokenTree::Group(Group::new(
            written_delimiter(*delimiter),
            tokens.iter().map(emit_token).collect(),
        )),
        GeneratedToken::ByteText(material) => TokenTree::Literal(Literal::byte_string(material)),
        GeneratedToken::Number(value) => TokenTree::Literal(Literal::u64_unsuffixed(*value)),
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
    }
}
