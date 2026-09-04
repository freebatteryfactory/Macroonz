//! An independent reader rebuilds a real identity's preimage from the published SPECIFICATION and requires the compiler's minted value to match it.
//!
//! # Independence
//!
//! Not one encoding function, constant, or spelling below is imported from the crate under judgement.
//! The subject names, the role names, the role slots, the grammars, the grammar versions, the anchoring discriminants, the profile stem, and the derive-key grammar are written out in full here, from the specification stated on `Transcript`, the discriminant table on `Anchoring`, the roster on `Role`, the profile constants declared beside it, and the grammar stated on `Profile`.
//!
//! What IS shared is the digest, deliberately: both sides call BLAKE3.
//! A lane that reimplemented the hash would be testing an arithmetic exercise rather than a specification, and the thing under judgement is whether the specification says enough for somebody else to derive the same identity — which is exactly what a reader of a published receipt has to be able to do.
//!
//! # One version per grammar
//!
//! There is no single profile version to restate.
//! Each grammar carries its OWN position, its declared name sits in the derive-key context ahead of that position, and both are members of the transcript — so a grammar below is a name and a number written out together, and following a bump is an edit to one grammar's number and to no other's.
//!
//! # Reversals
//!
//! A match that could not fail proves nothing.
//! Three negative controls run beside the positive ones: an encoder that drops the material's length prefix, one that assembles the derive-key context with the subject and the role transposed, and one that writes the generator's name and shape position into the preimage — a pair no grammar names.

use macroonz_compiler::request;
use macroonz_compiler::{
    CrateBinding, Door, Expansion, GeneratedToken, GeneratedTree, Kind, NoQuestions, Producer,
    Request, Role, SoleRole, TextCapture, names_are_separating,
};

// ---------------------------------------------------------------------------
// The specification, restated here in full.
// ---------------------------------------------------------------------------

/// The stem every grammar and every subject this compiler owns is declared under.
const STEM: &str = "macroonz/identity";

/// One preimage grammar: the declared name that is its segment of the derive-key context and a member of every transcript written under it, and its own version position.
///
/// The two travel together because the specification pairs them: a position belongs to one grammar and to no other, so a number restated on its own would be a number this lane could not say the meaning of.
#[derive(Clone, Copy)]
struct Grammar {
    /// The grammar's declared name.
    name: &'static str,
    /// That grammar's own version position.
    version: u32,
}

/// One seat: its declared name, its slot counted from the roster's first row, and the grammar a transcript at that seat stands in.
///
/// The grammar is read off the seat here for the reason it is read off the seat in the compiler — no road takes a grammar beside a role — so this lane cannot derive one grammar's preimage under another's ladder.
#[derive(Clone, Copy)]
struct Seat {
    /// The seat's declared name.
    name: &'static str,
    /// The seat's published slot.
    slot: u8,
    /// The grammar a transcript at this seat stands in.
    grammar: Grammar,
}

/// The captured-declaration grammar, at the position it was first declared with.
const CAPTURED_DECLARATION: Grammar = Grammar {
    name: "captured-declaration",
    version: 1,
};

/// The captured-helper grammar, at the position it was first declared with.
const CAPTURED_HELPER: Grammar = Grammar {
    name: "captured-helper",
    version: 1,
};

/// The projection-intent grammar, at the position it was first declared with.
const PROJECTION_INTENT: Grammar = Grammar {
    name: "projection-intent",
    version: 2,
};

/// The owner-qualified projection-kind grammar, at the position it was first declared with.
const PROJECTION_KIND: Grammar = Grammar {
    name: "projection-kind",
    version: 1,
};

/// The projection-content grammar, at the position it was first declared with.
const PROJECTION_CONTENT: Grammar = Grammar {
    name: "projection-content",
    version: 1,
};

/// The generated-unit grammar, at the position it was first declared with.
const GENERATED_UNIT: Grammar = Grammar {
    name: "generated-unit",
    version: 2,
};

