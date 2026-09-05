//! The item home's trait implementations.
//!
//! Every refusal here renders as one sentence about a declared item boundary and names no caller, no attribute, and no product.

use super::{AuthoredItemKind, AuthoredItemReadIssue, AuthoredItemReadRefusal};

impl core::fmt::Display for AuthoredItemKind {
    fn fmt(&self, into: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        into.write_str(match self {
            Self::Module => "module",
            Self::Structure => "structure",
            Self::Enumeration => "enumeration",
            Self::Union => "union",
            Self::Trait => "trait",
            Self::Function => "function",
            Self::Implementation => "implementation",
            Self::TypeAlias => "type alias",
            Self::Constant => "constant",
            Self::Static => "static",
            Self::Use => "use item",
            Self::ExternalCrate => "external-crate item",
        })
    }
}

impl core::fmt::Display for AuthoredItemReadIssue {
    fn fmt(&self, into: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ItemMissing => into.write_str("the declared item boundary carries no token"),
            Self::ItemKindMissing => into.write_str(
                "the declared item boundary carries no supported structural item-family keyword",
            ),
            Self::ItemNameMissing(kind) => {
                write!(
                    into,
                    "the declared {kind} carries no identifier in its name seat"
                )
            }
            Self::ItemBoundaryUnfinished(kind) => write!(
                into,
                "the declared {kind} boundary ends without a braced body or semicolon"
            ),
            Self::LensRangeContradiction => into.write_str(
                "an authored-item structural coordinate does not belong to its captured boundary",
            ),
        }
    }
}

impl core::fmt::Display for AuthoredItemReadRefusal {
    fn fmt(&self, into: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.token() {
            Some(token) => write!(into, "{} at captured span {}", self.issue(), token.index()),
            None => write!(into, "{} at the declaration boundary", self.issue()),
        }
    }
}

impl core::error::Error for AuthoredItemReadRefusal {}
