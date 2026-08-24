//! The trial home's declarations: the kind, its one seat, the question it owes, the row vocabulary in the harness's own field shape, and the payload one stamped trial table is written from.
//!
//! Declarations only.
//! Every road that reaches a private field lives in `type_guard.rs`, this file's own child.
//!
//! Nothing here has a seat for the producer's own act or for the consumption target's host facts; the home's README says where each of them lives instead.

use crate::bounded::{Bounded, NonEmpty};
use crate::descriptor::{FunctionName, HelperRefusal, ModuleName, Name, SupportName};

#[path = "type_guard.rs"]
mod guard;

/// Roles one row may carry.
///
/// A role is open classification, and a row carrying more than this has stopped classifying and started describing; the repair is a second row rather than a wider roster.
pub const ROLE_LIMIT: usize = 16;

/// Tags one row may carry.
///
/// Declared separately from [`ROLE_LIMIT`] rather than aliased to it: roles and tags are two capacities the harness declares as two rosters, and one number standing for both would be one authority answering two questions.
pub const TAG_LIMIT: usize = 16;

/// Rows one aggregate seat's group may declare.
///
/// Every row is one stamped lens function and one entry in the table the seat runs, so the group's size is what a consumer's test binary pays for.
pub const ROW_LIMIT: usize = 256;

/// Aggregate seats one stamped module may declare.
///
/// A seat is one ordinary test function selecting on one execution suite, and a module declaring more suites than this is a module whose rows belong to more than one world.
pub const SUITE_GROUP_LIMIT: usize = 32;

/// The transcript position a captured reading of this grammar is separated by.
///
/// Two helper readings of one declaration share the captured-helper role and are told apart by position alone, so the numbering is one closed space across the grammars this home declares: this one is the first.
pub const TRIAL_HELPER_POSITION: u32 = 0;

/// The kind one trial declaration produces: a stamped trial table, delivered to the consumer's test target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TrialTable;

/// The one seat a trial rendering fills.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrialRole {
    /// The stamped module carrying every declared row.
    Table,
}

/// The question a trial table owes beyond the universal ones.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrialQuestion {
    /// Which tests challenge the obligation this table stands for.
    WhichTestsChallenge,
}

/// The typed answer to [`TrialQuestion`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TrialAnswer {
    /// The table that challenges, and how many rows it declares.
    ChallengingTests {
        /// The table's own namespaced name.
        table: Name,
        /// How many rows stand under it, across every aggregate seat.
        rows: u64,
    },
}

/// The four namespaced references one row states about itself.
///
/// Every seat is required, because a row that could omit its claim, its subject, its check, or its population is a row the harness's closed field set refuses — and a shape that can express the refused row defers the refusal to somebody else's compiler.
///
/// The execution suite is not among them, and its absence is what keeps one suite from being authored twice: a row runs under exactly one aggregate seat, the seat is what a group declares, and a seat carrying rows whose own suite is a different name would be a seat that selects none of them.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct References {
    /// The claim this row serves.
    pub claim: Name,
    /// What this row exercises.
    pub subject: Name,
    /// The check that judges the subject.
    pub check: Name,
    /// The population that supplies this row's inputs.
    pub population: Name,
}

/// One descriptor row, in the harness's field shape, plus the lens the stamp declares it under.
///
/// The lens is not a row field — the harness's roster has no seat for it — and it is carried here because the stamp's grammar demands one: a row arrives as `<lens>: <expression>`, and a producer that did not name its lens would be handing the stamp an unnamable row.
///
/// There is no attachment seat and no origin seat.
/// An attachment's three parts live in the consumption target and arrive as expressions at the carrier's invocation; an origin is the producer's own act, composed inside the rendering, so a row that cannot express one cannot express the wrong one.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Row {
    lens: FunctionName,
    references: References,
    roles: Bounded<Name, ROLE_LIMIT>,
    tags: Bounded<Name, TAG_LIMIT>,
}

/// One aggregate seat's group: the function the stamp declares, the execution suite that seat selects on, and the rows declared under it.
///
/// The suite is stated here and inherited by every row under it, so the pairing a stamp cannot check at expansion is one no declaration can get wrong.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SuiteGroup {
    seat: FunctionName,
    suite: Name,
    rows: NonEmpty<Row, ROW_LIMIT>,
}

/// The complete payload one stamped trial table is declared from.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Trials {
    support: SupportName,
    module: ModuleName,
    table: Name,
    groups: NonEmpty<SuiteGroup, SUITE_GROUP_LIMIT>,
}

/// How one trial helper body was not read.
///
/// Its own type, because a diagnostic's family tag is a fact about the type: this grammar is a declaration's FIRST helper reading, and the mutation grammar is its second.
#[must_use = "a trial capture refusal names the cause and the token it was established at"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TrialCaptureError(HelperRefusal);
