//! The seat-deletion seat, executed: each repair the failure-path law killed is
//! restored HERE, by this plane, and the value it produces is shown to be about
//! something else while the live road refuses instead.
//!
//! # Why the mutant is testpak's and not the services'
//!
//! The law says a failed required seat is never repaired with an empty, default,
//! or neighbouring value. A law of that shape is only evidence if the repair can
//! be performed and seen to be wrong. The services no longer carry any of them —
//! that is the whole point — so the plane performs each one itself, out of the
//! public values a lawful compilation hands back, and states what the repair
//! would have produced beside what the road actually does.
//!
//! Two directions, always. The lawful road binds every required seat and closes;
//! the restored repair produces a well-formed, complete-looking value about a
//! different subject, and the seam it would have filled refuses with a typed
//! cause that names the seat.
//!
//! # What is unwritable is proven elsewhere
//!
//! Deleting a seat outright — a receipt without a closure, a rendering off the
//! membership-only draft, a closure nobody proved, a join outside the proof —
//! does not compile at all, and the compile-fail fixtures beside this file carry
//! that half. This file carries the seats that die by REFUSAL rather than by the
//! compiler, which is the half a type cannot state.
//!
//! Every declaration below is written here, beside the source text handed to the
//! services, and nothing asks the services what they decided.

use threadpak::types::ConstLimit;
use threadpak_macroc::derive_refusal::diagnose;
use threadpak_macroc::plane::{HumanTextLimit, RelatedIssueSubject};
use threadpak_macroc::{
    ClosureIssue, ExplanationBindingRefusal, ExplanationSeat, HumanProjection, MacrocPhase,
    ObservedClassification, PlannedMembership, ProjectionClosure, ProjectionIdentity,
    ProjectionPlanningIssue, RenderedImplementation, RenderedProjection, RenderedRole,
    RenderedUnit, ReproductionRoute, TextCompileRefusal, compile_refusal_text,
};

/// The declaration handed to the services: a single-cause family, whose shape
/// fixes a two-role output set.
const DECLARATION: &str = "#[refusal(family = \"testpak.demo\", shape = single_cause, \
    order(NotCanonical = \"not-canonical\", NotAdmitted = \"not-admitted\", \
    Unbounded = \"unbounded\"))] enum DemoFamily { NotAdmitted, Unbounded, NotCanonical, }";

/// A declaration naming a shape word the machine's roster does not admit. It is
/// refused at capture, which is the earliest seat on the road.
const SHAPE_NOT_ADMITTED: &str = "#[refusal(family = \"testpak.demo\", shape = tri_state)] \
    enum DemoFamily { NotAdmitted, }";

/// How many rendered roles this declaration's shape fixes, stated here rather
/// than counted off the plan.
const DECLARED_ROLE_COUNT: usize = 2;

/// The declared magnitude one human projection stands under, stated here rather
/// than read off the services. The over-long case is one byte past it.
const DECLARED_HUMAN_TEXT_MAGNITUDE: usize = 512;

/// One lawful closed expansion, or nothing where the road refused.
fn lawful() -> Result<threadpak_macroc::ClosedExpansion, ()> {
    compile_refusal_text(DECLARATION)
        .map(|(_, closed)| closed)
        .map_err(|_| ())
}

/// The identities one diagnostic's related set carries, in order.
fn related(
    diagnostic: &threadpak_macroc::MacrocDiagnostic,
) -> Vec<ProjectionIdentity<RelatedIssueSubject>> {
    diagnostic.related.iter().copied().collect()
}

/// The lawful road binds every required seat and closes over what it rendered.
///
/// Load-bearing in its own right. Every refusal below is only evidence because
/// the same road, on a lawful declaration, produces a receipt with each seat
/// occupied — a road that refused everything would satisfy every hostile
/// assertion here and be worthless.
#[test]
fn the_lawful_road_binds_every_required_seat() {
    assert!(lawful().is_ok_and(|closed| {
        closed.plan().membership().len() == DECLARED_ROLE_COUNT
            && closed.rendered().len() == DECLARED_ROLE_COUNT
            && closed.closure().reconstructed().len() == DECLARED_ROLE_COUNT
            && closed.explanation().len() == 9
            && !closed.emitted().is_empty()
            && closed.emitted() == closed.closure().emitted()
    }));
}

