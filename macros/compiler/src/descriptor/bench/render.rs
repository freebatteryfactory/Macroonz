//! The token half of the neutral benchmark road: one stamped table and one typed report-reader value.
//!
//! Every semantic fact is read from [`BenchmarkDeclaration`](super::BenchmarkDeclaration).
//! Every executable fact arrives through the carrier matcher from the consuming target.
//! The rendering calls only the harness's public constructors and never hosts, judges, measures, or interprets a report.

use super::{BenchmarkDeclaration, Budgets, ContentionPosture, Row, WorkFormula};
use crate::bounded::Overflow;
use crate::descriptor::emitting::row_metavariable;
use crate::descriptor::trial::{named_clause, table_schema_identity};
use crate::descriptor::vocabulary::{self, HarnessName, HarnessWord};
use crate::descriptor::{Emitter, Name};
use crate::token::{
    GeneratedDelimiter, GeneratedToken, bound_local, call, comma, comma_many, constant,
    documentation, group, metavariable, method_chain, roster, text_pair,
};

/// The parsed workload reference one row expression retains.
const WORKLOAD_LOCAL: &str = "workload";

/// The parsed preflight reference one row expression retains.
const PREFLIGHT_LOCAL: &str = "preflight";

/// The parsed planted-worse reference one row expression retains.
const PLANTED_WORSE_LOCAL: &str = "planted_worse";

/// The parsed complexity reference one row expression retains.
const COMPLEXITY_LOCAL: &str = "complexity";

/// The declared row one binding expression retains.
const ROW_LOCAL: &str = "row";

/// The executable attachment one binding expression retains.
const ATTACHMENT_LOCAL: &str = "attachment";

/// The target-supplied report-reader value's name.
const REPORT: &str = "REPORT";

/// One call to a namespaced reference's parser, with the row expression's own `?` on it.
fn parsed(reference: HarnessName, name: &Name) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut tokens = vocabulary::road(
        &[HarnessName::Bench, reference, HarnessName::Named],
        text_pair(name.namespace(), name.stem()),
    )?;
    tokens.push(GeneratedToken::alone('?'));
    Ok(tokens)
}

/// The axis's sizes as the bare comma-separated literals one `Vec` expression carries.
#[must_use]
pub fn axis_literals(row: &Row) -> Vec<GeneratedToken> {
    let mut sizes = Vec::new();
    for size in row.axis() {
        sizes.push(GeneratedToken::number(*size));
        sizes.push(GeneratedToken::alone(','));
    }
    sizes
}

/// The four exact budget values, in the constructor's named parameter order.
///
/// # Errors
///
/// Returns [`Overflow`] when the constructor expression exceeds the generated-token magnitude.
pub fn budgets(declared: &Budgets) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut tokens = vocabulary::road(
        &[
            HarnessName::Bench,
            HarnessName::DeclaredBudgets,
            HarnessName::Declared,
        ],
        comma_many(vec![
            vec![GeneratedToken::number(u64::from(declared.samples))],
            vec![GeneratedToken::number(u64::from(declared.warmups))],
            vec![GeneratedToken::number(declared.ratio_numerator)],
            vec![GeneratedToken::number(declared.ratio_denominator)],
        ]),
    )?;
    tokens.push(GeneratedToken::alone('?'));
    Ok(tokens)
}

/// The declared contention posture, as the arm the harness's closed choice names.
#[must_use]
pub fn contention(posture: ContentionPosture) -> Vec<GeneratedToken> {
    vocabulary::path(&[
        HarnessName::Bench,
        HarnessName::ContentionPosture,
        posture.arm(),
    ])
}

/// The declared work formula, where the operation states one.
///
/// # Errors
///
/// Returns [`Overflow`] when the owned formula expression exceeds the generated-token magnitude.
pub fn work_formula(declared: Option<&WorkFormula>) -> Result<Vec<GeneratedToken>, Overflow> {
    let Some(formula) = declared else {
        return Ok(crate::token::absolute_path(&[
            "core", "option", "Option", "None",
        ]));
    };
    let owned = method_chain(
        vec![GeneratedToken::byte_text(formula.bytes())],
        &["to_vec"],
    )?;
    let mut encoded = vocabulary::road(
        &[
            HarnessName::Bench,
            HarnessName::WorkFormula,
            HarnessName::Encoded,
        ],
        owned,
    )?;
    encoded.push(GeneratedToken::alone('?'));
    call(
        crate::token::absolute_path(&["core", "option", "Option", "Some"]),
        encoded,
    )
}

