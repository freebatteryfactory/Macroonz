//! The token half of the road: the exported shell, the ONE gate invocation
//! inside it, the stamped payload inside the gate's trials seat, the
//! constructor-calling expression each row is, and the private module the shell
//! splices into the gate's deferred seat.
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
//! the shell's own METAVARIABLE for the HARNESS twin, and the consumer's target
//! supplies it once, at the invocation, so a consumer that renamed the dependency
//! gets its own name back and this home never learns what the name is. The gate's
//! `harness:` clause receives the very same metavariable, which is what makes
//! that binding load-bearing rather than decorative: the gate proves the name the
//! consumer passed reaches the same schema-identity type the harness's own
//! `$crate` reaches, so a wrong name refuses at the door instead of as an
//! unresolved path somewhere inside the payload.
//!
//! The MACHINE twin is not spelled by this crossing and is not asked for at its
//! invocation. A generated row points at a check function the CONSUMPTION target
//! owns, which arrives as an expression rather than as a rendered path; and the
//! evaluation cargo the deferred seat carries spells the binding the DECLARATION
//! stated, rendered by the home that renders those implementations. The twin
//! stays on the wall's roster because the BENCH crossing rides this same carrier
//! and its rows do point at machine callables.
//!
//! # What the consumption target supplies
//!
//! Everything the producer cannot honestly state: the declared budgets, the
//! target and toolchain the runs stand on, the clock a duration is measured
//! against, and — per declared row, in declared order — the two revision
//! commitments and the callable that reaches its conclusion. The matcher names
//! each of them, so a delivery that is short one attachment does not match the
//! carrier at all rather than expanding into a row nothing runs.
//!
//! # What the expressions demand of the harness
//!
//! A row expression is declared by the stamp to answer with the trial-table
//! family, and the constructions on the road to a binding answer with five other
//! families. The emission writes `?`, which is the language's own conversion
//! rather than a variant this home invented inside a vocabulary it does not own.
//! `ROW_CONVERSIONS` in `type_contract.rs` is the complete map of those `?`
//! travels, stated once, and the address owns every arm it names.
//!
//! # The gate's expectation is a roster of canonical tokens
//!
//! The gate's `expected:` clause takes a bracketed roster of thirty-two decimal
//! byte values, and [`expectation_roster`] writes it from the services' own
//! checked-in expectation. The form is what makes the comparison sound: the gate
//! matches TOKENS, a byte string has many spellings of one value, and the
//! spelling on this side is the compiler's literal writer's rather than a
//! declaration's — so a byte string here would be a producer hostage to an
//! escaping convention nobody controls. An unsuffixed integer has exactly one
//! rendering, and the two sides are one token by construction.

