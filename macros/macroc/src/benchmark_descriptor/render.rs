//! The token half of the road: the bench table the carrier's gate forwards, and
//! the one-file reporter adapter that rides beside it.
//!
//! # The carrier is not rebuilt here
//!
//! The exported macro definition, its matcher, the gate invocation and the
//! expectation literal are the test-descriptor home's, read from there. This file
//! writes the two things the third crossing carries and nothing else, because a
//! carrier written twice is two carriers that agree until one of them is edited.
//!
//! # Tokens, not text
//!
//! Every path is spelled as segments, every literal is a typed literal whose
//! quoting the tree owns, and every brace is a group. Nothing here composes Rust
//! source.
//!
//! # The two literals this file writes directly
//!
//! The bench field roster declares COUNTS — the input-size axis, the declared
//! budgets — and BYTES — the work formula — and the generated token roster has an
//! arm for each ([`GeneratedToken::Number`], [`GeneratedToken::ByteText`]).
//! [`count_literal`] and [`byte_literal`] are the two seams that write them, and
//! each states the VALUE while the tree owns the spelling: an unsuffixed integer
//! takes the type the consumer's own seat declares, and the `b`, the quotes, and
//! every escape of a byte string are the tree's.

use super::{
    BenchAttachment, BenchReporterAdapter, BenchRow, BenchTablePayload, ContentionPosture,
    DeclaredBudgets, WorkFormula,
};
use crate::test_descriptor::{
    BoundPath, INVOCATION_CLAUSE, PROVENANCE_CLAUSE, ShellRenderIssue, bound_path, descriptor_path,
    documentation, group, metavariable, name_arguments, named_clause, parsed_name, roster,
    table_schema_identity,
};
use crate::token::{GeneratedDelimiter, GeneratedToken};

// ---------------------------------------------------------------------------
// The spellings the emission names at the address it writes to.
// ---------------------------------------------------------------------------

/// The stamp whose grammar the rendered bench payload is written in.
pub const BENCH_TABLE_STAMP: &str = "bench_table";

/// The clause each declared row is written under inside that payload.
pub const ROW_CLAUSE: &str = "row";

/// One row of the bench-row vocabulary.
pub const BENCH_ROW: &str = "BenchRow";

/// The road a bench row is declared by.
pub const BENCH_ROW_ROAD: &str = "declared";

/// The reference naming what is measured.
pub const WORKLOAD_REF: &str = "WorkloadRef";

/// The reference naming the correctness preflight.
pub const PREFLIGHT_REF: &str = "PreflightRef";

/// The reference naming the planted-worse falsifier.
pub const PLANTED_WORSE_REF: &str = "PlantedWorseRef";

/// The neutral reference a row's complexity claim is stated through.
pub const COMPLEXITY_CLAIM_REF: &str = "ComplexityClaimRef";

/// The declared contention posture's own type.
pub const CONTENTION_POSTURE: &str = "ContentionPosture";

/// The declared work formula's own type.
pub const WORK_FORMULA: &str = "WorkFormula";

/// The road a work formula is taken over its declaration's bytes by.
pub const WORK_FORMULA_ROAD: &str = "encoded";

/// The gate's declared tolerances, as the address carries them.
pub const DECLARED_BUDGETS: &str = "DeclaredBudgets";

/// The road the declared tolerances are stated by.
pub const DECLARED_BUDGETS_ROAD: &str = "declared";

/// One bench row married to the callables the host order invokes.
pub const BENCH_BINDING: &str = "BenchBinding";

/// The road a bench binding is married by.
pub const BENCH_BINDING_ROAD: &str = "bound";

/// The name the rendered adapter registers one measured function under.
pub const MEASURED_FUNCTION: &str = "measured";

/// The name the rendered adapter registers one planted-worse function under.
pub const PLANTED_WORSE_FUNCTION: &str = "planted_worse";

/// The road the rendered adapter hands a bench target to invoke.
pub const REPORT_FUNCTION: &str = "report";

/// The parameter every registered function takes: one point of the axis.
pub const SIZE_PARAMETER: &str = "size";

/// The backend attribute one registered function wears.
pub const BENCH_ATTRIBUTE: &str = "bench";

/// The backend clause the axis is handed to.
pub const ARGS_CLAUSE: &str = "args";

/// The backend road that keeps a measured value from being optimized away.
pub const BLACK_BOX: &str = "black_box";

/// The backend road that runs the registered functions.
pub const BACKEND_MAIN: &str = "main";

// ---------------------------------------------------------------------------
// The two literals the token roster spells.
// ---------------------------------------------------------------------------

