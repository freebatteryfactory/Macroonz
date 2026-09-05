//! The capture home's stated tables and its trait implementations.
//!
//! Every refusal this home raises renders as one sentence about a declared input and names no caller, no attribute, and no product.
//! A projection that is a constant per row sits beside the row rather than in a second file that has to be kept in step with it.

use super::{CaptureBound, CapturedAtom, CapturedPayload, LiteralReadCause, SpanResolutionRefusal};

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
