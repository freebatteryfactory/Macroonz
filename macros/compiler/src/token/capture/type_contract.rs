//! The capture home's stated tables and its trait implementations.
//!
//! Every refusal this home raises renders as one sentence about a declared input and names no caller, no attribute, and no product.
//! A projection that is a constant per row sits beside the row rather than in a second file that has to be kept in step with it.

use super::{
    CaptureBound, CapturedAtom, CapturedPayload, LiteralReadCause, SpanResolutionRefusal,
    TextLexicalCause, TextReadCause, TextReadRefusal,
};
use crate::bounded::Bounded;
use crate::diagnostic::{
    CAPTURE_FAMILY, Diagnostic, Family, IntrinsicRefused, LineBody, Observed, Phase, REPAIR_LIMIT,
    RefusalClass, Refused, Repair, Site, intrinsic_diagnostic,
};
use crate::request::Door;

impl From<CapturedAtom> for CapturedPayload {
    fn from(atom: CapturedAtom) -> Self {
        match atom {
            CapturedAtom::Word(word) => Self::Word(word),
            CapturedAtom::Punct(mark) => Self::Punct(mark),
            CapturedAtom::Text(text) => Self::Text(text),
            CapturedAtom::Number(number) => Self::Number(number),
            CapturedAtom::ByteText(material) => Self::ByteText(material),
            CapturedAtom::Character(character) => Self::Character(character),
            CapturedAtom::Byte(byte) => Self::Byte(byte),
            CapturedAtom::NulTerminatedText(material) => Self::NulTerminatedText(material),
            CapturedAtom::RawIdentifier(name) => Self::RawIdentifier(name),
            CapturedAtom::JointPunct(mark) => Self::JointPunct(mark),
        }
    }
}

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

impl core::fmt::Display for CaptureBound {
    fn fmt(&self, into: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        into.write_str(match self {
            Self::Depth => "the declared input nests deeper than the declared magnitude",
            Self::Level => {
                "one nesting level of the declared input carries more tokens than the declared magnitude"
            }
            Self::Tree => "the declared input carries more tokens than the declared magnitude",
            Self::Work => "reading the declared input spent the declared capture-work budget",
        })
    }
}

impl core::error::Error for CaptureBound {}

impl core::fmt::Display for LiteralReadCause {
    fn fmt(&self, into: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        into.write_str(match self {
            Self::NotAKnownForm => {
                "the declared input carries a literal written in a form this grammar has no row for"
            }
            Self::NotReadable => {
                "the declared input carries a literal this grammar could not read the value of"
            }
        })
    }
}

impl core::error::Error for LiteralReadCause {}

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

impl core::fmt::Display for SpanResolutionRefusal {
    fn fmt(&self, into: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            into,
            "the producer's span table carries {} position(s) and does not reach handle {}",
            self.reaches,
            self.handle.index()
        )
    }
}

impl core::error::Error for SpanResolutionRefusal {}