/// The rendered-unit grammar, at the position it was first declared with.
///
/// Two seats stand over this one grammar — the rendered unit, and the digest of exactly that unit's bytes — so it is named twice below and neither seat carries a second version.
const RENDERED_UNIT: Grammar = Grammar {
    name: "rendered-unit",
    version: 1,
};

/// The closed-expansion grammar, at the position it was first declared with.
const CLOSED_EXPANSION: Grammar = Grammar {
    name: "closed-expansion",
    version: 1,
};

/// The seat one captured declaration's commitment stands at.
const CAPTURE_SEAT: Seat = Seat {
    name: "captured-declaration",
    slot: 0,
    grammar: CAPTURED_DECLARATION,
};

/// The seat one captured helper's commitment stands at.
const HELPER_SEAT: Seat = Seat {
    name: "captured-helper",
    slot: 15,
    grammar: CAPTURED_HELPER,
};

/// The seat one projection intent stands at.
const INTENT_SEAT: Seat = Seat {
    name: "projection-intent",
    slot: 9,
    grammar: PROJECTION_INTENT,
};

/// The seat an owner-qualified kind identity stands at.
const KIND_SEAT: Seat = Seat {
    name: "projection-kind",
    slot: 17,
    grammar: PROJECTION_KIND,
};

/// The seat one kind-specific content commitment stands at.
const CONTENT_SEAT: Seat = Seat {
    name: "projection-content",
    slot: 16,
    grammar: PROJECTION_CONTENT,
};

/// The seat one generated unit's semantic key stands at.
const KEY_SEAT: Seat = Seat {
    name: "generated-unit",
    slot: 3,
    grammar: GENERATED_UNIT,
};

/// The seat one rendered unit stands at.
const RENDERED_SEAT: Seat = Seat {
    name: "rendered-unit",
    slot: 4,
    grammar: RENDERED_UNIT,
};

/// The seat the digest of one rendered unit's bytes stands at.
const DIGEST_SEAT: Seat = Seat {
    name: "output-bytes",
    slot: 5,
    grammar: RENDERED_UNIT,
};

/// The seat one closed expansion stands at.
const EXPANSION_SEAT: Seat = Seat {
    name: "closed-expansion",
    slot: 8,
    grammar: CLOSED_EXPANSION,
};

/// The anchoring discriminant for a rooted transcript.
const ROOTED: u8 = 0;

/// The anchoring discriminant for a transcript under another identity this compiler derived.
///
/// Two rather than one: the owner-minted posture holds position one, and a position is appended and never renumbered, so this lane restates the value the table declares rather than the count of the postures it uses.
const UNDER_PROJECTION: u8 = 2;

/// The generator's declared name, spelled out rather than imported.
///
/// It reaches exactly one place below: the reversal that proves the generator is NOT a member.
const GENERATOR: &str = "macroonz";

/// The generator's shape position, used on the same terms — the value a preimage WOULD carry if it named the generator at all.
const GENERATOR_SHAPE: u32 = 1;

/// This lane's own length framing: eight big-endian bytes.
fn framed_length(length: usize, into: &mut Vec<u8>) {
    let width = u64::try_from(length).unwrap_or(u64::MAX);
    into.extend_from_slice(&width.to_be_bytes());
}

/// This lane's own length-prefixed byte string.
fn framed(material: &[u8], into: &mut Vec<u8>) {
    framed_length(material.len(), into);
    into.extend_from_slice(material);
}

/// This lane's own reading of the derive-key context for one subject at one seat.
fn context(seat: Seat, subject: &str) -> String {
    let name = seat.grammar.name;
    let version = seat.grammar.version;
    let role = seat.name;
    format!("{STEM}/{name}/v{version}/{STEM}/{subject}/{role}")
}

