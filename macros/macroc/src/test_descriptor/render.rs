//! The token half of the road: the exported shell, the gate invocation inside
//! it, the stamped payload inside that, and the constructor-calling expression
//! each row is.
//!
//! # Tokens, not text
//!
//! Every path is spelled as segments, every literal is a typed literal whose
//! quoting the tree owns, and every brace is a group. No function here composes
//! Rust source. The Rust a person reads is [`crate::token::GeneratedTree`]'s own
//! projection, which is a projection of what is emitted rather than the thing
//! itself.
//!
//! # The rename-twin splice
//!
//! Nothing here spells a crate name. Every path the emission writes begins with
//! the shell's own METAVARIABLE for one of the two twins — `$machine` or
//! `$harness` — and the consumer's target supplies both once, at the invocation,
//! so a consumer that renamed either dependency gets its own name back and this
//! home never learns what the name is. The gate's `harness:` clause receives the
//! very same metavariable, which is what makes that binding load-bearing rather
//! than decorative: the gate proves the name the consumer passed reaches the same
//! schema-identity type the harness's own `$crate` reaches, so a wrong name
//! refuses at the door instead of as an unresolved path somewhere inside the
//! payload.
//!
//! # What the expressions demand of the harness
//!
//! A row expression is declared by the stamp to answer with the binding family,
//! and the constructions on the road to a binding answer with five other
//! families. The emission writes `?`, which is the language's own conversion
//! rather than a variant this home invented inside a vocabulary it does not own.
//! `CONVERSIONS_OWED` in `type_contract.rs` is the complete bill, stated once.
//!
//! # The one thing this file cannot spell
//!
//! The gate's `expected:` clause takes a BYTE STRING literal and the generated
//! token roster has no arm that writes one. [`expectation_literal`] is the single
//! seam where that bites, and it refuses rather than writing the identifier `b`
//! beside a text literal: that pair is two tokens, the gate's pattern matches
//! one, and the consumer would be told the published pair is incoherent when the
//! truth is that the producer could not spell its own expectation.

use super::{
    BoundPath, CrateFacing, DescriptorRow, RevisionReference, RevisionStanding, RowAttachment,
    ShellName, ShellRenderIssue, SuiteGroup, TrialTablePayload, WallName,
};
use crate::plane::GeneratedTokenLimit;
use crate::planning::{EXPECTED_GENERATED_SUPPORT_SCHEMA_ID, ExpectedGeneratedSupportSchemaId};
use crate::token::{GeneratedDelimiter, GeneratedToken};
use threadpak::types::ConstLimit;

// ---------------------------------------------------------------------------
// The spellings the emission names at the address it writes to.
// ---------------------------------------------------------------------------

/// The module the harness's descriptor vocabulary lives under.
pub const DESCRIPTOR_MODULE: &str = "descriptor";

/// The gate the shell's body invokes.
pub const GATE_MACRO: &str = "generated_support";

/// The gate's clause carrying the producer's own expectation.
pub const EXPECTED_CLAUSE: &str = "expected";

/// The gate's clause carrying the harness binding.
pub const HARNESS_CLAUSE: &str = "harness";

/// The stamp clause carrying the table's stated provenance.
pub const PROVENANCE_CLAUSE: &str = "provenance";

/// The stamp clause carrying the consumer's declared budgets.
///
/// It is also the shell's own parameter for them: an argument rather than a
/// rendered constant, and deliberately, because budgets are the consumer's
/// declaration and the stamp seats them in a `const` item so an ambient fact
/// cannot appear among them. A producer that wrote its own would be declaring how
/// long somebody else's machine may spend.
pub const INVOCATION_CLAUSE: &str = "invocation";

/// The road every namespaced reference in the harness's vocabulary is parsed by.
pub const NAME_ROAD: &str = "named";

/// The road a namespaced clause of the stamp's grammar is spelled by.
pub const NAMED_CLAUSE: &str = "named";

/// The root schema declaration a produced table pins against.
pub const SCHEMA_TYPE: &str = "GeneratedSupportSchema";

