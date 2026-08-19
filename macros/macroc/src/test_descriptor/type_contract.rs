//! The test-descriptor home's declarative surface: the tables and trait
//! implementations this home states rather than computes.
//!
//! Two declarations stand here.
//!
//! The REFUSAL FAMILY's declared shape: an issue collection, because one
//! crossing renders several parts independently — the carrier rides two of them,
//! the bench crossing three — and each can outgrow the token magnitude on its
//! own, so no primary issue is ever elected and a caller repairing a seam one
//! part per attempt is a caller this home failed.
//!
//! The ROW CONVERSION MAP: which `From` each `?` in a rendered row expression
//! travels through, and who owns it. It is a constant table over a closed roster
//! rather than a sentence in a README, so a reader can read the map back and a
//! compiler keeps the match exhaustive when a row is added.

use super::{CrateFacing, ShellRendering};
use threadpak::refusal::{FamilyShape, RefusalFamily};

impl RefusalFamily for ShellRendering {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
    const SELECTION_ORDER: &'static [&'static str] = &[];
}

impl CrateFacing {
    /// The shell's own metavariable spelling for this twin.
    ///
    /// The one place a twin becomes a name, so the shell's MATCHER and every path
    /// its body renders take the same answer from the same road rather than two
    /// spellings that agree until one of them is edited.
    ///
    /// A constant answer over a closed roster, so a third twin admitted later
    /// stops the compiler here until somebody says what the consumer calls it.
    #[must_use]
    pub const fn parameter(self) -> &'static str {
        match self {
            Self::Machine => "machine",
            Self::Harness => "harness",
        }
    }
}

/// One conversion a rendered row expression's `?` travels through: the refusal a
/// part constructor answers with, and the family the row expression is declared
/// to refuse in.
///
/// # Authority
///
/// **This is the DISCHARGE RECORD, and the address owns every arm on it.** The
/// stamp declares a row expression's type as
/// `Result<Binding<…>, TrialTableRefusal>`, the part constructors on the road to
/// a binding answer with their own families, and the descriptor home's own
/// `type_contract` declares one `From` per family into exactly that type — so
/// every `?` this home writes has a lawful discharge already published at the
/// address. The record is what a reader joins the two sides by; the roster that
/// SETTLES it is the address's, and this table is never a second copy of that
/// authority.
///
/// Writing `?` rather than naming a variant is the least-invention road: a
/// variant spelled here would be this home legislating inside a vocabulary it
/// does not own, while `From` is the machine-neutral conversion the language
/// already declares. That the discharge exists is the address's statement; what
/// this table adds is which construction in THIS emission reaches it.
///
/// # Nonclaims
///
/// It claims nothing about the arm each conversion lands on. Which variant of
/// the trial-table family a refusal becomes is the address's declaration, stated
/// once in its own roster; a copy of that mapping here would be a second answer
/// to a question this home does not own.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RowConversion {
    /// The part constructor's own refusal family, as the harness spells it.
    pub from: &'static str,
    /// The family the row expression is declared to refuse in.
    pub into: &'static str,
    /// Which construction on the road to a binding answers with `from`.
    pub raised_by: &'static str,
}

/// Every conversion one rendered row expression stands on, in the order the
/// expression performs the constructions.
///
/// Five rows and no more: every other construction on the road to a binding is
/// total, and a row here for a total road would name a conversion nobody can be
/// handed a value to perform.
///
/// The last two are the SCHEMA PIN's, and they are on this record for a
/// structural reason rather than a convenient one: a row carrying producer facts
/// is refused by the binding constructor unless the binding names the schema the
/// producer emitted against, so every generated row derives that identity inside
/// its own expression and every generated row therefore travels the two
/// conversions the derivation's refusals discharge through.
pub const ROW_CONVERSIONS: [RowConversion; 5] = [
    RowConversion {
        from: "NameRefusal",
        into: "TrialTableRefusal",
        raised_by: "the namespaced reference parsers",
    },
    RowConversion {
        from: "ClassificationRefusal",
        into: "TrialTableRefusal",
        raised_by: "the classification constructor",
    },
    RowConversion {
        from: "RowRefusal",
        into: "TrialTableRefusal",
        raised_by: "the row constructor",
    },
    RowConversion {
        from: "SchemaRefusal",
        into: "TrialTableRefusal",
        raised_by: "the published root schema declaration",
    },
    RowConversion {
        from: "EncodeRefusal",
        into: "TrialTableRefusal",
        raised_by: "the schema identity derivation",
    },
];