/// This lane's own reading of the preimage through its anchor member.
fn through_anchor(seat: Seat, subject: &str, anchoring: u8, anchor: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::new();
    framed(STEM.as_bytes(), &mut bytes);
    framed(seat.grammar.name.as_bytes(), &mut bytes);
    bytes.extend_from_slice(&seat.grammar.version.to_be_bytes());
    framed(subject.as_bytes(), &mut bytes);
    framed(seat.name.as_bytes(), &mut bytes);
    bytes.push(seat.slot);
    bytes.push(anchoring);
    framed(anchor, &mut bytes);
    bytes
}

/// This lane's own reading of the complete ten-member preimage.
fn transcript(
    seat: Seat,
    subject: &str,
    anchoring: u8,
    anchor: &[u8],
    material: &[u8],
    position: u32,
) -> Vec<u8> {
    let mut bytes = through_anchor(seat, subject, anchoring, anchor);
    framed(material, &mut bytes);
    bytes.extend_from_slice(&position.to_be_bytes());
    bytes
}

/// The identity this lane derives from its own facts, under a context it composed itself.
fn specified(
    seat: Seat,
    subject: &str,
    anchoring: u8,
    anchor: &[u8],
    material: &[u8],
    position: u32,
) -> [u8; 32] {
    blake3::derive_key(
        &context(seat, subject),
        &transcript(seat, subject, anchoring, anchor, material, position),
    )
}

// ---------------------------------------------------------------------------
// The declaration this lane derives real identities over.
// ---------------------------------------------------------------------------

/// The kind this lane renders: one unit, at the declaration site.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Greeting;

impl Kind for Greeting {
    const NAME: &'static str = "lane.greeting";
    type Content = &'static str;
    type Role = SoleRole;
    type Question = NoQuestions;
}

/// The one value that says who is asking.
const DOOR: Door = Door::declared(
    "lane",
    "lane.greeting.grammar",
    "lane::greeting",
    CrateBinding::declared("demo"),
    Producer {
        namespace: "lane",
        name: "greeting",
    },
);

/// One declared input this lane hands the compiler.
const DECLARATION: &str = "struct Greeting { line: Line }";

/// One helper input this lane reads beside the declaration.
const HELPER: &str = "support = greeting_support,";

/// The word the renderer writes, so this lane can state the rendered material it re-derives over.
const RENDERED_WORD: &str = "greeting";

/// The one seat this kind's roster carries, by the name this lane restates rather than imports.
const SEAT_NAME: &str = "sole";

/// The kind's declared name, restated rather than imported on the same terms.
const KIND_NAME: &str = "lane.greeting";

/// The producer namespace the door qualifies the kind under.
const PRODUCER_NAMESPACE: &str = "lane";

/// The producer name the door qualifies the kind under.
const PRODUCER_NAME: &str = "greeting";

/// The kind-specific content this request carries.
const CONTENT: &str = "greeting";

/// The captured input this lane hands over, and the expansion the compiler produces from it.
///
/// The capture's own canonical bytes are the INPUT to a derivation rather than part of the encoding under judgement: a reader of a published receipt is handed the material and asked to re-derive the name.
fn produced() -> Option<(Vec<u8>, Expansion<Greeting>)> {
    let read = TextCapture::read(DECLARATION).ok()?;
    let capture = read.input().clone();
    let material = capture.canonical_bytes();
    let bound = Request::<Greeting>::over(capture, CONTENT, &DOOR)
        .render(|_plan, out| {
            out.unit(
                SoleRole::Sole,
                GeneratedTree::assembled(vec![GeneratedToken::word(RENDERED_WORD)])?,
            )
        })
        .ok()?;
    Some((material, bound))
}

/// The published subject-name grammar separates lawful rosters and refuses every boundary that could collapse one context into another.
#[test]
fn subject_names_obey_the_complete_published_grammar() {
    let lawful = [Vec::<&str>::new(), vec!["alpha"], vec!["alpha-7", "beta2"]];
    let refused = [
        vec!["Bad"],
        vec!["alpha", "Bad"],
        vec!["alpha", "alpha"],
        vec!["alpha", "beta", "alpha"],
        vec!["-alpha"],
        vec!["alpha-"],
        vec!["alpha--beta"],
        vec!["alpha_beta"],
    ];

    for names in lawful {
        assert!(names_are_separating(&names), "lawful roster {names:?}");
    }
    for names in refused {
        assert!(!names_are_separating(&names), "refused roster {names:?}");
    }
}

