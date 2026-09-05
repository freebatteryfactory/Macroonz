//! Reading one authored bench declaration out of a typed token tree.
//!
//! # The authored grammar
//!
//! ```text
//! #[<helper>(
//!     support = <exported name>,
//!     table_function = <table function name>,
//!     table = named("<namespace>", "<stem>"),
//!     reporter = <reporter module name>,
//!
//!     <lens> {
//!         workload = named("<namespace>", "<stem>"),
//!         preflight = named("<namespace>", "<stem>"),
//!         planted_worse = named("<namespace>", "<stem>"),
//!         complexity = named("<namespace>", "<stem>"),
//!         axis = [<size>, <size>, ...],
//!         samples = <count>,
//!         warmups = <count>,
//!         ratio_numerator = <count>,
//!         ratio_denominator = <count>,
//!         formula = "<work formula>",
//!         observe = [named("<namespace>", "<stem>"), ...],
//!     },
//! )]
//! ```
//!
//! The helper's own spelling is the caller's, which is why `<helper>` stands where a word would: a door registers the attribute it wants and hands the same [`Grammar`] to this reading, so a refusal names the word an author actually wrote.
//!
//! `formula` may be left out; every other row clause is required.
//! An operation that declares no work formula states that by carrying none.
//!
//! Every count is one unsuffixed decimal literal, because a count that arrives typed, based, or separated is a spelling this reading would have to interpret, and interpreting a spelling is deciding what an author meant by a value it could not read.
//!
//! Callables, the judge, the complete preflight, and the report reader are target-owned expressions and therefore are not authored here.
//!
//! # What has no clause, and why
//!
//! The contention posture has none, because one arm is all the declared facts support and a clause with one lawful value is a sentence that says what silence already says.
//! The producer's own act and the consumption target's host facts have none, on the trial grammar's own terms: the first is composed inside the rendering from the emitter the caller declares, and the second arrives as expressions at the carrier's invocation.
//!
//! # Order
//!
//! Clause order inside a body is free and is read by key.
//! Order between ROSTER members is meaning and is preserved: the rows in the order they were written, each axis in the order its sizes were written, and each observation roster in the order its references were written.

use super::{
    BenchCaptureError, BenchmarkDeclaration, Budgets, ContentionPosture, Measurement, References,
    Reporter, Row, WorkFormula,
};
use crate::descriptor::clause::{
    Clause, assigned, assignment_clauses, declaration_clauses, identifier, named_reference,
    named_value, number,
};
use crate::descriptor::{
    CaptureCause, DeclarationError, FunctionName, Grammar, ModuleName, Name, SupportName,
};
use crate::token::{CapturedDelimiter, CapturedTokenTree, SpanHandle};
use core::convert::Infallible;

/// The clause naming the exported support name.
const SUPPORT: &str = "support";

/// The clause naming the stamped table function.
const TABLE_FUNCTION: &str = "table_function";

/// The clause naming the authored table.
const TABLE: &str = "table";

/// The clause naming the report-reader module.
const REPORTER: &str = "reporter";

/// The row clause naming what is measured.
const WORKLOAD: &str = "workload";

/// The row clause naming the correctness preflight's reference.
const PREFLIGHT: &str = "preflight";

/// The row clause naming the planted-worse falsifier's reference.
const PLANTED_WORSE: &str = "planted_worse";

/// The row clause naming the neutral complexity claim.
const COMPLEXITY: &str = "complexity";

/// The row clause stating the input-size axis.
const AXIS: &str = "axis";

/// The row clause stating how many samples the gate takes at each point.
const SAMPLES: &str = "samples";

/// The row clause stating how many warmup iterations run before sampling.
const WARMUPS: &str = "warmups";

/// The row clause stating the exact gap ratio's numerator.
const RATIO_NUMERATOR: &str = "ratio_numerator";

/// The row clause stating the exact gap ratio's denominator.
const RATIO_DENOMINATOR: &str = "ratio_denominator";

/// The row clause stating the declared work formula.
const FORMULA: &str = "formula";

/// The row clause stating the work observations the gate reads.
const OBSERVE: &str = "observe";

/// The clause keys this grammar declares at a declaration's own level.
const DECLARABLE: [&str; 4] = [SUPPORT, TABLE_FUNCTION, TABLE, REPORTER];

