//! Reading one authored trial declaration out of a typed token tree.
//!
//! The [authored grammar](super#authored-grammar) belongs to this home's public documentation.

use super::{References, Row, SuiteGroup, TrialCaptureError, Trials};
use crate::descriptor::clause::{
    Clause, assigned, assignment_clauses, declaration_clauses, identifier, named_reference,
    named_value,
};
use crate::descriptor::{
    CaptureCause, DeclarationError, FunctionName, Grammar, ModuleName, Name, SupportName,
};
use crate::token::{CapturedDelimiter, CapturedTokenTree, SpanHandle};
use core::convert::Infallible;

/// The clause naming the exported support name.
const SUPPORT: &str = "support";

/// The clause naming the stamped module.
const MODULE: &str = "module";

/// The clause naming the authored table.
const TABLE: &str = "table";

/// The word one aggregate seat's group opens with.
const SUITE: &str = "suite";

/// The row clause naming the claim a row serves.
const CLAIM: &str = "claim";

/// The row clause naming the roles a row carries.
const ROLES: &str = "roles";

/// The row clause naming the tags a row carries.
const TAGS: &str = "tags";

/// The row clause naming what a row exercises.
const SUBJECT: &str = "subject";

/// The row clause naming the check that judges the subject.
const CHECK: &str = "check";

/// The row clause naming the population that supplies a row's inputs.
const POPULATION: &str = "population";

/// The clause keys this grammar declares at a declaration's own level.
const DECLARABLE: [&str; 3] = [SUPPORT, MODULE, TABLE];

/// The clause keys one row admits.
///
/// Its own roster rather than the declaration level's, because the two levels admit different keys and one roster standing for both would let a table's clause be written inside a row and read as lawful.
const DECLARABLE_ROW: [&str; 6] = [CLAIM, ROLES, TAGS, SUBJECT, CHECK, POPULATION];

/// Read one trial payload out of the helper attribute's body.
///
/// # Errors
///
/// Returns [`TrialCaptureError`] where the tokens do not say a trial declaration, and where the values they say are not a lawful declaration — each at the token the clause it was established at sits at.
pub fn captured(
    body: &[&CapturedTokenTree],
    at: SpanHandle,
    grammar: Grammar,
) -> Result<Trials, TrialCaptureError> {
    let clauses = declaration_clauses(grammar, body, &DECLARABLE, suite_clause, refused)?;
    let support = SupportName::declared(identifier(grammar, &clauses, SUPPORT, at, refused)?)
        .map_err(|refusal| carried(grammar, refusal, at))?;
    let module = ModuleName::declared(identifier(grammar, &clauses, MODULE, at, refused)?)
        .map_err(|refusal| carried(grammar, refusal, at))?;
    let table = named_reference(grammar, &clauses, TABLE, at, refused, carried)?;

    let mut groups: Vec<SuiteGroup> = Vec::new();
    for clause in &clauses {
        if let Some(suite) = clause.nested_value() {
            groups.push(suite_group(grammar, suite.seat, &suite.suite, &suite.rows)?);
        }
    }
    Trials::declared(support, module, table, groups)
        .map_err(|refusal| carried(grammar, refusal, at))
}

/// One established grammar refusal at one token.
const fn refused(grammar: Grammar, cause: CaptureCause, at: SpanHandle) -> TrialCaptureError {
    TrialCaptureError::grammar_refused(grammar, cause, at)
}

/// One vocabulary refusal carried whole, at the token the value was read from.
const fn carried(grammar: Grammar, refusal: DeclarationError, at: SpanHandle) -> TrialCaptureError {
    TrialCaptureError::vocabulary_refused(grammar, refusal, at)
}

/// One trial-owned suite clause.
struct SuiteClause<'trees> {
    seat: &'trees CapturedTokenTree,
    suite: Vec<&'trees CapturedTokenTree>,
    rows: Vec<&'trees CapturedTokenTree>,
}

/// Read one trial-owned suite where the group states one.
fn suite_clause<'trees>(
    grammar: Grammar,
    group: &[&'trees CapturedTokenTree],
) -> Result<Option<SuiteClause<'trees>>, TrialCaptureError> {
    let Some((head, rest)) = group.split_first() else {
        return Ok(None);
    };
    if head.word() != Some(SUITE) {
        return Ok(None);
    }
    let opening = head.span();
    let malformed = || refused(grammar, CaptureCause::GroupUnread, opening);
    let (seat, after_seat) = rest.split_first().ok_or_else(malformed)?;
    if seat.word().is_none() {
        return Err(refused(grammar, CaptureCause::GroupUnread, seat.span()));
    }
    let (assigned_by, after_assignment) = after_seat.split_first().ok_or_else(malformed)?;
    if assigned_by.punct() != Some('=') {
        return Err(refused(
            grammar,
            CaptureCause::GroupUnread,
            assigned_by.span(),
        ));
    }
    let (body, suite) = after_assignment.split_last().ok_or_else(malformed)?;
    match body.group() {
        Some((CapturedDelimiter::Brace, inner)) => Ok(Some(SuiteClause {
            seat,
            suite: suite.to_vec(),
            rows: inner.iter().collect(),
        })),
        Some(_) | None => Err(refused(grammar, CaptureCause::GroupUnread, body.span())),
    }
}

