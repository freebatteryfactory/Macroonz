//! The token half of the trial road: the stamped module, the constructor-calling expression each row is, and the two schema-identity expressions a produced table and a produced row pin against.
//!
//! # Tokens, not text
//!
//! Every path is spelled as segments, every literal is a typed literal whose quoting the tree owns, and every brace is a group.
//! No function here composes Rust source.
//!
//! # Nothing spells a crate
//!
//! Every path begins with the harness binding's own metavariable, and the carrier's invocation supplies it once, so a consumer that renamed the dependency gets its own name back.
//!
//! # What the expressions demand of the address
//!
//! A row expression is declared by the stamp to answer with the trial-table family, and the constructions on the road to a binding answer with their own.
//! The emission writes `?`, which is the language's own conversion rather than a variant this home invented inside a vocabulary it does not own.

use super::{Row, SuiteGroup, TrialTable, Trials};
use crate::bounded::Overflow;
use crate::descriptor::vocabulary::{self, HarnessName, HarnessWord};
use crate::descriptor::{Emitter, Name};
use crate::kind::Kind;
use crate::stamp::{Visibility, declared_reach_tokens};
use crate::token::{
    GeneratedDelimiter, GeneratedToken, bound_local, comma, group, metavariable, method_call,
    roster, text_pair,
};

/// The local a row expression binds its parsed subject route to.
const SUBJECT_LOCAL: &str = "subject";

/// The local a row expression binds its parsed check reference to.
const CHECK_LOCAL: &str = "check";

/// The local a row expression binds its declared row to.
const ROW_LOCAL: &str = "row";

/// The local a row expression binds its executable attachment to.
const ATTACHMENT_LOCAL: &str = "attachment";

/// The binding one closure parameter of the table's schema expression carries.
const DECLARED_LOCAL: &str = "declared";

/// One call to a namespaced reference's parser over two spellings, with the row expression's own `?` on it.
///
/// # Errors
///
/// Returns [`Overflow`] where the call outgrows the declared magnitude.
fn parsed(
    reference: HarnessName,
    namespace: &str,
    stem: &str,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut tokens = vocabulary::road(
        &[HarnessName::Descriptor, reference, HarnessName::Named],
        text_pair(namespace, stem),
    )?;
    tokens.push(GeneratedToken::alone('?'));
    Ok(tokens)
}

/// One call to a namespaced reference's parser over a declared name.
///
/// # Errors
///
/// Returns [`Overflow`] where the call outgrows the declared magnitude.
fn parsed_name(reference: HarnessName, name: &Name) -> Result<Vec<GeneratedToken>, Overflow> {
    parsed(reference, name.namespace(), name.stem())
}

/// The schema identity a produced TABLE pins against, in the shape the stamp's `against` clause requires: a result the stamp itself chains onto, with every refusal already mapped into the stamp's own family.
///
/// # Errors
///
/// Returns [`Overflow`] where the expression outgrows the declared magnitude.
pub fn table_schema_identity() -> Result<Vec<GeneratedToken>, Overflow> {
    let published = vocabulary::road(
        &[
            HarnessName::Descriptor,
            HarnessName::Schema,
            HarnessName::Published,
        ],
        Vec::new(),
    )?;
    let declared = method_call(
        published,
        "map_err",
        vocabulary::path(&[
            HarnessName::Descriptor,
            HarnessName::TableRefusal,
            HarnessName::SchemaNotDeclared,
        ]),
    )?;
    method_call(declared, "and_then", schema_closure()?)
}

/// The closure the table's schema expression chains: one root schema declaration in, its derived identity or the stamp's encoding refusal out.
fn schema_closure() -> Result<Vec<GeneratedToken>, Overflow> {
    let taken = method_call(
        vec![GeneratedToken::word(DECLARED_LOCAL)],
        HarnessName::Identity.spelling(),
        Vec::new(),
    )?;
    let mapped = method_call(
        taken,
        "map_err",
        vocabulary::path(&[
            HarnessName::Descriptor,
            HarnessName::TableRefusal,
            HarnessName::SchemaNotEncoded,
        ]),
    )?;
    let mut tokens = vec![
        GeneratedToken::alone('|'),
        GeneratedToken::word(DECLARED_LOCAL),
        GeneratedToken::alone('|'),
    ];
    tokens.extend(mapped);
    Ok(tokens)
}

