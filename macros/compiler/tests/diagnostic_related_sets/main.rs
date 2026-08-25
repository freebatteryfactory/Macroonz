//! A diagnostic's related set, judged from outside: the postures a public reader can tell apart, and the body identity an INDEPENDENT encoder re-derives.
//!
//! # Independence
//!
//! The compiler can prove that its set-building road behaves; it cannot prove that the road's output is what the SPECIFICATION says it is, because it would be asking itself.
//! So this lane rebuilds both levels from the published prose alone — the content grammar stated on `RelatedSet::derived_over`, the ten-member preimage stated on `Transcript`, the anchoring discriminant table, and the derive-key grammar on `Profile` — and requires the produced identities to match.
//!
//! Not one constant, encoder, or spelling below is imported from the crate under judgement.
//! What IS shared is the digest, deliberately: both sides call BLAKE3, and a lane that reimplemented the hash would be testing an arithmetic exercise rather than a specification.
//!
//! # Reversals
//!
//! A match that could not fail proves nothing.
//! Three negative controls run beside the positive ones: an encoder that concatenates the issues unframed must disagree, an encoder that derives the body under the ISSUE subject must disagree, and the crafted aliasing case — material that derives one identity at both levels under a single-subject grammar — must produce two.

use macroonz_compiler::{Capping, Family, RELATED_ISSUE_LIMIT, RelatedIdentity, RelatedSet};

// ---------------------------------------------------------------------------
// The specification, restated here in full.
// ---------------------------------------------------------------------------

/// The stem every grammar and every subject this compiler owns is declared under.
const STEM: &str = "macroonz/identity";

/// The preimage grammar both levels are derived under.
const GRAMMAR: &str = "diagnostic-relation";

/// That grammar's own version position.
///
/// One position per grammar: a bump under any other grammar moves nothing in this file, and that independence is the point of restating it here.
const GRAMMAR_VERSION: u32 = 1;

/// The subject a related set's WHOLE-BODY commitment is derived under.
const BODY_SUBJECT: &str = "related-body";

/// The subject one established issue's own identity is derived under.
///
/// A different key space from the body's, and that difference is the thing under judgement here.
const ISSUE_SUBJECT: &str = "related-issue";

/// The role both levels are derived at.
const ROLE: &str = "diagnostic-relation";

/// That role's declared slot, counted from the roster's first row.
///
/// A row is appended and never inserted, so this number moves only when a role is declared ahead of this one — which would rename every identity derived at every seat from the move onward.
const ROLE_SLOT: u8 = 14;

/// The anchoring discriminant for a rooted transcript.
const ANCHORING_ROOTED: u8 = 0;

/// Both levels' position, which is zero for every family: the family rides inside the content where a name fits, never in the position seat.
const POSITION: u32 = 0;

/// The name of the family every identity in this file is derived over, restated by hand for this lane's own encoder.
///
/// Any namespaced name would do; it is stated once so the lane and the compiler are asked about one family.
const FAMILY_NAME: &str = "lane/related-sets";

/// The same family as the value this lane hands the road under judgement.
const FAMILY: Family = Family::declared(FAMILY_NAME);

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

/// This lane's own reading of the CONTENT grammar a related identity is derived over: the family's name and the material, each framed by its length.
fn content(material: &[u8]) -> Vec<u8> {
    content_under(FAMILY_NAME, material)
}

/// The same content grammar under any family name, for the reversal that asks two families about one material.
fn content_under(family: &str, material: &[u8]) -> Vec<u8> {
    let mut composed = Vec::new();
    framed(family.as_bytes(), &mut composed);
    framed(material, &mut composed);
    composed
}

/// This lane's own reading of the BODY's material: every issue framed by its own length, in the order the issues were established.
fn body_material(issues: &[Vec<u8>]) -> Vec<u8> {
    let mut composed = Vec::new();
    for issue in issues {
        framed(issue, &mut composed);
    }
    composed
}

/// This lane's own reading of the derive-key context for one subject at one role.
fn context(subject: &str, role: &str) -> String {
    format!("{STEM}/{GRAMMAR}/v{GRAMMAR_VERSION}/{STEM}/{subject}/{role}")
}

/// This lane's own reading of the ten-member preimage, rooted, over one content.
fn transcript(subject: &str, material: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::new();
    framed(STEM.as_bytes(), &mut bytes);
    framed(GRAMMAR.as_bytes(), &mut bytes);
    bytes.extend_from_slice(&GRAMMAR_VERSION.to_be_bytes());
    framed(subject.as_bytes(), &mut bytes);
    framed(ROLE.as_bytes(), &mut bytes);
    bytes.push(ROLE_SLOT);
    bytes.push(ANCHORING_ROOTED);
    framed(&[], &mut bytes);
    framed(material, &mut bytes);
    bytes.extend_from_slice(&POSITION.to_be_bytes());
    bytes
}