/// The clause keys one row admits.
///
/// Its own roster rather than the declaration level's, because the two levels admit different keys and one roster standing for both would let a table's clause be written inside a row and read as lawful.
const DECLARABLE_ROW: [&str; 11] = [
    WORKLOAD,
    PREFLIGHT,
    PLANTED_WORSE,
    COMPLEXITY,
    AXIS,
    SAMPLES,
    WARMUPS,
    RATIO_NUMERATOR,
    RATIO_DENOMINATOR,
    FORMULA,
    OBSERVE,
];

/// Read one bench payload out of the helper attribute's body.
///
/// # Errors
///
/// Returns [`BenchCaptureError`] where the tokens do not say a bench declaration, and where the values they say are not a lawful declaration — each at the token the clause it was established at sits at.
pub fn captured(
    body: &[&CapturedTokenTree],
    at: SpanHandle,
    grammar: Grammar,
) -> Result<BenchmarkDeclaration, BenchCaptureError> {
    let clauses = declaration_clauses(
        grammar,
        body,
        &DECLARABLE,
        |_grammar, group| Ok(row_clause(group)),
        refused,
    )?;
    let support = SupportName::declared(identifier(grammar, &clauses, SUPPORT, at, refused)?)
        .map_err(|refusal| carried(grammar, refusal, at))?;
    let table_function =
        FunctionName::declared(identifier(grammar, &clauses, TABLE_FUNCTION, at, refused)?)
            .map_err(|refusal| carried(grammar, refusal, at))?;
    let table = named_reference(grammar, &clauses, TABLE, at, refused, carried)?;
    let reporter_module =
        ModuleName::declared(identifier(grammar, &clauses, REPORTER, at, refused)?)
            .map_err(|refusal| carried(grammar, refusal, at))?;

    let mut rows: Vec<Row> = Vec::new();
    for clause in &clauses {
        if let Some(stated) = clause.nested_value() {
            rows.push(row(grammar, stated.lens, &stated.body, stated.at)?);
        }
    }
    BenchmarkDeclaration::declared(
        support,
        table_function,
        table,
        rows,
        Reporter::declared(reporter_module),
    )
    .map_err(|refusal| carried(grammar, refusal, at))
}

/// One established grammar refusal at one token.
const fn refused(grammar: Grammar, cause: CaptureCause, at: SpanHandle) -> BenchCaptureError {
    BenchCaptureError::grammar_refused(grammar, cause, at)
}

/// One vocabulary refusal carried whole, at the token the value was read from.
const fn carried(grammar: Grammar, refusal: DeclarationError, at: SpanHandle) -> BenchCaptureError {
    BenchCaptureError::vocabulary_refused(grammar, refusal, at)
}

/// One bench-owned nested row clause.
struct RowClause<'trees> {
    lens: &'trees str,
    body: Vec<&'trees CapturedTokenTree>,
    at: SpanHandle,
}

/// Read one bench-owned nested row where the group states one.
fn row_clause<'trees>(group: &[&'trees CapturedTokenTree]) -> Option<RowClause<'trees>> {
    if let [head, body] = group
        && let Some(lens) = head.word()
        && let Some((CapturedDelimiter::Brace, inner)) = body.group()
    {
        return Some(RowClause {
            lens,
            body: inner.iter().collect(),
            at: head.span(),
        });
    }
    None
}

/// One unsuffixed decimal literal, read at the exact width of the seat it fills.
fn axis_number<Number: core::str::FromStr>(
    grammar: Grammar,
    tree: &CapturedTokenTree,
) -> Result<Number, BenchCaptureError> {
    let spelling = tree
        .number()
        .ok_or_else(|| refused(grammar, CaptureCause::ClauseUnread, tree.span()))?;
    if !spelling.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(refused(grammar, CaptureCause::ClauseUnread, tree.span()));
    }
    spelling
        .parse::<Number>()
        .map_err(|_| refused(grammar, CaptureCause::NumberBeyondSeat, tree.span()))
}

