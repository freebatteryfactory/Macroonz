//! The token half of the bench road: the table the carrier's gate forwards, and the one-file reporter adapter that rides beside it.
//!
//! # The two literals this file writes directly
//!
//! The bench field roster declares COUNTS — the input-size axis, the declared budgets — and BYTES — the work formula — and the generated token roster has an arm for each.
//! Each states the VALUE while the tree owns the spelling: an unsuffixed integer takes the type the consumer's own seat declares, and the `b`, the quotes, and every escape of a byte string are the tree's.
//!
//! # The payload's grammar is the trial table's
//!
//! Clause for clause, with one row clause in place of the suite groups: two deliveries pass one gate under one pin, and a payload that read differently would be a second grammar for one wall.
//! The schema expression a produced table pins against is read from the trial home for the same reason — one address, one spelling.

use super::{Attachment, BackendRoad, Benches, Budgets, ContentionPosture, Row, WorkFormula};
use crate::bounded::Overflow;
use crate::descriptor::trial::{named_clause, table_schema_identity};
use crate::descriptor::vocabulary::{self, HarnessName, HarnessWord};
use crate::descriptor::{Binding, BoundPath, Emitter, Name};
use crate::token::{
    GeneratedDelimiter, GeneratedToken, absolute_path, attribute, call, documentation, group,
    metavariable, roster, text_pair, twin_path,
};

/// The name the adapter registers one measured function under.
const MEASURED: &str = "measured";

/// The name the adapter registers one planted-worse function under.
const PLANTED_WORSE: &str = "planted_worse";

/// The road the adapter hands a bench target to invoke.
const REPORT: &str = "report";

/// The parameter every registered function takes: one point of the axis.
const SIZE: &str = "size";

/// One path a caller declared, spelled from the binding it was rooted at.
#[must_use]
pub fn path(bound: &BoundPath) -> Vec<GeneratedToken> {
    let spelled: Vec<&str> = bound.segments().iter().map(String::as_str).collect();
    twin_path(bound.binding().name(), &spelled)
}

/// One call to a namespaced reference's parser, with the row expression's own `?` on it.
///
/// # Errors
///
/// Returns [`Overflow`] where the call outgrows the declared magnitude.
fn parsed(reference: HarnessName, name: &Name) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut tokens = vocabulary::road(
        &[HarnessName::Descriptor, reference, HarnessName::Named],
        text_pair(name.namespace(), name.stem()),
    )?;
    tokens.push(GeneratedToken::alone('?'));
    Ok(tokens)
}

/// The axis's sizes as the bare comma-separated literals both the row expression and the backend attribute write.
///
/// One road, two readers: a roster and an attribute argument list are two delimiters around one sequence, and rendering the sequence twice would be two axes that agree until one of them is edited.
#[must_use]
pub fn axis_literals(row: &Row) -> Vec<GeneratedToken> {
    let mut sizes: Vec<GeneratedToken> = Vec::new();
    for size in row.axis() {
        sizes.push(GeneratedToken::number(*size));
        sizes.push(GeneratedToken::alone(','));
    }
    sizes
}

/// The gate's declared tolerances, in the position order [`BUDGET_ORDER`](super::BUDGET_ORDER) states.
///
/// # Errors
///
/// Returns [`Overflow`] where the call outgrows the declared magnitude.
pub fn budgets(declared: &Budgets) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut arguments: Vec<GeneratedToken> = Vec::new();
    for tolerance in [declared.samples, declared.warmup, declared.ratio_threshold] {
        arguments.push(GeneratedToken::number(tolerance));
        arguments.push(GeneratedToken::alone(','));
    }
    vocabulary::road(
        &[
            HarnessName::Descriptor,
            HarnessName::DeclaredBudgets,
            HarnessName::Declared,
        ],
        arguments,
    )
}

/// The declared contention posture, as the arm the schema's closed choice names.
#[must_use]
pub fn contention(posture: ContentionPosture) -> Vec<GeneratedToken> {
    vocabulary::path(&[
        HarnessName::Descriptor,
        HarnessName::ContentionPosture,
        posture.arm(),
    ])
}

/// The declared work formula, where the operation states one.
///
/// # Errors
///
/// Returns [`Overflow`] where the construction outgrows the declared magnitude.
pub fn work_formula(declared: Option<&WorkFormula>) -> Result<Vec<GeneratedToken>, Overflow> {
    let Some(formula) = declared else {
        return Ok(absolute_path(&["core", "option", "Option", "None"]));
    };
    let mut encoded = vocabulary::road(
        &[
            HarnessName::Descriptor,
            HarnessName::WorkFormula,
            HarnessName::Encoded,
        ],
        vec![GeneratedToken::byte_text(formula.bytes())],
    )?;
    encoded.push(GeneratedToken::alone('?'));
    call(
        absolute_path(&["core", "option", "Option", "Some"]),
        encoded,
    )
}

