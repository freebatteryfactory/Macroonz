//! What a person is shown: the Rust source text a generated tree projects, and
//! the sentence the seam's span refusal renders itself as.
//!
//! Every projection here is one-way. Nothing in the services reads a projection
//! back, derives an identity from one, or decides anything by one — the artifact
//! is the tree, and the refusal is the typed value. These exist so that a
//! producer reporting an unresolvable handle composes no sentence of its own.
//!
//! The bound refusal's sentence is not here: `CaptureBound` is a closed roster
//! stamped at its declaration, and its prose is one of the columns that
//! declaration states. A projection that is a constant per row belongs beside
//! the row rather than in a second file that has to be kept in step with it.

use super::SpanResolutionRefusal;
use super::{GeneratedDelimiter, GeneratedSpacing, GeneratedToken};

impl SpanResolutionRefusal {
    /// The refusal rendered for a person. A projection of the typed value:
    /// nothing reads it back, and it exists so that a caller reporting an
    /// unresolvable handle composes no sentence of its own.
    #[must_use]
    pub fn described(self) -> String {
        format!(
            "the producer's span table carries {} position(s) and does not reach handle {}",
            self.reaches,
            self.handle.index()
        )
    }
}

/// Project one generated token into Rust source text.
pub(super) fn inspect_token(token: &GeneratedToken, into: &mut String) {
    match token {
        GeneratedToken::Word(word) => {
            into.push_str(word);
            into.push(' ');
        }
        GeneratedToken::Punct { mark, spacing } => {
            into.push(*mark);
            if *spacing == GeneratedSpacing::Alone {
                into.push(' ');
            }
        }
        GeneratedToken::Text(text) => {
            into.push('"');
            for character in text.chars() {
                if character == '"' || character == '\\' {
                    into.push('\\');
                }
                into.push(character);
            }
            into.push('"');
            into.push(' ');
        }
        GeneratedToken::Group { delimiter, tokens } => {
            let (open, close) = match delimiter {
                GeneratedDelimiter::Parenthesis => ('(', ')'),
                GeneratedDelimiter::Brace => ('{', '}'),
                GeneratedDelimiter::Bracket => ('[', ']'),
            };
            into.push(open);
            into.push(' ');
            for inner in tokens.iter() {
                inspect_token(inner, into);
            }
            into.push(close);
            into.push(' ');
        }
    }
}