/// The kind's declared name and the seat's own, each framed, which is what a seat's identities are derived over.
///
/// The kind's name first: roles are open and reusable across kinds, so the kind is the ancestor that keeps two kinds sharing one capture and one roster from sharing a unit.
fn seat_material(kind: &[u8; 32], content: &[u8; 32]) -> Vec<u8> {
    let mut material = Vec::new();
    framed(kind, &mut material);
    framed(content, &mut material);
    framed(SEAT_NAME.as_bytes(), &mut material);
    material
}

/// The owner-qualified kind identity this lane independently derives.
fn kind_identity() -> [u8; 32] {
    let mut material = Vec::new();
    framed(PRODUCER_NAMESPACE.as_bytes(), &mut material);
    framed(PRODUCER_NAME.as_bytes(), &mut material);
    framed(KIND_NAME.as_bytes(), &mut material);
    specified(KIND_SEAT, "projection-kind", ROOTED, &[], &material, 0)
}

/// The content commitment this lane independently derives under one capture.
fn content_identity(capture: &[u8; 32], kind: &[u8; 32]) -> [u8; 32] {
    let mut canonical_content = Vec::new();
    framed(CONTENT.as_bytes(), &mut canonical_content);
    let mut material = Vec::new();
    framed(kind, &mut material);
    framed(&canonical_content, &mut material);
    specified(
        CONTENT_SEAT,
        "projection-content",
        UNDER_PROJECTION,
        capture,
        &material,
        0,
    )
}

// ---------------------------------------------------------------------------
// The lane.
// ---------------------------------------------------------------------------

/// The specification re-derives a real captured declaration's commitment.
///
/// The positive control on a real mint: the value below is what the compiler committed to while planning an ordinary request, not a specimen built for the comparison.
#[test]
fn the_specification_re_derives_a_real_captured_declaration_commitment() -> Result<(), ()> {
    let (material, bound) = produced().ok_or(())?;
    let commitment = bound.plan().account().commitment();
    assert_eq!(
        commitment.as_bytes(),
        &specified(
            CAPTURE_SEAT,
            "captured-declaration",
            ROOTED,
            &[],
            &material,
            0
        )
    );
    Ok(())
}

/// The specification re-derives one helper identity at every descriptor-helper position.
///
/// The declaration material remains the anchor for all three, while the position is the only moving member, so agreement establishes the mint and pairwise inequality establishes the closed position space without consulting the compiler's constants.
#[test]
fn the_specification_re_derives_the_three_captured_helper_positions() -> Result<(), ()> {
    let declaration = TextCapture::read(DECLARATION).map_err(|_refusal| ())?;
    let helper = TextCapture::read(HELPER).map_err(|_refusal| ())?;
    let declaration_material = declaration.input().canonical_bytes();
    let helper_material = helper.input().canonical_bytes();
    let anchor = specified(
        CAPTURE_SEAT,
        "captured-declaration",
        ROOTED,
        &[],
        &declaration_material,
        0,
    );
    let first = request::committed_helper(declaration.input(), helper.input(), 0);
    let second = request::committed_helper(declaration.input(), helper.input(), 1);
    let third = request::committed_helper(declaration.input(), helper.input(), 2);
    for (position, actual) in [(0, first), (1, second), (2, third)] {
        let expected = specified(
            HELPER_SEAT,
            "captured-helper",
            UNDER_PROJECTION,
            &anchor,
            &helper_material,
            position,
        );
        assert_eq!(actual.as_bytes(), &expected);
    }
    assert_ne!(first, second);
    assert_ne!(first, third);
    assert_ne!(second, third);
    Ok(())
}