/// The shortened-complete-set repair, restored: it produces a membership that is
/// honest about a plan nobody declared.
///
/// The killed repair took a two-role complete set whose construction failed and
/// handed back the first member alone. The mutant below builds exactly that
/// value — a one-member membership over a declaration whose shape fixes two —
/// and it is well-formed, complete-looking, and about a smaller claim. The
/// closure refuses to close the real rendering over it, naming the role the
/// smaller claim dropped.
#[test]
fn a_shortened_complete_set_proves_a_smaller_claim() {
    assert!(lawful().is_ok_and(|closed| {
        let family = closed
            .plan()
            .membership()
            .under(RenderedImplementation::RenderedFamilyImpl)
            .cloned();
        family.is_some_and(|member| {
            let shortened = PlannedMembership::complete(member, []);
            let smaller =
                shortened.len() == 1 && closed.plan().membership().len() == DECLARED_ROLE_COUNT;
            let refused = ProjectionClosure::proved(
                closed.plan().identity(),
                &shortened,
                closed.closure().rendered().clone(),
            )
            .is_err_and(|refusal| {
                *refusal.issues.first()
                    == ClosureIssue::MemberUnplanned {
                        role: RenderedImplementation::RenderedCauseOrderImpl,
                    }
            });
            smaller && refused
        })
    }));
}

/// The neighbouring-digest repair, restored: the first rendered unit's digest is
/// a digest of the wrong bytes.
///
/// The killed repair answered the explanation's output-and-digest seat with the
/// FIRST rendered unit's digest, whatever it was a digest of. The mutant below
/// builds a rendering whose first unit is the neighbour, which is a rendering
/// the renderer is free to produce — role order is declared, rendering order is
/// not — and the digest that repair would have read is not the family role's.
///
/// The seat refuses instead, and it names itself. The refusal is exhibited as
/// the typed value rather than reached through the live road, because the live
/// road cannot reach it: a rendering missing the family role never closes, so
/// the closure refuses one seat earlier. Both halves are asserted.
#[test]
fn a_neighbouring_digest_answers_about_another_value() -> Result<(), ()> {
    let closed = lawful()?;
    let family = closed
        .rendered()
        .under(RenderedImplementation::RenderedFamilyImpl)
        .cloned()
        .ok_or(())?;
    let neighbour = closed
        .rendered()
        .under(RenderedImplementation::RenderedCauseOrderImpl)
        .cloned()
        .ok_or(())?;
    let reordered = RenderedProjection::materialized(neighbour.clone(), vec![family.clone()]);
    assert!(reordered.is_ok_and(|reordered| {
        reordered
            .units()
            .next()
            .is_some_and(|first| first.digest() != family.digest())
    }));
    // The live road refuses one seat earlier: with no unit under the family role
    // there is nothing to close over, so the explanation is never asked its
    // question.
    let closed_earlier = ProjectionClosure::proved(
        closed.plan().identity(),
        closed.plan().membership(),
        RenderedProjection::of_one(neighbour),
    );
    assert!(closed_earlier.is_err_and(|refusal| {
        *refusal.issues.first()
            == ClosureIssue::MemberMissing {
                role: RenderedImplementation::RenderedFamilyImpl,
            }
    }));
    Ok(())
}

