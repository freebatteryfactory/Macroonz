//! The mutation seat: testpak damages a lawful artifact and proves each lane
//! catches what that lane CLAIMS to catch.
//!
//! The mutations are testpak's, not the services'. The services no longer carry
//! a road that renders a deliberately defective artifact — a generator writing
//! its own exam is rehearsed only against the defects it already imagined.
//!
//! The declaration and the order it declares are stated HERE, twice over: once
//! as the source text handed to the services, and once as the two rosters the
//! judge compares against. Nothing below asks the services what the declared
//! order was.

use threadpak_macroc::compile_refusal_text;
use threadpak_testpak::{
    ARTIFACT_MUTATIONS, ArtifactMutation, DeclaredStructure, LaneOwnership, RenderVerdict,
    StructuralDisagreement, StructuralVerdict, judge_declared_order, judge_structure, mutated,
};

/// The declaration handed to the services. The order clause deliberately does
/// not follow the body's layout.
const DECLARATION: &str = "#[refusal(family = \"testpak.demo\", shape = single_cause, \
    order(NotCanonical = \"not-canonical\", NotAdmitted = \"not-admitted\", \
    Unbounded = \"unbounded\"))] enum DemoFamily { NotAdmitted, Unbounded, NotCanonical, }";

/// The declared spellings, stated independently of the services.
const DECLARED_SPELLINGS: [&str; 3] = ["NotCanonical", "NotAdmitted", "Unbounded"];

/// The declared stable identities, stated independently of the services. The
/// caller writes them out in full HERE precisely because the derive composes
/// them from a family identity and three local keys — so the two statements are
/// independent.
const DECLARED_IDENTITIES: [&str; 3] = [
    "testpak.demo.not-canonical",
    "testpak.demo.not-admitted",
    "testpak.demo.unbounded",
];

/// The trait paths the artifact declares, in the order it declares them —
/// stated here, spelled out, rather than read off the thing under judgement.
const DECLARED_TRAITS: [&str; 2] = [
    "::threadpak::refusal::RefusalFamily",
    "::threadpak::refusal::CauseOrderDeclaration",
];

/// The whole structural declaration lane B is held to. Every roster in it is
/// authored beside [`DECLARATION`], by the same hand that wrote the declaration
/// and by nothing downstream of it.
const DECLARED_STRUCTURE: DeclaredStructure<'static> = DeclaredStructure {
    target: "DemoFamily",
    traits: &DECLARED_TRAITS,
    shape: "SingleCause",
    spellings: &DECLARED_SPELLINGS,
    identities: &DECLARED_IDENTITIES,
};

/// The lawful artifact, as the receipt-rich road produced and closed over it.
fn lawful() -> Result<String, ()> {
    compile_refusal_text(DECLARATION)
        .map(|(_, closed)| closed.inspected())
        .map_err(|_| ())
}

/// The lawful rendering passes lane A.
///
/// Load-bearing in its own right. A checker that rejected everything would catch
/// every mutation below and be worthless.
#[test]
fn the_lawful_rendering_conforms() {
    assert_eq!(
        lawful().map(|text| judge_declared_order(&text, &DECLARED_SPELLINGS, &DECLARED_IDENTITIES)),
        Ok(RenderVerdict::Conforms)
    );
}

/// Every mutation lane A OWNS is caught by lane A.
///
/// The claim is narrow on purpose: lane A reads bytes, so it is held to the
/// mutations that change the exact declared textual forms it anchors on. The
/// mutations it does not own are asserted separately, below, as NOT caught —
/// which is the honest half nobody writes.
#[test]
fn lane_a_catches_every_mutation_lane_a_owns() {
    // The lawful artifact is the control every assertion below rests on. A run
    // that could not produce it has tested nothing, and fails here rather than
    // passing over an empty string.
    let text = lawful().unwrap_or_default();
    assert!(
        !text.is_empty(),
        "the lawful artifact did not compile through the receipt-rich road"
    );
    let owned: Vec<ArtifactMutation> = ARTIFACT_MUTATIONS
        .into_iter()
        .filter(|mutation| mutation.owned_by() == LaneOwnership::ByteProfile)
        .collect();
    assert!(!owned.is_empty(), "lane A owns no mutation at all");
    for mutation in owned {
        let damaged = mutated(&text, mutation).unwrap_or_default();
        assert!(
            !damaged.is_empty() && damaged != text,
            "the lawful artifact carries nothing for `{}` to damage",
            mutation.described()
        );
        let verdict = judge_declared_order(&damaged, &DECLARED_SPELLINGS, &DECLARED_IDENTITIES);
        assert_ne!(
            verdict,
            RenderVerdict::Conforms,
            "lane A owns `{}` and did not catch it",
            mutation.described()
        );
    }
}