/// The specification re-derives a real intent, over the preimage the account publishes.
///
/// The intent is the layer equivalence is compared at, so an independent reader has to be able to re-derive one from a published receipt without holding the request that produced it.
#[test]
fn the_specification_re_derives_a_real_intent() -> Result<(), ()> {
    let (_material, bound) = produced().ok_or(())?;
    let account = bound.plan().account();
    assert_eq!(
        account.intent().as_bytes(),
        &specified(
            INTENT_SEAT,
            "projection-intent",
            ROOTED,
            &[],
            &account.intent_bytes(),
            0
        )
    );
    Ok(())
}

/// The specification re-derives the owner-qualified kind and the content commitment bound under the capture.
#[test]
fn the_specification_re_derives_the_content_binding() -> Result<(), ()> {
    let (_material, bound) = produced().ok_or(())?;
    let account = bound.plan().account();
    let kind = kind_identity();
    assert_eq!(account.kind().as_bytes(), &kind);
    assert_eq!(
        account.content_commitment().as_bytes(),
        &content_identity(account.commitment().as_bytes(), &kind)
    );
    Ok(())
}

/// The specification re-derives a real semantic key, and the two identities taken over the bytes that answer it.
///
/// The chain is what a reader follows: the seat's key hangs off the declaration's commitment, and both the rendered unit and the digest of its bytes hang off that key.
#[test]
fn the_specification_re_derives_a_real_key_and_the_identities_over_its_bytes() -> Result<(), ()> {
    let (_material, bound) = produced().ok_or(())?;
    let commitment = bound.plan().account().commitment();
    let kind = kind_identity();
    let content = content_identity(commitment.as_bytes(), &kind);
    let unit = bound.closure().rendered().under(SoleRole::Sole).ok_or(())?;
    let rendered = unit.bytes();

    let key = specified(
        KEY_SEAT,
        "generated-unit",
        UNDER_PROJECTION,
        commitment.as_bytes(),
        &seat_material(&kind, &content),
        0,
    );
    assert_eq!(unit.semantic_key().as_bytes(), &key);
    assert_eq!(
        unit.identity().as_bytes(),
        &specified(
            RENDERED_SEAT,
            "rendered-unit",
            UNDER_PROJECTION,
            &key,
            &rendered,
            0
        )
    );
    assert_eq!(
        unit.digest().as_bytes(),
        &specified(
            DIGEST_SEAT,
            "output-bytes",
            UNDER_PROJECTION,
            &key,
            &rendered,
            0
        )
    );
    Ok(())
}

/// The specification re-derives the sealed expansion's own identity.
///
/// Exactly two members over the closure's anchor, and each absence is the no-double-entry law: the deliveries are inside the anchor, and the kind and the account are inside member one.
#[test]
fn the_specification_re_derives_the_sealed_expansions_own_identity() -> Result<(), ()> {
    let (_material, bound) = produced().ok_or(())?;
    let mut content = Vec::new();
    framed(bound.plan().identity().as_bytes(), &mut content);
    framed(bound.explain().identity().as_bytes(), &mut content);
    assert_eq!(
        bound.identity().as_bytes(),
        &specified(
            EXPANSION_SEAT,
            "closed-expansion",
            UNDER_PROJECTION,
            bound.closure().identity().as_bytes(),
            &content,
            0
        )
    );
    Ok(())
}

/// Two seats standing over ONE grammar derive apart.
///
/// The rendered unit and the digest of exactly that unit's bytes share a grammar, an anchor, a material, and a position, and they are still two names — separated by the subject in the derive-key context and by the seat inside the transcript.
#[test]
fn two_seats_over_one_grammar_derive_apart() -> Result<(), ()> {
    let (_material, bound) = produced().ok_or(())?;
    let unit = bound.closure().rendered().under(SoleRole::Sole).ok_or(())?;
    assert_eq!(RENDERED_SEAT.grammar.name, DIGEST_SEAT.grammar.name);
    assert_ne!(unit.identity().as_bytes(), unit.digest().as_bytes());
    Ok(())
}