/// Every explanation seat that can fail to bind names itself, and the three name
/// three different things.
///
/// The three seats were three different neighbouring-value repairs — the first
/// planned member whatever its role, the first rendered unit's digest whatever
/// it was a digest of, a hardcoded owner fact nobody's plan cited. A caller
/// repairing a derivation needs to know which of the three failed, so the
/// refusal is typed per seat and the distinction survives into the diagnostic:
/// each projects under `SeatAbsent`, each names its own seat in the line, and
/// each derives its own related identity.
#[test]
fn each_explanation_seat_refuses_under_its_own_name() {
    let seats = [
        ExplanationSeat::PlannedFamilyMember,
        ExplanationSeat::ProvedFamilyDigest,
        ExplanationSeat::DeclaredAssumption,
    ];
    let mut identities: Vec<ProjectionIdentity<RelatedIssueSubject>> = Vec::new();
    for seat in seats {
        let diagnostic =
            diagnose::explanation_refused(&ExplanationBindingRefusal::RequiredOutputAbsent {
                seat,
            });
        assert!(matches!(
            diagnostic.observed,
            ObservedClassification::SeatAbsent
        ));
        assert!(matches!(diagnostic.phase, MacrocPhase::Inspection));
        assert!(diagnostic.summary.shown().contains(seat.described()));
        assert_eq!(diagnostic.repairs.len(), 1);
        identities.extend(related(&diagnostic));
    }
    let counted = identities.len();
    identities.sort_unstable_by_key(|identity| *identity.as_bytes());
    identities.dedup();
    assert_eq!(
        identities.len(),
        counted,
        "two explanation seats derived one identity"
    );
}

/// A doubled role refuses at the declaration AND at the closure, and the two
/// refusals are not one another.
///
/// One defect, two seats, two vocabularies. The membership road refuses to
/// DECLARE a set carrying two members under one role, naming the role slot and
/// the count. The closure refuses to PROVE against a set that carries one
/// anyway, because a role-to-unit match over a doubled role elects one of the
/// two and a proof that elected its own subject proves nothing.
///
/// Both classify what they observed as an identity disagreement, which is
/// exactly why the related identities matter: a projection that collapsed the
/// families would have handed a caller one sentence under one classification for
/// two different repairs.
#[test]
fn a_doubled_role_refuses_at_the_declaration_and_at_the_closure() -> Result<(), ()> {
    let closed = lawful()?;
    let member = closed
        .plan()
        .membership()
        .under(RenderedImplementation::RenderedFamilyImpl)
        .cloned()
        .ok_or(())?;
    let unit = closed
        .rendered()
        .under(RenderedImplementation::RenderedFamilyImpl)
        .cloned()
        .ok_or(())?;
    let doubled = PlannedMembership::complete(member.clone(), [member.clone()]);
    let planning = PlannedMembership::declared(member.clone(), vec![member])
        .err()
        .ok_or(())?;
    let closure = ProjectionClosure::proved(
        closed.plan().identity(),
        &doubled,
        RenderedProjection::of_one(unit),
    )
    .err()
    .ok_or(())?;
    assert!(
        *planning.issues.first()
            == ProjectionPlanningIssue::MembershipDoubled {
                role_slot: RenderedImplementation::RenderedFamilyImpl.slot(),
                observed: 2,
            }
            && *closure.issues.first()
                == ClosureIssue::MemberPlannedTwice {
                    role: RenderedImplementation::RenderedFamilyImpl,
                    observed: 2,
                }
    );
    let first = diagnose::planning_refused(&planning);
    let second = diagnose::closure_refused(&closure);
    assert!(
        matches!(first.observed, ObservedClassification::IdentityDisagreement)
            && matches!(
                second.observed,
                ObservedClassification::IdentityDisagreement
            )
            && first.summary != second.summary
            && related(&first) != related(&second)
    );
    Ok(())
}

/// Two refusal families reaching the diagnostic keep their distinctions, even
/// where they classify alike.
///
/// Five steps of the road refuse in five vocabularies, and all five used to
/// collapse into one sentence under one classification with an empty related
/// set. A closure that dropped a role and an explanation that could not bind its
/// digest both observe an absent seat — and they are different absences, of
/// different things, repaired differently. The related identities say so.
#[test]
fn two_families_observing_one_classification_are_still_two_refusals() {
    assert!(lawful().is_ok_and(|closed| {
        let unit = closed
            .rendered()
            .under(RenderedImplementation::RenderedFamilyImpl)
            .cloned();
        unit.is_some_and(|unit| {
            ProjectionClosure::proved(
                closed.plan().identity(),
                closed.plan().membership(),
                RenderedProjection::of_one(unit),
            )
            .is_err_and(|refusal| {
                let from_closure = diagnose::closure_refused(&refusal);
                let from_explanation = diagnose::explanation_refused(
                    &ExplanationBindingRefusal::RequiredOutputAbsent {
                        seat: ExplanationSeat::ProvedFamilyDigest,
                    },
                );
                matches!(
                    from_closure.observed,
                    ObservedClassification::SeatAbsent
                ) && matches!(
                    from_explanation.observed,
                    ObservedClassification::SeatAbsent
                ) && from_closure.summary != from_explanation.summary
                    && related(&from_closure) != related(&from_explanation)
                    // The closure's line names the role it was established at,
                    // which is the distinction a shared sentence lost first.
                    && from_closure
                        .summary
                        .shown()
                        .contains(RenderedImplementation::RenderedCauseOrderImpl.described())
            })
        })
    }));
}

