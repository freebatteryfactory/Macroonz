//! Planned-versus-rendered reversals that exercise the proof pass through values built on lawful public roads.

use super::{DECLARATION, DOOR, OTHER_DECLARATION, Pair, Seat, expansion, spelled};
use macroonz_compiler::{
    Closure, ClosureIssue, Expansion, Profile, RenderedProjection, Request, TextCapture, Version,
};

/// One expansion under a caller-declared rendering profile.
fn expansion_under(profile: Profile) -> Option<Expansion<Pair>> {
    let read = TextCapture::read(DECLARATION).ok()?;
    Request::<Pair>::over(read.input().clone(), "pair", &DOOR)
        .profile(profile)
        .render(|_plan, out| {
            out.unit(Seat::Head, spelled("head")?)?;
            out.unit(Seat::Tail, spelled("tail")?)
        })
        .ok()
}

/// A unit from another lawful plan occupies the right seat but answers to another semantic key, and closure refuses it before accepting the sibling that still belongs.
#[test]
fn a_foreign_unit_cannot_occupy_a_planned_seat() -> Result<(), ()> {
    let planned = expansion(DECLARATION).ok_or(())?;
    let foreign = expansion(OTHER_DECLARATION).ok_or(())?;
    let foreign_head = foreign
        .closure()
        .rendered()
        .under(Seat::Head)
        .ok_or(())?
        .clone();
    let planned_tail = planned
        .closure()
        .rendered()
        .under(Seat::Tail)
        .ok_or(())?
        .clone();
    let rendering = RenderedProjection::materialized(vec![foreign_head, planned_tail])
        .map_err(|_refusal| ())?;
    let refusal = Closure::proved(planned.plan(), rendering).err().ok_or(())?;
    assert_eq!(
        refusal.first_issue(),
        &ClosureIssue::SemanticKeyMismatch { role: Seat::Head }
    );
    Ok(())
}

/// A doubled rendered seat and its missing sibling are co-establishable, so one refusal carries both findings in roster order.
#[test]
fn doubled_and_missing_seats_refuse_together() -> Result<(), ()> {
    let planned = expansion(DECLARATION).ok_or(())?;
    let head = planned
        .closure()
        .rendered()
        .under(Seat::Head)
        .ok_or(())?
        .clone();
    let rendering =
        RenderedProjection::materialized(vec![head.clone(), head]).map_err(|_refusal| ())?;
    let refusal = Closure::proved(planned.plan(), rendering).err().ok_or(())?;
    let issues: Vec<ClosureIssue<Seat>> = refusal.issues().iter().copied().collect();
    assert_eq!(
        issues,
        vec![
            ClosureIssue::MemberDuplicated {
                role: Seat::Head,
                observed: 2,
            },
            ClosureIssue::MemberMissing { role: Seat::Tail },
        ]
    );
    Ok(())
}

/// Two lawful plans over the same declaration and content retain one semantic roster while requiring different rendering profiles, so neither rendering can close over the other plan.
#[test]
fn a_rendering_under_another_profile_cannot_close_over_the_plan() -> Result<(), ()> {
    let planned = expansion(DECLARATION).ok_or(())?;
    let other_profile = Profile::declared("lane", "other-rendering", Version::declared(1));
    let rendered_elsewhere = expansion_under(other_profile).ok_or(())?;
    let refusal = Closure::proved(
        planned.plan(),
        rendered_elsewhere.closure().rendered().clone(),
    )
    .err()
    .ok_or(())?;
    let issues: Vec<ClosureIssue<Seat>> = refusal.issues().iter().copied().collect();
    assert_eq!(
        issues,
        vec![
            ClosureIssue::MaterializationMismatch { role: Seat::Head },
            ClosureIssue::MaterializationMismatch { role: Seat::Tail },
        ]
    );
    Ok(())
}
