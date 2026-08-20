//! The related set, judged from outside: the postures a public reader can tell
//! apart, and the body identity an INDEPENDENT encoder re-derives.
//!
//! # Independence
//!
//! `macroc` proves that its set-building road behaves. It cannot prove that the
//! road's output is what the SPECIFICATION says it is, because it would be
//! asking itself: a producer comparing its own value against its own helper
//! agrees for the same reason it exists.
//!
//! So this file rebuilds the body's identity from the published content grammar
//! with its own encoder — its own length framing, its own field order, its own
//! domain-string assembly, its own subject and role spellings, its own preimage
//! family and version — and requires the produced identity to match. Not one
//! encoding function, constant, or spelling is imported from
//! `threadpak-macroc`.
//!
//! The prose this file is written from is the mint site's own construction
//! paragraph on `RelatedSet::derived_over`, which states the content grammar in
//! full and names the role, both subjects, and the profile constant the family
//! segment and version come off — over the frame the transcript specification on
//! `ProjectionTranscript` states, with the rooted posture's own discriminant
//! byte read off the table published on `TranscriptAnchoring`, and the
//! derive-key grammar spelled on `IdentityProfile`. Nothing here was read off an
//! encoder body.
//!
//! `independent_identity_transcript.rs` judges the TRANSCRIPT grammar; this file
//! judges the CONTENT one mint site composes.
//!
//! What IS shared is the digest, and deliberately: both sides call BLAKE3. A
//! lane that reimplemented the hash would be testing an arithmetic exercise
//! rather than a specification.
//!
//! # Reversals
//!
//! A match that could not fail proves nothing. Three negative controls run
//! beside the positive ones: an encoder that frames the body's material by
//! concatenating the issues unframed must DISAGREE, an encoder that derives the
//! body under the ISSUE subject must disagree, and the crafted aliasing case —
//! material that derives one identity at both levels under a single-subject
//! grammar — must produce two.
//!
//! # The compiler's half
//!
//! Assembling a set out of an independently supplied carry and completion, and
//! reaching the two identity levels as separate public inputs, do not compile at
//! all. Those two halves are the compile-fail fixtures
//! `a-related-set-married-to-another-completion.rs` and
//! `a-related-set-assembled-from-two-levels.rs`. This file carries the half a
//! type cannot state.

use threadpak::types::ConstLimit;
use threadpak_macroc::diagnostics::RelatedIssueLimit;
use threadpak_macroc::{RelatedIdentity, RelatedSet, RelatedSetCompletion};

// ---------------------------------------------------------------------------
// The specification, restated here in full.
// ---------------------------------------------------------------------------

/// The profile stem, spelled out rather than imported.
///
/// One stem for every family. What separates two families is the family segment
/// beside it, never a stem a family chose for itself.
const PROFILE_STEM: &str = "threadpak/macroc/projection-identity";

/// The PREIMAGE FAMILY both levels are derived under, spelled out rather than
/// imported.
///
/// It is not passed anywhere: the mint site names the diagnostic-relation role
/// and the family follows from the role, so this lane reads it off the same role
/// and carries no second opinion about which grammar these preimages belong to.
///
/// Both levels stood under the CLOSED-EXPANSION role, which put every related
/// identity in every diagnostic on the terminal's version ladder — so a widening
/// of what a terminal commits to renamed them, and neither level holds a member
/// of that grammar. This lane restates the family they stand in now, and the two
/// names are two key spaces: a value derived under the old one is not reachable
/// from this file at all.
const PREIMAGE_FAMILY: &str = "diagnostic-relation";

/// That family's OWN version position, spelled out rather than imported.
///
/// One position per grammar, and this is the diagnostic-relation grammar's. It
/// moves here deliberately when the published family moves — this lane writes
/// both the derive-key context and the transcript itself, so the constant is
/// the whole of following a bump — and a bump under any OTHER family, the
/// terminal's included, moves nothing in this file. That independence is the
/// point of the move and it is now a property this lane would notice losing.
const PREIMAGE_FAMILY_VERSION: u32 = 1;