/// The empty-projection repair, restored: an over-long rendering refuses rather
/// than becoming a blank one.
///
/// The magnitude is stated here, not read off the services, so the assertion is
/// between two independent statements. One byte under it is admitted and carries
/// its whole length; one byte over it refuses. Neither answer is an empty
/// projection, which is what the killed repair produced.
#[test]
fn an_over_long_projection_refuses_rather_than_blanking() {
    assert_eq!(HumanTextLimit::MAX, DECLARED_HUMAN_TEXT_MAGNITUDE);
    let fitting = "x".repeat(DECLARED_HUMAN_TEXT_MAGNITUDE);
    let admitted = HumanProjection::<HumanTextLimit>::projected(&fitting);
    assert!(admitted.is_ok_and(|projection| {
        projection.len() == DECLARED_HUMAN_TEXT_MAGNITUDE && !projection.is_empty()
    }));

    let oversized = "x".repeat(DECLARED_HUMAN_TEXT_MAGNITUDE.saturating_add(1));
    assert!(HumanProjection::<HumanTextLimit>::projected(&oversized).is_err());
}

/// A refused declaration reaches a caller as a complete diagnostic, and every
/// seat a diagnostic owes is occupied.
///
/// The last required seat on the list is the diagnostic's own. A refusal that
/// arrived without its phase, its classification, its repair, or its
/// reproduction route would be a complaint rather than an answer — and the
/// reproduction route in particular is what makes the callable road a road
/// rather than a promise, since this whole test reaches it without a proc-macro
/// anywhere in the path.
#[test]
fn a_refused_declaration_carries_every_diagnostic_seat() {
    let refused = compile_refusal_text(SHAPE_NOT_ADMITTED).map(|(_, closed)| closed.identity());
    assert!(refused.is_err_and(|refusal| match refusal {
        TextCompileRefusal::Refused(carried) => {
            let (_, diagnostic) = *carried;
            matches!(diagnostic.phase, MacrocPhase::Capture)
                && matches!(
                    diagnostic.observed,
                    ObservedClassification::ContractDisagreement
                )
                && !diagnostic.summary.shown().is_empty()
                && diagnostic.repairs.len() == 1
                && matches!(
                    diagnostic.reproduction,
                    ReproductionRoute::CallableServices { .. }
                )
        }
        TextCompileRefusal::NotReadable(_) => false,
    }));
}

/// A rendered unit answers for its own bytes, so the two units of one derivation
/// are never interchangeable.
///
/// This is the fact every neighbouring-value repair above depended on being
/// false. Both units are rendered from one declaration under one profile to one
/// destination, and they still carry different semantic keys and different
/// digests, because the key is derived over the ROLE and the digest over the
/// bytes that role produced.
#[test]
fn the_two_rendered_units_are_never_interchangeable() {
    assert!(lawful().is_ok_and(|closed| {
        let family = closed
            .rendered()
            .under(RenderedImplementation::RenderedFamilyImpl)
            .cloned();
        let neighbour = closed
            .rendered()
            .under(RenderedImplementation::RenderedCauseOrderImpl)
            .cloned();
        family.is_some_and(|family| {
            neighbour.is_some_and(|neighbour| {
                RenderedUnit::digest(&family) != RenderedUnit::digest(&neighbour)
                    && family.semantic_key() != neighbour.semantic_key()
                    && family.identity() != neighbour.identity()
            })
        })
    }));
}