/// The road to the harness's published root schema declaration.
pub const SCHEMA_PUBLISHED: &str = "published";

/// The road from a root schema declaration to its derived identity.
pub const SCHEMA_IDENTITY: &str = "identity";

/// The stamp's own refusal family, which the table's provenance expression maps
/// into.
pub const TABLE_REFUSAL: &str = "TrialTableRefusal";

/// The stamp refusal arm a refused root schema declaration reaches.
pub const SCHEMA_NOT_DECLARED: &str = "SchemaNotDeclared";

/// The stamp refusal arm a refused schema encoding reaches.
pub const SCHEMA_NOT_ENCODED: &str = "SchemaNotEncoded";

/// The claim reference a row serves.
pub const CLAIM_REF: &str = "ClaimRef";

/// The one aggregate seat a row runs under by default.
pub const EXECUTION_SUITE: &str = "ExecutionSuite";

/// One open classification a row carries.
pub const ROLE_REF: &str = "Role";

/// One open label a row carries beside its roles.
pub const TAG_REF: &str = "Tag";

/// The two open rosters, as the harness carries them.
pub const CLASSIFICATION: &str = "Classification";

/// The road the two rosters are taken as authored by.
pub const CLASSIFICATION_ROAD: &str = "authored";

/// The typed selection of what is under test.
pub const SUBJECT_ROUTE: &str = "SubjectRoute";

/// The check that judges the subject.
pub const CHECK_REF: &str = "CheckRef";

/// The generated population that supplies a row's inputs.
pub const POPULATION_REF: &str = "PopulationRef";

/// The declaration door a generated row was authored through.
pub const DOOR_REF: &str = "DoorRef";

/// The projection that emitted a generated row.
pub const PROJECTION_REF: &str = "ProjectionRef";

/// What a producer's own act contributed to a generated row.
pub const PRODUCER_FACTS: &str = "ProducerFacts";

/// The road producer facts are stated by.
pub const PRODUCER_FACTS_ROAD: &str = "emitted";

/// Where a row came from.
pub const ORIGIN: &str = "Origin";

/// The one origin arm a producer may emit.
pub const ORIGIN_GENERATED: &str = "Generated";

/// One row of the harness's denominator.
pub const ROW: &str = "Row";

/// The road a row is declared by.
pub const ROW_ROAD: &str = "declared";

/// One revision identity and the posture it is held under.
pub const REVISION_BINDING: &str = "RevisionBinding";

/// What makes one row executable.
pub const ATTACHMENT: &str = "ExecutableAttachment";

/// The road an attachment is bound by.
pub const ATTACHMENT_ROAD: &str = "attached";

/// Whether a producer stands behind one binding, and which schema it emitted
/// against.
pub const PROVENANCE: &str = "Provenance";

/// The provenance arm a produced binding carries.
pub const PROVENANCE_PRODUCED: &str = "Produced";

/// The provenance seat naming the producer.
pub const PROVENANCE_PRODUCER_SEAT: &str = "producer";

/// The provenance seat naming the schema identity.
pub const PROVENANCE_SCHEMA_SEAT: &str = "schema";

/// The producer that emitted a binding against a published schema.
pub const PRODUCER_NAME: &str = "ProducerName";

/// One row married to one executable attachment.
pub const BINDING: &str = "Binding";

/// The road a binding is married by.
pub const BINDING_ROAD: &str = "bound";

// ---------------------------------------------------------------------------
// The token primitives every crossing of the wall shares.
// ---------------------------------------------------------------------------

/// The issue a tree that outgrew the declared token magnitude amounts to.
///
/// One bound, read from one place, by every home that rides this carrier.
#[must_use]
pub fn unbounded() -> ShellRenderIssue {
    ShellRenderIssue::ShellTreeUnbounded {
        bound: u64::try_from(GeneratedTokenLimit::MAX).unwrap_or(u64::MAX),
    }
}