/// One declared count, as the literal token the address's constructor takes.
///
/// Written UNSUFFIXED, because the consumer's type position is what types it: the
/// literal lands in a seat the address already declares, so one road writes a
/// count into a `u32` seat, a `u64` seat, and a `usize` seat without being told
/// which. A suffix would state a second type beside the one the address declares.
///
/// It takes the value and nothing else. A field NAME beside it named the seat a
/// refusal reported, and there is no refusal to report: a count the roster can
/// spell is spelled, and a parameter kept for a report nobody writes is a value
/// this road decided and nothing reads.
#[must_use]
pub fn count_literal(value: u64) -> GeneratedToken {
    GeneratedToken::number(value)
}

/// One declared byte string, as the literal token the address's constructor
/// takes.
///
/// The material is stated and the spelling is the tree's — the `b`, the quotes,
/// and every escape — so no caller composes `b"…"` out of a word and a quoted
/// string. It takes no field name, on exactly [`count_literal`]'s terms.
#[must_use]
pub fn byte_literal(material: &[u8]) -> GeneratedToken {
    GeneratedToken::byte_text(material)
}

// ---------------------------------------------------------------------------
// The bench row expression.
// ---------------------------------------------------------------------------

/// One row's declared input-size axis, as the roster of counts the address takes.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the roster outgrows
/// the declared token magnitude.
pub fn axis(row: &BenchRow) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    roster(axis_literals(row))
}

/// The axis's sizes as the bare comma-separated literals both the row expression
/// and the backend attribute write.
///
/// One road, two readers: a roster and an attribute argument list are two
/// delimiters around one sequence, and rendering the sequence twice would be two
/// axes that agree until one of them is edited.
fn axis_literals(row: &BenchRow) -> Vec<GeneratedToken> {
    let mut sizes: Vec<GeneratedToken> = Vec::new();
    for size in row.axis() {
        sizes.push(count_literal(*size));
        sizes.push(GeneratedToken::alone(','));
    }
    sizes
}

/// The gate's declared tolerances, in the position order the schema's roster
/// states and this home's own budget table names.
///
/// The three named seats are written in exactly
/// [`BUDGET_ORDER`](super::BUDGET_ORDER)'s order, which is what that table is
/// stated for: the schema's roster is positional, this home's seats are named,
/// and the mapping between the two is a table a reader joins by rather than an
/// order inferred from the array below.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the call outgrows the
/// declared token magnitude.
pub fn budgets(declared: &DeclaredBudgets) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let stated: [u64; 3] = [declared.samples, declared.warmup, declared.ratio_threshold];
    let mut arguments: Vec<GeneratedToken> = Vec::new();
    for tolerance in stated {
        arguments.push(count_literal(tolerance));
        arguments.push(GeneratedToken::alone(','));
    }
    let mut tokens = descriptor_path(&[DECLARED_BUDGETS, DECLARED_BUDGETS_ROAD]);
    tokens.push(group(GeneratedDelimiter::Parenthesis, arguments)?);
    Ok(tokens)
}

/// The declared contention posture, as the arm the schema's closed choice names.
#[must_use]
pub fn contention(posture: ContentionPosture) -> Vec<GeneratedToken> {
    descriptor_path(&[CONTENTION_POSTURE, posture.arm()])
}

/// The declared work formula, where the operation states one.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the construction
/// outgrows the declared token magnitude.
pub fn work_formula(
    declared: Option<&WorkFormula>,
) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let Some(formula) = declared else {
        return Ok(GeneratedToken::absolute_path(&[
            "core", "option", "Option", "None",
        ]));
    };
    let material: Vec<u8> = formula.bytes().copied().collect();
    let mut encoded = descriptor_path(&[WORK_FORMULA, WORK_FORMULA_ROAD]);
    encoded.push(group(
        GeneratedDelimiter::Parenthesis,
        vec![byte_literal(&material)],
    )?);
    encoded.push(GeneratedToken::alone('?'));
    let mut tokens = GeneratedToken::absolute_path(&["core", "option", "Option", "Some"]);
    tokens.push(group(GeneratedDelimiter::Parenthesis, encoded)?);
    Ok(tokens)
}

/// One row's work observations, as the roster of callable paths the binding takes.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the roster outgrows the
/// declared token magnitude.
pub fn observations(attachment: &BenchAttachment) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let mut named: Vec<GeneratedToken> = Vec::new();
    for observed in attachment.observations() {
        named.extend(bound_path(observed));
        named.push(GeneratedToken::alone(','));
    }
    roster(named)
}

