//! The seam's stated tables and its trait implementations.
//!
//! Every refusal this home raises renders as one sentence about a declared input and names no caller, no attribute, and no product.
//! A projection that is a constant per row sits beside the row rather than in a second file that has to be kept in step with it.

use super::{
    CaptureBound, LiteralReadCause, SpanResolutionRefusal, TextReadCause, TextReadRefusal,
};

impl CaptureBound {
    /// Every bound a capture can run past, in declaration order.
    pub const ALL: &'static [Self] = &[Self::Depth, Self::Level, Self::Tree, Self::Work];

    /// The stable name of the magnitude this row is about.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Depth => "depth",
            Self::Level => "level",
            Self::Tree => "tree",
            Self::Work => "work",
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

impl LiteralReadCause {
    /// Every way a literal spelling can fail to be read, in declaration order.
    pub const ALL: &'static [Self] = &[Self::NotAKnownForm, Self::NotReadable];

    /// The stable name of this row.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::NotAKnownForm => "not-a-known-form",
            Self::NotReadable => "not-readable",
        }
    }
}

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
            Self::NotTerminated => into.write_str("a text literal was never closed"),
            Self::NotEscapeFree => {
                into.write_str("a text literal carries an escape sequence this route does not read")
            }
            Self::NotBalanced => into.write_str("a delimited group was never closed"),
            Self::NotOpened => into.write_str("a closing delimiter arrived with no group open"),
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