use super::{
    BoundPath, CrateFacing, DeclarationDoor, DeferredDelivery, DescriptorRow,
    GENERATED_ROW_PROJECTION, GENERATED_TABLE_PRODUCER, PRODUCER_NAMESPACE, ShellName,
    ShellRenderIssue, SuiteGroup, TrialDelivery, TrialLensName, TrialTablePayload, WallName,
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

/// The gate's clause carrying the trial material the gate forwards to the stamp.
///
/// One of the two cargo seats the published grammar always writes. Its content
/// is the harness's own grammar, and the gate reads it as such.
pub const TRIALS_CLAUSE: &str = "trials";

/// The gate's clause carrying the token trees the gate never parses.
///
/// The other cargo seat, and the one that makes the gate a gate for everything
/// the carrier delivers rather than for the rows alone: on a matched pin the
/// gate emits this seat verbatim, and on a mismatch it emits its refusal and
/// neither seat.
pub const DEFERRED_CLAUSE: &str = "deferred";

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

/// The stamp clause carrying the target and toolchain the runs stand on.
///
/// A shell parameter for exactly the reason the budgets are one, and a stronger
/// one besides: nothing in the harness DERIVES a triple or a toolchain identity,
/// they enter an execution key, and a producer that guessed at either would buy a
/// consumer a cache hit nothing verified.
pub const TARGET_CLAUSE: &str = "target";

/// The stamp clause carrying the harness wall-measurement source.
///
/// A shell parameter on the same terms. A caller with no measurement to offer
/// declares the harness's unavailable clock, whose reading stays distinct
/// from an observed zero.
pub const CLOCK_CLAUSE: &str = "clock";

/// The shell clause carrying one executable attachment per declared row.
///
/// The seat that makes a generated row runnable at all. A row's callable and its
/// two revision commitments live in the CONSUMPTION target — the check functions
/// are written in the test target that invokes the carrier, which is not the
/// crate the declaration sits in — so they arrive here as expressions and keep
/// the invocation site's own hygiene.
pub const ATTACHMENTS_CLAUSE: &str = "attachments";

/// The attachment seat carrying one row's subject revision commitment.
pub const SUBJECT_REVISION_SEAT: &str = "subject_revision";

/// The attachment seat carrying one row's check revision commitment.
pub const CHECK_REVISION_SEAT: &str = "check_revision";

/// The attachment seat carrying the callable that reaches one row's conclusion.
pub const CALL_SEAT: &str = "call";

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

/// Two spellings as the two text literals a namespaced parser takes, comma
/// separated.
///
/// The spelling road rather than the value road, because two of this home's own
/// names — the producer's and the projection's — are declared as constants and
/// never become a [`WallName`]: a road that required one would have to build a
/// checked value out of literals this crate wrote, and would then carry a refusal
/// for the empty case those literals rule out.
#[must_use]
pub fn spelled_arguments(namespace: &str, stem: &str) -> Vec<GeneratedToken> {
    vec![
        GeneratedToken::text(namespace),
        GeneratedToken::alone(','),
        GeneratedToken::text(stem),
    ]
}

/// One namespaced name as the two text literals its parser takes, comma
/// separated.
#[must_use]
pub fn name_arguments(name: &WallName) -> Vec<GeneratedToken> {
    spelled_arguments(name.namespace(), name.stem())
}

/// One call to a namespaced reference's parser over two spellings, with the row
/// expression's own `?` on it.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the call outgrows the
/// declared token magnitude.
pub fn parsed_spelling(
    reference: &str,
    namespace: &str,
    stem: &str,
) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let mut tokens = descriptor_path(&[reference, NAME_ROAD]);
    tokens.push(group(
        GeneratedDelimiter::Parenthesis,
        spelled_arguments(namespace, stem),
    )?);
    tokens.push(GeneratedToken::alone('?'));
    Ok(tokens)
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
    parsed_spelling(reference, name.namespace(), name.stem())
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
/// the bracketed roster of decimal byte values the gate's opening arm matches.
///
/// The value is read from the services' one checked-in expectation and from
/// nowhere else, so what this emission carries and the constant a schema rewrite
/// moves are one fact rather than two.
///
/// # Why a roster of numbers and not one byte string
///
/// **The gate matches TOKENS, and a byte string has many spellings of one
/// value.** `b"\x71"` and `b"q"` carry one byte and are two tokens, and the
/// spelling on THIS side is the compiler's own literal writer's choice rather
/// than a declaration anybody made — so a hand-written pattern on the other side
/// would be hostage to an escaping convention nobody controls. An unsuffixed
/// integer has exactly one rendering, so the two sides are one token by
/// construction.
///
/// The first producer to reach the gate rendered a byte string and was refused
/// over a value both sides agreed on. That refusal is why this road writes
/// numbers.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the roster outgrows the
/// declared token magnitude.
pub fn expectation_roster() -> Result<GeneratedToken, ShellRenderIssue> {
    expectation_roster_of(&EXPECTED_GENERATED_SUPPORT_SCHEMA_ID)
}

/// The same roster, over an expectation a caller holds — the road a posture flip
/// travels, since an expectation under another posture is a different type and
/// the same rendering.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the roster outgrows the
/// declared token magnitude.
pub fn expectation_roster_of<Posture>(
    expectation: &ExpectedGeneratedSupportSchemaId<Posture>,
) -> Result<GeneratedToken, ShellRenderIssue> {
    let material: &[u8; 32] = expectation.as_bytes();
    let mut bytes: Vec<GeneratedToken> = Vec::new();
    for byte in material {
        bytes.push(GeneratedToken::number(u64::from(*byte)));
        bytes.push(GeneratedToken::alone(','));
    }
    group(GeneratedDelimiter::Bracket, bytes)
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

/// The local the row expression binds one parsed subject route to.
const SUBJECT_LOCAL: &str = "subject";

/// The local the row expression binds one parsed check reference to.
const CHECK_LOCAL: &str = "check";

/// The local the row expression binds its declared row to.
const ROW_LOCAL: &str = "row";

/// The local the row expression binds its executable attachment to.
const ATTACHMENT_LOCAL: &str = "attachment";

/// The metavariable one row's attachment seat arrives under, composed from the
/// row's own lens and the seat's name.
///
/// The lens is a Rust identifier by construction and the seat's name is one of
/// three declared words, so the composition is an identifier too — and two
/// distinct lenses compose two distinct metavariables, which is what lets one
/// matcher name every row's three seats without a register of what it has already
/// spelled.
#[must_use]
pub fn attachment_metavariable(lens: &TrialLensName, seat: &str) -> String {
    let spelling = lens.spelling();
    format!("{spelling}_{seat}")
}

/// One `let <name> = <expression>;` statement.
///
/// The row expression is a BLOCK rather than one nested call, because the subject
/// route and the check reference are each needed twice — once by the row and once
/// by the attachment — and the binding constructor's whole job is to establish
/// that the two agree. Parsing each of them twice would make that check compare
/// two separately parsed values, which passes for a different reason than the one
/// it was written for; binding each once makes the agreement structural.
#[must_use]
pub fn bound_local(name: &str, expression: Vec<GeneratedToken>) -> Vec<GeneratedToken> {
    let mut tokens = vec![
        GeneratedToken::word("let"),
        GeneratedToken::word(name),
        GeneratedToken::alone('='),
    ];
    tokens.extend(expression);
    tokens.push(GeneratedToken::alone(';'));
    tokens
}

/// One row's executable attachment: the two locals the row already parsed, the
/// two revision commitments the consumption target declared, and the callable it
/// named.
///
/// # Where the three arguments come from
///
/// The invocation, not the declaration. A generated row points at a check
/// function the CONSUMPTION target owns, and that target is neither the crate the
/// declaration sits in nor either of the two crates the wall lets a consumer
/// rename — so there is no crate binding a rendered path could be rooted at and
/// no honest way for this side to spell one. What the shell writes is the
/// metavariable each seat arrives under, and the test target's own hygiene
/// resolves the expression it supplied.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the call outgrows the
/// declared token magnitude.
pub fn attachment(declared: &DescriptorRow) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let lens = declared.lens();
    let mut arguments = vec![
        GeneratedToken::word(SUBJECT_LOCAL),
        GeneratedToken::alone(','),
        GeneratedToken::word(CHECK_LOCAL),
        GeneratedToken::alone(','),
    ];
    arguments.extend(metavariable(&attachment_metavariable(
        lens,
        SUBJECT_REVISION_SEAT,
    )));
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(metavariable(&attachment_metavariable(
        lens,
        CHECK_REVISION_SEAT,
    )));
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(metavariable(&attachment_metavariable(lens, CALL_SEAT)));
    arguments.push(GeneratedToken::alone(','));
    let mut tokens = descriptor_path(&[ATTACHMENT, ATTACHMENT_ROAD]);
    tokens.push(group(GeneratedDelimiter::Parenthesis, arguments)?);
    Ok(tokens)
}

/// One row's origin: the generated arm, and the producer facts inside it.
///
/// # Authority
///
/// **Both facts are the PRODUCER's and neither is read off a declaration.** The
/// door is a row of the closed [`DeclarationDoor`] roster, which no authored
/// declaration can reach, and the projection is this home's own declared
/// spelling. A row that could state either would be an authored declaration
/// signing an act it did not perform.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the call outgrows the
/// declared token magnitude.
pub fn origin(door: DeclarationDoor) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let mut facts = parsed_spelling(DOOR_REF, PRODUCER_NAMESPACE, door.stable_name())?;
    facts.push(GeneratedToken::alone(','));
    facts.extend(parsed_spelling(
        PROJECTION_REF,
        PRODUCER_NAMESPACE,
        GENERATED_ROW_PROJECTION,
    )?);
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

/// One row, in the harness's closed field order, over the two locals the block
/// already parsed.
///
/// # The suite is the group's
///
/// It arrives as a parameter rather than off the row, because a row states no
/// suite: one aggregate seat selects on one suite and every row under it runs
/// under that one, so the seat is the single seat where it is written. A row that
/// carried its own would be one fact stated twice, and two lawful spellings that
/// disagree would produce a seat that selects none of its rows.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the call outgrows the
/// declared token magnitude.
pub fn declared_row(
    declared: &DescriptorRow,
    suite: &WallName,
    door: DeclarationDoor,
) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let references = declared.references();
    let mut arguments = parsed_name(CLAIM_REF, &references.claim)?;
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(parsed_name(EXECUTION_SUITE, suite)?);
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(classification(declared)?);
    arguments.push(GeneratedToken::alone(','));
    arguments.push(GeneratedToken::word(SUBJECT_LOCAL));
    arguments.push(GeneratedToken::alone(','));
    arguments.push(GeneratedToken::word(CHECK_LOCAL));
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(parsed_name(POPULATION_REF, &references.population)?);
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(origin(door)?);
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
/// # Authority
///
/// Both seats are the PRODUCER's. The name is this home's declared spelling and
/// the schema identity is derived inside the expression itself, so neither is a
/// value an authored declaration could have supplied.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the construction
/// outgrows the declared token magnitude.
pub fn provenance() -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let mut seats = vec![
        GeneratedToken::word(PROVENANCE_PRODUCER_SEAT),
        GeneratedToken::alone(':'),
    ];
    seats.extend(parsed_spelling(
        PRODUCER_NAME,
        PRODUCER_NAMESPACE,
        GENERATED_TABLE_PRODUCER,
    )?);
    seats.push(GeneratedToken::alone(','));
    seats.push(GeneratedToken::word(PROVENANCE_SCHEMA_SEAT));
    seats.push(GeneratedToken::alone(':'));
    seats.extend(row_schema_identity()?);
    seats.push(GeneratedToken::alone(','));
    let mut tokens = descriptor_path(&[PROVENANCE, PROVENANCE_PRODUCED]);
    tokens.push(group(GeneratedDelimiter::Brace, seats)?);
    Ok(tokens)
}