/// One bench row, in the harness's declared field order.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where any of the row's
/// rosters, calls, or the row expression itself outgrows the declared token
/// magnitude.
pub fn declared_row(row: &BenchRow) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let references = row.references();
    let measurement = row.measurement();
    let mut arguments = parsed_name(WORKLOAD_REF, &references.workload)?;
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(axis(row)?);
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(parsed_name(PREFLIGHT_REF, &references.correctness_preflight)?);
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(parsed_name(PLANTED_WORSE_REF, &references.planted_worse)?);
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(budgets(&measurement.budgets)?);
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(contention(measurement.contention));
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(work_formula(measurement.work_formula.as_ref())?);
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(parsed_name(
        COMPLEXITY_CLAIM_REF,
        &references.complexity_claim,
    )?);
    arguments.push(GeneratedToken::alone(','));
    let mut tokens = descriptor_path(&[BENCH_ROW, BENCH_ROW_ROAD]);
    tokens.push(group(GeneratedDelimiter::Parenthesis, arguments)?);
    tokens.push(GeneratedToken::alone('?'));
    Ok(tokens)
}

/// One complete bench row expression: the row married to the callables the host
/// order invokes, in the order it invokes them, under the consumer's own declared
/// budgets.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the row, its
/// callables, its observations, or the expression itself outgrows the declared
/// token magnitude.
pub fn bench_row_expression(row: &BenchRow) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let attachment = row.attachment();
    let mut arguments = declared_row(row)?;
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(bound_path(&attachment.measured));
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(bound_path(&attachment.planted_worse));
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(bound_path(&attachment.preflight));
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(metavariable(INVOCATION_CLAUSE));
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(observations(attachment)?);
    arguments.push(GeneratedToken::alone(','));
    let mut tokens = descriptor_path(&[BENCH_BINDING, BENCH_BINDING_ROAD]);
    tokens.push(group(GeneratedDelimiter::Parenthesis, arguments)?);
    Ok(tokens)
}

/// The bench table the carrier's gate forwards, in the stamp grammar the trial
/// table's own payload is written in.
///
/// The grammar is deliberately the trial table's, clause for clause, with one
/// row clause in place of the suite groups: the two crossings pass one gate under
/// one pin, and a payload that read differently would be a second grammar for one
/// wall.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where a row expression or
/// the payload itself outgrows the declared token magnitude.
pub fn bench_table(payload: &BenchTablePayload) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let mut body = vec![
        GeneratedToken::word(PROVENANCE_CLAUSE),
        GeneratedToken::alone(':'),
        GeneratedToken::word("produced"),
        group(
            GeneratedDelimiter::Parenthesis,
            name_arguments(payload.producer()),
        )?,
        GeneratedToken::word("against"),
    ];
    body.extend(table_schema_identity()?);
    body.push(GeneratedToken::alone(','));
    body.push(GeneratedToken::word(INVOCATION_CLAUSE));
    body.push(GeneratedToken::alone(':'));
    body.extend(metavariable(INVOCATION_CLAUSE));
    body.push(GeneratedToken::alone(','));
    for row in payload.rows() {
        body.push(GeneratedToken::word(ROW_CLAUSE));
        body.push(GeneratedToken::word(row.lens()));
        body.push(GeneratedToken::alone(':'));
        body.extend(bench_row_expression(row)?);
        body.push(GeneratedToken::alone(','));
    }
    let mut tokens = vec![
        GeneratedToken::word("mod"),
        GeneratedToken::word(payload.module()),
    ];
    tokens.extend(named_clause(payload.table())?);
    tokens.push(group(GeneratedDelimiter::Brace, body)?);
    Ok(tokens)
}

// ---------------------------------------------------------------------------
// The one-file reporter adapter.
// ---------------------------------------------------------------------------

/// One path rooted at the adapter's declared backend.
fn backend_path(backend: &str, road: &str) -> Vec<GeneratedToken> {
    vec![
        GeneratedToken::word(backend),
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        GeneratedToken::word(road),
    ]
}

/// The backend attribute one registered function wears, carrying the row's own
/// axis as the argument roster the backend measures across.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the attribute outgrows
/// the declared token magnitude.
pub fn bench_attribute(
    backend: &str,
    row: &BenchRow,
) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let mut body = backend_path(backend, BENCH_ATTRIBUTE);
    body.push(group(
        GeneratedDelimiter::Parenthesis,
        vec![
            GeneratedToken::word(ARGS_CLAUSE),
            GeneratedToken::alone('='),
            group(GeneratedDelimiter::Bracket, axis_literals(row))?,
        ],
    )?);
    Ok(vec![
        GeneratedToken::alone('#'),
        group(GeneratedDelimiter::Bracket, body)?,
    ])
}