/// One row's work observations, as the roster of callable paths the binding takes.
///
/// # Errors
///
/// Returns [`Overflow`] where the roster outgrows the declared magnitude.
pub fn observations(attachment: &Attachment) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut named: Vec<GeneratedToken> = Vec::new();
    for observed in attachment.observations() {
        named.extend(path(observed));
        named.push(GeneratedToken::alone(','));
    }
    roster(named)
}

/// One bench row, in the harness's declared field order.
///
/// # Errors
///
/// Returns [`Overflow`] where the row expression outgrows the declared magnitude.
pub fn declared_row(row: &Row) -> Result<Vec<GeneratedToken>, Overflow> {
    let references = row.references();
    let measurement = row.measurement();
    let mut arguments = parsed(HarnessName::WorkloadRef, &references.workload)?;
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(roster(axis_literals(row))?);
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(parsed(
        HarnessName::PreflightRef,
        &references.correctness_preflight,
    )?);
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(parsed(
        HarnessName::PlantedWorseRef,
        &references.planted_worse,
    )?);
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(budgets(&measurement.budgets)?);
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(contention(measurement.contention));
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(work_formula(measurement.work_formula.as_ref())?);
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(parsed(
        HarnessName::ComplexityClaimRef,
        &references.complexity_claim,
    )?);
    arguments.push(GeneratedToken::alone(','));
    let mut tokens = vocabulary::road(
        &[
            HarnessName::Descriptor,
            HarnessName::BenchRow,
            HarnessName::Declared,
        ],
        arguments,
    )?;
    tokens.push(GeneratedToken::alone('?'));
    Ok(tokens)
}

/// One complete bench row expression: the row married to the callables the host order invokes, in the order it invokes them, under the consumer's own declared budgets.
///
/// # Errors
///
/// Returns [`Overflow`] where the expression outgrows the declared magnitude.
pub fn row_expression(row: &Row) -> Result<Vec<GeneratedToken>, Overflow> {
    let attachment = row.attachment();
    let mut arguments = declared_row(row)?;
    for called in [
        &attachment.measured,
        &attachment.planted_worse,
        &attachment.preflight,
    ] {
        arguments.push(GeneratedToken::alone(','));
        arguments.extend(path(called));
    }
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(metavariable(HarnessWord::Invocation.spelling()));
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(observations(attachment)?);
    arguments.push(GeneratedToken::alone(','));
    vocabulary::road(
        &[
            HarnessName::Descriptor,
            HarnessName::BenchBinding,
            HarnessName::Bound,
        ],
        arguments,
    )
}

/// The matcher clauses one bench delivery's carrier must bind: the declaring crate's name, and the invocation the table and every row read.
///
/// The declaring clause is an identifier because it names a crate the consumer may have renamed, exactly as the carrier's own harness clause does; the invocation is an expression the consumption target owns.
/// Both seats of the bench form spell these and no others — every further name a row needs is either data the table parses or a path already rooted at one of the two crate bindings.
#[must_use]
pub fn matched_clauses() -> Vec<GeneratedToken> {
    let mut clauses = crate::support::matched_clause(Binding::Declaring.name(), "ident");
    clauses.extend(crate::support::matched_clause(
        HarnessWord::Invocation.spelling(),
        "expr",
    ));
    clauses
}

/// The bench table the carrier's gate forwards.
///
/// # Errors
///
/// Returns [`Overflow`] where a row expression or the payload itself outgrows the declared magnitude.
pub fn bench_table(payload: &Benches, emitter: Emitter) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut body = vocabulary::key(HarnessWord::Provenance);
    body.push(GeneratedToken::word(HarnessWord::Produced.spelling()));
    body.push(group(
        GeneratedDelimiter::Parenthesis,
        text_pair(emitter.namespace, emitter.producer),
    )?);
    body.push(GeneratedToken::word(HarnessWord::Against.spelling()));
    body.extend(table_schema_identity()?);
    body.push(GeneratedToken::alone(','));
    body.extend(vocabulary::key(HarnessWord::Invocation));
    body.extend(metavariable(HarnessWord::Invocation.spelling()));
    body.push(GeneratedToken::alone(','));
    for row in payload.rows() {
        body.push(GeneratedToken::word(HarnessWord::Row.spelling()));
        body.push(GeneratedToken::word(row.lens().spelling()));
        body.push(GeneratedToken::alone(':'));
        body.extend(row_expression(row)?);
        body.push(GeneratedToken::alone(','));
    }
    let mut tokens = vec![
        GeneratedToken::word("mod"),
        GeneratedToken::word(payload.module().spelling()),
    ];
    tokens.extend(named_clause(payload.table())?);
    tokens.push(group(GeneratedDelimiter::Brace, body)?);
    Ok(tokens)
}

/// One path rooted at the adapter's declared backend.
fn backend_path(backend: &str, road: BackendRoad) -> Vec<GeneratedToken> {
    vec![
        GeneratedToken::word(backend),
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        GeneratedToken::word(road.spelling()),
    ]
}

