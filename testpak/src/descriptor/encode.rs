//! The canonical bytes this home's two preimage-bearing values commit to: one
//! generated-support schema declaration, and one authored row.
//!
//! These bytes are PREIMAGES — of the generated-support schema identity, which
//! is derived from them ([`GeneratedSupportSchema::identity`]), and of the row
//! revision identity, which the report instrument derives from them
//! ([`RowRevisionId::over`](crate::report::RowRevisionId::over)). The bytes are
//! never "the id", and no reader is meant to parse meaning out of them: they
//! exist so that one value has exactly one byte string, and so that a change to
//! any member of that value moves the derived identity.
//!
//! # The schema specification
//!
//! The encoding is a function of the declaration and of nothing else — no
//! clock, no environment, no source text, no iteration order that is not the
//! declared one. It is stated completely here, because an independent reader
//! re-deriving this identity writes its own encoder from this paragraph and
//! imports nothing.
//!
//! Two primitives:
//!
//! - `u32be(n)` / `u64be(n)` — the integer in four or eight big-endian bytes.
//! - `bytes(x)` — `u64be(len(x))` followed by the bytes of `x`. Every
//!   variable-length member is written this way, so no two member sequences can
//!   be cut at a different boundary and produce one byte string.
//!
//! The declaration, in exactly this order, with no separators and no padding:
//!
//! | # | member | encoding |
//! | - | ------ | -------- |
//! | 1 | encoding version | `u32be` |
//! | 2 | descriptor member | member tag `1`, then its roster |
//! | 3 | mutation-point member | member tag `2`, then its roster |
//! | 4 | bench member | member tag `3`, then its roster |
//!
//! A roster is `u64be(field count)` followed by each field in declared order. A
//! field is `bytes(name)`, then its shape, then one byte for its cardinality
//! slot. A shape is one byte for its slot; the closed-choice shape additionally
//! writes `u64be(arm count)` followed by `bytes(arm)` for each arm in declared
//! order. The slots are the closed tables in `type_contract.rs`.
//!
//! Nothing is folded on the way in: every name and every arm spelling is
//! written at full length, so the derived identity is the only compression
//! anywhere in the derivation.
//!
//! # The row specification
//!
//! The row preimage is the COMPLETE descriptor content of one row: every field
//! the row declares, including the arm its origin carries and everything that
//! arm earns. It is stated completely in [`encode_row`], under the same two
//! primitives and the same framing law, so no two rows this vocabulary considers
//! different are cut into one byte string.
//!
//! The two preimages here are never compared with each other. They are derived
//! under different domain tags, so a schema identity and a row revision identity
//! are unrelated values rather than neighbouring ones, whatever their bytes.
//!
//! The two encodings carry separate version constants, because they move for
//! separate reasons: how a schema declaration is cut and how a row is cut are
//! two decisions, and one bump must not rename identities under the other.

use super::types::{
    AdmissionFacts, EncodeRefusal, FieldShape, GeneratedSupportSchema, NamespacedName, Origin, Row,
    SchemaField, SynthesisFacts,
};
use crate::identity::ContentAddress;

/// The version of the schema encoding itself.
///
/// It rides the preimage, so changing how the bytes are cut moves every derived
/// identity — a new encoding can never be mistaken for the old one over the
/// same declaration.
const SCHEMA_ENCODING_VERSION: u32 = 1;

/// The version of the row encoding itself.
///
/// Its own constant rather than the schema encoding's, for the reason this
/// file's page states: the two encodings move for separate reasons, and a bump
/// to one must rename nothing derived under the other.
const ROW_ENCODING_VERSION: u32 = 1;

/// The tag the descriptor member is written under.
const DESCRIPTOR_MEMBER_TAG: u8 = 1;

/// The tag the mutation-point member is written under.
const MUTATION_POINT_MEMBER_TAG: u8 = 2;

/// The tag the bench member is written under.
const BENCH_MEMBER_TAG: u8 = 3;

