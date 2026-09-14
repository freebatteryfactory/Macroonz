//! The trial home's declarations: the kind, its one seat, the question it owes, the row vocabulary in the harness's own field shape, and the payload one stamped trial table is written from.
//!
//! Declarations only.
//! Every road that reaches a private field lives in `type_guard.rs`, this file's own child.
//!
//! Producer authority is supplied by the emitter; optional target fragments stay inert until the carrier is consumed.

use crate::bounded::{Bounded, NonEmpty};
use crate::descriptor::{FunctionName, HelperRefusal, ModuleName, Name, SupportName};
use crate::token::GeneratedTree;

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
/// The three attribute-helper readings of one declaration share the captured-helper role and are told apart by position alone: this one is the first.
pub const TRIAL_HELPER_POSITION: u32 = 0;

/// The kind one trial declaration produces: a stamped trial table, delivered at the declaration site inside the carrier's stamped seat.
///
/// The unit's delivery is the declaration site because the table is stamp-grammar material and not Rust: it rides INERT inside the exported carrier the ordinary build merely defines, and it reaches the consumer's test target only when that target invokes the carrier and the gate forwards the seat to its stamp.
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
/// Attachment expressions may be stated beside the row or supplied at carrier invocation.
/// Both forms resolve in the consuming target; generated origin remains the producer's own act.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Row {
    lens: FunctionName,
    references: References,
    roles: Bounded<Name, ROLE_LIMIT>,
    tags: Bounded<Name, TAG_LIMIT>,
    attachment: Option<AttachmentExpressions>,
}

/// Nonempty expression templates for the consuming target's revisions and independent callable.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttachmentExpressions {
    subject_revision: GeneratedTree,
    check_revision: GeneratedTree,
    call: GeneratedTree,
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
    input_type: Option<GeneratedTree>,
}

/// How one trial helper body was not read.
///
/// Its own type, because a diagnostic's family tag is a fact about the type: this grammar is a declaration's FIRST helper reading, and the mutation grammar is its second.
#[must_use = "a trial capture refusal names the cause and the token it was established at"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TrialCaptureError(HelperRefusal);
