//! The cursor home's trait implementations.
//!
//! Every refusal here renders as one sentence about a captured sequence and names no caller, no clause, and no product.

use super::{
    CaptureExpectation, CaptureReadIssue, CaptureReadRefusal, CapturedDelimiter, CapturedSpacing,
};

impl core::fmt::Display for CaptureExpectation {
    fn fmt(&self, into: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Token => into.write_str("one captured token"),
            Self::Word(word) => write!(into, "the ordinary word `{word}`"),
            Self::Identifier => into.write_str("one ordinary or raw identifier"),
            Self::Number => into.write_str("one numeric literal"),
            Self::Punctuation { mark, spacing } => {
                let posture = match spacing {
                    CapturedSpacing::Alone => "standing alone",
                    CapturedSpacing::Joint => "joined to what follows",
                };
                write!(into, "the punctuation `{mark}` {posture}")
            }
            Self::Group(delimiter) => {
                let name = match delimiter {
                    CapturedDelimiter::Parenthesis => "parenthesized",
                    CapturedDelimiter::Brace => "braced",
                    CapturedDelimiter::Bracket => "bracketed",
                    CapturedDelimiter::Bare => "invisibly grouped",
                };
                write!(into, "one {name} token group")
            }
        }
    }
}

impl core::fmt::Display for CaptureReadIssue {
    fn fmt(&self, into: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Missing(expected) => {
                write!(into, "the captured sequence ended before {expected}")
            }
            Self::Unexpected(expected) => {
                write!(into, "the next captured token is not {expected}")
            }
            Self::InputRemaining => {
                into.write_str("the captured sequence carries an unconsumed token")
            }
            Self::SequenceUnbounded { limit } => write!(
                into,
                "the captured sequence carries more members than its declared magnitude of {limit}"
            ),
            Self::SequenceMemberDidNotAdvance => into.write_str(
                "the separated-sequence member reader returned without consuming a token",
            ),
            Self::CursorRangeContradiction => into.write_str(
                "the capture cursor's consumed range does not belong to its captured sequence",
            ),
        }
    }
}

impl core::fmt::Display for CaptureReadRefusal {
    fn fmt(&self, into: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.token() {
            Some(token) => write!(into, "{} at captured span {}", self.issue(), token.index()),
            None => write!(into, "{} at the declaration boundary", self.issue()),
        }
    }
}

impl core::error::Error for CaptureReadRefusal {}