/// One complete row expression: a BLOCK that parses the subject and the check
/// once, builds the row and the attachment over those two values, and marries
/// them under the provenance the producer emitted them against.
///
/// # Why a block
///
/// The binding constructor's whole job is to establish that the row's subject
/// route and check reference are the ATTACHMENT's. A rendering that parsed each
/// name twice would hand it two separately parsed values, and the check would
/// pass because two parses of one spelling agree — which is a different statement
/// from the one the constructor was written to make. One parse, two consumers,
/// and the agreement is structural.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the expression outgrows
/// the declared token magnitude.
pub fn row_expression(
    declared: &DescriptorRow,
    suite: &WallName,
    door: DeclarationDoor,
) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let references = declared.references();
    let mut body = bound_local(
        SUBJECT_LOCAL,
        parsed_name(SUBJECT_ROUTE, &references.subject)?,
    );
    body.extend(bound_local(
        CHECK_LOCAL,
        parsed_name(CHECK_REF, &references.check)?,
    ));
    body.extend(bound_local(ROW_LOCAL, declared_row(declared, suite, door)?));
    body.extend(bound_local(ATTACHMENT_LOCAL, attachment(declared)?));

    let mut arguments = vec![
        GeneratedToken::word(ROW_LOCAL),
        GeneratedToken::alone(','),
        GeneratedToken::word(ATTACHMENT_LOCAL),
        GeneratedToken::alone(','),
    ];
    arguments.extend(provenance()?);
    arguments.push(GeneratedToken::alone(','));
    let mut married = descriptor_path(&[BINDING, BINDING_ROAD]);
    married.push(group(GeneratedDelimiter::Parenthesis, arguments)?);
    body.extend(married);

    Ok(vec![group(GeneratedDelimiter::Brace, body)?])
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
    door: DeclarationDoor,
) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let mut rows: Vec<GeneratedToken> = Vec::new();
    for declared in seated.rows() {
        rows.push(GeneratedToken::word(declared.lens().spelling()));
        rows.push(GeneratedToken::alone(':'));
        rows.extend(row_expression(declared, seated.suite(), door)?);
        rows.push(GeneratedToken::alone(','));
    }
    let mut tokens = vec![
        GeneratedToken::word("suite"),
        GeneratedToken::word(seated.seat().spelling()),
    ];
    tokens.extend(named_clause(seated.suite())?);
    tokens.push(group(GeneratedDelimiter::Brace, rows)?);
    Ok(tokens)
}