/// One bracketed roster of namespaced references a row clause assigns, or an empty roster where the clause is absent.
fn roster(
    grammar: Grammar,
    clauses: &[Clause<'_, Infallible>],
    key: &str,
) -> Result<Vec<Name>, TrialCaptureError> {
    let Some((value, at)) = assigned(clauses, key) else {
        return Ok(Vec::new());
    };
    let [bracketed] = value else {
        return Err(refused(grammar, CaptureCause::RosterUnread, at));
    };
    let Some((CapturedDelimiter::Bracket, inner)) = bracketed.group() else {
        return Err(refused(
            grammar,
            CaptureCause::RosterUnread,
            bracketed.span(),
        ));
    };
    let mut named: Vec<Name> = Vec::new();
    let mut group: Vec<&CapturedTokenTree> = Vec::new();
    for tree in inner {
        if tree.punct() == Some(',') {
            if group.is_empty() {
                return Err(refused(
                    grammar,
                    CaptureCause::SeparatorDangling,
                    tree.span(),
                ));
            }
            named.push(named_value(
                grammar,
                &group,
                bracketed.span(),
                refused,
                carried,
            )?);
            group.clear();
        } else {
            group.push(tree);
        }
    }
    if !group.is_empty() {
        named.push(named_value(
            grammar,
            &group,
            bracketed.span(),
            refused,
            carried,
        )?);
    }
    Ok(named)
}

/// One aggregate seat's group: the seat, the suite it selects on, and the rows declared under it.
fn suite_group(
    grammar: Grammar,
    seat: &CapturedTokenTree,
    suite: &[&CapturedTokenTree],
    rows: &[&CapturedTokenTree],
) -> Result<SuiteGroup, TrialCaptureError> {
    let at = seat.span();
    let spelling = seat
        .word()
        .ok_or_else(|| refused(grammar, CaptureCause::GroupUnread, at))?;
    let named =
        FunctionName::declared(spelling).map_err(|refusal| carried(grammar, refusal, at))?;
    let selected = named_value(grammar, suite, at, refused, carried)?;
    let mut declared: Vec<Row> = Vec::new();
    let mut group: Vec<&CapturedTokenTree> = Vec::new();
    for tree in rows {
        if tree.punct() == Some(',') {
            if group.is_empty() {
                return Err(refused(
                    grammar,
                    CaptureCause::SeparatorDangling,
                    tree.span(),
                ));
            }
            declared.push(row(grammar, &group, at)?);
            group.clear();
        } else {
            group.push(tree);
        }
    }
    if !group.is_empty() {
        declared.push(row(grammar, &group, at)?);
    }
    SuiteGroup::declared(named, selected, declared).map_err(|refusal| carried(grammar, refusal, at))
}

/// One row: the lens it is declared under, and the references it states about itself.
///
/// The seat's own token is the fallback site rather than an invented position: a group that spells no row is a fact about the seat that declared it.
fn row(
    grammar: Grammar,
    group: &[&CapturedTokenTree],
    seat: SpanHandle,
) -> Result<Row, TrialCaptureError> {
    let [named, body] = group else {
        let at = group.first().map_or(seat, |tree| tree.span());
        return Err(refused(grammar, CaptureCause::RowUnread, at));
    };
    let at = named.span();
    let spelling = named
        .word()
        .ok_or_else(|| refused(grammar, CaptureCause::RowUnread, at))?;
    let lens = FunctionName::declared(spelling).map_err(|refusal| carried(grammar, refusal, at))?;
    let Some((CapturedDelimiter::Brace, inner)) = body.group() else {
        return Err(refused(grammar, CaptureCause::RowUnread, body.span()));
    };
    let trees: Vec<&CapturedTokenTree> = inner.iter().collect();
    let clauses = assignment_clauses(grammar, &trees, &DECLARABLE_ROW, refused)?;
    let references = References {
        claim: named_reference(grammar, &clauses, CLAIM, at, refused, carried)?,
        subject: named_reference(grammar, &clauses, SUBJECT, at, refused, carried)?,
        check: named_reference(grammar, &clauses, CHECK, at, refused, carried)?,
        population: named_reference(grammar, &clauses, POPULATION, at, refused, carried)?,
    };
    let roles = roster(grammar, &clauses, ROLES)?;
    let tags = roster(grammar, &clauses, TAGS)?;
    Row::declared(lens, references, roles, tags).map_err(|refusal| carried(grammar, refusal, at))
}