/// An encoder that writes the material without its length prefix disagrees.
///
/// That is what the prefix buys, and it is proven rather than asserted: without it two members could be cut at another boundary and encode identically, and every match above would hold for an encoder admitting exactly that.
#[test]
fn an_encoder_that_drops_the_material_length_prefix_disagrees() -> Result<(), ()> {
    let (material, bound) = produced().ok_or(())?;
    let mut unframed = through_anchor(CAPTURE_SEAT, "captured-declaration", ROOTED, &[]);
    unframed.extend_from_slice(&material);
    unframed.extend_from_slice(&0_u32.to_be_bytes());
    let derived = blake3::derive_key(&context(CAPTURE_SEAT, "captured-declaration"), &unframed);
    assert_ne!(bound.plan().account().commitment().as_bytes(), &derived);
    Ok(())
}

/// A derive-key context assembled with the grammar and the subject transposed disagrees.
///
/// Domain separation is load-bearing, so a reader that got the context's order wrong must fail rather than quietly agreeing because the transcript happened to match.
/// The digest's seat is the one this is observable at: its grammar is `rendered-unit` and its subject is `output-bytes`, so the two segments genuinely differ and swapping them is a different key space rather than the same string.
#[test]
fn a_context_with_the_grammar_and_the_subject_transposed_disagrees() -> Result<(), ()> {
    let (_material, bound) = produced().ok_or(())?;
    let commitment = bound.plan().account().commitment();
    let kind = kind_identity();
    let content = content_identity(commitment.as_bytes(), &kind);
    let unit = bound.closure().rendered().under(SoleRole::Sole).ok_or(())?;
    let key = specified(
        KEY_SEAT,
        "generated-unit",
        UNDER_PROJECTION,
        commitment.as_bytes(),
        &seat_material(&kind, &content),
        0,
    );
    let grammar = DIGEST_SEAT.grammar.name;
    let version = DIGEST_SEAT.grammar.version;
    let subject = DIGEST_SEAT.name;
    assert_ne!(grammar, subject);
    let transposed = format!("{STEM}/{subject}/v{version}/{STEM}/{grammar}/{subject}");
    let derived = blake3::derive_key(
        &transposed,
        &transcript(
            DIGEST_SEAT,
            "output-bytes",
            UNDER_PROJECTION,
            &key,
            &unit.bytes(),
            0,
        ),
    );
    assert_ne!(unit.digest().as_bytes(), &derived);
    Ok(())
}

/// An encoder that writes the generator's name and shape position into the preimage disagrees.
///
/// The generator is carried on the derivation record and named by no grammar, so a transcript carrying it is a preimage this specification does not describe.
/// What the absence buys is stated where it is spent: the same rendered bytes stay the same artifact across the producers that emitted them, and a shape bump renames nothing.
#[test]
fn an_encoder_that_writes_the_generator_into_the_preimage_disagrees() -> Result<(), ()> {
    let (material, bound) = produced().ok_or(())?;
    let mut twelve = transcript(
        CAPTURE_SEAT,
        "captured-declaration",
        ROOTED,
        &[],
        &material,
        0,
    );
    framed(GENERATOR.as_bytes(), &mut twelve);
    twelve.extend_from_slice(&GENERATOR_SHAPE.to_be_bytes());
    let derived = blake3::derive_key(&context(CAPTURE_SEAT, "captured-declaration"), &twelve);
    assert_ne!(bound.plan().account().commitment().as_bytes(), &derived);
    Ok(())
}

/// The facts about the seat and the kind this lane restated are the facts the compiler declares.
///
/// The kind's declared name and the seat's are together what its semantic key is derived over, and the slot is the position every transcript at that seat carries — so a lane that restated any of them wrongly would be re-deriving something the compiler never wrote, and would say so here rather than four tests later.
#[test]
fn the_seat_this_lane_restated_is_the_seat_the_compiler_declares() {
    assert_eq!(SoleRole::Sole.name(), SEAT_NAME);
    assert_eq!(SoleRole::Sole.slot(), 0);
    assert_eq!(<Greeting as Kind>::NAME, KIND_NAME);
}
