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
//! preimage family each role stands in, each family's own version position, the
//! anchoring discriminants, and the profile stem are written out in full here,
//! from the published prose and from nothing else: the transcript specification
//! stated on `ProjectionTranscript`, the discriminant table published on
//! `TranscriptAnchoring`, the `ProjectionRole` and `PreimageFamily` rosters, the
//! per-family profile constants declared beside them, the derive-key grammar
//! stated on `IdentityProfile`, and the plane's own README.
//!
//! Nothing below was read off an encoder body. The anchoring discriminants were
//! the one thing this lane once had to assume; they are declared now, beside the
//! postures they stand for, and this lane reads them from that declaration.
//!
//! What IS shared is the digest itself, and deliberately: both sides call
//! BLAKE3. A lane that reimplemented the hash would be testing an arithmetic
//! exercise instead of a specification, and BLAKE3 is not the thing under
//! judgement here. The thing under judgement is whether the specification says
//! enough for somebody else to derive the same identity — which is exactly what
//! a reader of a published receipt has to be able to do.
//!
//! # One version per grammar
//!
//! There is no single profile version for this lane to restate. Each preimage
//! family carries its OWN position, the family's declared name sits in the
//! derive-key context ahead of that position, and both are members of the
//! transcript — so a family below is a name and a number written out together,
//! and following a bump is an edit to one family's number and to no other's.
//!
//! # Reversals
//!
//! A match that could not fail proves nothing. Three negative controls run
//! beside the positive ones: an encoder that drops the content's length prefix,
//! an encoder that assembles the domain string in the wrong order, and an
//! encoder that writes the generator's name and schema version into the
//! preimage — the pair the retired spelling carried and no family's grammar
//! names today. All three must DISAGREE with the services. If any agreed, the
//! positive match would be evidence of nothing.

use threadpak_macroc::plane::{CapturedDeclarationSubject, GeneratedUnitSubject, PlanSubject};
use threadpak_macroc::{
    ProjectionIdentity, ProjectionRole, ProjectionTranscript, TextCapture, TranscriptAnchoring,
    captured,
};

// ---------------------------------------------------------------------------
// The specification, restated here in full.
// ---------------------------------------------------------------------------

/// The profile stem, spelled out rather than imported.
///
/// One stem for every family. What separates two families is the family segment
/// beside it, never a stem a family chose for itself.
const PROFILE_STEM: &str = "threadpak/macroc/projection-identity";

/// One preimage family as this lane restates it: the declared name that is the
/// family's segment of the derive-key context and the family member of every
/// transcript written under it, and that family's own version position.
///
/// The two travel together because the specification pairs them. A position
/// belongs to one family and to no other, so a number restated on its own would
/// be a number this lane could not say the meaning of — and position one of one
/// family and position one of another are two key spaces rather than one space
/// reached twice.
///
/// The numbers move HERE, deliberately and visibly, when a published family
/// moves: a family's name and version ride both the derive-key context and the
/// transcript, and this lane writes both out itself, so restating them is the
/// whole of following a bump.
#[derive(Clone, Copy)]
struct JudgedFamily {
    /// The family's declared name.
    name: &'static str,
    /// That family's own version position.
    version: u32,
}

/// The captured-declaration family, at position two: the captured token roster
/// gained the literal forms it had been answering with a neighbour's row, so a
/// declaration carrying a byte string, a raw text, a character, or a byte —
/// every one of them lawful before and every one of them encoded under the
/// numeric row carrying its own spelling — encodes to different content now, as
/// does a text whose escapes are read rather than carried. The material moved,
/// so the names derived over it moved, and this is where following that bump is
/// an edit to one family's number.
const CAPTURED_DECLARATION_FAMILY: JudgedFamily = JudgedFamily {
    name: "captured-declaration",
    version: 2,
};

/// The plan family, at the position it was first declared with.
const PLAN_FAMILY: JudgedFamily = JudgedFamily {
    name: "plan",
    version: 1,
};

/// The origin-node family, at the position it was first declared with.
const ORIGIN_NODE_FAMILY: JudgedFamily = JudgedFamily {
    name: "origin-node",
    version: 1,
};

