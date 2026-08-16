//! The independent transcript lane: testpak rebuilds a projection identity's
//! preimage from the published SPECIFICATION and requires the services' minted
//! value to match it.
//!
//! # Independence
//!
//! Everything below that turns a transcript into bytes is written here. The
//! encoder is this file's own: its own length framing, its own field order, its
//! own domain-string assembly, its own byte-for-byte spelling of every member.
//!
//! Not one encoding function, constant, or spelling is imported from
//! `threadpak-macroc`. The subject names, the role names, the role slots, the
//! anchoring discriminants, the profile stem, the profile version, the generator
//! name, and the generator schema version are all written out in full here, from
//! the specification on `ProjectionTranscript`.
//!
//! What IS shared is the digest itself, and deliberately: both sides call
//! BLAKE3. A lane that reimplemented the hash would be testing an arithmetic
//! exercise instead of a specification, and BLAKE3 is not the thing under
//! judgement here. The thing under judgement is whether the specification says
//! enough for somebody else to derive the same identity — which is exactly what
//! a reader of a published receipt has to be able to do.
//!
//! # Reversals
//!
//! A match that could not fail proves nothing. Two negative controls run beside
//! the positive one: an encoder that drops the content's length prefix, and an
//! encoder that assembles the domain string in the wrong order. Both must
//! DISAGREE with the services. If either agreed, the positive match would be
//! evidence of nothing.

use threadpak_macroc::plane::{CapturedDeclarationSubject, GeneratedUnitSubject, PlanSubject};
use threadpak_macroc::{
    ProjectionIdentity, ProjectionRole, ProjectionTranscript, TextCapture, TranscriptAnchoring,
    captured,
};

// ---------------------------------------------------------------------------
// The specification, restated here in full.
// ---------------------------------------------------------------------------

/// The profile stem, spelled out rather than imported.
const PROFILE_STEM: &str = "threadpak/macroc/projection-identity";

/// The profile version, spelled out rather than imported.
///
/// Version 2 is the version this lane judges, restated here rather than read
/// from the producer — a lane that imported the version would agree with a
/// producer that silently changed it.
const PROFILE_VERSION: u32 = 2;

/// The generator's declared name, spelled out rather than imported.
const GENERATOR_PROFILE: &str = "threadpak-macroc";

/// The generator's schema version, spelled out rather than imported.
const GENERATOR_SCHEMA: u32 = 1;

/// The role slots, spelled out rather than imported, in the roster order the
/// specification states.
const ROLE_SLOTS: [(&str, u8); 9] = [
    ("captured-declaration", 0),
    ("plan", 1),
    ("origin-node", 2),
    ("generated-unit", 3),
    ("rendered-unit", 4),
    ("output-bytes", 5),
    ("bundle", 6),
    ("closure", 7),
    ("closed-expansion", 8),
];

/// The anchoring discriminant for a rooted transcript.
const ANCHORING_ROOTED: u8 = 0;

/// The anchoring discriminant for a transcript under a plane identity.
const ANCHORING_UNDER_PROJECTION: u8 = 2;

/// This lane's own length framing: eight big-endian bytes.
fn judge_length(length: usize, into: &mut Vec<u8>) {
    let width = u64::try_from(length).unwrap_or(u64::MAX);
    into.extend_from_slice(&width.to_be_bytes());
}

/// This lane's own length-prefixed byte string.
fn judge_bytes(material: &[u8], into: &mut Vec<u8>) {
    judge_length(material.len(), into);
    into.extend_from_slice(material);
}

/// The role slot this lane reads for one role name, by its own table.
fn judge_role_slot(role: &str) -> Option<u8> {
    ROLE_SLOTS
        .iter()
        .find(|(name, _)| *name == role)
        .map(|(_, slot)| *slot)
}

/// This lane's own derive-key context, assembled by the published grammar.
fn judge_context(subject: &str, role: &str) -> String {
    format!("{PROFILE_STEM}/v{PROFILE_VERSION}/{subject}/{role}")
}

/// This lane's own transcript: the eleven members of the specification, in
/// order.
fn judge_transcript(
    subject: &str,
    role: &str,
    anchoring: u8,
    anchor: &[u8],
    content: &[u8],
    position: u32,
) -> Option<Vec<u8>> {
    let slot = judge_role_slot(role)?;
    let mut bytes = Vec::new();
    judge_bytes(PROFILE_STEM.as_bytes(), &mut bytes);
    bytes.extend_from_slice(&PROFILE_VERSION.to_be_bytes());
    judge_bytes(subject.as_bytes(), &mut bytes);
    judge_bytes(role.as_bytes(), &mut bytes);
    bytes.push(slot);
    bytes.push(anchoring);
    judge_bytes(anchor, &mut bytes);
    judge_bytes(content, &mut bytes);
    bytes.extend_from_slice(&position.to_be_bytes());
    judge_bytes(GENERATOR_PROFILE.as_bytes(), &mut bytes);
    bytes.extend_from_slice(&GENERATOR_SCHEMA.to_be_bytes());
    Some(bytes)
}

/// The identity this lane derives, by the published specification and nothing
/// else.
fn judge_identity(
    subject: &str,
    role: &str,
    anchoring: u8,
    anchor: &[u8],
    content: &[u8],
    position: u32,
) -> Option<[u8; 32]> {
    let transcript = judge_transcript(subject, role, anchoring, anchor, content, position)?;
    Some(blake3::derive_key(
        &judge_context(subject, role),
        &transcript,
    ))
}

// ---------------------------------------------------------------------------
// The declaration this lane derives a real identity over.
// ---------------------------------------------------------------------------

