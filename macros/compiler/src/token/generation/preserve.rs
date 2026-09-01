//! Projecting a captured fragment into the generated-token vocabulary without a source-string round trip.
//!
//! The capture remains the identity-bearing authored reading.
//! This projection exists only where a renderer must repeat exact caller-owned Rust inside generated output.

use super::{
    FragmentGenerationIssue, FragmentGenerationRefusal, GeneratedDelimiter, GeneratedLiteral,
    GeneratedToken, GeneratedTree,
};
use crate::token::{CapturedDelimiter, CapturedFragment, CapturedPayload, CapturedTokenTree};

impl CapturedFragment<'_> {
    /// Project this exact captured fragment into one generated tree.
    ///
    /// Every token is converted structurally.
    /// No inspected source string is built or parsed, and the captured fragment remains available with its original span handles.
    ///
    /// # Errors
    ///
    /// Returns the exact captured span where an admitted generated literal or generated-token magnitude refuses.
    pub fn generated(self) -> Result<GeneratedTree, FragmentGenerationRefusal> {
        let tokens = preserved_tokens(self.tokens())?;
        GeneratedTree::assembled(tokens).map_err(|_| FragmentGenerationRefusal {
            issue: FragmentGenerationIssue::Unbounded,
            at: self.first_span().or(self.enclosing_span()),
        })
    }
}

/// Preserve one already bounded captured-token slice for a compiler-owned structural lens.
pub(crate) fn preserved_tokens(
    tokens: &[CapturedTokenTree],
) -> Result<Vec<GeneratedToken>, FragmentGenerationRefusal> {
    tokens.iter().map(preserved_token).collect()
}

impl FragmentGenerationRefusal {
    /// The exact conversion issue this projection established.
    pub const fn issue(self) -> FragmentGenerationIssue {
        self.issue
    }

    /// The exact captured producer span available at the refusal site.
    #[must_use]
    pub const fn token(self) -> Option<crate::token::SpanHandle> {
        self.at
    }
}

/// Preserve one captured token and every nested token below it.
fn preserved_token(token: &CapturedTokenTree) -> Result<GeneratedToken, FragmentGenerationRefusal> {
    match token.payload() {
        CapturedPayload::Word(word) => Ok(GeneratedToken::word(word)),
        CapturedPayload::Punct(mark) => Ok(GeneratedToken::alone(*mark)),
        CapturedPayload::Text(text) => Ok(GeneratedToken::text(text)),
        CapturedPayload::Number(spelling) => GeneratedLiteral::number(spelling)
            .map(GeneratedToken::literal)
            .map_err(|issue| literal_refusal(issue, token)),
        CapturedPayload::Group { delimiter, trees } => {
            preserved_group(*delimiter, trees.as_slice(), token)
        }
        CapturedPayload::ByteText(material) => Ok(GeneratedToken::byte_text(material)),
        CapturedPayload::Character(character) => Ok(GeneratedToken::literal(
            GeneratedLiteral::character(*character),
        )),
        CapturedPayload::Byte(byte) => Ok(GeneratedToken::literal(GeneratedLiteral::byte(*byte))),
        CapturedPayload::NulTerminatedText(material) => {
            GeneratedLiteral::nul_terminated_text(material)
                .map(GeneratedToken::literal)
                .map_err(|issue| literal_refusal(issue, token))
        }
        CapturedPayload::RawIdentifier(name) => Ok(GeneratedToken::raw_identifier(name)),
        CapturedPayload::JointPunct(mark) => Ok(GeneratedToken::joint(*mark)),
    }
}

/// Preserve one captured group with its written or invisible delimiter.
fn preserved_group(
    delimiter: CapturedDelimiter,
    members: &[CapturedTokenTree],
    source: &CapturedTokenTree,
) -> Result<GeneratedToken, FragmentGenerationRefusal> {
    let tokens = members
        .iter()
        .map(preserved_token)
        .collect::<Result<Vec<_>, _>>()?;
    GeneratedToken::group(generated_delimiter(delimiter), tokens).map_err(|_| {
        FragmentGenerationRefusal {
            issue: FragmentGenerationIssue::Unbounded,
            at: Some(source.span()),
        }
    })
}

/// The generated delimiter corresponding to one captured delimiter.
const fn generated_delimiter(delimiter: CapturedDelimiter) -> GeneratedDelimiter {
    match delimiter {
        CapturedDelimiter::Parenthesis => GeneratedDelimiter::Parenthesis,
        CapturedDelimiter::Brace => GeneratedDelimiter::Brace,
        CapturedDelimiter::Bracket => GeneratedDelimiter::Bracket,
        CapturedDelimiter::Bare => GeneratedDelimiter::Bare,
    }
}

/// Bind one generated-literal refusal to its exact captured token.
const fn literal_refusal(
    issue: super::GeneratedLiteralRefusal,
    token: &CapturedTokenTree,
) -> FragmentGenerationRefusal {
    FragmentGenerationRefusal {
        issue: FragmentGenerationIssue::Literal(issue),
        at: Some(token.span()),
    }
}