/// One `<clause>: $<clause>,` pair, where the value is the shell's own
/// metavariable for a fact the consumption target declared.
///
/// The three host clauses the stamp requires are written through one road, so a
/// clause the stamp adds is added here once and cannot be spelled two ways
/// between the matcher and the body.
#[must_use]
pub fn host_clause(clause: &str) -> Vec<GeneratedToken> {
    let mut tokens = vec![GeneratedToken::word(clause), GeneratedToken::alone(':')];
    tokens.extend(metavariable(clause));
    tokens.push(GeneratedToken::alone(','));
    tokens
}

/// The three host facts a stamped table stands on, in the order the stamp's
/// grammar states them: the declared budgets, the target and toolchain the runs
/// stand on, and the clock a duration is measured against.
///
/// # Authority
///
/// **All three are the CONSUMPTION target's, and all three are required.** A
/// producer that wrote its own budgets would be declaring how long somebody
/// else's machine may spend; one that wrote its own target would be guessing at a
/// coordinate of a cache key that nothing in the harness derives; one that wrote
/// its own clock would be stating what a nanosecond reading is worth on a host it
/// has never seen. So each is a shell parameter, and the stamp is handed the
/// metavariable rather than a value.
pub const HOST_CLAUSES: [&str; 3] = [INVOCATION_CLAUSE, TARGET_CLAUSE, CLOCK_CLAUSE];

