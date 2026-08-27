//! The generation home's invariant nucleus.

use super::{GeneratedDelimiter, GeneratedSpacing, GeneratedToken, GeneratedTree};
use crate::bounded::{Bounded, Overflow};

impl GeneratedToken {
    /// Append this token's canonical bytes to a containing compiler-owned encoding.
    pub(crate) fn encode_into(&self, into: &mut Vec<u8>) {
        super::super::encode::encode_generated(self, into);
    }

    /// One ordinary word.
    #[must_use]
    pub fn word(spelling: &str) -> Self {
        Self::Word(spelling.to_owned())
    }

    /// One raw identifier, stated without its `r#` spelling marker.
    #[must_use]
    pub fn raw_identifier(name: &str) -> Self {
        Self::RawIdentifier(name.to_owned())
    }

    /// One punctuation mark that joins what follows.
    #[must_use]
    pub const fn joint(mark: char) -> Self {
        Self::Punct {
            mark,
            spacing: GeneratedSpacing::Joint,
        }
    }

    /// One punctuation mark that stands alone.
    #[must_use]
    pub const fn alone(mark: char) -> Self {
        Self::Punct {
            mark,
            spacing: GeneratedSpacing::Alone,
        }
    }

    /// One text literal.
    #[must_use]
    pub fn text(content: &str) -> Self {
        Self::Text(content.to_owned())
    }

    /// One byte-string literal, over the material a caller holds.
    #[must_use]
    pub fn byte_text(material: &[u8]) -> Self {
        Self::ByteText(material.to_vec())
    }

    /// One unsuffixed integer literal.
    #[must_use]
    pub const fn number(value: u64) -> Self {
        Self::Number(value)
    }

    /// One delimited group.
    ///
    /// # Errors
    ///
    /// Returns [`Overflow`] where the group carries more tokens than the declared magnitude admits.
    pub fn group(delimiter: GeneratedDelimiter, tokens: Vec<Self>) -> Result<Self, Overflow> {
        Bounded::new(tokens).map(|tokens| Self::Group { delimiter, tokens })
    }

    /// One fixed-arity delimited group whose fit is settled at compile time.
    #[must_use]
    pub(crate) fn fixed_group<const N: usize>(
        delimiter: GeneratedDelimiter,
        tokens: [Self; N],
    ) -> Self {
        Self::Group {
            delimiter,
            tokens: Bounded::from_array(tokens),
        }
    }
}

impl GeneratedTree {
    /// Assemble one generated tree.
    ///
    /// # Errors
    ///
    /// Returns [`Overflow`] where the tree carries more top-level tokens than the declared magnitude admits.
    pub fn assembled(tokens: Vec<GeneratedToken>) -> Result<Self, Overflow> {
        Bounded::new(tokens).map(|tokens| Self { tokens })
    }

    /// The top-level tokens, in the order they were written.
    #[must_use]
    pub fn tokens(&self) -> &[GeneratedToken] {
        self.tokens.as_slice()
    }

    /// How many top-level tokens the tree carries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    /// Whether the tree carries nothing.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    /// Join one tree onto another, producing the tree that carries both.
    ///
    /// # Errors
    ///
    /// Returns [`Overflow`] where the joined tree outgrows the declared magnitude.
    pub fn joined(&self, other: &Self) -> Result<Self, Overflow> {
        let mut tokens = self.tokens.as_slice().to_vec();
        tokens.extend_from_slice(other.tokens.as_slice());
        Self::assembled(tokens)
    }

    /// The Rust source text this tree projects, for a person to read.
    #[must_use]
    pub fn inspected(&self) -> String {
        let mut rendered = String::new();
        for token in self.tokens.as_slice() {
            super::super::inspect::inspect_token(token, &mut rendered);
        }
        rendered
    }

    /// The tree's canonical bytes.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for token in self.tokens.as_slice() {
            token.encode_into(&mut bytes);
        }
        bytes
    }
}