/// One delimited group, with a tree past the declared magnitude refused in this
/// home's own vocabulary.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the group carries more
/// tokens than the declared magnitude admits.
pub fn group(
    delimiter: GeneratedDelimiter,
    tokens: Vec<GeneratedToken>,
) -> Result<GeneratedToken, ShellRenderIssue> {
    GeneratedToken::group(delimiter, tokens).map_err(|_| unbounded())
}

/// One shell metavariable, as the two tokens that spell it.
///
/// The `$` is written JOINT so the projection a person reads is `$harness` rather
/// than `$ harness`; the token pair is the same either way, and nothing parses
/// the projection.
#[must_use]
pub fn metavariable(spelling: &str) -> Vec<GeneratedToken> {
    vec![GeneratedToken::joint('$'), GeneratedToken::word(spelling)]
}

/// One path rooted at a rename twin, spelled from that twin's metavariable and
/// the segments the caller named.
#[must_use]
pub fn twin_path(facing: CrateFacing, segments: &[&str]) -> Vec<GeneratedToken> {
    let mut tokens = metavariable(facing.parameter());
    for segment in segments {
        tokens.push(GeneratedToken::joint(':'));
        tokens.push(GeneratedToken::alone(':'));
        tokens.push(GeneratedToken::word(segment));
    }
    tokens
}

/// One path into the harness's descriptor vocabulary.
#[must_use]
pub fn descriptor_path(segments: &[&str]) -> Vec<GeneratedToken> {
    let mut spelled = vec![DESCRIPTOR_MODULE];
    spelled.extend_from_slice(segments);
    twin_path(CrateFacing::Harness, &spelled)
}

/// One path a caller declared, spelled from the twin it was rooted at.
#[must_use]
pub fn bound_path(path: &BoundPath) -> Vec<GeneratedToken> {
    let mut tokens = metavariable(path.facing().parameter());
    for segment in path.segments() {
        tokens.push(GeneratedToken::joint(':'));
        tokens.push(GeneratedToken::alone(':'));
        tokens.push(GeneratedToken::word(segment.as_str()));
    }
    tokens
}

/// One namespaced name as the two text literals its parser takes, comma
/// separated.
#[must_use]
pub fn name_arguments(name: &WallName) -> Vec<GeneratedToken> {
    vec![
        GeneratedToken::text(name.namespace()),
        GeneratedToken::alone(','),
        GeneratedToken::text(name.stem()),
    ]
}

/// One call to a namespaced reference's parser, with the row expression's own `?`
/// on it.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the call outgrows the
/// declared token magnitude.
pub fn parsed_name(
    reference: &str,
    name: &WallName,
) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let mut tokens = descriptor_path(&[reference, NAME_ROAD]);
    tokens.push(group(GeneratedDelimiter::Parenthesis, name_arguments(name))?);
    tokens.push(GeneratedToken::alone('?'));
    Ok(tokens)
}

/// The `::std::vec![…]` the harness's roster-taking constructors are handed.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the roster outgrows the
/// declared token magnitude.
pub fn roster(items: Vec<GeneratedToken>) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let mut tokens = GeneratedToken::absolute_path(&["std", "vec"]);
    tokens.push(GeneratedToken::alone('!'));
    tokens.push(group(GeneratedDelimiter::Bracket, items)?);
    Ok(tokens)
}

/// The producer's own expectation of the generated-support schema identity, as
/// the literal token the gate's opening arm matches.
///
/// The value is read from the services' one checked-in expectation and from
/// nowhere else, so the literal this emission carries and the constant the
/// publication operation rewrites are one fact rather than two.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::PinLiteralNotSpellable`] under the token roster as
/// it stands: the gate's clause takes a byte-string literal, and the generated
/// token roster's only literal arm is a TEXT literal.
pub fn expectation_literal() -> Result<GeneratedToken, ShellRenderIssue> {
    expectation_literal_of(&EXPECTED_GENERATED_SUPPORT_SCHEMA_ID)
}

