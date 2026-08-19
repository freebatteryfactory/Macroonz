//! The test-descriptor home's declarative surface: the tables and trait
//! implementations this home states rather than computes.
//!
//! Three declarations stand here.
//!
//! The LIMIT FAMILIES: each family's capacity authority and its magnitude are
//! written on adjacent rows, so a family cannot be declared on the compile-time
//! ladder while wearing another road's authority — [`Limit::Authority`] resolves
//! to one type, and naming [`DeclaredMagnitude`] there is what makes
//! [`ConstLimit`] implementable at all. The families themselves are declared
//! beside the capacities they govern in `types.rs`; what a family is FOR is said
//! there, and the number is said here.
//!
//! The REFUSAL FAMILY's declared shape: an issue collection, because a shell can
//! need a literal spelling the token vocabulary lacks AND outgrow the token
//! magnitude at once, and a caller repairing a seam one gap per attempt is a
//! caller this home failed.
//!
//! The CONVERSION BILL: the exact `From` implementations the harness owes before
//! a rendered row expression type-checks at a consumer's site. It is a constant
//! table over a closed roster rather than a sentence in a README, so a reader can
//! read the bill back and a compiler keeps the match exhaustive when a row is
//! added.

use super::{
    CrateFacing, PathSegmentLimit, RoleLimit, RowLimit, ShellIssueLimit, ShellRendering,
    SuiteGroupLimit, TagLimit,
};
use threadpak::refusal::{FamilyShape, RefusalFamily};
use threadpak::types::{ConstLimit, DeclaredMagnitude, Limit};

impl Limit for PathSegmentLimit {
    type Authority = DeclaredMagnitude;
}

impl ConstLimit for PathSegmentLimit {
    const MAX: usize = 8;
}

impl Limit for RoleLimit {
    type Authority = DeclaredMagnitude;
}

impl ConstLimit for RoleLimit {
    const MAX: usize = 16;
}

impl Limit for TagLimit {
    type Authority = DeclaredMagnitude;
}

impl ConstLimit for TagLimit {
    const MAX: usize = 16;
}

impl Limit for RowLimit {
    type Authority = DeclaredMagnitude;
}

impl ConstLimit for RowLimit {
    const MAX: usize = 256;
}

impl Limit for SuiteGroupLimit {
    type Authority = DeclaredMagnitude;
}

impl ConstLimit for SuiteGroupLimit {
    const MAX: usize = 32;
}

impl Limit for ShellIssueLimit {
    type Authority = DeclaredMagnitude;
}

impl ConstLimit for ShellIssueLimit {
    /// Sixteen. The rendering's issues are facts about the token vocabulary and
    /// about the harness's refusal composition, so their count is bounded by the
    /// number of distinct spellings one shell needs rather than by the rows it
    /// carries. Written as the number rather than as a product of the row
    /// magnitude beside it: a magnitude derived from another magnitude reads as a
    /// fact when it is a choice.
    const MAX: usize = 16;
}

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

/// One conversion the harness owes before a rendered row expression type-checks
/// at a consumer's site: the refusal a part constructor answers with, and the
/// family the row expression is declared to refuse in.
///
/// # Authority
///
/// **The bill is stated and never worked around.** The stamp declares a row
/// expression's type as `Result<Binding<…>, BindingRefusal>`, the part
/// constructors on the road to a binding answer with three other families, and no
/// arm of the binding family carries any of them — so a generated expression that
/// builds its own parts has no lawful discharge and this home writes `?`, which
/// is exactly the standard conversion the harness's own families are one
/// implementation away from admitting.
///
/// Writing `?` rather than naming a variant is the least-invention road: a
/// variant spelled here would be this home legislating inside a vocabulary it
/// does not own, while `From` is the machine-neutral conversion the language
/// already declares. The rendering states the requirement; the mailbox side owns
/// whether it is met by a `From` implementation, by an added arm, or by a total
/// name constructor that removes the refusal entirely.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConversionOwed {
    /// The part constructor's own refusal family, as the harness spells it.
    pub from: &'static str,
    /// The family the row expression is declared to refuse in.
    pub into: &'static str,
    /// Which construction on the road to a binding answers with `from`.
    pub raised_by: &'static str,
}

/// The complete conversion bill one rendered row expression stands on, in the
/// order the expression performs the constructions.
///
/// Five rows and no more: every other construction on the road to a binding is
/// total, and a bill that listed a total road would be asking for a conversion
/// nobody can be handed a value to perform.
///
/// The last two are the SCHEMA PIN's, and they are on this bill for a structural
/// reason rather than a convenient one: a row carrying producer facts is refused
/// by the binding constructor unless the binding names the schema the producer
/// emitted against, so every generated row derives that identity inside its own
/// expression and every generated row therefore owes the two refusals the
/// derivation answers with.
pub const CONVERSIONS_OWED: [ConversionOwed; 5] = [
    ConversionOwed {
        from: "NameRefusal",
        into: "BindingRefusal",
        raised_by: "the namespaced reference parsers",
    },
    ConversionOwed {
        from: "ClassificationRefusal",
        into: "BindingRefusal",
        raised_by: "the classification constructor",
    },
    ConversionOwed {
        from: "RowRefusal",
        into: "BindingRefusal",
        raised_by: "the row constructor",
    },
    ConversionOwed {
        from: "SchemaRefusal",
        into: "BindingRefusal",
        raised_by: "the published root schema declaration",
    },
    ConversionOwed {
        from: "EncodeRefusal",
        into: "BindingRefusal",
        raised_by: "the schema identity derivation",
    },
];
