//! Trait contracts for exact generated literals and preserved-fragment refusals.

use super::{
    FragmentGenerationIssue, FragmentGenerationRefusal, GeneratedLiteralRefusal,
    GeneratedRowRefusal,
};

impl core::fmt::Display for GeneratedRowRefusal {
    fn fmt(&self, into: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            into,
            "generated row {} refused: {}",
            self.position(),
            self.cause()
        )
    }
}

impl core::error::Error for GeneratedRowRefusal {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        Some(self.cause_ref())
    }
}

impl core::fmt::Display for GeneratedLiteralRefusal {
    fn fmt(&self, into: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        into.write_str(match self {
            Self::NotANumber => "the exact literal spelling is not one numeric literal",
            Self::InteriorNul => "the C-string literal material contains an interior NUL byte",
        })
    }
}

impl core::error::Error for GeneratedLiteralRefusal {}

impl core::fmt::Display for FragmentGenerationIssue {
    fn fmt(&self, into: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Literal(issue) => write!(into, "{issue}"),
            Self::Unbounded => {
                into.write_str("the preserved fragment exceeds the generated-token magnitude")
            }
        }
    }
}

impl core::fmt::Display for FragmentGenerationRefusal {
    fn fmt(&self, into: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.token() {
            Some(token) => write!(into, "{} at captured span {}", self.issue(), token.index()),
            None => write!(into, "{} at the fragment boundary", self.issue()),
        }
    }
}

impl core::error::Error for FragmentGenerationRefusal {}