/// The visibility the stamped module and every item inside it carry.
///
/// `pub(crate)`, and the reach is exactly the consumption TARGET: the stamp
/// lands in a test binary or in a `cfg(test)` module, so crate visibility there
/// reaches the seats and the table a parity lane reads and reaches nothing a
/// consumer publishes. The stamp carries one visibility onto the module, its
/// table function, its target function, and its two constants together, so no
/// public road ever ends at a private one.
///
/// Narrower than `pub` on purpose, and wider than nothing on purpose. Written
/// with no visibility at all, the table function is private to a module nobody
/// can name, and a generated table could be observed only through whether its
/// seats ran — which makes "the generated road and the hand road state one
/// trial" a claim no lane could take.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the group outgrows the
/// declared token magnitude.
pub fn stamped_visibility() -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    Ok(vec![
        GeneratedToken::word("pub"),
        group(
            GeneratedDelimiter::Parenthesis,
            vec![GeneratedToken::word("crate")],
        )?,
    ])
}

/// The stamped module the gate forwards: the table's name, its stated provenance,
/// the consumer's three declared host facts, and every aggregate seat.
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
            spelled_arguments(PRODUCER_NAMESPACE, GENERATED_TABLE_PRODUCER),
        )?,
        GeneratedToken::word("against"),
    ];
    body.extend(table_schema_identity()?);
    body.push(GeneratedToken::alone(','));
    for clause in HOST_CLAUSES {
        body.extend(host_clause(clause));
    }
    for seated in payload.groups() {
        body.extend(suite_group(seated, payload.door())?);
    }
    let mut tokens = stamped_visibility()?;
    tokens.push(GeneratedToken::word("mod"));
    tokens.push(GeneratedToken::word(payload.module().spelling()));
    tokens.extend(named_clause(payload.table())?);
    tokens.push(group(GeneratedDelimiter::Brace, body)?);
    Ok(tokens)
}

/// The gate invocation the shell's body IS: the producer's expectation, the
/// harness binding, the trials seat, and the deferred seat.
///
/// # Both cargo seats, always
///
/// The published grammar writes four clauses and the two cargo seats are the
/// last of them, each a braced group. A seat may be EMPTY — a crossing that
/// declared no rows renders `trials: {}`, and one that deferred nothing renders
/// `deferred: {}` — and it is still written, because a gate arm that had to
/// match two clause shapes would be two arms and one pin would open two doors.
///
/// # What each seat is for
///
/// `trials:` carries material under the HARNESS's own grammar, which the gate
/// forwards to its stamp. `deferred:` carries token trees the gate never parses
/// and emits verbatim. The two are separate seats because they are two
/// vocabularies: folding the deferred trees in beside the rows would hand the
/// stamp items it has no clause for, and standing them outside the invocation
/// would release them on a pin MISMATCH — the exact leak the seat closes.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where either seat, or the
/// invocation around them, outgrows the declared token magnitude.
pub fn gate_invocation(
    expectation: GeneratedToken,
    trials: Vec<GeneratedToken>,
    deferred: Vec<GeneratedToken>,
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
    clauses.push(GeneratedToken::word(TRIALS_CLAUSE));
    clauses.push(GeneratedToken::alone(':'));
    clauses.push(group(GeneratedDelimiter::Brace, trials)?);
    clauses.push(GeneratedToken::alone(','));
    clauses.push(GeneratedToken::word(DEFERRED_CLAUSE));
    clauses.push(GeneratedToken::alone(':'));
    clauses.push(group(GeneratedDelimiter::Brace, deferred)?);
    clauses.push(GeneratedToken::alone(','));
    let mut tokens = twin_path(CrateFacing::Harness, &[GATE_MACRO]);
    tokens.push(GeneratedToken::alone('!'));
    tokens.push(group(GeneratedDelimiter::Brace, clauses)?);
    Ok(tokens)
}