/// The same literal, over an expectation a caller holds — the road the
/// publication operation's own posture flip travels, since a verified-derived
/// expectation is a different type and the same rendering.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::PinLiteralNotSpellable`] on exactly
/// [`expectation_literal`]'s terms.
pub fn expectation_literal_of<Posture>(
    expectation: &ExpectedGeneratedSupportSchemaId<Posture>,
) -> Result<GeneratedToken, ShellRenderIssue> {
    let material: &[u8; 32] = expectation.as_bytes();
    Err(ShellRenderIssue::PinLiteralNotSpellable {
        width: u64::try_from(material.len()).unwrap_or(u64::MAX),
    })
}

// ---------------------------------------------------------------------------
// The two schema-identity expressions.
// ---------------------------------------------------------------------------

/// The schema identity a produced TABLE pins against, in the shape the stamp's
/// `against` clause requires: a `Result` the stamp itself chains onto, with every
/// refusal already mapped into the stamp's own family.
///
/// It is a different expression from the row's, and the difference is the seat
/// each one sits in: the stamp calls `and_then` on this one, so an unwrapped
/// identity here would not compile, while a row's binding takes the identity
/// itself.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the expression outgrows
/// the declared token magnitude.
pub fn table_schema_identity() -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let mut tokens = descriptor_path(&[SCHEMA_TYPE, SCHEMA_PUBLISHED]);
    tokens.push(group(GeneratedDelimiter::Parenthesis, Vec::new())?);
    tokens.push(GeneratedToken::alone('.'));
    tokens.push(GeneratedToken::word("map_err"));
    tokens.push(group(
        GeneratedDelimiter::Parenthesis,
        descriptor_path(&[TABLE_REFUSAL, SCHEMA_NOT_DECLARED]),
    )?);
    tokens.push(GeneratedToken::alone('.'));
    tokens.push(GeneratedToken::word("and_then"));
    tokens.push(group(GeneratedDelimiter::Parenthesis, schema_closure()?)?);
    Ok(tokens)
}

/// The closure the table's provenance expression chains: one root schema
/// declaration in, its derived identity or the stamp's encoding refusal out.
fn schema_closure() -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let mut tokens = vec![
        GeneratedToken::alone('|'),
        GeneratedToken::word("declared"),
        GeneratedToken::alone('|'),
        GeneratedToken::word("declared"),
        GeneratedToken::alone('.'),
        GeneratedToken::word(SCHEMA_IDENTITY),
        group(GeneratedDelimiter::Parenthesis, Vec::new())?,
        GeneratedToken::alone('.'),
        GeneratedToken::word("map_err"),
    ];
    tokens.push(group(
        GeneratedDelimiter::Parenthesis,
        descriptor_path(&[TABLE_REFUSAL, SCHEMA_NOT_ENCODED]),
    )?);
    Ok(tokens)
}

/// The schema identity one produced ROW's binding pins against, with the row
/// expression's own `?` on each refusal.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the expression outgrows
/// the declared token magnitude.
pub fn row_schema_identity() -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let mut tokens = descriptor_path(&[SCHEMA_TYPE, SCHEMA_PUBLISHED]);
    tokens.push(group(GeneratedDelimiter::Parenthesis, Vec::new())?);
    tokens.push(GeneratedToken::alone('?'));
    tokens.push(GeneratedToken::alone('.'));
    tokens.push(GeneratedToken::word(SCHEMA_IDENTITY));
    tokens.push(group(GeneratedDelimiter::Parenthesis, Vec::new())?);
    tokens.push(GeneratedToken::alone('?'));
    Ok(tokens)
}

// ---------------------------------------------------------------------------
// The row expression.
// ---------------------------------------------------------------------------

/// One revision binding, under the posture the caller declared and over the
/// address it named.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the call outgrows the
/// declared token magnitude.
pub fn revision_binding(
    reference: &RevisionReference,
) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let road = match reference.standing {
        RevisionStanding::Derived => "derived",
        RevisionStanding::Declared => "declared",
        RevisionStanding::Untracked => "untracked",
    };
    let mut tokens = descriptor_path(&[REVISION_BINDING, road]);
    tokens.push(group(
        GeneratedDelimiter::Parenthesis,
        bound_path(&reference.address),
    )?);
    Ok(tokens)
}

