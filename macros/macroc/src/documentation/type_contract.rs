//! The documentation home's declarative surface: the tables and trait
//! implementations this home states rather than computes.
//!
//! Two declarations stand here.
//!
//! The REFUSAL FAMILY's declared shape: an issue collection, because a plan may
//! cover several facets nobody wrote while an item writes several sections nobody
//! covered, and a caller repairing coverage one section per attempt is a caller
//! this home failed.
//!
//! The FACT ROSTER: what each typed fact traces to and whether this home can
//! spell it today. It is a constant table over a closed roster rather than a
//! sentence in a README, so a reader can read the discipline back and the
//! compiler keeps the roster and the table the same length.

use super::DocumentationCoverage;
use threadpak::refusal::{FamilyShape, RefusalFamily};

impl RefusalFamily for DocumentationCoverage {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
    const SELECTION_ORDER: &'static [&'static str] = &[];
}

/// Where one typed fact's rendered sentence comes from, and whether this home can
/// write it today.
///
/// # Authority
///
/// **Every rendered sentence traces to exactly one row of this table or to the
/// owner's own text, and there is no third source.** The table is what makes
/// "never invented prose" readable back rather than promised: a reader checks that
/// each row's source is a typed value the fact carries, and the compiler checks
/// that no arm of the roster is missing a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FactSource {
    /// The fact arm this row is about, by the spelling the declaration uses.
    pub fact: &'static str,
    /// The typed value the sentence is composed from.
    pub traces_to: &'static str,
    /// Whether this home can spell the sentence under the vocabularies as they
    /// stand.
    pub spellable: FactSpelling,
}

/// Whether one fact's sentence can be written today, and on whose vocabulary that
/// turns.
///
/// Not a boolean: an unspellable fact is unspellable for a stated reason that
/// names the seat closing it, and a bare `false` would say a sentence could not be
/// written without saying whose declaration would let it be.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FactSpelling {
    /// The sentence is composed from typed values this home already reads.
    Spellable,
    /// The sentence needs a declared name the owning roster does not carry, and
    /// this is the seat that closes it.
    AwaitingDeclaredName {
        /// The roster that owes the name.
        roster: &'static str,
    },
}

/// The complete fact roster, one row per admitted arm, in the roster's own order.
///
/// Five rows and no more, because the roster is five: a row added here without an
/// arm beside it, or an arm added without a row, is a length disagreement the
/// declaration itself carries.
///
/// The last row is the one this home cannot spell, and it says exactly why: the
/// machine's facet roster is a plain enum with no declared stable name, so a
/// sentence naming a facet would be these services legislating a spelling inside a
/// vocabulary the machine owns.
pub const FACT_ROSTER: [FactSource; 5] = [
    FactSource {
        fact: "ProjectionKindName",
        traces_to: "the kind's own declared stable name",
        spellable: FactSpelling::Spellable,
    },
    FactSource {
        fact: "CausingDeclaration",
        traces_to: "the entry account's one anchored cause address",
        spellable: FactSpelling::Spellable,
    },
    FactSource {
        fact: "OutputIdentity",
        traces_to: "the planned member's semantic key",
        spellable: FactSpelling::Spellable,
    },
    FactSource {
        fact: "Assumption",
        traces_to: "one cited owner fact, minted or declared",
        spellable: FactSpelling::Spellable,
    },
    FactSource {
        fact: "CoveredFacet",
        traces_to: "one facet the plan covers",
        spellable: FactSpelling::AwaitingDeclaredName {
            roster: "the machine's declaration home, on Facet",
        },
    },
];
