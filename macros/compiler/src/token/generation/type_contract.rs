//! Trait contracts for exact generated literals and preserved-fragment refusals.

use super::{
    FragmentGenerationIssue, FragmentGenerationRefusal, GeneratedLiteralRefusal,
    GeneratedRowRefusal, GeneratedTokenIssue, GeneratedTree, GeneratedTreeRefusal,
};

impl GeneratedTokenIssue {
    /// The stable lexical-role slot used by owning refusal encodings.
    #[must_use]
    pub const fn slot(self) -> u8 {
        match self {
            Self::Word => 0,
            Self::RawIdentifier => 1,
            Self::Punctuation => 2,
        }
    }
}

impl core::fmt::Display for GeneratedTokenIssue {
    fn fmt(&self, into: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        into.write_str(match self {
            Self::Word => "the spelling is not one Rust word token",
            Self::RawIdentifier => "the name is not one permitted Rust raw identifier",
            Self::Punctuation => "the character is not one Rust punctuation token",
        })
    }
}

impl core::error::Error for GeneratedTokenIssue {}

impl core::fmt::Display for GeneratedTreeRefusal {
    fn fmt(&self, into: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Unbounded(overflow) => write!(into, "{overflow}"),
            Self::Token { position, issue } => {
                write!(into, "generated token {position} refused: {issue}")
            }
        }
    }
}

impl core::error::Error for GeneratedTreeRefusal {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Unbounded(overflow) => Some(overflow),
            Self::Token { issue, .. } => Some(issue),
        }
    }
}

impl From<crate::bounded::Overflow> for GeneratedTreeRefusal {
    fn from(overflow: crate::bounded::Overflow) -> Self {
        Self::Unbounded(overflow)
    }
}

impl core::fmt::Debug for GeneratedTree {
    fn fmt(&self, into: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        into.debug_tuple("GeneratedTree")
            .field(&self.tokens())
            .finish()
    }
}

impl PartialEq for GeneratedTree {
    fn eq(&self, other: &Self) -> bool {
        self.tokens() == other.tokens()
    }
}

impl Eq for GeneratedTree {}

impl core::hash::Hash for GeneratedTree {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(self.tokens(), state);
    }
}

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
            Self::Token(issue) => write!(into, "{issue}"),
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
