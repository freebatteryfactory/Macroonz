//! The planted defective expansion, and the proof that the checker notices.
//!
//! Every macro family in this repository owes a deliberately defective
//! expansion that testpak rejects. This is the refusal-family derive's.
//!
//! The declaration and the order it declares are stated HERE, twice over: once
//! as the source text handed to the services, and once as the two rosters the
//! judge compares against. Nothing below asks the services what the declared
//! order was.

use threadpak_macroc::derive_refusal::{PlantedDefect, captured};
use threadpak_testpak::{RenderVerdict, judge_declared_order};

/// The declaration handed to the services, as a token stream renders it. The
/// order clause deliberately does not follow the body's layout.
const DECLARATION: &str = "#[refusal(shape = single_cause, order(NotCanonical = \
    \"testpak.demo.not-canonical\", NotAdmitted = \"testpak.demo.not-admitted\", \
    Unbounded = \"testpak.demo.unbounded\"))] enum DemoFamily { NotAdmitted, Unbounded, \
    NotCanonical, }";

/// The declared spellings, stated independently of the services.
const DECLARED_SPELLINGS: [&str; 3] = ["NotCanonical", "NotAdmitted", "Unbounded"];

/// The declared stable identities, stated independently of the services.
const DECLARED_IDENTITIES: [&str; 3] = [
    "testpak.demo.not-canonical",
    "testpak.demo.not-admitted",
    "testpak.demo.unbounded",
];

/// The three verdicts one run of the judge produces: over the lawful rendering,
/// over the permuted-order defect, and over the recycled-identity defect.
fn verdicts() -> Result<(RenderVerdict, RenderVerdict, RenderVerdict), ()> {
    let derivation = captured(DECLARATION).map_err(|_| ())?.planned();
    let lawful = derivation.rendered();
    let permuted = derivation.rendered_with_planted_defect(PlantedDefect::SelectionOrderPermuted);
    let recycled = derivation.rendered_with_planted_defect(PlantedDefect::CauseIdentityRecycled);
    Ok((
        judge_declared_order(lawful.source(), &DECLARED_SPELLINGS, &DECLARED_IDENTITIES),
        judge_declared_order(permuted.source(), &DECLARED_SPELLINGS, &DECLARED_IDENTITIES),
        judge_declared_order(recycled.source(), &DECLARED_SPELLINGS, &DECLARED_IDENTITIES),
    ))
}

/// The planted defective expansion is rejected, AND the lawful one passes.
///
/// Both halves are load-bearing. A checker that rejected everything would
/// satisfy the first and fail the second, which is exactly how a defect check
/// stops being evidence of anything.
#[test]
fn the_planted_defects_are_rejected_and_the_lawful_rendering_passes() {
    assert_eq!(
        verdicts(),
        Ok((
            RenderVerdict::Conforms,
            RenderVerdict::Deviates,
            RenderVerdict::Deviates
        ))
    );
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