/// The generated-unit family, at the position it was first declared with.
const GENERATED_UNIT_FAMILY: JudgedFamily = JudgedFamily {
    name: "generated-unit",
    version: 1,
};

/// The rendered-unit family, at the position it was first declared with.
///
/// Two roles stand over this one grammar — the rendered unit, and the digest of
/// exactly that unit's bytes — so the roster below names it twice and neither
/// row carries a second version.
const RENDERED_UNIT_FAMILY: JudgedFamily = JudgedFamily {
    name: "rendered-unit",
    version: 1,
};

/// The bundle family, at the position it was first declared with.
const BUNDLE_FAMILY: JudgedFamily = JudgedFamily {
    name: "bundle",
    version: 1,
};

/// The closure family, at the position it was first declared with.
const CLOSURE_FAMILY: JudgedFamily = JudgedFamily {
    name: "closure",
    version: 1,
};

/// The closed-expansion family, at the position it was first declared with.
const CLOSED_EXPANSION_FAMILY: JudgedFamily = JudgedFamily {
    name: "closed-expansion",
    version: 1,
};

/// The projection-intent family, at the position it was first declared with.
const PROJECTION_INTENT_FAMILY: JudgedFamily = JudgedFamily {
    name: "projection-intent",
    version: 1,
};

/// The explanation family, at position two: the related-projection seat's
/// disposition grammar widened when the profile-unavailable posture gained the
/// owner-fact citation, and that citation is written into the disposition's
/// canonical bytes — a member the position-one preimage did not carry. The
/// widening sits inside the typed answers, which is why this family moved and
/// no neighbouring one did.
const EXPLANATION_FAMILY: JudgedFamily = JudgedFamily {
    name: "explanation",
    version: 2,
};

/// The declaration-documentation family, at the position it was first declared
/// with.
///
/// It shares its SUBJECT with the semantic commitment and is separated from it
/// by this name and this version, which is the separation a shared subject is
/// safe under.
const DECLARATION_DOCUMENTATION_FAMILY: JudgedFamily = JudgedFamily {
    name: "declaration-documentation",
    version: 1,
};

/// The declared-name family, at the position it was first declared with.
const DECLARED_NAME_FAMILY: JudgedFamily = JudgedFamily {
    name: "declared-name",
    version: 1,
};

/// The generator-version family, at the position it was first declared with.
///
/// It is the identity a plan's context NAMES as the generator it was produced
/// under, and it is not the provenance record's generator — which no preimage
/// anywhere carries, as the third reversal below proves.
const GENERATOR_VERSION_FAMILY: JudgedFamily = JudgedFamily {
    name: "generator-version",
    version: 1,
};

/// The diagnostic-relation family, at the position it was first declared with.
const DIAGNOSTIC_RELATION_FAMILY: JudgedFamily = JudgedFamily {
    name: "diagnostic-relation",
    version: 1,
};

/// The role roster, spelled out rather than imported, in the roster order the
/// specification states: each role's declared name, its slot — which IS its
/// place in that order, counted from the first row — and the preimage family a
/// transcript at that role stands in.
///
/// The family is read off the role here for the reason it is read off the role
/// in the services: no road below takes a family beside a role, so this lane
/// cannot derive one family's preimage under another family's ladder either.
///
/// Every declared family is reached by a role, so this lane restates a version
/// for each of the fourteen and leaves none of them unjudged. The roster is
/// fifteen rows and the families are fourteen because two roles stand over the
/// rendered-unit grammar — the roster's one place where a name repeats, and it
/// repeats deliberately.
///
/// The five rows at the end were added so five preimages stopped standing on a
/// neighbour's grammar. That is a fact about NAMES, so this lane holds it as
/// names: a declared name derived under `declared-name` and a diagnostic's
/// relation derived under `diagnostic-relation` reach different context strings
/// than the plan and closed-expansion ladders they used to ride, and this table
/// is where that difference is written out rather than imported.
const ROLE_ROSTER: [(&str, u8, JudgedFamily); 15] = [
    ("captured-declaration", 0, CAPTURED_DECLARATION_FAMILY),
    ("plan", 1, PLAN_FAMILY),
    ("origin-node", 2, ORIGIN_NODE_FAMILY),
    ("generated-unit", 3, GENERATED_UNIT_FAMILY),
    ("rendered-unit", 4, RENDERED_UNIT_FAMILY),
    ("output-bytes", 5, RENDERED_UNIT_FAMILY),
    ("bundle", 6, BUNDLE_FAMILY),
    ("closure", 7, CLOSURE_FAMILY),
    ("closed-expansion", 8, CLOSED_EXPANSION_FAMILY),
    ("projection-intent", 9, PROJECTION_INTENT_FAMILY),
    ("explanation", 10, EXPLANATION_FAMILY),
    (
        "declaration-documentation",
        11,
        DECLARATION_DOCUMENTATION_FAMILY,
    ),
    ("declared-name", 12, DECLARED_NAME_FAMILY),
    ("generator-version", 13, GENERATOR_VERSION_FAMILY),
    ("diagnostic-relation", 14, DIAGNOSTIC_RELATION_FAMILY),
];