/// The mutations lane A does NOT own are, in fact, not caught by lane A.
///
/// This is the assertion that keeps the ownership ledger honest. Without it,
/// "lane A owns these four" is an unfalsifiable label: a lane that happened to
/// catch everything would make the split meaningless, and a lane that caught
/// nothing would look identical to one that caught its share.
///
/// The decoy is the sharpest case. A comment carrying the anchored bytes is
/// invisible to a reader that does not know what a comment is — and a reader
/// that learned would have started implementing Rust.
#[test]
fn lane_a_does_not_catch_what_lane_a_does_not_own() {
    // The lawful artifact is the control every assertion below rests on. A run
    // that could not produce it has tested nothing, and fails here rather than
    // passing over an empty string.
    let text = lawful().unwrap_or_default();
    assert!(
        !text.is_empty(),
        "the lawful artifact did not compile through the receipt-rich road"
    );
    let decoy = mutated(&text, ArtifactMutation::DecoyInComment);
    assert!(decoy.is_some_and(|damaged| {
        damaged != text
            && judge_declared_order(&damaged, &DECLARED_SPELLINGS, &DECLARED_IDENTITIES)
                == RenderVerdict::Conforms
    }));
    let unplanned = mutated(&text, ArtifactMutation::UnplannedOutputAdded);
    assert!(unplanned.is_some_and(|damaged| {
        damaged != text
            && judge_declared_order(&damaged, &DECLARED_SPELLINGS, &DECLARED_IDENTITIES)
                == RenderVerdict::Conforms
    }));
}

/// The lawful rendering passes lane B.
///
/// Load-bearing exactly as lane A's control is: a structural reader that
/// disagreed with everything would catch all four mutations below and be
/// worthless. This is the half that says the reader can also say yes.
#[test]
fn the_lawful_rendering_is_structurally_conforming() {
    assert_eq!(
        lawful().map(|text| judge_structure(&text, &DECLARED_STRUCTURE)),
        Ok(StructuralVerdict::Conforms)
    );
}

/// Every mutation lane B OWNS is caught by lane B.
///
/// These are the four the ownership ledger records as structural, and until the
/// lane existed no test here caught any of them. The claim stays exactly as
/// narrow as the lane: each mutation below changes what the artifact DECLARES,
/// and that is what is read back — not whether the result would compile.
#[test]
fn lane_b_catches_every_mutation_lane_b_owns() {
    // The lawful artifact is the control every assertion below rests on. A run
    // that could not produce it has tested nothing, and fails here rather than
    // passing over an empty string.
    let text = lawful().unwrap_or_default();
    assert!(
        !text.is_empty(),
        "the lawful artifact did not compile through the receipt-rich road"
    );
    let owned: Vec<ArtifactMutation> = ARTIFACT_MUTATIONS
        .into_iter()
        .filter(|mutation| mutation.owned_by() == LaneOwnership::Structural)
        .collect();
    assert!(!owned.is_empty(), "lane B owns no mutation at all");
    for mutation in owned {
        let damaged = mutated(&text, mutation).unwrap_or_default();
        assert!(
            !damaged.is_empty() && damaged != text,
            "the lawful artifact carries nothing for `{}` to damage",
            mutation.described()
        );
        let verdict = judge_structure(&damaged, &DECLARED_STRUCTURE);
        assert_ne!(
            verdict,
            StructuralVerdict::Conforms,
            "lane B owns `{}` and did not catch it",
            mutation.described()
        );
    }
}

/// Lane B names the structural fact it disagreed about, and the four it owns
/// each land on a different one.
///
/// A lane that answered "no" to everything for one reason would pass the test
/// above while measuring one thing four times. These four questions are
/// genuinely different questions, and the answers say so.
///
/// The decoy is the pair that makes the two lanes' split visible in one line.
/// Lane A reports `Conforms` on it — the anchored bytes are present, in a
/// comment — while lane B reads the constant the artifact actually declares and
/// finds the selection order reversed. Neither reader is wrong; they were asked
/// different questions.
#[test]
fn lane_b_names_the_structural_fact_it_found() {
    let text = lawful().unwrap_or_default();
    assert!(
        !text.is_empty(),
        "the lawful artifact did not compile through the receipt-rich road"
    );
    let found = |mutation| {
        judge_structure(
            &mutated(&text, mutation).unwrap_or_default(),
            &DECLARED_STRUCTURE,
        )
    };
    assert_eq!(
        found(ArtifactMutation::ImplTargetAltered),
        StructuralVerdict::Deviates(StructuralDisagreement::ImplementationTarget)
    );
    assert_eq!(
        found(ArtifactMutation::TraitPathWrong),
        StructuralVerdict::Deviates(StructuralDisagreement::TraitPath)
    );
    assert_eq!(
        found(ArtifactMutation::UnplannedOutputAdded),
        StructuralVerdict::Deviates(StructuralDisagreement::OutputCardinality)
    );
    assert_eq!(
        found(ArtifactMutation::DecoyInComment),
        StructuralVerdict::Deviates(StructuralDisagreement::SelectionOrder)
    );
    assert_eq!(
        found(ArtifactMutation::OutputDuplicated),
        StructuralVerdict::Deviates(StructuralDisagreement::DuplicateImplementation)
    );
}

