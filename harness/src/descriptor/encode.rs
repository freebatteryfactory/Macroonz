//! The canonical bytes this home's preimage-bearing values commit to: one root schema declaration, one authored row, and one trial's coordinates.
//!
//! These bytes are preimages, never identities, and no reader is meant to parse meaning out of them.
//! They exist so that one value has exactly one byte string, and so that a change to any member of that value moves the identity derived from it.
//! The encoding is a function of the value and of nothing else — no clock, no environment, no source text, no iteration order that is not the declared one.
//! It is stated completely here, because an independent party re-deriving one of these identities writes its own encoder from this page and imports nothing.
//!
//! # Two primitives
//!
//! - `u32be(n)` and `u64be(n)` — the integer in four or eight big-endian bytes.
//! - `bytes(x)` — `u64be(len(x))` followed by the bytes of `x`.
//!
//! Every variable-length member is framed, so no two member sequences can be cut at a different boundary and produce one byte string.
//! A name is `bytes(namespace)` then `bytes(stem)`, two framed members rather than a joined spelling.
//! Nothing is folded on the way in, so the derived identity is the only compression anywhere in the derivation.
//!
//! # The schema declaration
//!
//! | # | member | encoding |
//! | - | ------ | -------- |
//! | 1 | encoding version | `u32be` |
//! | 2 | descriptor member | member tag `1`, then its roster |
//! | 3 | mutation-discovery member | member tag `2`, then its roster |
//! | 4 | bench member | member tag `3`, then its roster |
//!
//! A roster is `u64be(field count)` followed by each field in declared order.
//! A field is `bytes(name)`, then its shape, then one byte for its cardinality slot.
//! A shape is one byte for its slot; the closed-choice shape additionally writes `u64be(arm count)` followed by `bytes(arm)` for each arm in declared order.
//!
//! # The row
//!
//! | # | member | encoding |
//! | - | ------ | -------- |
//! | 1 | encoding version | `u32be`, the row encoding's own |
//! | 2 | claim | the reference's name |
//! | 3 | execution suite | the reference's name |
//! | 4 | roles | `u64be(count)`, then each role's name |
//! | 5 | tags | `u64be(count)`, then each tag's name |
//! | 6 | subject route | the reference's name |
//! | 7 | check reference | the reference's name |
//! | 8 | population | the reference's name |
//! | 9 | origin | one byte, [`Origin::slot`], then the arm's own members |
//!
//! | slot | arm | members |
//! | ---- | --- | ------- |
//! | 1 | hand-written | nothing |
//! | 2 | generated | the door's name, then the projection's name |
//! | 3 | candidate | one byte, [`SynthesisFacts::slot`]; the survivor arm then writes the mutation point's name |
//! | 4 | admitted-replay | `bytes(proposal address)`, one byte for the ground, the destination suite's name, `bytes(replay address)` |
//! | 5 | admitted-discharge | `bytes(proposal address)`, the destination suite's name |
//!
//! The member order is the descriptor schema's declared reading order, so the roster a producer emits against and the bytes a row commits to are read the same way round.
//! The discharge arm writes no ground byte, and that is the elision law: its ground is forced by the arm, so a second byte would state a value that could not have been anything else.
//!
//! Roles and tags are written in storage order rather than authoring order, so two rows carrying the same labels encode identically however they were written.
//! The schema identity, the producer's provenance, and the two revision bindings are absent, because none of them is a row field.
//!
//! The two encodings carry separate version constants, because how a schema is cut and how a row is cut are two decisions and one bump must rename nothing under the other.
//! Their preimages are never compared with each other: they are derived under different domain tags, so their identities are unrelated values rather than neighbouring ones.

use super::types::{
    AdmissionGround, CheckRef, ClaimRef, Classification, DESCRIPTOR_PROJECTIONS,
    DescriptorProjection, DischargeAdmission, EncodeRefusal, ExecutionSuite, FieldCardinality,
    FieldShape, GeneratedSupportSchema, NamespacedName, Origin, PopulationRef, ReplayAdmission,
    SchemaField, SubjectRoute, SynthesisFacts, TrialCoordinates, generated_support_members,
    origin_declarations,
};
use crate::identity::ContentAddress;