/// The anchoring discriminant for a rooted transcript.
///
/// Read off the discriminant table published beside the postures themselves,
/// which is where a byte an independent reader needs belongs. This lane once
/// had to assume these two numbers off the posture order, and said so here;
/// they are declared now, so the assumption is retired and this constant is a
/// restatement like every other in this file.
const ANCHORING_ROOTED: u8 = 0;

/// The anchoring discriminant for a transcript under a plane identity, read off
/// the same published table.
///
/// Two rather than one: the owner-minted posture holds position one, and a
/// position is appended and never renumbered, so this lane restates the value
/// the table declares rather than the count of the postures it uses.
const ANCHORING_UNDER_PROJECTION: u8 = 2;

/// The generator's declared name, spelled out rather than imported.
///
/// It reaches exactly one place below: the reversal that proves the generator
/// is NOT a member. The specification carries it on the derivation record and
/// names it in no family's grammar.
const GENERATOR_PROFILE: &str = "threadpak-macroc";

/// The generator's schema version, spelled out rather than imported, and used
/// on the same terms — the value a preimage WOULD carry if the retired spelling
/// still wrote it.
const GENERATOR_SCHEMA: u32 = 3;

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

/// The slot and the preimage family this lane reads for one role name, by its
/// own roster.
fn judge_role(role: &str) -> Option<(u8, JudgedFamily)> {
    ROLE_ROSTER
        .iter()
        .find(|(name, _, _)| *name == role)
        .map(|(_, slot, family)| (*slot, *family))
}

/// This lane's own derive-key context, assembled by the published grammar: the
/// stem, the family, the family's version, the subject, the role.
///
/// The family is not a parameter here either. It is read off the role, so a
/// caller in this file cannot name one family's key space while writing
/// another's transcript.
fn judge_context(subject: &str, role: &str) -> Option<String> {
    let (_, family) = judge_role(role)?;
    let name = family.name;
    let version = family.version;
    Some(format!("{PROFILE_STEM}/{name}/v{version}/{subject}/{role}"))
}

/// This lane's own transcript: the ten members of the specification, in order.
///
/// There is no generator member. The generator is provenance, no family's
/// grammar names it, and a transcript carrying it would be a preimage this
/// specification does not describe — which the third reversal below proves
/// rather than asserts.
fn judge_transcript(
    subject: &str,
    role: &str,
    anchoring: u8,
    anchor: &[u8],
    content: &[u8],
    position: u32,
) -> Option<Vec<u8>> {
    let (slot, family) = judge_role(role)?;
    let mut bytes = Vec::new();
    judge_bytes(PROFILE_STEM.as_bytes(), &mut bytes);
    judge_bytes(family.name.as_bytes(), &mut bytes);
    bytes.extend_from_slice(&family.version.to_be_bytes());
    judge_bytes(subject.as_bytes(), &mut bytes);
    judge_bytes(role.as_bytes(), &mut bytes);
    bytes.push(slot);
    bytes.push(anchoring);
    judge_bytes(anchor, &mut bytes);
    judge_bytes(content, &mut bytes);
    bytes.extend_from_slice(&position.to_be_bytes());
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
    let context = judge_context(subject, role)?;
    Some(blake3::derive_key(&context, &transcript))
}