/// The schema identity one produced ROW's binding pins against, with the row expression's own `?` on each refusal.
///
/// # Errors
///
/// Returns [`Overflow`] where the expression outgrows the declared magnitude.
pub fn row_schema_identity() -> Result<Vec<GeneratedToken>, Overflow> {
    let mut tokens = vocabulary::road(
        &[
            HarnessName::Descriptor,
            HarnessName::Schema,
            HarnessName::Published,
        ],
        Vec::new(),
    )?;
    tokens.push(GeneratedToken::alone('?'));
    tokens.push(GeneratedToken::alone('.'));
    tokens.push(GeneratedToken::word(HarnessName::Identity.spelling()));
    tokens.push(group(GeneratedDelimiter::Parenthesis, Vec::new())?);
    tokens.push(GeneratedToken::alone('?'));
    Ok(tokens)
}

/// The metavariable one row's attachment seat arrives under, composed from the row's own lens and the seat's name.
///
/// The lens is a Rust identifier by construction and the seat's name is one of three declared words, so the composition is an identifier too — and two distinct lenses compose two distinct metavariables, which is what lets one matcher name every row's three seats without a register of what it has already spelled.
#[must_use]
pub fn attachment_metavariable(lens: &str, seat: HarnessWord) -> String {
    crate::descriptor::emitting::row_metavariable(lens, seat)
}

/// One row's executable attachment: the two locals the row already parsed, the two revision commitments the consumption target declared, and the callable it named.
///
/// The three arguments come from the invocation and not from the declaration: a generated row points at a check function the consumption target owns, and there is no crate binding a rendered path could be rooted at.
///
/// # Errors
///
/// Returns [`Overflow`] where the call outgrows the declared magnitude.
pub fn attachment(declared: &Row) -> Result<Vec<GeneratedToken>, Overflow> {
    let lens = declared.lens().spelling();
    let mut arguments = vec![
        GeneratedToken::word(SUBJECT_LOCAL),
        GeneratedToken::alone(','),
        GeneratedToken::word(CHECK_LOCAL),
        GeneratedToken::alone(','),
    ];
    for seat in [
        HarnessWord::SubjectRevision,
        HarnessWord::CheckRevision,
        HarnessWord::Call,
    ] {
        arguments.extend(metavariable(&attachment_metavariable(lens, seat)));
        arguments.push(GeneratedToken::alone(','));
    }
    vocabulary::road(
        &[
            HarnessName::Descriptor,
            HarnessName::Attachment,
            HarnessName::Attached,
        ],
        arguments,
    )
}

/// One row's origin: the generated arm, and the producer facts inside it.
///
/// Both facts are the producer's and neither is read off a declaration: the door and the projection are the emitter's, so a row that could state either would be an authored declaration signing an act it did not perform.
///
/// # Errors
///
/// Returns [`Overflow`] where the call outgrows the declared magnitude.
pub fn origin(emitter: Emitter) -> Result<Vec<GeneratedToken>, Overflow> {
    let door = parsed(HarnessName::DoorRef, emitter.namespace, emitter.door)?;
    let projection = parsed(
        HarnessName::ProjectionRef,
        emitter.namespace,
        <TrialTable as Kind>::NAME,
    )?;
    let facts = vocabulary::road(
        &[
            HarnessName::Descriptor,
            HarnessName::ProducerFacts,
            HarnessName::Emitted,
        ],
        comma(door, projection),
    )?;
    vocabulary::road(
        &[
            HarnessName::Descriptor,
            HarnessName::Origin,
            HarnessName::Generated,
        ],
        facts,
    )
}

/// One row's classification: the two open rosters, each parsed label by label.
///
/// # Errors
///
/// Returns [`Overflow`] where the call outgrows the declared magnitude.
pub fn classification(declared: &Row) -> Result<Vec<GeneratedToken>, Overflow> {
    let roles = labels(HarnessName::RoleRef, declared.roles())?;
    let tags = labels(HarnessName::TagRef, declared.tags())?;
    let mut tokens = vocabulary::road(
        &[
            HarnessName::Descriptor,
            HarnessName::Classification,
            HarnessName::Authored,
        ],
        comma(roster(roles)?, roster(tags)?),
    )?;
    tokens.push(GeneratedToken::alone('?'));
    Ok(tokens)
}