/// One row's declaration-owned observation references, parsed into the attachment's roster.
///
/// # Errors
///
/// Returns [`Overflow`] when the parsed observation roster exceeds the generated-token magnitude.
pub fn observations(row: &Row) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut named = Vec::new();
    for observation in row.observations() {
        named.extend(parsed(HarnessName::WorkObservationRef, observation)?);
        named.push(GeneratedToken::alone(','));
    }
    roster(named)
}

/// One row in the harness's exact declaration shape, over the four parsed reference locals.
///
/// # Errors
///
/// Returns [`Overflow`] when the row declaration exceeds the generated-token magnitude.
pub fn declared_row(row: &Row) -> Result<Vec<GeneratedToken>, Overflow> {
    let references = call(
        vocabulary::path(&[
            HarnessName::Bench,
            HarnessName::BenchReferences,
            HarnessName::Declared,
        ]),
        comma_many(vec![
            vec![GeneratedToken::word(WORKLOAD_LOCAL)],
            vec![GeneratedToken::word(PREFLIGHT_LOCAL)],
            vec![GeneratedToken::word(PLANTED_WORSE_LOCAL)],
            vec![GeneratedToken::word(COMPLEXITY_LOCAL)],
        ]),
    )?;

    let mut axis = vocabulary::road(
        &[
            HarnessName::Bench,
            HarnessName::InputSizeAxis,
            HarnessName::Declared,
        ],
        roster(axis_literals(row))?,
    )?;
    axis.push(GeneratedToken::alone('?'));

    let measurement = row.measurement();
    let measurement = call(
        vocabulary::path(&[
            HarnessName::Bench,
            HarnessName::BenchMeasurement,
            HarnessName::Declared,
        ]),
        comma_many(vec![
            axis,
            budgets(&measurement.budgets)?,
            contention(measurement.contention),
            work_formula(measurement.work_formula.as_ref())?,
        ]),
    )?;

    let mut tokens = call(
        vocabulary::path(&[
            HarnessName::Bench,
            HarnessName::BenchRow,
            HarnessName::Declared,
        ]),
        comma(references, measurement),
    )?;
    tokens.push(GeneratedToken::alone('?'));
    Ok(tokens)
}

/// One complete row expression, joining declaration-owned meaning to target-owned execution facts.
///
/// # Errors
///
/// Returns [`Overflow`] when the joined declaration and attachment exceed the generated-token magnitude.
pub fn row_expression(row: &Row) -> Result<Vec<GeneratedToken>, Overflow> {
    let references = row.references();
    let mut body = bound_local(
        WORKLOAD_LOCAL,
        parsed(HarnessName::WorkloadRef, &references.workload)?,
    );
    body.extend(bound_local(
        PREFLIGHT_LOCAL,
        parsed(HarnessName::PreflightRef, &references.correctness_preflight)?,
    ));
    body.extend(bound_local(
        PLANTED_WORSE_LOCAL,
        parsed(HarnessName::PlantedWorseRef, &references.planted_worse)?,
    ));
    body.extend(bound_local(
        COMPLEXITY_LOCAL,
        parsed(
            HarnessName::ComplexityClaimRef,
            &references.complexity_claim,
        )?,
    ));
    body.extend(bound_local(ROW_LOCAL, declared_row(row)?));

    let lens = row.lens().spelling();
    let mut attachment = vocabulary::road(
        &[
            HarnessName::Bench,
            HarnessName::BenchAttachment,
            HarnessName::Attached,
        ],
        comma_many(vec![
            vec![GeneratedToken::word(WORKLOAD_LOCAL)],
            metavariable(&row_metavariable(lens, HarnessWord::Measured)),
            vec![GeneratedToken::word(PLANTED_WORSE_LOCAL)],
            metavariable(&row_metavariable(lens, HarnessWord::PlantedWorse)),
            metavariable(&row_metavariable(lens, HarnessWord::Judge)),
            observations(row)?,
        ]),
    )?;
    attachment.push(GeneratedToken::alone('?'));
    body.extend(bound_local(ATTACHMENT_LOCAL, attachment));

    body.extend(vocabulary::road(
        &[
            HarnessName::Bench,
            HarnessName::BenchBinding,
            HarnessName::Bound,
        ],
        comma_many(vec![
            vec![GeneratedToken::word(ROW_LOCAL)],
            vec![GeneratedToken::word(ATTACHMENT_LOCAL)],
            metavariable(&row_metavariable(lens, HarnessWord::Preflight)),
        ]),
    )?);
    Ok(vec![group(GeneratedDelimiter::Brace, body)?])
}