/// The subject a related set's WHOLE-BODY commitment is derived under, spelled
/// out rather than imported.
const BODY_SUBJECT: &str = "related-body";

/// The subject one established issue's own identity is derived under, spelled
/// out rather than imported. It is a different name space from the body's, and
/// that difference is the thing under judgement here.
const ISSUE_SUBJECT: &str = "related-issue";

/// The role both levels are derived at, spelled out rather than imported.
const ROLE: &str = "diagnostic-relation";

/// That role's declared slot, read off the roster order the specification
/// states rather than from the producer: the diagnostic relation is the
/// fifteenth row of the fifteen-row role roster, and a slot IS its row's place
/// counted from the first.
///
/// A row is appended and never inserted, so this number moves only when a role
/// is declared ahead of this one — which would renumber every slot from the move
/// onward and rename every identity derived under them.
const ROLE_SLOT: u8 = 14;

/// The anchoring discriminant for a rooted transcript.
///
/// Read off the discriminant table published beside the postures themselves.
/// This lane once had to assume the byte off the posture order and said so here;
/// it is declared now, so the assumption is retired and this constant is a
/// restatement like every other in this file.
const ANCHORING_ROOTED: u8 = 0;

/// The DIAGNOSTIC family tag every identity in this file is derived over. Any
/// byte would do; it is stated once so the judge and the services are asked
/// about the same family.
///
/// A different thing from the preimage family above, and the two are worth
/// keeping apart: this byte is material — the first byte of the content one
/// mint site composes, and the roster position that content's transcript
/// carries. The preimage family is the GRAMMAR that transcript is written
/// under, and it is a declared name rather than a byte a caller chose.
const FAMILY: u8 = 3;

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

/// This lane's own reading of the CONTENT grammar a related identity is derived
/// over: the family tag, then the material framed by its length.
fn judge_content(material: &[u8]) -> Vec<u8> {
    let mut content = vec![FAMILY];
    judge_bytes(material, &mut content);
    content
}

/// This lane's own reading of the BODY's material: every issue, each framed by
/// its own length, in the order the issues were established.
fn judge_body_material(issues: &[Vec<u8>]) -> Vec<u8> {
    let mut framed = Vec::new();
    for issue in issues {
        judge_bytes(issue, &mut framed);
    }
    framed
}

/// This lane's own derive-key context, assembled by the published grammar: the
/// stem, the family, the family's version, the subject, the role.
fn judge_context(subject: &str) -> String {
    format!("{PROFILE_STEM}/{PREIMAGE_FAMILY}/v{PREIMAGE_FAMILY_VERSION}/{subject}/{ROLE}")
}

/// This lane's own transcript: the ten members of the specification, in order,
/// for a rooted derivation at the diagnostic-relation role, at the roster
/// position the mint site states — the family tag.
///
/// There is no generator member. The generator is provenance, no family's
/// grammar names it, and a transcript carrying it would be a preimage this
/// specification does not describe.
fn judge_transcript(subject: &str, content: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::new();
    judge_bytes(PROFILE_STEM.as_bytes(), &mut bytes);
    judge_bytes(PREIMAGE_FAMILY.as_bytes(), &mut bytes);
    bytes.extend_from_slice(&PREIMAGE_FAMILY_VERSION.to_be_bytes());
    judge_bytes(subject.as_bytes(), &mut bytes);
    judge_bytes(ROLE.as_bytes(), &mut bytes);
    bytes.push(ROLE_SLOT);
    bytes.push(ANCHORING_ROOTED);
    judge_bytes(&[], &mut bytes);
    judge_bytes(content, &mut bytes);
    bytes.extend_from_slice(&u32::from(FAMILY).to_be_bytes());
    bytes
}

/// The identity this lane derives, by the published specification and nothing
/// else.
fn judge_identity(subject: &str, content: &[u8]) -> [u8; 32] {
    blake3::derive_key(&judge_context(subject), &judge_transcript(subject, content))
}