/// One open roster's labels, each parsed through its own reference.
fn labels(reference: HarnessName, names: &[Name]) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut tokens: Vec<GeneratedToken> = Vec::new();
    for name in names {
        tokens.extend(parsed_name(reference, name)?);
        tokens.push(GeneratedToken::alone(','));
    }
    Ok(tokens)
}

/// One row, in the harness's declared field order, over the two locals the block already parsed.
///
/// The suite arrives as a parameter rather than off the row, because a row states no suite: one aggregate seat selects on one suite and every row under it runs under that one.
///
/// # Errors
///
/// Returns [`Overflow`] where the call outgrows the declared magnitude.
pub fn declared_row(
    declared: &Row,
    suite: &Name,
    emitter: Emitter,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let references = declared.references();
    let mut arguments = parsed_name(HarnessName::ClaimRef, &references.claim)?;
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(parsed_name(HarnessName::ExecutionSuite, suite)?);
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(classification(declared)?);
    arguments.push(GeneratedToken::alone(','));
    arguments.push(GeneratedToken::word(SUBJECT_LOCAL));
    arguments.push(GeneratedToken::alone(','));
    arguments.push(GeneratedToken::word(CHECK_LOCAL));
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(parsed_name(
        HarnessName::PopulationRef,
        &references.population,
    )?);
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(origin(emitter)?);
    arguments.push(GeneratedToken::alone(','));
    let mut tokens = vocabulary::road(
        &[
            HarnessName::Descriptor,
            HarnessName::RowType,
            HarnessName::Declared,
        ],
        arguments,
    )?;
    tokens.push(GeneratedToken::alone('?'));
    Ok(tokens)
}

/// The provenance one produced binding states: the producer that emitted it, and the schema identity it emitted against.
///
/// # Errors
///
/// Returns [`Overflow`] where the construction outgrows the declared magnitude.
pub fn provenance(emitter: Emitter) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut seats = vocabulary::key(HarnessWord::Producer);
    seats.extend(parsed(
        HarnessName::ProducerName,
        emitter.namespace,
        emitter.producer,
    )?);
    seats.push(GeneratedToken::alone(','));
    seats.extend(vocabulary::key(HarnessWord::Schema));
    seats.extend(row_schema_identity()?);
    seats.push(GeneratedToken::alone(','));
    let mut tokens = vocabulary::path(&[
        HarnessName::Descriptor,
        HarnessName::ProvenanceType,
        HarnessName::ProducedProvenance,
    ]);
    tokens.push(group(GeneratedDelimiter::Brace, seats)?);
    Ok(tokens)
}

/// One complete row expression: a block that parses the subject and the check once, builds the row and the attachment over those two values, and marries them under the provenance the producer emitted them against.
///
/// The binding constructor's whole job is to establish that the row's subject route and check reference are the attachment's.
/// A rendering that parsed each name twice would hand it two separately parsed values, and the check would pass because two parses of one spelling agree — a different statement from the one the constructor was written to make.
///
/// # Errors
///
/// Returns [`Overflow`] where the expression outgrows the declared magnitude.
pub fn row_expression(
    declared: &Row,
    suite: &Name,
    emitter: Emitter,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let references = declared.references();
    let mut body = bound_local(
        SUBJECT_LOCAL,
        parsed_name(HarnessName::SubjectRoute, &references.subject)?,
    );
    body.extend(bound_local(
        CHECK_LOCAL,
        parsed_name(HarnessName::CheckRef, &references.check)?,
    ));
    body.extend(bound_local(
        ROW_LOCAL,
        declared_row(declared, suite, emitter)?,
    ));
    body.extend(bound_local(ATTACHMENT_LOCAL, attachment(declared)?));

    let mut arguments = vec![
        GeneratedToken::word(ROW_LOCAL),
        GeneratedToken::alone(','),
        GeneratedToken::word(ATTACHMENT_LOCAL),
        GeneratedToken::alone(','),
    ];
    arguments.extend(provenance(emitter)?);
    arguments.push(GeneratedToken::alone(','));
    body.extend(vocabulary::road(
        &[
            HarnessName::Descriptor,
            HarnessName::BindingType,
            HarnessName::Bound,
        ],
        arguments,
    )?);
    Ok(vec![group(GeneratedDelimiter::Brace, body)?])
}