/// Lane B says so when there was nothing to read, rather than passing by
/// default.
///
/// `Unparsable` is lane B's `Unreadable`: a failure class of its own, never a
/// skip. A reader that returned `Conforms` for text it could not parse would
/// disarm every assertion above it.
#[test]
fn a_rendering_that_is_not_rust_is_unparsable_not_conforming() {
    assert_eq!(
        judge_structure("impl {{{ not rust", &DECLARED_STRUCTURE),
        StructuralVerdict::Unparsable
    );
}

/// Every declared mutation actually damages the lawful artifact.
///
/// A mutation that silently applied to nothing would sit in the roster looking
/// like coverage while testing an unchanged string.
#[test]
fn every_declared_mutation_damages_the_artifact() {
    // The lawful artifact is the control every assertion below rests on. A run
    // that could not produce it has tested nothing, and fails here rather than
    // passing over an empty string.
    let text = lawful().unwrap_or_default();
    assert!(
        !text.is_empty(),
        "the lawful artifact did not compile through the receipt-rich road"
    );
    for mutation in ARTIFACT_MUTATIONS {
        let damaged = mutated(&text, mutation);
        assert!(
            damaged.is_some_and(|damaged| damaged != text),
            "`{}` damaged nothing",
            mutation.described()
        );
    }
}

/// The judge reads the artifact and reports what it found, and says so when
/// there is nothing to read rather than passing by default.
#[test]
fn a_rendering_with_no_projection_at_all_is_unreadable_not_conforming() {
    assert_eq!(
        judge_declared_order("struct NothingWasRenderedHere;", &[], &[]),
        RenderVerdict::Unreadable
    );
}

/// The rehearsed false alarm: a LAWFUL rendering whose anchor text has been
/// shifted by whitespace is `Unreadable`, and is `Unreadable` loudly.
///
/// This is the case the judge's alarm exists for and the case that is easiest to
/// mistake for noise. The rendering below is correct — same order, same
/// identities, same everything the declaration states — and the only change is
/// blank space inside the const item the reader anchors on. The reader loses its
/// anchor, reads nothing, and must say `Unreadable` rather than fall through to
/// a verdict about content it never saw.
///
/// Rehearsing it here means the alarm is known to sound before anyone has to
/// interpret one in anger. And it fixes the response: when a real rendering
/// changes shape, the anchor in `03_judge/mod.rs` is re-stated to match the new
/// shape, deliberately and visibly. It is never loosened — no whitespace
/// trimming, no prefix matching, no looser fallback — because a reader widened
/// until it matches again has stopped reading the artifact and started agreeing
/// with the renderer.
#[test]
fn a_whitespace_shifted_lawful_rendering_is_unreadable() {
    assert_eq!(shifted_anchor_verdicts(), Ok(FalseAlarmRehearsal::ThePair));
}

/// What the rehearsal proves, as one value: the unshifted lawful rendering
/// conformed AND the whitespace-shifted one was `Unreadable`. Stating it as one
/// answer keeps the control from being quietly dropped later.
#[derive(Debug, PartialEq, Eq)]
enum FalseAlarmRehearsal {
    /// Both halves held.
    ThePair,
    /// The unshifted control did not conform: the rendering itself was wrong,
    /// so the rehearsal proves nothing about the anchor.
    ControlDidNotConform(RenderVerdict),
    /// The anchor text was not present to shift.
    NothingToShift,
    /// The shifted rendering produced some verdict other than `Unreadable`.
    ShiftedWasNotUnreadable(RenderVerdict),
}

/// Renders the lawful artifact, judges it, then shifts blank space inside the
/// anchored const item and judges it again.
fn shifted_anchor_verdicts() -> Result<FalseAlarmRehearsal, ()> {
    let source = lawful()?;

    let control = judge_declared_order(&source, &DECLARED_SPELLINGS, &DECLARED_IDENTITIES);
    if control != RenderVerdict::Conforms {
        return Ok(FalseAlarmRehearsal::ControlDidNotConform(control));
    }

    let shifted = source.replace(
        "const SELECTION_ORDER : & 'static [ & 'static str ] = &",
        "const  SELECTION_ORDER:&'static[&'static str]=&",
    );
    if shifted == source {
        return Ok(FalseAlarmRehearsal::NothingToShift);
    }

    let alarmed = judge_declared_order(&shifted, &DECLARED_SPELLINGS, &DECLARED_IDENTITIES);
    if alarmed == RenderVerdict::Unreadable {
        Ok(FalseAlarmRehearsal::ThePair)
    } else {
        Ok(FalseAlarmRehearsal::ShiftedWasNotUnreadable(alarmed))
    }
}