/// The canonical bytes of one root schema declaration.
///
/// # Errors
///
/// Refuses a length that does not fit the sixty-four bit width the encoding
/// declares. The encoder states its widths rather than guessing at one; on
/// every target this crate is built for the case is unreachable.
pub fn encode_generated_support_schema(
    schema: &GeneratedSupportSchema,
) -> Result<Vec<u8>, EncodeRefusal> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&SCHEMA_ENCODING_VERSION.to_be_bytes());
    push_member(&mut bytes, DESCRIPTOR_MEMBER_TAG, schema.descriptor().fields())?;
    push_member(&mut bytes, MUTATION_POINT_MEMBER_TAG, schema.mutation_point().fields())?;
    push_member(&mut bytes, BENCH_MEMBER_TAG, schema.bench().fields())?;
    Ok(bytes)
}

/// One member: its tag, then its roster.
fn push_member(out: &mut Vec<u8>, tag: u8, fields: &[SchemaField]) -> Result<(), EncodeRefusal> {
    out.push(tag);
    push_count(out, fields.len())?;
    for field in fields {
        push_text(out, field.name())?;
        push_shape(out, field.shape())?;
        out.push(field.cardinality().slot());
    }
    Ok(())
}

/// One shape: its slot, and the arm spellings the closed-choice shape carries.
fn push_shape(out: &mut Vec<u8>, shape: FieldShape) -> Result<(), EncodeRefusal> {
    out.push(shape.slot());
    match shape {
        FieldShape::ClosedChoice(arms) => {
            push_count(out, arms.len())?;
            for arm in arms {
                push_text(out, arm)?;
            }
        }
        FieldShape::NamespacedName
        | FieldShape::ContentAddress
        | FieldShape::Bytes
        | FieldShape::Count => {}
    }
    Ok(())
}

/// One length-prefixed text.
fn push_text(out: &mut Vec<u8>, text: &str) -> Result<(), EncodeRefusal> {
    push_count(out, text.len())?;
    out.extend_from_slice(text.as_bytes());
    Ok(())
}

/// One count, at the declared width.
fn push_count(out: &mut Vec<u8>, count: usize) -> Result<(), EncodeRefusal> {
    let declared = u64::try_from(count).map_err(|_| EncodeRefusal::LengthPastEncodingWidth)?;
    out.extend_from_slice(&declared.to_be_bytes());
    Ok(())
}

// ---------------------------------------------------------------------------
// The row preimage.
// ---------------------------------------------------------------------------