/// One registered function: the backend's attribute, one point of the axis in,
/// and the named callable's result handed to the backend's black box.
///
/// # Errors
///
/// Returns whatever the attribute refuses with, and
/// [`ShellRenderIssue::ShellTreeUnbounded`] where the function outgrows the
/// declared token magnitude.
pub fn registered_function(
    backend: &str,
    row: &BenchRow,
    spelling: &str,
    called: &BoundPath,
) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let mut tokens = bench_attribute(backend, row)?;
    tokens.push(GeneratedToken::word("fn"));
    tokens.push(GeneratedToken::word(spelling));
    tokens.push(group(
        GeneratedDelimiter::Parenthesis,
        vec![
            GeneratedToken::word(SIZE_PARAMETER),
            GeneratedToken::alone(':'),
            GeneratedToken::word("usize"),
        ],
    )?);
    let mut call = bound_path(called);
    call.push(group(
        GeneratedDelimiter::Parenthesis,
        vec![GeneratedToken::word(SIZE_PARAMETER)],
    )?);
    let mut boxed = backend_path(backend, BLACK_BOX);
    boxed.push(group(GeneratedDelimiter::Parenthesis, call)?);
    boxed.push(GeneratedToken::alone(';'));
    tokens.push(group(GeneratedDelimiter::Brace, boxed)?);
    Ok(tokens)
}

/// One row's own module inside the adapter: the measured function and the
/// planted-worse function, registered side by side so the backend produces both
/// curves and the gate has two to separate.
///
/// A module per row rather than two suffixed names in one namespace, because a
/// suffix is a spelling two distinct lenses can collide at — and the lens
/// namespace this home closes is the lens set itself, not the set of names a
/// suffix would derive from it.
///
/// # Errors
///
/// Returns whatever the registered functions refuse with.
pub fn row_module(backend: &str, row: &BenchRow) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let attachment = row.attachment();
    let mut body = registered_function(backend, row, MEASURED_FUNCTION, &attachment.measured)?;
    body.extend(registered_function(
        backend,
        row,
        PLANTED_WORSE_FUNCTION,
        &attachment.planted_worse,
    )?);
    let mut tokens = documentation(ROW_MODULE_SENTENCE)?;
    tokens.push(GeneratedToken::word("mod"));
    tokens.push(GeneratedToken::word(row.lens()));
    tokens.push(group(GeneratedDelimiter::Brace, body)?);
    Ok(tokens)
}

/// The one-file reporter adapter: one module per row, and the single road a bench
/// target calls to make the backend run.
///
/// Every backend-naming token below is written from the adapter's own declared
/// backend and from nowhere else, which is what makes that value the ONE swap
/// point rather than one of several places a backend name enters.
///
/// # Errors
///
/// Returns whatever the row modules refuse with, and
/// [`ShellRenderIssue::ShellTreeUnbounded`] where the adapter outgrows the
/// declared token magnitude.
pub fn reporter_adapter(
    adapter: &BenchReporterAdapter,
    payload: &BenchTablePayload,
) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let backend = adapter.backend().spelling();
    let mut body: Vec<GeneratedToken> = Vec::new();
    for row in payload.rows() {
        body.extend(row_module(backend, row)?);
    }
    body.extend(report_road(backend)?);
    let mut tokens = documentation(ADAPTER_SENTENCE)?;
    tokens.push(GeneratedToken::word("mod"));
    tokens.push(GeneratedToken::word(adapter.module()));
    tokens.push(group(GeneratedDelimiter::Brace, body)?);
    Ok(tokens)
}

/// The single road a bench target calls: run the registered functions through the
/// bound backend, and nothing else.
fn report_road(backend: &str) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let mut tokens = documentation(REPORT_SENTENCE)?;
    tokens.push(GeneratedToken::word("pub"));
    tokens.push(group(
        GeneratedDelimiter::Parenthesis,
        vec![GeneratedToken::word("crate")],
    )?);
    tokens.push(GeneratedToken::word("fn"));
    tokens.push(GeneratedToken::word(REPORT_FUNCTION));
    tokens.push(group(GeneratedDelimiter::Parenthesis, Vec::new())?);
    let mut body = backend_path(backend, BACKEND_MAIN);
    body.push(group(GeneratedDelimiter::Parenthesis, Vec::new())?);
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
