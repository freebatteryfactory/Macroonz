//! The text home's trait implementations and the intrinsic diagnostic projection of a refused text read.
//!
//! Every refusal here renders as one sentence about declared text and names no caller, no attribute, and no product.

use super::{TextLexicalCause, TextReadCause, TextReadRefusal};
use crate::bounded::Bounded;
use crate::diagnostic::{
    CAPTURE_FAMILY, Diagnostic, Family, IntrinsicRefused, LineBody, Observed, Phase, REPAIR_LIMIT,
    RefusalClass, Refused, Repair, Site, intrinsic_diagnostic,
};
use crate::request::Door;

impl core::fmt::Display for TextLexicalCause {
    fn fmt(&self, into: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        into.write_str(match self {
            Self::BlockCommentNotTerminated => "a block comment was never closed",
            Self::InvalidIdentifier => {
                "an identifier contains a character the compiler lexer rejects"
            }
            Self::UnknownPrefix => {
                "a token prefix requires parser context this boundary does not own"
            }
            Self::UnknownLifetimePrefix => {
                "a lifetime prefix requires parser context this boundary does not own"
            }
            Self::GuardedStringPrefix => {
                "a guarded-string prefix requires parser context this boundary does not own"
            }
            Self::MalformedLiteral => "a literal carries a malformed low-level spelling",
            Self::LifetimeStartsWithNumber => "a lifetime begins with a number",
            Self::Frontmatter => "frontmatter is not Rust token input at this boundary",
            Self::UnknownToken => "the compiler lexer reported no lawful Rust token kind",
        })
    }
}

impl core::error::Error for TextLexicalCause {}

impl core::fmt::Display for TextReadCause {
    fn fmt(&self, into: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotTerminated => into.write_str("a literal was never closed"),
            Self::NotEscapeFree => into
                .write_str("a literal carries an escape sequence the literal owner could not read"),
            Self::NotBalanced => into.write_str("a delimited group was never closed"),
            Self::NotOpened => into.write_str("a closing delimiter arrived with no group open"),
            Self::SourceBytesUnbounded => into.write_str(
                "the declared text carries more source bytes than the declared magnitude",
            ),
            Self::Lexical(cause) => write!(into, "{cause}"),
            Self::Unbounded(bound) => write!(into, "{bound}"),
        }
    }
}

impl core::fmt::Display for TextReadRefusal {
    fn fmt(&self, into: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(into, "{} at byte {}", self.cause, self.at)
    }
}

impl core::error::Error for TextReadRefusal {}

impl TextReadRefusal {
    /// Project this refused text read at the byte coordinate it established.
    ///
    /// No caller supplies a coordinate or a phase on this road, so a pre-capture diagnostic cannot be paired with a different byte, semantic-origin coordinate, or refusal vocabulary.
    pub fn diagnostic(&self, door: &Door) -> Diagnostic {
        intrinsic_diagnostic(&IntrinsicTextRead(self), door)
    }
}

/// The private diagnostic projection that keeps a text refusal off the caller-placed public road.
struct IntrinsicTextRead<'refusal>(&'refusal TextReadRefusal);

impl IntrinsicRefused for IntrinsicTextRead<'_> {
    fn site(&self) -> Site {
        Site::before_capture(self.0.coordinate())
    }
}

impl Refused for IntrinsicTextRead<'_> {
    const PHASE: Phase = Phase::Capture;
    const FAMILY: Family = CAPTURE_FAMILY;

    fn class(&self) -> RefusalClass {
        RefusalClass::DeclarationNotRead
    }

    fn first(&self) -> String {
        self.0.cause.to_string()
    }

    fn observed(&self) -> Observed {
        match self.0.cause {
            TextReadCause::SourceBytesUnbounded | TextReadCause::Unbounded(_) => {
                Observed::BoundExceeded
            }
            TextReadCause::NotTerminated
            | TextReadCause::NotEscapeFree
            | TextReadCause::NotBalanced
            | TextReadCause::NotOpened
            | TextReadCause::Lexical(_) => Observed::ContractDisagreement,
        }
    }

    fn body(&self) -> LineBody {
        LineBody::SingleCause
    }

    fn related(&self) -> Vec<Vec<u8>> {
        Vec::new()
    }

    fn repairs(&self) -> Bounded<Repair, REPAIR_LIMIT> {
        Bounded::empty()
    }
}