/// One `named(<namespace>, <stem>)` clause, as the stamp's grammar spells it.
///
/// # Errors
///
/// Returns [`Overflow`] where the clause outgrows the declared magnitude.
pub fn named_clause(name: &Name) -> Result<Vec<GeneratedToken>, Overflow> {
    Ok(vec![
        GeneratedToken::word(HarnessName::Named.spelling()),
        group(
            GeneratedDelimiter::Parenthesis,
            text_pair(name.namespace(), name.stem()),
        )?,
    ])
}

/// One aggregate seat's group, as the stamp's grammar spells it.
///
/// # Errors
///
/// Returns [`Overflow`] where the group outgrows the declared magnitude.
pub fn suite_group(seated: &SuiteGroup, emitter: Emitter) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut rows: Vec<GeneratedToken> = Vec::new();
    for declared in seated.rows() {
        rows.push(GeneratedToken::word(declared.lens().spelling()));
        rows.push(GeneratedToken::alone(':'));
        rows.extend(row_expression(declared, seated.suite(), emitter)?);
        rows.push(GeneratedToken::alone(','));
    }
    let mut tokens = vec![
        GeneratedToken::word(HarnessWord::Suite.spelling()),
        GeneratedToken::word(seated.seat().spelling()),
    ];
    tokens.extend(named_clause(seated.suite())?);
    tokens.push(group(GeneratedDelimiter::Brace, rows)?);
    Ok(tokens)
}

/// The matcher clauses one trial table's carrier must bind: exactly the metavariables the stamped module spells, in the order it spells them.
///
/// The three host facts come first and every row's three attachment seats follow, group by group in declared order, so the invocation a person writes reads in the same order as the module the stamp writes.
/// Every clause takes an expression, because each is a value the consumption target owns — a revision commitment, a callable, a clock — arriving where that target's own hygiene reaches its own items.
///
/// The carrier's own binding clause is not here: the carrier asks for it always, whatever cargo it composes, so it is composed where the matcher is.
#[must_use]
pub fn matched_clauses(payload: &Trials) -> Vec<GeneratedToken> {
    let mut clauses: Vec<GeneratedToken> = Vec::new();
    for host in [
        HarnessWord::Invocation,
        HarnessWord::Target,
        HarnessWord::Clock,
    ] {
        clauses.extend(crate::support::matched_clause(host.spelling(), "expr"));
    }
    for seated in payload.groups() {
        for declared in seated.rows() {
            let lens = declared.lens().spelling();
            for seat in [
                HarnessWord::SubjectRevision,
                HarnessWord::CheckRevision,
                HarnessWord::Call,
            ] {
                clauses.extend(crate::support::matched_clause(
                    &attachment_metavariable(lens, seat),
                    "expr",
                ));
            }
        }
    }
    clauses
}

/// The stamped module the carrier's gate forwards: the table's name, its stated provenance, the consumer's three declared host facts, and every aggregate seat.
///
/// The visibility is `pub(crate)`, and the reach is exactly the consumption target: the stamp lands in a test binary, so crate visibility there reaches the seats and the table a lane reads and reaches nothing a consumer publishes.
///
/// # Errors
///
/// Returns [`Overflow`] where the module outgrows the declared magnitude.
pub fn stamped_module(payload: &Trials, emitter: Emitter) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut body = vocabulary::key(HarnessWord::Provenance);
    body.push(GeneratedToken::word(HarnessWord::Produced.spelling()));
    body.push(group(
        GeneratedDelimiter::Parenthesis,
        text_pair(emitter.namespace, emitter.producer),
    )?);
    body.push(GeneratedToken::word(HarnessWord::Against.spelling()));
    body.extend(table_schema_identity()?);
    body.push(GeneratedToken::alone(','));
    for host in [
        HarnessWord::Invocation,
        HarnessWord::Target,
        HarnessWord::Clock,
    ] {
        body.extend(vocabulary::key(host));
        body.extend(metavariable(host.spelling()));
        body.push(GeneratedToken::alone(','));
    }
    for seated in payload.groups() {
        body.extend(suite_group(seated, emitter)?);
    }
    let mut tokens = declared_reach_tokens(Visibility::Crate)?;
    tokens.push(GeneratedToken::word("mod"));
    tokens.push(GeneratedToken::word(payload.module().spelling()));
    tokens.extend(named_clause(payload.table())?);
    tokens.push(group(GeneratedDelimiter::Brace, body)?);
    Ok(tokens)
}