// ---------------------------------------------------------------------------
// The declaration this lane derives a real identity over.
// ---------------------------------------------------------------------------

/// One lawful declaration, handed to the services so that the identity compared
/// below is a production mint rather than a specimen built for the comparison.
///
/// It carries NO prose, deliberately: the semantic walk drops documentation
/// attributes before it encodes, so for this declaration alone the input's own
/// canonical bytes and the material that walk encodes are one byte string. That
/// is what lets this lane hand the input's bytes to its own encoder — and it is
/// exactly why the documented twin below exists, because a lane that only ever
/// read this text would never make the walk drop anything.
const DECLARATION: &str = "#[refusal(family = \"testpak.transcript\", shape = single_cause, \
    order(NotAdmitted = \"not-admitted\", Unbounded = \"unbounded\"))] \
    enum TranscriptFamily { NotAdmitted, Unbounded, }";

/// The same declaration with prose written on it, on the family seat and on one
/// variant.
///
/// Semantically identical to its twin above by the published grammar: a
/// documentation comment is an attribute by the time it reaches the capture, and
/// every one of them is dropped from the semantic walk. Nothing else about the
/// two texts differs, so the semantic name they capture to must be one name.
const DOCUMENTED_DECLARATION: &str = "#[doc = \"The transcript family.\"] \
    #[refusal(family = \"testpak.transcript\", shape = single_cause, \
    order(NotAdmitted = \"not-admitted\", Unbounded = \"unbounded\"))] \
    enum TranscriptFamily { #[doc = \"Not admitted.\"] NotAdmitted, Unbounded, }";

/// The anchor the anchored vectors below stand under.
const ANCHOR: [u8; 32] = [7; 32];

/// What the services mint for one declaration's text, read through the public
/// roads only: the input's own canonical bytes, the SEMANTIC commitment, and the
/// DOCUMENTATION commitment beside it.
///
/// The first of the three is the input's bytes and not the semantic walk's
/// material — those are the same string only where a declaration wrote no prose.
/// The distinction is the whole subject of the second test below, so the reader
/// is handed the honest value and the tests say which of them they may use it
/// for.
fn produced_capture(source: &str) -> Option<(Vec<u8>, [u8; 32], [u8; 32])> {
    let read = TextCapture::read(source).ok()?;
    let content = read.input().canonical_bytes();
    let surface = captured(read.input()).ok()?;
    Some((
        content,
        *surface.identity().as_bytes(),
        *surface.documentation_identity().as_bytes(),
    ))
}

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
///
/// This fixture wrote no prose, so the bytes handed in ARE the material the
/// semantic walk encodes. That is a property of the fixture and never of the
/// grammar, and the test below is the one that says so.
#[test]
fn the_specification_re_derives_a_real_captured_declaration_identity() {
    assert!(
        produced_capture(DECLARATION).is_some_and(|(content, semantic, _)| {
            judge_identity(
                "captured-declaration",
                "captured-declaration",
                ANCHORING_ROOTED,
                &[],
                &content,
                0,
            )
            .is_some_and(|rebuilt| rebuilt == semantic)
        })
    );
}