/// The backend attribute one registered function wears, carrying the row's own axis as the argument roster the backend measures across.
///
/// # Errors
///
/// Returns [`Overflow`] where the attribute outgrows the declared magnitude.
pub fn bench_attribute(backend: &str, row: &Row) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut body = backend_path(backend, BackendRoad::Bench);
    body.push(group(
        GeneratedDelimiter::Parenthesis,
        vec![
            GeneratedToken::word(BackendRoad::Args.spelling()),
            GeneratedToken::alone('='),
            group(GeneratedDelimiter::Bracket, axis_literals(row))?,
        ],
    )?);
    attribute(body)
}

/// One registered function: the backend's attribute, one point of the axis in, and the named callable's result handed to the backend's black box.
///
/// # Errors
///
/// Returns [`Overflow`] where the function outgrows the declared magnitude.
pub fn registered_function(
    backend: &str,
    row: &Row,
    spelling: &str,
    called: &BoundPath,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut tokens = bench_attribute(backend, row)?;
    tokens.push(GeneratedToken::word("fn"));
    tokens.push(GeneratedToken::word(spelling));
    tokens.push(group(
        GeneratedDelimiter::Parenthesis,
        vec![
            GeneratedToken::word(SIZE),
            GeneratedToken::alone(':'),
            GeneratedToken::word("usize"),
        ],
    )?);
    let measured = call(path(called), vec![GeneratedToken::word(SIZE)])?;
    let mut boxed = call(backend_path(backend, BackendRoad::BlackBox), measured)?;
    boxed.push(GeneratedToken::alone(';'));
    tokens.push(group(GeneratedDelimiter::Brace, boxed)?);
    Ok(tokens)
}

/// One row's own module inside the adapter: the measured function and the planted-worse function, registered side by side so the backend produces both curves and the gate has two to separate.
///
/// A module per row rather than two suffixed names in one namespace, because a suffix is a spelling two distinct lenses can collide at — and the lens namespace this home closes is the lens set itself, not the set of names a suffix would derive from it.
///
/// # Errors
///
/// Returns [`Overflow`] where the module outgrows the declared magnitude.
pub fn row_module(backend: &str, row: &Row) -> Result<Vec<GeneratedToken>, Overflow> {
    let attachment = row.attachment();
    let mut body = registered_function(backend, row, MEASURED, &attachment.measured)?;
    body.extend(registered_function(
        backend,
        row,
        PLANTED_WORSE,
        &attachment.planted_worse,
    )?);
    let mut tokens = documentation(ROW_MODULE_SENTENCE)?;
    tokens.push(GeneratedToken::word("mod"));
    tokens.push(GeneratedToken::word(row.lens().spelling()));
    tokens.push(group(GeneratedDelimiter::Brace, body)?);
    Ok(tokens)
}

/// The one-file reporter adapter: one module per row, and the single road a bench target calls to make the backend run.
///
/// Every backend-naming token below is written from the adapter's own declared backend and from nowhere else, which is what makes that value the one swap point rather than one of several places a backend name enters.
///
/// # Errors
///
/// Returns [`Overflow`] where the adapter outgrows the declared magnitude.
pub fn reporter_adapter(payload: &Benches) -> Result<Vec<GeneratedToken>, Overflow> {
    let backend = payload.backend();
    let mut body: Vec<GeneratedToken> = Vec::new();
    for row in payload.rows() {
        body.extend(row_module(backend, row)?);
    }
    body.extend(report_road(backend)?);
    let mut tokens = documentation(ADAPTER_SENTENCE)?;
    tokens.push(GeneratedToken::word("mod"));
    tokens.push(GeneratedToken::word(payload.adapter().module().spelling()));
    tokens.push(group(GeneratedDelimiter::Brace, body)?);
    Ok(tokens)
}

/// The single road a bench target calls: run the registered functions through the bound backend, and nothing else.
fn report_road(backend: &str) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut tokens = documentation(REPORT_SENTENCE)?;
    tokens.push(GeneratedToken::word("pub"));
    tokens.push(group(
        GeneratedDelimiter::Parenthesis,
        vec![GeneratedToken::word("crate")],
    )?);
    tokens.push(GeneratedToken::word("fn"));
    tokens.push(GeneratedToken::word(REPORT));
    tokens.push(group(GeneratedDelimiter::Parenthesis, Vec::new())?);
    let mut body = call(backend_path(backend, BackendRoad::Main), Vec::new())?;
    body.push(GeneratedToken::alone(';'));
    tokens.push(group(GeneratedDelimiter::Brace, body)?);
    Ok(tokens)
}

/// The sentence one row's registered module documents itself with.
const ROW_MODULE_SENTENCE: &str = "One bench row's two registered functions: the measured \
     realization, and the deliberately worse one the gate must separate from it.";

/// The sentence the adapter's report road documents itself with.
const REPORT_SENTENCE: &str = "Run the registered functions through the bound backend. It reports \
     and never renders a verdict: the declared order — preflight, planted-worse gate, then \
     measurement — is the bench host's, and an adapter that ran it would be a second host.";

/// The sentence the adapter documents itself with.
const ADAPTER_SENTENCE: &str = "The one-file reporter adapter: the single file a consumer swaps to \
     change measurement backends. Every backend-naming token here is written from one declared \
     value.";