/// What one shell writes into the gate's TRIALS seat: the stamped payload where
/// the caller declared one, and nothing where it declared none.
///
/// # A carrier with no rows is not a carrier with nothing to carry
///
/// The rows a descriptor states about itself arrive whole from the caller, so a
/// door holding no payload has no rows to declare — and it may still have cargo
/// to defer. This road is what makes that delivery writable: the seat renders
/// empty, the deferred seat renders full, and the gate carries one invocation
/// exactly as it does for a crossing that declared both.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the stamped payload
/// outgrows the declared token magnitude.
pub(crate) fn trial_cargo(
    declared: TrialDelivery<'_>,
) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    match declared {
        TrialDelivery::NothingDeclared => Ok(Vec::new()),
        TrialDelivery::Declared(payload) => stamped_module(payload),
    }
}

/// The private module one shell splices into the gate's DEFERRED seat: the local
/// subject the cargo's implementations stand over, the cargo itself, and one
/// constant per selection the cargo reads.
///
/// # Where it stands, and why there
///
/// INSIDE the gate invocation, in the seat the published grammar reserves for
/// trees the gate never parses. It used to stand beside the invocation, and the
/// cost was exact: a pin MISMATCH suppressed the rows while releasing this
/// module, so a consumer whose published pair was incoherent was handed one
/// refusal AND a module of evaluation copies to compile. Everything the carrier
/// delivers rides behind the same pin or nothing does.
///
/// It is still not written into the harness's grammar, and that is what the two
/// seats are for: the trials seat is the harness's vocabulary, the deferred seat
/// is opaque token trees the gate forwards verbatim, and this module lands in
/// the second.
///
/// # What the module is for
///
/// A deferred implementation is a copy of one the declaration site already
/// carries, so a copy rendered for the type the declaration named would be that
/// implementation declared twice where the declaration is — and, once the copy
/// reaches a consumer's test target, a foreign trait implemented for a foreign
/// type. The module answers both at once: it declares a type the target owns,
/// the copies stand over that, and the module's own name is the shell's
/// content-addressed spelling, so nothing outside the expansion can reach the
/// subject and two shells in one crate never collide.
///
/// # Ordering
///
/// The subject is declared first, the cargo second — its own items include the
/// active-point rosters — and the constants last, because a constant stands at a
/// row of a roster the cargo declares. Rust resolves a module's items without
/// regard to order, so the order is for the reader, and the reader is who it is
/// written for.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the module outgrows
/// the declared token magnitude.
pub(crate) fn deferred_module(
    name: &ShellName,
    deferred: DeferredDelivery<'_>,
) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let cargo = match deferred {
        // An expansion that planned no member into this carrier splices no
        // module at all: a module carrying no cargo would declare a subject
        // nothing implements and constants nothing reads, which is a different
        // thing from a carrier nothing was ever deferred into.
        DeferredDelivery::NothingDeferred => return Ok(Vec::new()),
        DeferredDelivery::Carried(carried) => carried,
    };
    let mut body = vec![
        GeneratedToken::word("struct"),
        GeneratedToken::word(cargo.subject()),
        GeneratedToken::alone(';'),
    ];
    body.extend(cargo.tree().tokens().cloned());
    for selector in cargo.selectors() {
        body.push(GeneratedToken::word("const"));
        body.push(GeneratedToken::word(selector.constant()));
        body.push(GeneratedToken::alone(':'));
        body.push(GeneratedToken::word(selector.active_enum()));
        body.push(GeneratedToken::alone('='));
        body.push(GeneratedToken::word(selector.active_enum()));
        body.push(GeneratedToken::joint(':'));
        body.push(GeneratedToken::alone(':'));
        body.push(GeneratedToken::word(selector.variant()));
        body.push(GeneratedToken::alone(';'));
    }
    Ok(vec![
        GeneratedToken::word("mod"),
        GeneratedToken::word(name.deferred_module().as_str()),
        group(GeneratedDelimiter::Brace, body)?,
    ])
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

