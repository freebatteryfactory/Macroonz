//! The independent transcript lane: testpak rebuilds a projection identity's
//! preimage from the published SPECIFICATION and requires the services' minted
//! value to match it.
//!
//! # Independence
//!
//! The concrete specification facts are this file's own; `TestPak`'s transcript
//! oracle is the one implementation that frames, composes, and derives them.
//!
//! Not one encoding function, constant, or spelling is imported from
//! `threadpak-macroc`. The exercised subject names, role names, role slots,
//! preimage families, family positions, the
//! anchoring discriminants, and the profile stem are written out in full here,
//! from the published prose and from nothing else: the transcript specification
//! stated on `ProjectionTranscript`, the discriminant table published on
//! `TranscriptAnchoring`, the `ProjectionRole` and `PreimageFamily` rosters, the
//! per-family profile constants declared beside them, the derive-key grammar
//! stated on `IdentityProfile`, and the plane's own README.
//!
//! Nothing below was read off a producer encoder body. The anchoring discriminants were
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
//! preimage — a pair no exercised family's grammar names. All three must
//! DISAGREE with the services. If any agreed, the
//! positive match would be evidence of nothing.

use threadpak_macroc::plane::{CapturedDeclarationSubject, GeneratedUnitSubject, PlanSubject};
use threadpak_macroc::{
    ProjectionIdentity, ProjectionRole, ProjectionTranscript, TextCapture, TranscriptAnchoring,
    captured,
};
use threadpak_testpak::oracle::{
    DerivedIdentity, ORACLE_CAUSE_FAMILY, SpecifiedContext, TranscriptDerivation, TranscriptVerdict,
};
use threadpak_testpak::report::{FailureClass, FindingCause, FindingLocation, TrialConclusion};

// ---------------------------------------------------------------------------
// The specification, restated here in full.
// ---------------------------------------------------------------------------

/// The profile stem's published context segments.
///
/// One stem for every family. What separates two families is the family segment beside it, never a stem a family chose for itself.
const PROFILE_STEM_SEGMENTS: [&str; 3] = ["threadpak", "macroc", "projection-identity"];

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

/// The captured-declaration family, at the position it was first declared with.
const CAPTURED_DECLARATION_FAMILY: JudgedFamily = JudgedFamily {
    name: "captured-declaration",
    version: 1,
};