/// The matcher clauses one benchmark carrier consumes.
#[must_use]
pub fn matched_clauses(payload: &BenchmarkDeclaration) -> Vec<GeneratedToken> {
    let mut clauses = crate::support::matched_clause(HarnessWord::Reporter.spelling(), "expr");
    for row in payload.rows() {
        let lens = row.lens().spelling();
        for seat in [
            HarnessWord::Measured,
            HarnessWord::PlantedWorse,
            HarnessWord::Judge,
            HarnessWord::Preflight,
        ] {
            clauses.extend(crate::support::matched_clause(
                &row_metavariable(lens, seat),
                "expr",
            ));
        }
    }
    clauses
}

/// The benchmark-table stamp payload released by the generated-support gate.
///
/// # Errors
///
/// Returns [`Overflow`] when the stamped table and its bindings exceed the generated-token magnitude.
pub fn bench_table(
    payload: &BenchmarkDeclaration,
    emitter: Emitter,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut body = vocabulary::key(HarnessWord::Provenance);
    body.extend(produced_provenance(emitter)?);
    body.push(GeneratedToken::alone(','));
    body.extend(vocabulary::key(HarnessWord::Bindings));
    let mut bindings = Vec::new();
    for row in payload.rows() {
        bindings.extend(row_expression(row)?);
        bindings.push(GeneratedToken::alone(','));
    }
    body.push(group(GeneratedDelimiter::Bracket, bindings)?);
    body.push(GeneratedToken::alone(','));

    let mut tokens = crate_visibility()?;
    tokens.push(GeneratedToken::word("fn"));
    tokens.push(GeneratedToken::word(payload.table_function().spelling()));
    tokens.extend(named_clause(payload.table())?);
    tokens.push(group(GeneratedDelimiter::Brace, body)?);
    Ok(tokens)
}

/// The provenance expression a generated benchmark table carries.
fn produced_provenance(emitter: Emitter) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut producer = vocabulary::road(
        &[
            HarnessName::Descriptor,
            HarnessName::ProducerName,
            HarnessName::Named,
        ],
        text_pair(emitter.namespace, emitter.producer),
    )?;
    producer.push(GeneratedToken::alone('?'));

    let mut schema = table_schema_identity()?;
    schema.push(GeneratedToken::alone('?'));

    let mut fields = vocabulary::key(HarnessWord::Producer);
    fields.extend(producer);
    fields.push(GeneratedToken::alone(','));
    fields.extend(vocabulary::key(HarnessWord::Schema));
    fields.extend(schema);
    fields.push(GeneratedToken::alone(','));

    let mut tokens = vocabulary::path(&[
        HarnessName::Descriptor,
        HarnessName::ProvenanceType,
        HarnessName::ProducedProvenance,
    ]);
    tokens.push(group(GeneratedDelimiter::Brace, fields)?);
    Ok(tokens)
}

/// The typed report-reader value released beside the table.
///
/// # Errors
///
/// Returns [`Overflow`] when the report-reader module exceeds the generated-token magnitude.
pub fn reporter(payload: &BenchmarkDeclaration) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut report_type = vec![GeneratedToken::word("fn")];
    let mut borrowed = vec![GeneratedToken::alone('&')];
    borrowed.extend(vocabulary::path(&[
        HarnessName::Bench,
        HarnessName::BenchReport,
    ]));
    report_type.push(group(GeneratedDelimiter::Parenthesis, borrowed)?);

    let mut body = crate_visibility()?;
    body.extend(constant(
        REPORT,
        report_type,
        metavariable(HarnessWord::Reporter.spelling()),
    ));

    let mut tokens = documentation(REPORTER_SENTENCE)?;
    tokens.extend(crate_visibility()?);
    tokens.push(GeneratedToken::word("mod"));
    tokens.push(GeneratedToken::word(payload.reporter().module().spelling()));
    tokens.push(group(GeneratedDelimiter::Brace, body)?);
    Ok(tokens)
}

/// The `pub(crate)` visibility used by both generated benchmark items.
fn crate_visibility() -> Result<Vec<GeneratedToken>, Overflow> {
    Ok(vec![
        GeneratedToken::word("pub"),
        group(
            GeneratedDelimiter::Parenthesis,
            vec![GeneratedToken::word("crate")],
        )?,
    ])
}

/// The report-reader module's product sentence.
const REPORTER_SENTENCE: &str =
    "The target-supplied reader for reports produced from this benchmark declaration.";