// ---------------------------------------------------------------------------
// Reading a produced set through its public readers only.
// ---------------------------------------------------------------------------

/// One issue's canonical material, distinct per seed.
fn material(seed: u32) -> Vec<u8> {
    seed.to_be_bytes().to_vec()
}

/// The whole-body commitment one produced set carries, read by the LEVEL it
/// states rather than by the position it happens to sit at.
fn produced_body(set: &RelatedSet) -> Option<[u8; 32]> {
    set.carried().iter().find_map(|carried| match *carried {
        RelatedIdentity::Body(body) => Some(*body.as_bytes()),
        RelatedIdentity::Issue(_) => None,
    })
}

/// Every per-issue identity one produced set carries, in order.
fn produced_issues(set: &RelatedSet) -> Vec<[u8; 32]> {
    set.carried()
        .iter()
        .filter_map(|carried| match *carried {
            RelatedIdentity::Issue(issue) => Some(*issue.as_bytes()),
            RelatedIdentity::Body(_) => None,
        })
        .collect()
}

// ---------------------------------------------------------------------------
// The lane.
// ---------------------------------------------------------------------------

/// The positive control on the reader itself: a complete set carries exactly one
/// whole-body commitment and one identity per established issue.
///
/// Load-bearing. Every disagreement below is only evidence because the ordinary
/// case produces a set with both levels standing where the specification says
/// they stand.
#[test]
fn a_complete_set_carries_one_body_and_one_identity_per_issue() {
    let issues: Vec<Vec<u8>> = (1..=3).map(material).collect();
    let set = RelatedSet::derived_over(FAMILY, &issues);
    assert!(matches!(set.completion(), RelatedSetCompletion::Complete));
    assert!(produced_body(&set).is_some());
    assert_eq!(produced_issues(&set).len(), issues.len());
    assert_eq!(set.carried().len(), issues.len().saturating_add(1));
}

/// A complete set and a truncated one do not share a readable posture.
///
/// The claim is about what a HOLDER of a diagnostic can tell, so it is asked
/// entirely through the public readers: the completion posture, the count
/// carried, and the levels present. A truncated set that read like a complete
/// one would be the coarser answer wearing the shape of the whole one, and a
/// reader would have nothing to compare it against.
#[test]
fn a_complete_set_and_a_truncated_set_do_not_read_alike() {
    let magnitude = u32::try_from(RelatedIssueLimit::MAX).unwrap_or(u32::MAX);
    let fits: Vec<Vec<u8>> = (1..=3).map(material).collect();
    let over: Vec<Vec<u8>> = (1..=magnitude).map(material).collect();

    let complete = RelatedSet::derived_over(FAMILY, &fits);
    let truncated = RelatedSet::derived_over(FAMILY, &over);

    // The postures disagree, and the truncated one names the count it dropped
    // rather than merely reporting that something was dropped.
    assert!(matches!(
        complete.completion(),
        RelatedSetCompletion::Complete
    ));
    assert!(matches!(
        truncated.completion(),
        RelatedSetCompletion::ReportTruncated(truncation)
            if truncation.omitted().get() == RelatedIssueLimit::MAX
    ));

    // And the sets themselves disagree: the complete one carries every issue's
    // own identity, the truncated one carries the whole-body commitment alone.
    assert_eq!(produced_issues(&complete).len(), fits.len());
    assert!(produced_issues(&truncated).is_empty());
    assert!(produced_body(&truncated).is_some());
    assert_eq!(truncated.carried().len(), 1);

    // The coarser commitment a truncation carries is still a commitment to
    // THESE issues: change one of the DROPPED issues and the identity that
    // survives changes with it.
    let mut other = over.clone();
    other.pop();
    other.push(material(u32::MAX));
    assert_ne!(
        produced_body(&truncated),
        produced_body(&RelatedSet::derived_over(FAMILY, &other))
    );
}