/// The version of the schema encoding itself.
///
/// It rides the preimage, so changing how the bytes are cut moves every derived identity.
const SCHEMA_ENCODING_VERSION: u32 = 1;

/// The version of the row encoding itself.
///
/// Its own constant rather than the schema encoding's: the two move for separate reasons, and a bump to one must rename nothing derived under the other.
const ROW_ENCODING_VERSION: u32 = 2;

impl AdmissionGround {
    /// The byte this ground is written as in a row's canonical preimage.
    #[must_use]
    pub const fn slot(self) -> u8 {
        match self {
            Self::MutantKilled => 1,
            Self::ClaimPinned => 2,
            Self::ObligationDischarged => 3,
        }
    }
}

macro_rules! implement_origin_slots {
    ($( $variant:ident $(($payload:pat))? => $spelling:literal => $slot:literal, )+) => {
        impl Origin {
            /// The byte this arm is written as in a row's canonical preimage.
            ///
            /// The origin roster projects both this match and the schema's closed-choice spellings, so their order cannot drift independently.
            #[must_use]
            pub const fn slot(self) -> u8 {
                match self {
                    $(
                        Self::$variant $(($payload))? => $slot,
                    )+
                }
            }
        }
    };
}

origin_declarations!(implement_origin_slots);

impl SynthesisFacts {
    /// The byte this arm is written as in a row's canonical preimage.
    #[must_use]
    pub const fn slot(self) -> u8 {
        match self {
            Self::Survivor(_) => 1,
            Self::ProofGap => 2,
        }
    }
}

impl FieldShape {
    /// The byte this shape is written as in the schema's canonical preimage.
    #[must_use]
    pub const fn slot(self) -> u8 {
        match self {
            Self::NamespacedName => 1,
            Self::ContentAddress => 2,
            Self::ClosedChoice(_) => 3,
            Self::Bytes => 4,
            Self::Count => 5,
            Self::MutationAlternative => 6,
        }
    }
}

impl FieldCardinality {
    /// The byte this cardinality is written as in the schema's canonical preimage.
    #[must_use]
    pub const fn slot(self) -> u8 {
        match self {
            Self::ExactlyOne => 1,
            Self::ZeroOrOne => 2,
            Self::ZeroOrMore => 3,
            Self::OneOrMore => 4,
        }
    }
}

macro_rules! push_generated_support_members {
    ([$bytes:ident, $schema:ident]; $( $member:ident: $member_type:ty => $fields:ident => $tag:literal, )+) => {
        $(
            push_member(&mut $bytes, $tag, $schema.$member().fields())?;
        )+
    };
}

/// The canonical bytes of one root schema declaration.
///
/// # Errors
///
/// Refuses a length that does not fit the sixty-four bit width the encoding declares.
/// The encoder states its widths rather than guessing at one; on every target this crate is built for the case is unreachable.
pub fn encode_generated_support_schema(
    schema: &GeneratedSupportSchema,
) -> Result<Vec<u8>, EncodeRefusal> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&SCHEMA_ENCODING_VERSION.to_be_bytes());
    generated_support_members!(push_generated_support_members, bytes, schema);
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
        | FieldShape::Count
        | FieldShape::MutationAlternative => {}
    }
    Ok(())
}