/// One `<name>: $<name>:<fragment>,` clause of the shell's matcher.
#[must_use]
pub fn matched_clause(name: &str, fragment: &str) -> Vec<GeneratedToken> {
    let mut tokens = vec![GeneratedToken::word(name), GeneratedToken::alone(':')];
    tokens.extend(metavariable(name));
    tokens.push(GeneratedToken::alone(':'));
    tokens.push(GeneratedToken::word(fragment));
    tokens.push(GeneratedToken::alone(','));
    tokens
}

/// One row's attachment clause, as the matcher spells it: the row's own lens, and
/// the three seats the consumption target fills for it.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the clause outgrows the
/// declared token magnitude.
pub fn matched_attachment(lens: &TrialLensName) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let mut seats: Vec<GeneratedToken> = Vec::new();
    for seat in [SUBJECT_REVISION_SEAT, CHECK_REVISION_SEAT, CALL_SEAT] {
        let named = attachment_metavariable(lens, seat);
        seats.push(GeneratedToken::word(seat));
        seats.push(GeneratedToken::alone(':'));
        seats.extend(metavariable(&named));
        seats.push(GeneratedToken::alone(':'));
        seats.push(GeneratedToken::word("expr"));
        seats.push(GeneratedToken::alone(','));
    }
    Ok(vec![
        GeneratedToken::word(lens.spelling()),
        group(GeneratedDelimiter::Brace, seats)?,
        GeneratedToken::alone(','),
    ])
}

/// The shell's matcher: exactly the facts the delivery it guards consumes, each
/// named at the invocation so a reader of the call site sees what it supplies.
///
/// # What a delivery asks for, and what it does not
///
/// The HARNESS binding is asked for always: every constructor the shell renders
/// is rooted at it, and the gate's own `harness:` clause is what proves the name
/// the consumer passed reaches the same schema-identity type the harness's
/// `$crate` reaches.
///
/// The MACHINE binding is not asked for here, and the absence is a fact about
/// this delivery rather than a change to the wall. A trial delivery's rows point
/// at check functions the CONSUMPTION target owns and its deferred cargo spells
/// the machine binding the DECLARATION stated, so nothing this crossing renders
/// is rooted at a machine metavariable — and an argument a consumer supplies that
/// nothing spells is a value the plan decided and nothing read.
/// [`CrateFacing`] keeps both twins because the BENCH crossing rides this same
/// carrier and its rows do point at machine callables.
///
/// The three host clauses and the attachment roster are asked for exactly when
/// the delivery carries ROWS. A delivery whose trials seat is empty stamps no
/// table, so there is no budget to spend, no target to stand on, no clock to
/// measure with, and no row to attach a callable to.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the matcher outgrows
/// the declared token magnitude.
pub(crate) fn matcher(
    declared: TrialDelivery<'_>,
) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let mut tokens = matched_clause(CrateFacing::Harness.parameter(), "ident");
    let TrialDelivery::Declared(payload) = declared else {
        return Ok(tokens);
    };
    for clause in HOST_CLAUSES {
        tokens.extend(matched_clause(clause, "expr"));
    }
    let mut attachments: Vec<GeneratedToken> = Vec::new();
    for seated in payload.groups() {
        for row in seated.rows() {
            attachments.extend(matched_attachment(row.lens())?);
        }
    }
    tokens.push(GeneratedToken::word(ATTACHMENTS_CLAUSE));
    tokens.push(GeneratedToken::alone(':'));
    tokens.push(group(GeneratedDelimiter::Brace, attachments)?);
    tokens.push(GeneratedToken::alone(','));
    Ok(tokens)
}

/// The exported carrier: a hidden `#[macro_export]` definition under the mangled
/// name, with one rule matching what the delivery consumes and expanding to the
/// caller-supplied body.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the definition outgrows
/// the declared token magnitude.
pub fn exported_shell(
    name: &ShellName,
    matched: Vec<GeneratedToken>,
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
    let mut rule = vec![group(GeneratedDelimiter::Parenthesis, matched)?];
    rule.push(GeneratedToken::joint('='));
    rule.push(GeneratedToken::alone('>'));
    rule.push(group(GeneratedDelimiter::Brace, body)?);
    rule.push(GeneratedToken::alone(';'));
    tokens.push(group(GeneratedDelimiter::Brace, rule)?);
    Ok(tokens)
}