/// One row's executable attachment: the references it is over, a posture-bearing
/// revision binding for each, and the callable itself.
///
/// The two references are re-parsed here rather than shared with the row's own,
/// because the binding constructor's whole job is to check that the two agree —
/// and a rendering that spliced one value into both seats would make that check
/// compare a value with itself.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the call outgrows the
/// declared token magnitude.
pub fn attachment(
    declared: &DescriptorRow,
    executes: &RowAttachment,
) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let mut arguments = parsed_name(SUBJECT_ROUTE, &declared.references().subject)?;
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(parsed_name(CHECK_REF, &declared.references().check)?);
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(revision_binding(&executes.subject_revision)?);
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(revision_binding(&executes.check_revision)?);
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(bound_path(&executes.call));
    arguments.push(GeneratedToken::alone(','));
    let mut tokens = descriptor_path(&[ATTACHMENT, ATTACHMENT_ROAD]);
    tokens.push(group(GeneratedDelimiter::Parenthesis, arguments)?);
    Ok(tokens)
}

/// One row's origin: the generated arm, and the producer facts inside it.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the call outgrows the
/// declared token magnitude.
pub fn origin(declared: &DescriptorRow) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let mut facts = parsed_name(DOOR_REF, &declared.origin().door)?;
    facts.push(GeneratedToken::alone(','));
    facts.extend(parsed_name(PROJECTION_REF, &declared.origin().projection)?);
    let mut emitted = descriptor_path(&[PRODUCER_FACTS, PRODUCER_FACTS_ROAD]);
    emitted.push(group(GeneratedDelimiter::Parenthesis, facts)?);
    let mut tokens = descriptor_path(&[ORIGIN, ORIGIN_GENERATED]);
    tokens.push(group(GeneratedDelimiter::Parenthesis, emitted)?);
    Ok(tokens)
}

/// One row's classification: the two open rosters, each parsed label by label.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the call outgrows the
/// declared token magnitude.
pub fn classification(declared: &DescriptorRow) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let mut roles: Vec<GeneratedToken> = Vec::new();
    for role in declared.roles() {
        roles.extend(parsed_name(ROLE_REF, role)?);
        roles.push(GeneratedToken::alone(','));
    }
    let mut tags: Vec<GeneratedToken> = Vec::new();
    for tag in declared.tags() {
        tags.extend(parsed_name(TAG_REF, tag)?);
        tags.push(GeneratedToken::alone(','));
    }
    let mut arguments = roster(roles)?;
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(roster(tags)?);
    let mut tokens = descriptor_path(&[CLASSIFICATION, CLASSIFICATION_ROAD]);
    tokens.push(group(GeneratedDelimiter::Parenthesis, arguments)?);
    tokens.push(GeneratedToken::alone('?'));
    Ok(tokens)
}

/// One row, in the harness's closed field order.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the call outgrows the
/// declared token magnitude.
pub fn declared_row(declared: &DescriptorRow) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let references = declared.references();
    let mut arguments = parsed_name(CLAIM_REF, &references.claim)?;
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(parsed_name(EXECUTION_SUITE, &references.execution_suite)?);
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(classification(declared)?);
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(parsed_name(SUBJECT_ROUTE, &references.subject)?);
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(parsed_name(CHECK_REF, &references.check)?);
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(parsed_name(POPULATION_REF, &references.population)?);
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(origin(declared)?);
    arguments.push(GeneratedToken::alone(','));
    let mut tokens = descriptor_path(&[ROW, ROW_ROAD]);
    tokens.push(group(GeneratedDelimiter::Parenthesis, arguments)?);
    tokens.push(GeneratedToken::alone('?'));
    Ok(tokens)
}