/// One lawful declaration, handed to the services so that the identity compared
/// below is a production mint rather than a specimen built for the comparison.
const DECLARATION: &str = "#[refusal(family = \"testpak.transcript\", shape = single_cause, \
    order(NotAdmitted = \"not-admitted\", Unbounded = \"unbounded\"))] \
    enum TranscriptFamily { NotAdmitted, Unbounded, }";

/// The anchor the anchored vectors below stand under.
const ANCHOR: [u8; 32] = [7; 32];

// ---------------------------------------------------------------------------
// The lane.
// ---------------------------------------------------------------------------

/// The positive control on a REAL mint: the captured-declaration identity of a
/// declaration the services actually read.
///
/// The content is the capture's own canonical bytes, which is an INPUT to the
/// transcript and not part of the encoding under judgement — a reader of a
/// published receipt is handed the material and asked to re-derive the name.
/// Everything from that material to the thirty-two bytes is this lane's.
#[test]
fn the_specification_re_derives_a_real_captured_declaration_identity() {
    let read = TextCapture::read(DECLARATION).map_err(|_| ());
    assert!(read.is_ok_and(|read| {
        let content = read.input().canonical_bytes();
        captured(read.input()).is_ok_and(|surface| {
            judge_identity(
                "captured-declaration",
                "captured-declaration",
                ANCHORING_ROOTED,
                &[],
                &content,
                0,
            )
            .is_some_and(|rebuilt| rebuilt == *surface.identity().as_bytes())
        })
    }));
}

/// The positive control across postures and subjects: rooted and anchored, over
/// three different subjects, each re-derived from the specification alone.
#[test]
fn the_specification_re_derives_every_posture_and_subject() {
    let content: &[u8] = b"independent-lane-content";

    let anchored = ProjectionTranscript::under(
        ProjectionRole::GeneratedUnit,
        TranscriptAnchoring::UnderProjectionIdentity(ANCHOR),
        content,
        3,
    );
    assert!(
        judge_identity(
            "generated-unit",
            "generated-unit",
            ANCHORING_UNDER_PROJECTION,
            &ANCHOR,
            content,
            3,
        )
        .is_some_and(|rebuilt| rebuilt
            == *ProjectionIdentity::<GeneratedUnitSubject>::derived(anchored).as_bytes())
    );

    let rooted = ProjectionTranscript::rooted(ProjectionRole::Plan, content, 11);
    assert!(
        judge_identity("plan", "plan", ANCHORING_ROOTED, &[], content, 11).is_some_and(|rebuilt| {
            rebuilt == *ProjectionIdentity::<PlanSubject>::derived(rooted).as_bytes()
        })
    );

    let empty = ProjectionTranscript::rooted(ProjectionRole::CapturedDeclaration, &[], 0);
    assert!(
        judge_identity(
            "captured-declaration",
            "captured-declaration",
            ANCHORING_ROOTED,
            &[],
            &[],
            0,
        )
        .is_some_and(|rebuilt| rebuilt
            == *ProjectionIdentity::<CapturedDeclarationSubject>::derived(empty).as_bytes())
    );
}

/// The rehearsed reversal, first form: an encoder that writes the content
/// without its length prefix must DISAGREE.
///
/// This is what the prefix buys, and it is proven rather than asserted: without
/// it the match above would hold for an encoder that admits two cuts of one byte
/// string.
#[test]
fn an_encoder_that_drops_the_content_length_prefix_disagrees() {
    let content: &[u8] = b"independent-lane-content";
    let mut bytes = Vec::new();
    judge_bytes(PROFILE_STEM.as_bytes(), &mut bytes);
    bytes.extend_from_slice(&PROFILE_VERSION.to_be_bytes());
    judge_bytes(b"plan", &mut bytes);
    judge_bytes(b"plan", &mut bytes);
    bytes.push(1);
    bytes.push(ANCHORING_ROOTED);
    judge_bytes(&[], &mut bytes);
    bytes.extend_from_slice(content);
    bytes.extend_from_slice(&11_u32.to_be_bytes());
    judge_bytes(GENERATOR_PROFILE.as_bytes(), &mut bytes);
    bytes.extend_from_slice(&GENERATOR_SCHEMA.to_be_bytes());

    let unprefixed = blake3::derive_key(&judge_context("plan", "plan"), &bytes);
    let minted = ProjectionIdentity::<PlanSubject>::derived(ProjectionTranscript::rooted(
        ProjectionRole::Plan,
        content,
        11,
    ));
    assert_ne!(unprefixed, *minted.as_bytes());
}

/// The rehearsed reversal, second form: a context assembled with the subject and
/// the role transposed must DISAGREE.
///
/// Domain separation is load-bearing, so a lane that got the grammar wrong must
/// fail rather than quietly agreeing because the transcript happened to match.
#[test]
fn a_context_with_subject_and_role_transposed_disagrees() {
    let content: &[u8] = b"independent-lane-content";
    let transcript = judge_transcript(
        "generated-unit",
        "output-bytes",
        ANCHORING_UNDER_PROJECTION,
        &ANCHOR,
        content,
        3,
    );
    assert!(transcript.is_some_and(|transcript| {
        let transposed = blake3::derive_key(
            &format!("{PROFILE_STEM}/v{PROFILE_VERSION}/output-bytes/generated-unit"),
            &transcript,
        );
        let minted =
            ProjectionIdentity::<GeneratedUnitSubject>::derived(ProjectionTranscript::under(
                ProjectionRole::OutputBytes,
                TranscriptAnchoring::UnderProjectionIdentity(ANCHOR),
                content,
                3,
            ));
        transposed != *minted.as_bytes()
    }));
}