/// The exclusion, proven rather than described: two declarations differing only
/// in the prose written on them capture to ONE semantic name, and it is the name
/// this lane derives from the undocumented twin's own bytes.
///
/// The published grammar drops every documentation attribute from the semantic
/// walk and commits to the rows under a family of their own, so a declaration
/// whose prose changed keeps its semantic name — which is what an implementation
/// projection standing on that name is entitled to. A lane that only ever read
/// an undocumented declaration passes without touching the claim: the walk it
/// exercises drops nothing, so the exclusion costs nothing and proves nothing.
///
/// The two commitments over the documented surface are required to DISAGREE, and
/// so are the two documentation commitments. They are a second READING of one
/// surface: a documentation name equal to the semantic one would be a second
/// name for the first reading rather than a name for the second, and a
/// documented declaration agreeing with its undocumented twin HERE would mean
/// the rows reached no preimage at all.
#[test]
fn a_documented_declaration_keeps_its_undocumented_twin_semantic_name() {
    let documented = produced_capture(DOCUMENTED_DECLARATION);
    assert!(
        produced_capture(DECLARATION).is_some_and(|(content, semantic, plain_prose)| {
            judge_identity(
                "captured-declaration",
                "captured-declaration",
                ANCHORING_ROOTED,
                &[],
                &content,
                0,
            )
            .is_some_and(|rebuilt| {
                rebuilt == semantic
                    && documented.is_some_and(|(_, twin, twin_prose)| {
                        twin == rebuilt && twin_prose != twin && twin_prose != plain_prose
                    })
            })
        })
    );
}

/// The positive control across postures and subjects: rooted and anchored, over
/// three different subjects, each re-derived from the specification alone.
///
/// Three subjects and three families at once — a match that held for one
/// family's name and version and not for another's would be caught here rather
/// than by whoever met the other family first.
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
    judge_bytes(PLAN_FAMILY.name.as_bytes(), &mut bytes);
    bytes.extend_from_slice(&PLAN_FAMILY.version.to_be_bytes());
    judge_bytes(b"plan", &mut bytes);
    judge_bytes(b"plan", &mut bytes);
    bytes.push(1);
    bytes.push(ANCHORING_ROOTED);
    judge_bytes(&[], &mut bytes);
    bytes.extend_from_slice(content);
    bytes.extend_from_slice(&11_u32.to_be_bytes());

    let minted = ProjectionIdentity::<PlanSubject>::derived(ProjectionTranscript::rooted(
        ProjectionRole::Plan,
        content,
        11,
    ));
    assert!(
        judge_context("plan", "plan")
            .is_some_and(|context| blake3::derive_key(&context, &bytes) != *minted.as_bytes())
    );
}

/// The rehearsed reversal, second form: a context assembled with the subject and
/// the role transposed must DISAGREE.
///
/// Domain separation is load-bearing, so a lane that got the grammar wrong must
/// fail rather than quietly agreeing because the transcript happened to match.
/// The family segment stands where the published grammar puts it, ahead of the
/// version: the transposition under judgement here is of the two segments
/// BEHIND the version, and nothing else about the string moves.
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
        let family = RENDERED_UNIT_FAMILY.name;
        let version = RENDERED_UNIT_FAMILY.version;
        let transposed = blake3::derive_key(
            &format!("{PROFILE_STEM}/{family}/v{version}/output-bytes/generated-unit"),
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

/// The rehearsed reversal, third form: an encoder that writes the generator's
/// name and schema version into the preimage must DISAGREE.
///
/// The retired spelling carried exactly those two members behind the roster
/// position, and the published specification now names the generator in no
/// family's grammar at all. That is a claim about BYTES, so it is judged as
/// bytes: the twelve-member preimage is composed here, derived under the same
/// context as the ten-member one, and must reach a different name.
///
/// What the absence buys is stated where it is spent — the same rendered bytes
/// stay the same artifact across the producers that emitted them, and a
/// rendering-shape bump renames nothing. An encoder that quietly agreed here
/// would be an encoder for which that promise costs nothing and means nothing.
#[test]
fn an_encoder_that_writes_the_generator_into_the_preimage_disagrees() {
    let content: &[u8] = b"independent-lane-content";
    let transcript = judge_transcript("plan", "plan", ANCHORING_ROOTED, &[], content, 11);
    let minted = ProjectionIdentity::<PlanSubject>::derived(ProjectionTranscript::rooted(
        ProjectionRole::Plan,
        content,
        11,
    ));
    assert!(transcript.is_some_and(|ten| {
        let mut twelve = ten;
        judge_bytes(GENERATOR_PROFILE.as_bytes(), &mut twelve);
        twelve.extend_from_slice(&GENERATOR_SCHEMA.to_be_bytes());
        judge_context("plan", "plan")
            .is_some_and(|context| blake3::derive_key(&context, &twelve) != *minted.as_bytes())
    }));
}