/// The identity this lane derives from its own facts, under a context it composed itself.
fn specified(subject: &str, material: &[u8]) -> [u8; 32] {
    blake3::derive_key(&context(subject, ROLE), &transcript(subject, material))
}

// ---------------------------------------------------------------------------
// Reading a produced set through its public readers only.
// ---------------------------------------------------------------------------

/// One issue's canonical material, distinct per seed.
fn material(seed: u32) -> Vec<u8> {
    seed.to_be_bytes().to_vec()
}

/// The whole-body commitment one produced set carries, read by the LEVEL it states rather than by the position it sits at.
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

/// A complete set carries exactly one whole-body commitment and one identity per established issue.
///
/// Load-bearing: every disagreement below is only evidence because the ordinary case produces a set with both levels standing where the specification says they stand.
#[test]
fn a_complete_set_carries_one_body_and_one_identity_per_issue() {
    let issues: Vec<Vec<u8>> = (1..=3).map(material).collect();
    let set = RelatedSet::derived_over(FAMILY, &issues);
    assert_eq!(set.capping(), Capping::Complete);
    assert!(produced_body(&set).is_some());
    assert_eq!(produced_issues(&set).len(), issues.len());
    assert_eq!(set.carried().len(), issues.len().saturating_add(1));
}

/// A complete set and a capped one do not share a readable posture.
///
/// The claim is about what a HOLDER of a diagnostic can tell, so it is asked entirely through the public readers: the capping, the count carried, and the levels present.
/// The coarser commitment a capping carries is still a commitment to THESE issues, so changing one of the DROPPED issues changes the identity that survives.
#[test]
fn a_complete_set_and_a_capped_set_do_not_read_alike() -> Result<(), ()> {
    let magnitude = u32::try_from(RELATED_ISSUE_LIMIT).map_err(|_| ())?;
    let fits: Vec<Vec<u8>> = (1..=3).map(material).collect();
    let over: Vec<Vec<u8>> = (1..=magnitude).map(material).collect();

    let complete = RelatedSet::derived_over(FAMILY, &fits);
    let capped = RelatedSet::derived_over(FAMILY, &over);

    assert_eq!(complete.capping(), Capping::Complete);
    assert_eq!(
        capped.capping(),
        Capping::Truncated {
            omitted: RELATED_ISSUE_LIMIT,
        }
    );
    assert_eq!(produced_issues(&complete).len(), fits.len());
    assert!(produced_issues(&capped).is_empty());
    assert!(produced_body(&capped).is_some());
    assert_eq!(capped.carried().len(), 1);

    let mut other = over.clone();
    let _dropped = other.pop();
    other.push(material(u32::MAX));
    assert_ne!(
        produced_body(&capped),
        produced_body(&RelatedSet::derived_over(FAMILY, &other))
    );
    Ok(())
}

/// The specification alone re-derives both levels of a produced set.
///
/// The issues are the INPUT — a reader of a published receipt is handed the material and asked to re-derive the name — and everything from that material to the thirty-two bytes is this lane's own.
#[test]
fn the_specification_re_derives_both_levels_of_a_produced_set() {
    let issues: Vec<Vec<u8>> = (1..=4).map(material).collect();
    let set = RelatedSet::derived_over(FAMILY, &issues);

    assert_eq!(
        produced_body(&set),
        Some(specified(BODY_SUBJECT, &content(&body_material(&issues))))
    );

    let produced = produced_issues(&set);
    assert_eq!(produced.len(), issues.len());
    assert!(
        issues
            .iter()
            .zip(produced)
            .all(|(issue, published)| specified(ISSUE_SUBJECT, &content(issue)) == published)
    );
}

/// An encoder that concatenates the issues UNFRAMED disagrees.
///
/// That is what the per-issue length prefix buys, and it is proven rather than asserted: without it two different issue sets cut at another boundary reach one body material, and the match above would hold for an encoder admitting exactly that.
#[test]
fn an_encoder_that_drops_the_issue_framing_disagrees() {
    let issues: Vec<Vec<u8>> = (1..=4).map(material).collect();
    let set = RelatedSet::derived_over(FAMILY, &issues);

    let mut unframed = Vec::new();
    for issue in &issues {
        unframed.extend_from_slice(issue);
    }
    assert_ne!(
        produced_body(&set),
        Some(specified(BODY_SUBJECT, &content(&unframed)))
    );
}

/// An encoder that derives the whole-body commitment under the ISSUE subject disagrees.
///
/// The two levels are separated by the subject and by nothing else, so a lane that got the subject wrong must fail rather than quietly agreeing because the content happened to match.
#[test]
fn an_encoder_that_derives_the_body_under_the_issue_subject_disagrees() {
    let issues: Vec<Vec<u8>> = (1..=4).map(material).collect();
    let set = RelatedSet::derived_over(FAMILY, &issues);

    let composed = content(&body_material(&issues));
    assert_ne!(
        specified(BODY_SUBJECT, &composed),
        specified(ISSUE_SUBJECT, &composed)
    );
    assert_ne!(
        produced_body(&set),
        Some(specified(ISSUE_SUBJECT, &composed))
    );
}