/// The provenance one produced binding states: the producer that emitted it, and
/// the schema identity it emitted against.
///
/// Written as the variant's own construction because the harness publishes no
/// constructor road to it — the seats are public and the variant is the only
/// door, so this is the public surface rather than a way around one.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the construction
/// outgrows the declared token magnitude.
pub fn provenance(producer: &WallName) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let mut seats = vec![
        GeneratedToken::word(PROVENANCE_PRODUCER_SEAT),
        GeneratedToken::alone(':'),
    ];
    seats.extend(parsed_name(PRODUCER_NAME, producer)?);
    seats.push(GeneratedToken::alone(','));
    seats.push(GeneratedToken::word(PROVENANCE_SCHEMA_SEAT));
    seats.push(GeneratedToken::alone(':'));
    seats.extend(row_schema_identity()?);
    seats.push(GeneratedToken::alone(','));
    let mut tokens = descriptor_path(&[PROVENANCE, PROVENANCE_PRODUCED]);
    tokens.push(group(GeneratedDelimiter::Brace, seats)?);
    Ok(tokens)
}

/// One complete row expression: the row married to the attachment that executes
/// it, under the provenance the producer emitted it against.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the expression outgrows
/// the declared token magnitude.
pub fn row_expression(
    declared: &DescriptorRow,
    producer: &WallName,
) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let mut arguments = declared_row(declared)?;
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(attachment(declared, declared.attachment())?);
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(provenance(producer)?);
    arguments.push(GeneratedToken::alone(','));
    let mut tokens = descriptor_path(&[BINDING, BINDING_ROAD]);
    tokens.push(group(GeneratedDelimiter::Parenthesis, arguments)?);
    Ok(tokens)
}

// ---------------------------------------------------------------------------
// The stamped payload, the gate, and the carrier.
// ---------------------------------------------------------------------------

/// One `named(<namespace>, <stem>)` clause, as the stamp's grammar spells it.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the clause outgrows the
/// declared token magnitude.
pub fn named_clause(name: &WallName) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    Ok(vec![
        GeneratedToken::word(NAMED_CLAUSE),
        group(GeneratedDelimiter::Parenthesis, name_arguments(name))?,
    ])
}

/// One aggregate seat's group, as the stamp's grammar spells it.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the group outgrows the
/// declared token magnitude.
pub fn suite_group(
    seated: &SuiteGroup,
    producer: &WallName,
) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let mut rows: Vec<GeneratedToken> = Vec::new();
    for declared in seated.rows() {
        rows.push(GeneratedToken::word(declared.lens()));
        rows.push(GeneratedToken::alone(':'));
        rows.extend(row_expression(declared, producer)?);
        rows.push(GeneratedToken::alone(','));
    }
    let mut tokens = vec![
        GeneratedToken::word("suite"),
        GeneratedToken::word(seated.seat()),
    ];
    tokens.extend(named_clause(seated.suite())?);
    tokens.push(group(GeneratedDelimiter::Brace, rows)?);
    Ok(tokens)
}

/// The stamped module the gate forwards: the table's name, its stated provenance,
/// the consumer's declared budgets, and every aggregate seat.
///
/// The module is written with NO visibility, so it is private to the consumer's
/// test target: the stamp carries the visibility onto the module, its table
/// function, and its invocation constant together, and a public road out of a
/// generated test module is a public surface nobody asked for.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the module outgrows the
/// declared token magnitude.
pub fn stamped_module(
    payload: &TrialTablePayload,
) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
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
    for seated in payload.groups() {
        body.extend(suite_group(seated, payload.producer())?);
    }
    let mut tokens = vec![
        GeneratedToken::word("mod"),
        GeneratedToken::word(payload.module()),
    ];
    tokens.extend(named_clause(payload.table())?);
    tokens.push(group(GeneratedDelimiter::Brace, body)?);
    Ok(tokens)
}

/// The gate invocation the shell's body is: the producer's expectation, the
/// harness binding, and the cargo forwarded verbatim.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the invocation outgrows
/// the declared token magnitude.
pub fn gate_invocation(
    expectation: GeneratedToken,
    cargo: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let mut clauses = vec![
        GeneratedToken::word(EXPECTED_CLAUSE),
        GeneratedToken::alone(':'),
        expectation,
        GeneratedToken::alone(','),
        GeneratedToken::word(HARNESS_CLAUSE),
        GeneratedToken::alone(':'),
    ];
    clauses.extend(metavariable(CrateFacing::Harness.parameter()));
    clauses.push(GeneratedToken::alone(','));
    clauses.extend(cargo);
    let mut tokens = twin_path(CrateFacing::Harness, &[GATE_MACRO]);
    tokens.push(GeneratedToken::alone('!'));
    tokens.push(group(GeneratedDelimiter::Brace, clauses)?);
    Ok(tokens)
}