/// The plan family, at the position it was first declared with.
const PLAN_FAMILY: JudgedFamily = JudgedFamily {
    name: "plan",
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

/// The exercised role map, spelled out rather than imported: each role's declared name, its slot in the owner roster, and the preimage family a transcript at that role stands in.
///
/// The family is read off the role here for the reason it is read off the role in the services: no road below takes a family beside a role, so this lane cannot derive one family's preimage under another family's ladder.
/// This is not a complete product-role census; every row below is reached by this lane's identity, transcript, or context readings.
const EXERCISED_ROLES: [(&str, u8, JudgedFamily); 4] = [
    ("captured-declaration", 0, CAPTURED_DECLARATION_FAMILY),
    ("plan", 1, PLAN_FAMILY),
    ("generated-unit", 3, GENERATED_UNIT_FAMILY),
    ("output-bytes", 5, RENDERED_UNIT_FAMILY),
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

/// The slot and the preimage family this lane reads for one role name, by its
/// own roster.
fn specified_role(role: &str) -> Option<(u8, JudgedFamily)> {
    EXERCISED_ROLES
        .iter()
        .find(|(name, _, _)| *name == role)
        .map(|(_, slot, family)| (*slot, *family))
}

/// The independently specified derive-key context for one role and subject.
///
/// The family is not a parameter here either. It is read off the role, so a
/// caller in this file cannot name one family's key space while writing
/// another's transcript.
fn specified_context(subject: &str, role: &str) -> Option<SpecifiedContext> {
    let (_, family) = specified_role(role)?;
    let mut stem_and_family = PROFILE_STEM_SEGMENTS.to_vec();
    stem_and_family.push(family.name);
    SpecifiedContext::under_version(&stem_and_family, family.version, &[subject, role]).ok()
}

/// The independently specified transcript through its anchor member.
///
/// There is no generator member. The generator is provenance, no family's
/// grammar names it, and a transcript carrying it would be a preimage this
/// specification does not describe — which the third reversal below proves
/// rather than asserts.
fn specified_through_anchor(
    subject: &str,
    role: &str,
    anchoring: u8,
    anchor: &[u8],
) -> Option<TranscriptDerivation> {
    let (slot, family) = specified_role(role)?;
    let profile_stem = PROFILE_STEM_SEGMENTS.join("/");
    Some(
        TranscriptDerivation::opened()
            .framed_text(&profile_stem)
            .framed_text(family.name)
            .fixed32(family.version)
            .framed_text(subject)
            .framed_text(role)
            .discriminant(slot)
            .discriminant(anchoring)
            .framed(anchor),
    )
}

/// The ten-member transcript the published specification declares.
fn specified_transcript(
    subject: &str,
    role: &str,
    anchoring: u8,
    anchor: &[u8],
    content: &[u8],
    position: u32,
) -> Option<TranscriptDerivation> {
    Some(
        specified_through_anchor(subject, role, anchoring, anchor)?
            .framed(content)
            .fixed32(position),
    )
}

/// The identity `TestPak` derives from this lane's independently authored facts.
fn specified_identity(
    subject: &str,
    role: &str,
    anchoring: u8,
    anchor: &[u8],
    content: &[u8],
    position: u32,
) -> Option<DerivedIdentity> {
    let transcript = specified_transcript(subject, role, anchoring, anchor, content, position)?;
    Some(transcript.derived(&specified_context(subject, role)?))
}

/// The class and cause carried by one normalized transcript refusal.
fn refusal_signature(conclusion: &TrialConclusion) -> Option<(FailureClass, FindingCause)> {
    match conclusion {
        TrialConclusion::Passed => None,
        TrialConclusion::Refused(finding) => Some((finding.class(), finding.cause())),
    }
}

/// Whether one independent derivation agrees and normalizes to a pass.
fn transcript_agrees(rederived: Option<DerivedIdentity>, published: &[u8; 32]) -> bool {
    rederived.is_some_and(|identity| {
        let verdict = identity.compared(published);
        verdict == TranscriptVerdict::Agrees
            && verdict.concluded(FindingLocation::at(file!(), line!())) == TrialConclusion::Passed
    })
}

/// The exact verdict and normalized cause required of a hostile derivation.
fn asserts_transcript_disagreement(verdict: &TranscriptVerdict) {
    assert!(matches!(verdict, TranscriptVerdict::Disagrees(_)));
    assert_eq!(
        refusal_signature(&verdict.concluded(FindingLocation::at(file!(), line!()))),
        Some((
            FailureClass::OracleDisagreement,
            FindingCause::named(ORACLE_CAUSE_FAMILY, "transcript-derivation-disagreement"),
        ))
    );
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
/// is what lets this lane hand the input's bytes to the independent transcript
/// derivation — and it is exactly why the documented twin below exists, because
/// a lane that only ever read this text would never make the walk drop anything.
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
            transcript_agrees(
                specified_identity(
                    "captured-declaration",
                    "captured-declaration",
                    ANCHORING_ROOTED,
                    &[],
                    &content,
                    0,
                ),
                &semantic,
            )
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
            specified_identity(
                "captured-declaration",
                "captured-declaration",
                ANCHORING_ROOTED,
                &[],
                &content,
                0,
            )
            .is_some_and(|rebuilt| {
                rebuilt.compared(&semantic) == TranscriptVerdict::Agrees
                    && rebuilt
                        .compared(&semantic)
                        .concluded(FindingLocation::at(file!(), line!()))
                        == TrialConclusion::Passed
                    && documented.is_some_and(|(_, twin, twin_prose)| {
                        twin == *rebuilt.as_bytes()
                            && twin_prose != twin
                            && twin_prose != plain_prose
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
    assert!(transcript_agrees(
        specified_identity(
            "generated-unit",
            "generated-unit",
            ANCHORING_UNDER_PROJECTION,
            &ANCHOR,
            content,
            3,
        ),
        ProjectionIdentity::<GeneratedUnitSubject>::derived(anchored).as_bytes(),
    ));

    let rooted = ProjectionTranscript::rooted(ProjectionRole::Plan, content, 11);
    assert!(transcript_agrees(
        specified_identity("plan", "plan", ANCHORING_ROOTED, &[], content, 11),
        ProjectionIdentity::<PlanSubject>::derived(rooted).as_bytes(),
    ));

    let empty = ProjectionTranscript::rooted(ProjectionRole::CapturedDeclaration, &[], 0);
    assert!(transcript_agrees(
        specified_identity(
            "captured-declaration",
            "captured-declaration",
            ANCHORING_ROOTED,
            &[],
            &[],
            0,
        ),
        ProjectionIdentity::<CapturedDeclarationSubject>::derived(empty).as_bytes(),
    ));
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
    let minted = ProjectionIdentity::<PlanSubject>::derived(ProjectionTranscript::rooted(
        ProjectionRole::Plan,
        content,
        11,
    ));
    let malformed = specified_through_anchor("plan", "plan", ANCHORING_ROOTED, &[]).map(|opened| {
        content
            .iter()
            .fold(opened, |derivation, byte| derivation.discriminant(*byte))
            .fixed32(11u32)
    });
    let verdict = malformed
        .zip(specified_context("plan", "plan"))
        .map(|(derivation, context)| derivation.derived(&context).compared(minted.as_bytes()));
    assert!(verdict.as_ref().is_some_and(|read| {
        asserts_transcript_disagreement(read);
        true
    }));
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
    let transcript = specified_transcript(
        "generated-unit",
        "output-bytes",
        ANCHORING_UNDER_PROJECTION,
        &ANCHOR,
        content,
        3,
    );
    assert!(transcript.is_some_and(|transcript| {
        let mut stem_and_family = PROFILE_STEM_SEGMENTS.to_vec();
        stem_and_family.push(RENDERED_UNIT_FAMILY.name);
        let transposed = SpecifiedContext::under_version(
            &stem_and_family,
            RENDERED_UNIT_FAMILY.version,
            &["output-bytes", "generated-unit"],
        );
        let minted =
            ProjectionIdentity::<GeneratedUnitSubject>::derived(ProjectionTranscript::under(
                ProjectionRole::OutputBytes,
                TranscriptAnchoring::UnderProjectionIdentity(ANCHOR),
                content,
                3,
            ));
        transposed.is_ok_and(|context| {
            let verdict = transcript.derived(&context).compared(minted.as_bytes());
            asserts_transcript_disagreement(&verdict);
            true
        })
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
    let transcript = specified_transcript("plan", "plan", ANCHORING_ROOTED, &[], content, 11);
    let minted = ProjectionIdentity::<PlanSubject>::derived(ProjectionTranscript::rooted(
        ProjectionRole::Plan,
        content,
        11,
    ));
    assert!(transcript.is_some_and(|ten| {
        let twelve = ten.framed_text(GENERATOR_PROFILE).fixed32(GENERATOR_SCHEMA);
        specified_context("plan", "plan").is_some_and(|context| {
            let verdict = twelve.derived(&context).compared(minted.as_bytes());
            asserts_transcript_disagreement(&verdict);
            true
        })
    }));
}