/// The complete canonical bytes of one row's declared content — every field the row declares, and everything the arm its origin carries earns.
///
/// The road takes the declared values rather than a row, because it runs while the row is being born: [`Row::declared`](super::Row::declared) is its one caller, and its answer is what that row then owns for life.
/// The record home derives the row revision identity from these bytes and encodes nothing itself.
///
/// # Errors
///
/// Refuses a length that does not fit the sixty-four bit width the encoding declares, which is unreachable on every target this crate is built for.
pub(super) fn encode_row_content(
    claim: ClaimRef,
    execution_suite: ExecutionSuite,
    classification: &Classification,
    subject: SubjectRoute,
    check: CheckRef,
    population: PopulationRef,
    origin: Origin,
) -> Result<Vec<u8>, EncodeRefusal> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&ROW_ENCODING_VERSION.to_be_bytes());
    for projection in DESCRIPTOR_PROJECTIONS {
        match projection {
            DescriptorProjection::Claim => push_name(&mut bytes, claim.name())?,
            DescriptorProjection::ExecutionSuite => {
                push_name(&mut bytes, execution_suite.name())?;
            }
            DescriptorProjection::Roles => {
                push_count(&mut bytes, classification.roles().len())?;
                for role in classification.roles() {
                    push_name(&mut bytes, role.name())?;
                }
            }
            DescriptorProjection::Tags => {
                push_count(&mut bytes, classification.tags().len())?;
                for tag in classification.tags() {
                    push_name(&mut bytes, tag.name())?;
                }
            }
            DescriptorProjection::Subject => push_name(&mut bytes, subject.name())?,
            DescriptorProjection::Check => push_name(&mut bytes, check.name())?,
            DescriptorProjection::Population => push_name(&mut bytes, population.name())?,
            DescriptorProjection::Origin => push_origin(&mut bytes, origin)?,
        }
    }
    Ok(bytes)
}

/// The complete preimage one [`TrialKey`](super::TrialKey) is derived from: the claim, the subject route, the check, and the population, each as its reference's name, in that order.
///
/// The execution suite is absent because two rows differing only by suite are one trial run under two seats, and nothing about where the row is written appears either, so the key survives a file move and a rename.
///
/// # Errors
///
/// Returns [`EncodeRefusal`] where a member is longer than the width this home's framing declares.
pub(super) fn encode_trial_coordinates(
    coordinates: TrialCoordinates,
) -> Result<Vec<u8>, EncodeRefusal> {
    let mut out = Vec::new();
    push_name(&mut out, coordinates.claim().name())?;
    push_name(&mut out, coordinates.subject().name())?;
    push_name(&mut out, coordinates.check().name())?;
    push_name(&mut out, coordinates.population().name())?;
    Ok(out)
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
        Origin::AdmittedReplay(admitted) => push_replay_admission(out, admitted),
        Origin::AdmittedDischarge(admitted) => push_discharge_admission(out, admitted),
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

/// One replay-bearing admission: the proposal, the ground, the destination suite, and the capsule entry the act authored.
///
/// The ground is written at summary width, so one ground has one identity-bearing byte wherever it is encoded.
fn push_replay_admission(
    out: &mut Vec<u8>,
    admitted: ReplayAdmission,
) -> Result<(), EncodeRefusal> {
    push_address(out, admitted.proposal().address())?;
    out.push(admitted.admission().ground().slot());
    push_name(out, admitted.destination().name())?;
    push_address(out, admitted.replay().address())
}

/// One discharge admission: the proposal, then the destination suite.
///
/// No ground byte: the arm's own slot already states the one ground a discharge can stand on.
fn push_discharge_admission(
    out: &mut Vec<u8>,
    admitted: DischargeAdmission,
) -> Result<(), EncodeRefusal> {
    push_address(out, admitted.proposal().address())?;
    push_name(out, admitted.destination().name())
}

/// One namespaced name: the namespace, then the stem, each framed.
fn push_name(out: &mut Vec<u8>, name: NamespacedName) -> Result<(), EncodeRefusal> {
    push_text(out, name.namespace().written())?;
    push_text(out, name.stem().written())
}

/// One content address, framed at its own length like every other variable-length member.
fn push_address(out: &mut Vec<u8>, address: ContentAddress) -> Result<(), EncodeRefusal> {
    push_count(out, address.as_bytes().len())?;
    out.extend_from_slice(address.as_bytes());
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