/// The caller-named alias: an exported `macro_rules!` under the spelling the
/// declaration chose, whose one rule forwards its whole input to the hidden
/// carrier.
///
/// # Why an alias exists at all
///
/// The physical carrier is exported under the PLAN's identity at full width, so
/// two declarations in one crate can never mint one exported name — and no person
/// can know that spelling before the expansion runs. Rust has no road from a
/// runtime string or an associated constant to a macro invocation, so a carrier
/// with no second name is a carrier nobody can invoke. The alias is the address a
/// person types; the mangled name stays the collision-free vehicle.
///
/// # What it forwards
///
/// Every token, unread. The alias declares no grammar of its own — the hidden
/// carrier's matcher is the grammar, and a second matcher here would be a second
/// shape a caller has to satisfy and a second place for it to drift. A caller
/// whose input does not match reads the hidden carrier's own refusal.
///
/// The forward is spelled `$crate::<mangled>`, so it resolves inside whatever
/// crate the declaration site sits in whatever that crate is called — the same
/// mechanism the harness's own stamp reaches its defining crate by.
///
/// # Bounds
///
/// A delivery that declared no rows renders no alias. There is no name to render
/// one under — the support spelling rides the trial payload — and a carrier with
/// an empty trials seat carries nothing a person invokes it for.
///
/// Two declarations in one crate that chose one alias spelling collide as an
/// ordinary duplicate macro definition, at the consumer's own compiler, in the
/// consumer's own words. Nothing here keeps a register of what it has exported.
///
/// # Errors
///
/// Returns [`ShellRenderIssue::ShellTreeUnbounded`] where the definition outgrows
/// the declared token magnitude.
pub(crate) fn public_alias(
    name: &ShellName,
    declared: TrialDelivery<'_>,
) -> Result<Vec<GeneratedToken>, ShellRenderIssue> {
    let TrialDelivery::Declared(payload) = declared else {
        return Ok(Vec::new());
    };
    let mut tokens = documentation(ALIAS_SENTENCE)?;
    tokens.extend(attribute(vec![GeneratedToken::word("macro_export")])?);
    tokens.push(GeneratedToken::word("macro_rules"));
    tokens.push(GeneratedToken::alone('!'));
    tokens.push(GeneratedToken::word(payload.support().spelling()));

    let mut matched = metavariable("input");
    matched.push(GeneratedToken::alone(':'));
    matched.push(GeneratedToken::word("tt"));
    let mut repeated = vec![GeneratedToken::joint('$')];
    repeated.push(group(GeneratedDelimiter::Parenthesis, matched)?);
    repeated.push(GeneratedToken::alone('*'));

    let mut forwarded = metavariable("crate");
    forwarded.push(GeneratedToken::joint(':'));
    forwarded.push(GeneratedToken::alone(':'));
    forwarded.push(GeneratedToken::word(name.spelling()));
    forwarded.push(GeneratedToken::alone('!'));
    let mut carried = vec![GeneratedToken::joint('$')];
    carried.push(group(
        GeneratedDelimiter::Parenthesis,
        metavariable("input"),
    )?);
    carried.push(GeneratedToken::alone('*'));
    forwarded.push(group(GeneratedDelimiter::Brace, carried)?);

    let mut rule = vec![group(GeneratedDelimiter::Parenthesis, repeated)?];
    rule.push(GeneratedToken::joint('='));
    rule.push(GeneratedToken::alone('>'));
    rule.push(group(GeneratedDelimiter::Brace, forwarded)?);
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

/// The sentence the caller-named alias documents itself with.
///
/// Fixed text on exactly [`SHELL_SENTENCE`]'s terms. The alias's own NAME is
/// owner material — the author chose it — and the sentence beside it says what
/// the item is rather than restating what the author called it.
const ALIAS_SENTENCE: &str = "ThreadPak generated support: invoke this from a test target to \
     receive the trial rows this declaration states. It forwards every token to the hidden \
     plan-keyed carrier, whose matcher is the grammar and whose gate compares the producer's \
     expected schema identity against the harness's published one.";