/// The independent judge re-derives the whole-body commitment from the
/// specification alone, and the services' produced value matches it.
///
/// The issues are the INPUT — a reader of a published receipt is handed the
/// material and asked to re-derive the name. Everything from that material to
/// the thirty-two bytes is this lane's own: the body's framing, the content
/// grammar, the transcript, the derive-key context, and the subject spelling.
#[test]
fn the_specification_re_derives_the_produced_body_identity() {
    let issues: Vec<Vec<u8>> = (1..=4).map(material).collect();
    let set = RelatedSet::derived_over(FAMILY, &issues);

    let rebuilt = judge_identity(BODY_SUBJECT, &judge_content(&judge_body_material(&issues)));
    assert_eq!(produced_body(&set), Some(rebuilt));

    // The per-issue level re-derives too, each over its own framed material and
    // under its own subject, so the match above is not one lucky member of a
    // grammar this lane read wrong everywhere else.
    let rebuilt_issues: Vec<[u8; 32]> = issues
        .iter()
        .map(|issue| judge_identity(ISSUE_SUBJECT, &judge_content(issue)))
        .collect();
    assert_eq!(produced_issues(&set), rebuilt_issues);
}

/// Rehearsed reversal, first form: an encoder that concatenates the issues
/// UNFRAMED must disagree.
///
/// That is what the per-issue length prefix buys, and it is proven rather than
/// asserted: without it two different issue sets cut at another boundary reach
/// one body material, and the match above would hold for an encoder that admits
/// exactly that.
#[test]
fn an_encoder_that_drops_the_issue_framing_disagrees() {
    let issues: Vec<Vec<u8>> = (1..=4).map(material).collect();
    let set = RelatedSet::derived_over(FAMILY, &issues);

    let mut unframed = Vec::new();
    for issue in &issues {
        unframed.extend_from_slice(issue);
    }
    let rebuilt = judge_identity(BODY_SUBJECT, &judge_content(&unframed));
    assert_ne!(produced_body(&set), Some(rebuilt));
}

/// Rehearsed reversal, second form: an encoder that derives the whole-body
/// commitment under the ISSUE subject must disagree.
///
/// The two levels are separated by the subject and by nothing else, so a lane
/// that got the subject wrong must fail rather than quietly agreeing because the
/// content happened to match.
#[test]
fn an_encoder_that_derives_the_body_under_the_issue_subject_disagrees() {
    let issues: Vec<Vec<u8>> = (1..=4).map(material).collect();
    let set = RelatedSet::derived_over(FAMILY, &issues);

    let content = judge_content(&judge_body_material(&issues));
    assert_ne!(
        judge_identity(BODY_SUBJECT, &content),
        judge_identity(ISSUE_SUBJECT, &content)
    );
    assert_ne!(
        produced_body(&set),
        Some(judge_identity(ISSUE_SUBJECT, &content))
    );
}

/// The crafted collision: material that derives ONE identity at both levels
/// under a single-subject grammar derives two under the subject split.
///
/// The aliasing is constructed rather than described. A body's material is the
/// framing of its issues, so a one-issue set whose single issue's material IS
/// another set's framing composes, by the published content rule, the exact same
/// content bytes at the issue level that the other set composes at the body
/// level.
///
/// Under one subject those bytes are one derive-key context and one transcript
/// and therefore one identity; under two they are two.
#[test]
fn crafted_aliasing_material_derives_two_identities_now() {
    let inner: Vec<Vec<u8>> = vec![material(1), material(2)];
    let framing = judge_body_material(&inner);

    // The two contents, composed by the two published rules over the two
    // materials, are the same bytes. That is the collision, stated exactly.
    let body_content = judge_content(&framing);
    let aliasing_issue_content = judge_content(&framing);
    assert_eq!(body_content, aliasing_issue_content);

    // One subject over those bytes was one identity.
    assert_eq!(
        judge_identity(ISSUE_SUBJECT, &body_content),
        judge_identity(ISSUE_SUBJECT, &aliasing_issue_content)
    );

    // Two subjects over those same bytes are two, and the services' own produced
    // values are the two.
    let inner_set = RelatedSet::derived_over(FAMILY, &inner);
    let aliasing_set = RelatedSet::derived_over(FAMILY, &[framing]);
    let aliasing_issue = produced_issues(&aliasing_set);
    assert_eq!(aliasing_issue.len(), 1);
    assert!(produced_body(&inner_set).is_some());
    assert_ne!(
        produced_body(&inner_set).as_ref(),
        aliasing_issue.first(),
        "a crafted issue still aliases the body it was framed from"
    );
}