/// The COMPLETE canonical bytes of one authored row.
///
/// # Authority
///
/// These bytes are the row revision identity's preimage: the report instrument
/// derives that identity from them
/// ([`RowRevisionId::over`](crate::report::RowRevisionId::over)) and encodes
/// nothing itself, so a row is encoded by the home that owns it and by nobody
/// else.
///
/// The preimage is the COMPLETE descriptor content of the row — every field the
/// row declares, and everything the arm its origin carries earns. That
/// completeness is the whole claim: two rows this vocabulary considers
/// different always encode differently, which is what makes a moved identity
/// evidence that the row was edited.
///
/// # The specification
///
/// The two primitives and the framing law are this file's, stated once at its
/// page: `u32be(n)`, `u64be(n)`, and `bytes(x)` — `u64be(len(x))` then the
/// bytes of `x`. A NAME is `bytes(namespace)` followed by `bytes(stem)`, two
/// framed members rather than a joined spelling, so no pair of namespace and
/// stem can be re-cut into a different pair that encodes identically.
///
/// The members, in exactly this order, with no separators and no padding:
///
/// | # | member | encoding |
/// | - | ------ | -------- |
/// | 1 | encoding version | `u32be`, the row encoding's own version |
/// | 2 | claim | the reference's name |
/// | 3 | execution suite | the reference's name |
/// | 4 | roles | `u64be(count)`, then each role's name |
/// | 5 | tags | `u64be(count)`, then each tag's name |
/// | 6 | subject route | the reference's name |
/// | 7 | check reference | the reference's name |
/// | 8 | population | the reference's name |
/// | 9 | origin | one byte, [`Origin::slot`], then the arm's own members |
///
/// The order is the descriptor schema's declared reading order, so the roster a
/// producer emits against and the bytes a row commits to are read the same way
/// round.
///
/// The origin's arms carry exactly what they earn, and nothing writes a seat an
/// arm does not have:
///
/// | slot | arm | members |
/// | ---- | --- | ------- |
/// | 1 | hand-written | nothing |
/// | 2 | generated | the door's name, then the projection's name |
/// | 3 | candidate | one byte, [`SynthesisFacts::slot`]; the survivor arm then writes the mutation point's name, the proof-gap arm writes nothing |
/// | 4 | admitted-replay | `bytes(proposal address)`, the admission facts, `bytes(replay address)` |
/// | 5 | admitted-discharge | `bytes(proposal address)`, the admission facts |
///
/// The ADMISSION FACTS are one byte,
/// [`AdmissionGround::slot`](crate::descriptor::AdmissionGround::slot),
/// followed by the destination suite's name.
///
/// # Ordering
///
/// The roles and the tags are written in their STORAGE order — the set's, over
/// the namespace and then the stem — rather than in the order a hand happened
/// to author them. Two rows carrying the same labels therefore encode
/// identically however they were written, which is right: the rosters are sets,
/// a repeat is refused where the classification is built, and authoring order
/// carries no meaning that a revision identity should move with.
///
/// # Nonclaims
///
/// The generated-support schema identity, the producer's provenance, and the
/// two revision bindings are absent, because none of them is a row field: they
/// ride the binding and the table. A row revision therefore does not move when
/// a producer-facing schema changes, and moving it is never evidence that
/// anything the row EXECUTES has changed.
///
/// # Errors
///
/// Refuses a length that does not fit the sixty-four bit width the encoding
/// declares. The encoder states its widths rather than guessing at one; on
/// every target this crate is built for the case is unreachable.
pub fn encode_row(row: &Row) -> Result<Vec<u8>, EncodeRefusal> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&ROW_ENCODING_VERSION.to_be_bytes());
    push_name(&mut bytes, row.claim().name())?;
    push_name(&mut bytes, row.execution_suite().name())?;
    push_count(&mut bytes, row.roles().len())?;
    for role in row.roles() {
        push_name(&mut bytes, role.name())?;
    }
    push_count(&mut bytes, row.tags().len())?;
    for tag in row.tags() {
        push_name(&mut bytes, tag.name())?;
    }
    push_name(&mut bytes, row.subject().name())?;
    push_name(&mut bytes, row.check().name())?;
    push_name(&mut bytes, row.population().name())?;
    push_origin(&mut bytes, row.origin())
}

/// One origin: its slot, then exactly what its arm earns.
fn push_origin(out: &mut Vec<u8>, origin: Origin) -> Result<(), EncodeRefusal> {
    out.push(origin.slot());
    match origin {
        Origin::HandWritten => Ok(()),
        Origin::Generated(facts) => {
            push_name(out, facts.door().name())?;
            push_name(out, facts.projection().name())
        }
        Origin::Candidate(facts) => push_synthesis(out, facts),
        Origin::AdmittedReplay {
            proposal,
            admission,
            replay,
        } => {
            push_address(out, proposal.address())?;
            push_admission(out, admission)?;
            push_address(out, replay.address())
        }
        Origin::AdmittedDischarge {
            proposal,
            admission,
        } => {
            push_address(out, proposal.address())?;
            push_admission(out, admission)
        }
    }
}

/// One synthesis fact: its slot, and the opening the survivor arm names.
fn push_synthesis(out: &mut Vec<u8>, facts: SynthesisFacts) -> Result<(), EncodeRefusal> {
    out.push(facts.slot());
    match facts {
        SynthesisFacts::Survivor(point) => push_name(out, point.name()),
        SynthesisFacts::ProofGap => Ok(()),
    }
}

/// One admission's stated facts: the ground's slot, then the destination suite.
fn push_admission(out: &mut Vec<u8>, admission: AdmissionFacts) -> Result<(), EncodeRefusal> {
    out.push(admission.ground().slot());
    push_name(out, admission.destination().name())
}

/// One namespaced name: the namespace, then the stem, each framed.
fn push_name(out: &mut Vec<u8>, name: NamespacedName) -> Result<(), EncodeRefusal> {
    push_text(out, name.namespace())?;
    push_text(out, name.stem())
}

/// One content address, framed at its own length like every other
/// variable-length member, so no address can be re-cut against its neighbour.
fn push_address(out: &mut Vec<u8>, address: ContentAddress) -> Result<(), EncodeRefusal> {
    push_count(out, address.as_bytes().len())?;
    out.extend_from_slice(address.as_bytes());
    Ok(())
}