/// The bracketed axis of input sizes a row states.
fn axis(
    grammar: Grammar,
    clauses: &[Clause<'_, Infallible>],
    at: SpanHandle,
) -> Result<Vec<u64>, BenchCaptureError> {
    let (value, clause) =
        assigned(clauses, AXIS).ok_or_else(|| refused(grammar, CaptureCause::ClauseAbsent, at))?;
    let [bracketed] = value else {
        return Err(refused(grammar, CaptureCause::RosterUnread, clause));
    };
    let Some((CapturedDelimiter::Bracket, inner)) = bracketed.group() else {
        return Err(refused(
            grammar,
            CaptureCause::RosterUnread,
            bracketed.span(),
        ));
    };
    let mut sizes: Vec<u64> = Vec::new();
    let mut group: Vec<&CapturedTokenTree> = Vec::new();
    for tree in inner {
        if tree.punct() == Some(',') {
            match group.as_slice() {
                [] => {
                    return Err(refused(
                        grammar,
                        CaptureCause::SeparatorDangling,
                        tree.span(),
                    ));
                }
                [only] => sizes.push(axis_number::<u64>(grammar, only)?),
                [first, ..] => {
                    return Err(refused(grammar, CaptureCause::RosterUnread, first.span()));
                }
            }
            group.clear();
        } else {
            group.push(tree);
        }
    }
    match group.as_slice() {
        [] => {}
        [only] => sizes.push(axis_number::<u64>(grammar, only)?),
        [first, ..] => return Err(refused(grammar, CaptureCause::RosterUnread, first.span())),
    }
    Ok(sizes)
}

/// The declared work formula, where the row states one.
fn formula(
    grammar: Grammar,
    clauses: &[Clause<'_, Infallible>],
) -> Result<Option<WorkFormula>, BenchCaptureError> {
    let Some((value, at)) = assigned(clauses, FORMULA) else {
        return Ok(None);
    };
    let [only] = value else {
        return Err(refused(grammar, CaptureCause::ClauseUnread, at));
    };
    let text = only
        .text()
        .ok_or_else(|| refused(grammar, CaptureCause::ClauseUnread, only.span()))?;
    WorkFormula::encoded(text.as_bytes().to_vec())
        .map(Some)
        .map_err(|refusal| carried(grammar, refusal, only.span()))
}

/// The required bracketed roster of work-observation references a row states.
fn observations(
    grammar: Grammar,
    clauses: &[Clause<'_, Infallible>],
    at: SpanHandle,
) -> Result<Vec<Name>, BenchCaptureError> {
    let (value, at) = assigned(clauses, OBSERVE)
        .ok_or_else(|| refused(grammar, CaptureCause::ClauseAbsent, at))?;
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
    let mut observed: Vec<Name> = Vec::new();
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
            observed.push(named_value(
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
        observed.push(named_value(
            grammar,
            &group,
            bracketed.span(),
            refused,
            carried,
        )?);
    }
    Ok(observed)
}

/// One row: the lens it is declared under, and everything it states about how one workload is measured.
fn row(
    grammar: Grammar,
    lens: &str,
    body: &[&CapturedTokenTree],
    at: SpanHandle,
) -> Result<Row, BenchCaptureError> {
    let named = FunctionName::declared(lens).map_err(|refusal| carried(grammar, refusal, at))?;
    let clauses = assignment_clauses(grammar, body, &DECLARABLE_ROW, refused)?;
    let references = References {
        workload: named_reference(grammar, &clauses, WORKLOAD, at, refused, carried)?,
        correctness_preflight: named_reference(grammar, &clauses, PREFLIGHT, at, refused, carried)?,
        planted_worse: named_reference(grammar, &clauses, PLANTED_WORSE, at, refused, carried)?,
        complexity_claim: named_reference(grammar, &clauses, COMPLEXITY, at, refused, carried)?,
    };
    let sizes = axis(grammar, &clauses, at)?;
    let measurement = Measurement {
        budgets: Budgets {
            samples: number(grammar, &clauses, SAMPLES, at, refused)?,
            warmups: number(grammar, &clauses, WARMUPS, at, refused)?,
            ratio_numerator: number(grammar, &clauses, RATIO_NUMERATOR, at, refused)?,
            ratio_denominator: number(grammar, &clauses, RATIO_DENOMINATOR, at, refused)?,
        },
        contention: ContentionPosture::NoDeclaredContention,
        work_formula: formula(grammar, &clauses)?,
    };
    Row::declared(
        named,
        references,
        sizes,
        measurement,
        observations(grammar, &clauses, at)?,
    )
    .map_err(|refusal| carried(grammar, refusal, at))
}