/// Material that derives ONE identity at both levels under a single-subject grammar derives two under the subject split.
///
/// The aliasing is constructed rather than described: a body's material is the framing of its issues, so a one-issue set whose single issue's material IS another set's framing composes, by the published content rule, the exact same content bytes at the issue level that the other set composes at the body level.
/// Under one subject those bytes are one derive-key context and one transcript and therefore one identity; under two they are two.
#[test]
fn crafted_aliasing_material_derives_two_identities() {
    let inner: Vec<Vec<u8>> = vec![material(1), material(2)];
    let framing = body_material(&inner);
    let composed = content(&framing);

    assert_ne!(
        specified(BODY_SUBJECT, &composed),
        specified(ISSUE_SUBJECT, &composed)
    );

    let inner_set = RelatedSet::derived_over(FAMILY, &inner);
    let aliasing_set = RelatedSet::derived_over(FAMILY, &[framing]);
    let aliasing_issues = produced_issues(&aliasing_set);
    assert_eq!(aliasing_issues.len(), 1);
    assert_eq!(
        produced_body(&inner_set),
        Some(specified(BODY_SUBJECT, &composed))
    );
    assert_eq!(
        aliasing_issues.first(),
        Some(&specified(ISSUE_SUBJECT, &composed))
    );
    assert_ne!(produced_body(&inner_set).as_ref(), aliasing_issues.first());
}

/// A road that enumerated nothing carries nothing, at either level.
///
/// The capping and the empty set are the two ways a set can carry fewer identities than a reader might expect, and only one of them means anything was dropped.
#[test]
fn a_road_that_enumerated_nothing_carries_nothing_at_either_level() {
    let set = RelatedSet::nothing_enumerated();
    assert!(set.carried().is_empty());
    assert!(produced_body(&set).is_none());
    assert_eq!(set.capping(), Capping::Complete);
}

/// "Nothing was enumerated" has exactly ONE representation, whatever the family.
///
/// The family domain is every namespaced name and is not enumerable, so the population is a spread of deliberately unalike names — short, long, nested, and the lane's own — and the claim each is asked is the same: empty material reaches the one canonical empty relation.
/// Two representations of one state is the defect whichever a reader is handed, because a set carrying a whole-body commitment over empty material and a set carrying no identity at all both mean the road enumerated nothing.
#[test]
fn every_family_reaches_one_empty_relation() {
    let none: [Vec<u8>; 0] = [];
    let canonical = RelatedSet::nothing_enumerated();
    for name in ["a/b", "lane/related-sets", "some-owner/some-space", "x/y/z"] {
        assert_eq!(
            RelatedSet::derived_over(Family::declared(name), &none),
            canonical,
            "family {name} answers empty material with a second representation"
        );
    }
}

/// Two families over one material derive apart, at both levels.
///
/// This is the claim the family exists for: two issue spaces' identical bytes must never share an identity, and the separation is the name rather than anything in the material.
#[test]
fn two_families_over_one_material_derive_apart() {
    let issues: Vec<Vec<u8>> = (1..=2).map(material).collect();
    let one = RelatedSet::derived_over(FAMILY, &issues);
    let other = RelatedSet::derived_over(Family::declared("lane/another-space"), &issues);

    assert_ne!(produced_body(&one), produced_body(&other));
    assert_ne!(produced_issues(&one), produced_issues(&other));
    assert_ne!(
        specified(BODY_SUBJECT, &content(&body_material(&issues))),
        specified(
            BODY_SUBJECT,
            &content_under("lane/another-space", &body_material(&issues))
        )
    );
}

/// The body identity a deriving road would have carried over empty material is a real value, and no road hands it out.
///
/// Named rather than described: a road without the routing frames no issues, so its body material is empty and its commitment is the derivation over the family tag and a zero length.
/// That value is perfectly well formed, which is the point — it is an honest commitment to no issues, and a reader handed one could not tell it from a commitment to the issues of any other empty set at that family.
#[test]
fn the_body_identity_over_empty_material_is_reachable_by_no_road() {
    let none: [Vec<u8>; 0] = [];
    let over_empty = specified(BODY_SUBJECT, &content(&body_material(&none)));

    let derived = RelatedSet::derived_over(FAMILY, &none);
    assert_ne!(produced_body(&derived), Some(over_empty));
    assert!(produced_body(&derived).is_none());
    assert!(produced_issues(&derived).is_empty());

    // The value is not unreachable because it is unbuildable — this lane just built it.
    // A one-issue set whose single issue IS the empty framing derives that same content at the ISSUE level, and the subject split keeps the two apart.
    let aliasing = RelatedSet::derived_over(FAMILY, &[body_material(&none)]);
    assert_eq!(produced_issues(&aliasing).len(), 1);
    assert_ne!(produced_issues(&aliasing).first(), Some(&over_empty));
}