/// One `#[doc = "…"]` attribute, as the tokens that spell it.
///
/// The exported carrier carries one because it is a public item and the lint wall
/// denies an undocumented one — and because a reader who trips over a mangled
/// name at a crate root deserves a sentence saying what put it there.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the attribute outgrows
/// the declared token magnitude.
pub fn documentation(sentence: &str) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    attribute(vec![
        GeneratedToken::word("doc"),
        GeneratedToken::alone('='),
        GeneratedToken::text(sentence),
    ])
}

/// One attribute over the body a caller spelled.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the attribute outgrows
/// the declared token magnitude.
pub fn attribute(body: Vec<GeneratedToken>) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    Ok(vec![
        GeneratedToken::alone('#'),
        group(GeneratedDelimiter::Bracket, body)?,
    ])
}

/// The shell's matcher: the two rename twins and the consumer's declared budgets,
/// each named at the invocation so a reader of the call site sees what it
/// supplies.
#[must_use]
pub fn matcher() -> Vec<GeneratedToken> {
    let mut tokens = Vec::new();
    for facing in CrateFacing::ALL {
        tokens.push(GeneratedToken::word(facing.parameter()));
        tokens.push(GeneratedToken::alone(':'));
        tokens.extend(metavariable(facing.parameter()));
        tokens.push(GeneratedToken::alone(':'));
        tokens.push(GeneratedToken::word("ident"));
        tokens.push(GeneratedToken::alone(','));
    }
    tokens.push(GeneratedToken::word(INVOCATION_CLAUSE));
    tokens.push(GeneratedToken::alone(':'));
    tokens.extend(metavariable(INVOCATION_CLAUSE));
    tokens.push(GeneratedToken::alone(':'));
    tokens.push(GeneratedToken::word("expr"));
    tokens
}

/// The exported carrier: a hidden `#[macro_export]` definition under the mangled
/// name, with one rule whose body is the caller-supplied body.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the definition outgrows
/// the declared token magnitude.
pub fn exported_shell(
    name: &ShellName,
    body: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let mut tokens = documentation(SHELL_SENTENCE)?;
    tokens.extend(attribute(vec![
        GeneratedToken::word("doc"),
        group(
            GeneratedDelimiter::Parenthesis,
            vec![GeneratedToken::word("hidden")],
        )?,
    ])?);
    tokens.extend(attribute(vec![GeneratedToken::word("macro_export")])?);
    tokens.push(GeneratedToken::word("macro_rules"));
    tokens.push(GeneratedToken::alone('!'));
    tokens.push(GeneratedToken::word(name.spelling()));
    let mut rule = vec![group(GeneratedDelimiter::Parenthesis, matcher())?];
    rule.push(GeneratedToken::joint('='));
    rule.push(GeneratedToken::alone('>'));
    rule.push(group(GeneratedDelimiter::Brace, body)?);
    rule.push(GeneratedToken::alone(';'));
    tokens.push(group(GeneratedDelimiter::Brace, rule)?);
    Ok(tokens)
}

/// The sentence the exported carrier documents itself with.
///
/// Fixed text rather than a composed one: a doc string carrying a table name or a
/// declaration's spelling would put owner material into an item at the root of a
/// consumer's crate, and the carrier's whole posture is that it is machinery
/// nobody reads.
const SHELL_SENTENCE: &str = "ThreadPak generated support shell: deferred tokens the consumption \
     target invokes. Hidden and mangled because it is machinery; its body is one gate invocation, \
     and the gate compares the producer's expectation against the harness's published one before \
     any constructor reaches type checking.";