/// A road that enumerated nothing carries nothing, at either level.
///
/// Stated here because the truncation posture and the empty set are the two ways
/// a set can carry fewer identities than a reader might expect, and only one of
/// them means anything was dropped.
///
/// Load-bearing for the two tests below: they claim the derived road REACHES
/// this value, which says nothing unless this value is itself what the
/// specification describes.
#[test]
fn a_road_that_enumerated_nothing_reports_no_truncation() {
    let set = RelatedSet::nothing_enumerated();
    assert!(set.carried().is_empty());
    assert!(produced_body(&set).is_none());
    assert!(matches!(set.completion(), RelatedSetCompletion::Complete));
}

/// "Nothing was enumerated" has exactly ONE representation, over the whole
/// family-tag domain.
///
/// The population is the domain rather than a sample: the family tag is a `u8`,
/// so every value it can take is asked, and a routing that held for the family
/// this file happens to use and not for another would be caught here rather than
/// by whoever met the other family first.
///
/// Two representations of one state is the defect, whichever a reader is handed.
/// A set carrying a whole-body commitment over empty material and a set carrying
/// no identity at all both mean "the road enumerated nothing", so two
/// diagnostics that enumerated nothing would compare UNEQUAL — and a reader who
/// learned to recognize one shape would not recognize the other.
#[test]
fn every_family_tag_reaches_one_empty_relation() {
    let none: [Vec<u8>; 0] = [];
    let canonical = RelatedSet::nothing_enumerated();
    for family in u8::MIN..=u8::MAX {
        assert_eq!(
            RelatedSet::derived_over(family, &none),
            canonical,
            "family {family} answers empty material with a second representation"
        );
    }
}

/// The rehearsed reversal for the routing: the identity a DERIVING road would
/// have carried is a real value this lane can name, and no road in the services
/// hands it out.
///
/// Named rather than described. A road without the routing frames no issues, so
/// its body material is empty, and by the published content rule its whole-body
/// commitment is the derivation over `family_byte || u64be(0)` — which this file
/// composes with its own encoder below.
///
/// That value is perfectly well-formed and that is the point: it is not a corrupt
/// identity a check could notice, it is an honest commitment to no issues, and a
/// reader handed one has no way to tell it from a commitment to the issues of any
/// other empty set at that family.
///
/// If the routing were removed this test fails at the first assertion, because
/// the derived road produces exactly `over_empty`.
#[test]
fn the_body_identity_over_empty_material_is_reachable_by_no_road() {
    let none: [Vec<u8>; 0] = [];
    let over_empty = judge_identity(BODY_SUBJECT, &judge_content(&judge_body_material(&none)));

    let derived = RelatedSet::derived_over(FAMILY, &none);
    assert_ne!(
        produced_body(&derived),
        Some(over_empty),
        "the derived road still carries a whole-body commitment over empty material"
    );
    assert!(produced_body(&derived).is_none());
    assert!(produced_issues(&derived).is_empty());
    assert!(RelatedSet::nothing_enumerated().carried().is_empty());

    // The value is not unreachable because it is unbuildable — this lane just
    // built it. A one-issue set whose single issue IS the empty framing derives
    // that same content at the ISSUE level, and the subject split is what keeps
    // those two apart. So the routing removes a representation, and it does not
    // remove a name from the space.
    let framing_of_nothing = judge_body_material(&none);
    let aliasing = RelatedSet::derived_over(FAMILY, &[framing_of_nothing]);
    assert_eq!(produced_issues(&aliasing).len(), 1);
    assert_ne!(produced_issues(&aliasing).first(), Some(&over_empty));
}
