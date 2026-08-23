//! Each repair the failure-path law forbids is restored HERE, by this plane, and
//! the value it produces is shown to be about something else while the live road
//! refuses instead.
//!
//! # Ownership
//!
//! The law says a failed required seat is never repaired with an empty, default,
//! or neighbouring value. A law of that shape is only evidence if the repair can
//! be performed and seen to be wrong. The services carry none of them — that is
//! the whole point — so the plane performs each one itself, out of the public
//! values a lawful compilation hands back, and states what the repair produces
//! beside what the road actually does.
//!
//! Two directions, always. The lawful road binds every required seat and closes;
//! the restored repair produces a well-formed, complete-looking value about a
//! different subject, and the seam it would have filled refuses with a typed
//! cause that names the seat.
//!
//! # The compiler's half
//!
//! Deleting a seat outright — a closed expansion without a closure, a rendering
//! off the membership-only draft, a closure nobody proved, a join outside the
//! proof —
//! does not compile at all, and the compile-fail fixtures beside this file carry
//! that half. This file carries the seats that die by REFUSAL rather than by the
//! compiler, which is the half a type cannot state.
//!
//! Every declaration below is written here, beside the source text handed to the
//! services, and nothing asks the services what they decided.

use threadpak::types::{Bounded, ConstLimit};
use threadpak_macroc::derive_refusal::diagnose;
use threadpak_macroc::mutation_descriptor::MutationDeclarationCause;
use threadpak_macroc::plane::HumanTextLimit;
use threadpak_macroc::{
    AccountedExpansion, AssemblyIssue, ClosureIssue, DeriveImplProjection,
    ExplanationBindingRefusal, ExplanationSeat, HumanProjection, MachineAnchoring, MacrocPhase,
    ObservedClassification, PlanDecisions, PlannedMembership, ProjectionClosure, ProjectionPlan,
    ProjectionPlanningIssue, RefusalCompileContext, RefusalDeriveCapture, RefusalFamilyExpansion,
    RefusalOwnerFacts, RelatedIdentity, RenderedImplementation, RenderedProjection, RenderedRole,
    RenderedUnit, ReproductionRoute, SoleRenderedUnit, TextCapture, TextCompileRefusal,
    TrialDeclarationPosture, MutationDeclarationPosture, compile_declaration,
    compile_refusal_text,
};

/// The declaration handed to the services: a single-cause family, whose shape fixes its family and cause-order production members.
const DECLARATION: &str = "#[refusal(family = \"testpak.demo\", shape = single_cause, \
    order(NotCanonical = \"not-canonical\", NotAdmitted = \"not-admitted\", \
    Unbounded = \"unbounded\"))] enum DemoFamily { NotAdmitted, Unbounded, NotCanonical, }";

/// A declaration naming a shape word the machine's roster does not admit. It is
/// refused at capture, which is the earliest seat on the road.
const SHAPE_NOT_ADMITTED: &str = "#[refusal(family = \"testpak.demo\", shape = tri_state)] \
    enum DemoFamily { NotAdmitted, }";

const VALID_MUTATION_BODY: &str = r#"
    support = demo_mutations,
    module = generated_demo_mutations,
    family = named("testpak", "demo-evaluation"),
    map declared_order = named("testpak", "demo-order"),
    permit named("testpak", "demo-order") = ["declared-order-permutation"],
"#;

/// One lawful refusal-family expansion, or nothing where the road refused.
fn lawful() -> Result<RefusalFamilyExpansion, ()> {
    compile_refusal_text(DECLARATION)
        .map(|(_, closed)| closed)
        .map_err(|_| ())
}

/// Whether the callable door refused under this exact capture cause and classification.
fn refused_as(source: &str, cause: RefusalDeriveCapture, observed: ObservedClassification) -> bool {
    matches!(compile_refusal_text(source), Err(TextCompileRefusal::Refused(carried)) if {
        let (_, diagnostic) = &*carried;
        diagnostic.observed == observed
            && diagnostic.summary.shown().contains(cause.described())
    })
}

/// Whether the callable door refused under this exact mutation-helper cause and classification.
fn mutation_refused_as(
    source: &str,
    cause: MutationDeclarationCause,
    observed: ObservedClassification,
) -> bool {
    matches!(compile_refusal_text(source), Err(TextCompileRefusal::Refused(carried)) if {
        let (_, diagnostic) = &*carried;
        diagnostic.observed == observed
            && diagnostic.summary.shown().contains(cause.described())
    })
}

/// Compile the declaration and its generated-support carrier through the public callable road.
fn delivered(source: &str) -> Result<AccountedExpansion<RefusalFamilyExpansion>, ()> {
    let read = TextCapture::read(source).map_err(|_| ())?;
    let context = RefusalCompileContext {
        spans: read.spans().clone(),
        machine: MachineAnchoring::UnmintedAtThisSeam,
        owner_facts: RefusalOwnerFacts::declared(),
        nonclaims: Bounded::empty(),
    };
    compile_declaration(read.input(), &context).map_err(|_| ())
}

/// Compile one combined trial-and-mutation declaration whose mutation claim is the supplied stem.
fn delivered_with_trials(
    claim: &str,
) -> Result<AccountedExpansion<RefusalFamilyExpansion>, ()> {
    const TEMPLATE: &str = r#"
        #[refusal(family = "testpak.demo", shape = single_cause, order(A = "a", B = "b"))]
        #[threadpak_trials(
            support = demo_trials,
            module = generated_demo_trials,
            table = named("testpak", "demo-trials"),
            suite construction = named("testpak", "construction") {
                observes_a {
                    claim = named("testpak", "demo-order"),
                    roles = [named("testpak", "regression")],
                    tags = [named("testpak", "generated")],
                    subject = named("testpak", "demo-subject"),
                    check = named("testpak", "demo-check"),
                    population = named("testpak", "demo-population"),
                },
            },
        )]
        #[threadpak_mutations(
            module = generated_demo_mutations,
            family = named("testpak", "demo-evaluation"),
            map declared_order = named("testpak", "$claim"),
            permit named("testpak", "$claim") = ["declared-order-permutation"],
        )]
        enum DemoFamily { A, B }
    "#;
    delivered(&TEMPLATE.replace("$claim", claim))
}

/// One single-cause refusal declaration carrying the supplied mutation-helper body.
fn mutation_source(body: &str) -> String {
    format!(
        "#[refusal(family = \"testpak.demo\", shape = single_cause, \
         order(A = \"a\", B = \"b\"))] #[threadpak_mutations({body})] \
         enum DemoFamily {{ A, B }}"
    )
}

/// One plan with a substituted membership, re-planned through the same public
/// road the lawful one walked.
///
/// # Why the repair is made HERE
///
/// The two seats below restore a repair that produces a membership nobody
/// declared. That repair used to be performable at the CLOSURE, because `proved`
/// took a plan identity beside a loose membership and the two were separable —
/// which is exactly the hole this road no longer has. A membership now reaches a
/// proof only inside the plan that declares it, so the repair is made where a
/// membership actually lives.
///
/// Every other seat is read off the lawful plan and moved across unchanged, so
/// what differs between the two plans is the declared output set and nothing
/// beside it. The second plan derives its own identity, because a plan's
/// transcript commits to its membership — which is the same fact the old
/// separable pair could hide.
fn replanned(
    plan: &ProjectionPlan<DeriveImplProjection>,
    membership: PlannedMembership<RenderedImplementation>,
) -> Result<ProjectionPlan<DeriveImplProjection>, ()> {
    ProjectionPlan::planned(
        plan.account().clone(),
        plan.context().clone(),
        plan.content().clone(),
        PlanDecisions {
            membership,
            invalidation: plan.invalidation().clone(),
            trace: plan.trace().clone(),
            origin: plan.origin().clone(),
            nonclaims: plan.nonclaims().clone(),
        },
    )
    .map_err(|_| ())
}

/// The identities one diagnostic's related set carries, in order.
fn related(diagnostic: &threadpak_macroc::MacrocDiagnostic) -> Vec<RelatedIdentity> {
    diagnostic.related.carried().iter().copied().collect()
}

/// One related identity as the level it states and the bytes it is, so two of
/// them sort and compare without this plane deciding that a body and an issue
/// sharing bytes are one value.
fn level_and_bytes(identity: RelatedIdentity) -> (u8, [u8; 32]) {
    match identity {
        RelatedIdentity::Body(body) => (0, *body.as_bytes()),
        RelatedIdentity::Issue(issue) => (1, *issue.as_bytes()),
    }
}

/// The lawful road binds every required seat and closes over what it rendered.
///
/// Load-bearing in its own right. Every refusal below is only evidence because
/// the same road, on a lawful declaration, produces a bound expansion with each
/// seat occupied — a road that refused everything would satisfy every hostile
/// assertion here and be worthless.
#[test]
fn the_lawful_road_binds_every_required_seat() {
    assert!(lawful().is_ok_and(|closed| {
        let denominator = closed.plan().membership().count();
        denominator > 0
            && closed.rendered().count() == denominator
            && closed.closure().reconstructed().count() == denominator
            && closed.explanation().len() == 9
            && closed
                .emitted()
                .tokens()
                .is_some_and(|tree| tree.tokens().next().is_some())
    }));
}

/// The shortened-complete-set repair, restored: it produces a membership that is
/// honest about a plan nobody declared.
///
/// The repair takes a complete set whose construction failed,
/// and hands back the first member alone.
/// The mutant below builds exactly that value —
/// a one-member membership over a declaration whose shape fixes four —
/// and it is well-formed, complete-looking, and about a smaller claim.
///
/// The closure refuses to close the real rendering over the PLAN that declares
/// it, naming the role the smaller claim dropped — and the two plans carry two
/// identities, because a plan's transcript commits to its membership.
#[test]
fn a_shortened_complete_set_proves_a_smaller_claim() -> Result<(), ()> {
    let closed = lawful()?;
    let member = closed
        .plan()
        .membership()
        .under(RenderedImplementation::RenderedFamilyImpl)
        .cloned()
        .ok_or(())?;
    let shortened = PlannedMembership::complete(member, []);
    assert!(shortened.count() == 1 && closed.plan().membership().count() > shortened.count());
    let smaller = replanned(closed.plan(), shortened)?;
    assert_ne!(smaller.identity(), closed.plan().identity());
    let refused = ProjectionClosure::proved(&smaller, closed.closure().rendered().clone())
        .err()
        .ok_or(())?;
    assert_eq!(
        *refused.body().carried().first(),
        ClosureIssue::MemberUnplanned {
            role: RenderedImplementation::RenderedCauseOrderImpl,
        }
    );
    Ok(())
}

/// The neighbouring-digest repair, restored: the first rendered unit's digest is
/// a digest of the wrong bytes.
///
/// The repair answers the explanation's output-and-digest seat with the FIRST
/// rendered unit's digest, whatever it was a digest of.
///
/// The mutant below builds a rendering whose first unit is the neighbour — a
/// rendering the renderer is free to produce, since role order is declared and
/// rendering order is not — and the digest that repair would read is not the
/// family role's.
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
    let closed_earlier =
        ProjectionClosure::proved(closed.plan(), RenderedProjection::of_one(neighbour));
    assert!(closed_earlier.is_err_and(|refusal| {
        *refusal.body().carried().first()
            == ClosureIssue::MemberMissing {
                role: RenderedImplementation::RenderedFamilyImpl,
            }
    }));
    Ok(())
}

/// Every explanation seat that can fail to bind names itself, and the three name
/// three different things.
///
/// The three seats stand for three different neighbouring-value repairs: the
/// first planned member whatever its role, the first rendered unit's digest
/// whatever it was a digest of, and a hardcoded owner fact nobody's plan cited.
///
/// A caller repairing a derivation needs to know which of the three failed, so
/// the refusal is typed per seat and the distinction survives into the
/// diagnostic: each projects under `SeatAbsent`, each names its own seat in the
/// line, and each derives its own related identity.
#[test]
fn each_explanation_seat_refuses_under_its_own_name() {
    let seats = [
        ExplanationSeat::PlannedFamilyMember,
        ExplanationSeat::ProvedFamilyDigest,
        ExplanationSeat::DeclaredAssumption,
    ];
    let mut identities: Vec<RelatedIdentity> = Vec::new();
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
    identities.sort_unstable_by_key(|identity| level_and_bytes(*identity));
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
    let twice = replanned(closed.plan(), doubled)?;
    let closure = ProjectionClosure::proved(&twice, RenderedProjection::of_one(unit))
        .err()
        .ok_or(())?;
    assert!(
        *planning.body().carried().first()
            == ProjectionPlanningIssue::MembershipDoubled {
                role_slot: RenderedImplementation::RenderedFamilyImpl.slot(),
                observed: 2,
            }
            && *closure.body().carried().first()
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
/// Five steps of the road refuse in five vocabularies, and none of them
/// collapses into a shared sentence under one classification with an empty
/// related set.
///
/// A closure that dropped a role and an explanation that could not bind its
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
            ProjectionClosure::proved(closed.plan(), RenderedProjection::of_one(unit)).is_err_and(
                |refusal| {
                    let from_closure = diagnose::closure_refused(&refusal);
                    let from_explanation = diagnose::explanation_refused(
                        &ExplanationBindingRefusal::RequiredOutputAbsent {
                            seat: ExplanationSeat::ProvedFamilyDigest,
                        },
                    );
                    matches!(from_closure.observed, ObservedClassification::SeatAbsent)
                        && matches!(from_explanation.observed, ObservedClassification::SeatAbsent)
                        && from_closure.summary != from_explanation.summary
                        && related(&from_closure) != related(&from_explanation)
                        // The closure's line names the role it was established
                        // at, which is the distinction a shared sentence lost
                        // first.
                        && from_closure
                            .summary
                            .shown()
                            .contains(RenderedImplementation::RenderedCauseOrderImpl.described())
                },
            )
        })
    }));
}

/// The empty-projection repair, restored: an over-long rendering refuses rather
/// than becoming a blank one.
///
/// The boundary is read from its owner.
/// One value at it is admitted and carries its whole length; one byte over it refuses.
/// Neither answer is an empty projection, which is what the repair would produce.
#[test]
fn an_over_long_projection_refuses_rather_than_blanking() {
    let fitting = "x".repeat(HumanTextLimit::MAX);
    let admitted = HumanProjection::<HumanTextLimit>::projected(&fitting);
    assert!(admitted.is_ok_and(|projection| {
        projection.len() == HumanTextLimit::MAX && !projection.is_empty()
    }));

    let oversized = "x".repeat(HumanTextLimit::MAX.saturating_add(1));
    assert!(HumanProjection::<HumanTextLimit>::projected(&oversized).is_err());
}

/// A refused declaration reaches a caller as a complete diagnostic, and every
/// seat a diagnostic owes is occupied.
///
/// The last required seat on the list is the diagnostic's own. A refusal that
/// arrived without its phase, its classification, its repair, or its
/// reproduction route would be a complaint rather than an answer.
///
/// The reproduction route in particular is what makes the callable road a road
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

/// The refusal helper is a closed grammar: unknown, malformed, repeated, and multiply declared clauses never disappear behind a successful projection.
#[test]
fn the_refusal_helper_consumes_every_owned_token_once() {
    let unknown = "#[refusal(family = \"testpak.demo\", shape = single_cause, invented = value)] \
        enum DemoFamily { NotAdmitted, }";
    assert!(refused_as(
        unknown,
        RefusalDeriveCapture::NotADeclarableClause,
        ObservedClassification::ContractDisagreement,
    ));

    let malformed = "#[refusal(family = \"testpak.demo\", shape single_cause)] \
        enum DemoFamily { NotAdmitted, }";
    assert!(refused_as(
        malformed,
        RefusalDeriveCapture::NotAClause,
        ObservedClassification::ContractDisagreement,
    ));

    let repeated_clause = "#[refusal(family = \"testpak.demo\", shape = single_cause, \
        shape = single_cause)] enum DemoFamily { NotAdmitted, }";
    assert!(refused_as(
        repeated_clause,
        RefusalDeriveCapture::NotDeclaredOnce,
        ObservedClassification::IdentityDisagreement,
    ));

    let repeated_helper = "#[refusal(family = \"testpak.demo\", shape = single_cause)] \
        #[refusal(family = \"testpak.demo\", shape = single_cause)] \
        enum DemoFamily { NotAdmitted, }";
    assert!(refused_as(
        repeated_helper,
        RefusalDeriveCapture::NotDeclaredOnce,
        ObservedClassification::IdentityDisagreement,
    ));

    let leading_comma = "#[refusal(, family = \"testpak.demo\", shape = single_cause)] \
        enum DemoFamily { NotAdmitted, }";
    assert!(refused_as(
        leading_comma,
        RefusalDeriveCapture::NotAClause,
        ObservedClassification::ContractDisagreement,
    ));

    let doubled_comma = "#[refusal(family = \"testpak.demo\",, shape = single_cause)] \
        enum DemoFamily { NotAdmitted, }";
    assert!(refused_as(
        doubled_comma,
        RefusalDeriveCapture::NotAClause,
        ObservedClassification::ContractDisagreement,
    ));
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

/// The mutation helper is a closed grammar, and support has exactly one author across the trial and mutation readings.
#[test]
fn the_mutation_helper_consumes_every_owned_token_once() {
    let unknown = r#"
        #[refusal(family = "testpak.demo", shape = single_cause, order(A = "a", B = "b"))]
        #[threadpak_mutations(
            support = demo_mutations,
            module = generated_demo_mutations,
            family = named("testpak", "demo-evaluation"),
            invented = value,
            map declared_order = named("testpak", "demo-order"),
            permit named("testpak", "demo-order") = ["declared-order-permutation"],
        )]
        enum DemoFamily { A, B }
    "#;
    assert!(mutation_refused_as(
        unknown,
        MutationDeclarationCause::NotADeclarableClause,
        ObservedClassification::ContractDisagreement,
    ));

    let duplicate = r#"
        #[refusal(family = "testpak.demo", shape = single_cause, order(A = "a", B = "b"))]
        #[threadpak_mutations(
            support = demo_mutations,
            module = generated_demo_mutations,
            module = generated_demo_mutations_again,
            family = named("testpak", "demo-evaluation"),
            map declared_order = named("testpak", "demo-order"),
            permit named("testpak", "demo-order") = ["declared-order-permutation"],
        )]
        enum DemoFamily { A, B }
    "#;
    assert!(mutation_refused_as(
        duplicate,
        MutationDeclarationCause::NotDistinct,
        ObservedClassification::IdentityDisagreement,
    ));

    let missing_support = r#"
        #[refusal(family = "testpak.demo", shape = single_cause, order(A = "a", B = "b"))]
        #[threadpak_mutations(
            module = generated_demo_mutations,
            family = named("testpak", "demo-evaluation"),
            map declared_order = named("testpak", "demo-order"),
            permit named("testpak", "demo-order") = ["declared-order-permutation"],
        )]
        enum DemoFamily { A, B }
    "#;
    assert!(mutation_refused_as(
        missing_support,
        MutationDeclarationCause::SupportNotDeclared,
        ObservedClassification::SeatAbsent,
    ));

    let doubled_support = r#"
        #[refusal(family = "testpak.demo", shape = single_cause, order(A = "a", B = "b"))]
        #[threadpak_trials(
            support = demo_trials,
            module = generated_demo_trials,
            table = named("testpak", "demo-trials"),
            suite construction = named("testpak", "construction") {
                observes_a {
                    claim = named("testpak", "demo-order"),
                    roles = [named("testpak", "regression")],
                    tags = [named("testpak", "generated")],
                    subject = named("testpak", "demo-subject"),
                    check = named("testpak", "demo-check"),
                    population = named("testpak", "demo-population"),
                },
            },
        )]
        #[threadpak_mutations(
            support = demo_mutations,
            module = generated_demo_mutations,
            family = named("testpak", "demo-evaluation"),
            map declared_order = named("testpak", "demo-order"),
            permit named("testpak", "demo-order") = ["declared-order-permutation"],
        )]
        enum DemoFamily { A, B }
    "#;
    assert!(mutation_refused_as(
        doubled_support,
        MutationDeclarationCause::SupportAlreadyDeclared,
        ObservedClassification::IdentityDisagreement,
    ));

    let duplicate_helper = format!(
        "#[refusal(family = \"testpak.demo\", shape = single_cause, \
         order(A = \"a\", B = \"b\"))] \
         #[threadpak_mutations({VALID_MUTATION_BODY})] \
         #[threadpak_mutations({VALID_MUTATION_BODY})] enum DemoFamily {{ A, B }}"
    );
    assert!(mutation_refused_as(
        &duplicate_helper,
        MutationDeclarationCause::NotDeclaredOnce,
        ObservedClassification::IdentityDisagreement,
    ));

    let no_body = "#[refusal(family = \"testpak.demo\", shape = single_cause, \
        order(A = \"a\", B = \"b\"))] #[threadpak_mutations] enum DemoFamily { A, B }";
    assert!(mutation_refused_as(
        no_body,
        MutationDeclarationCause::NotBodied,
        ObservedClassification::SeatAbsent,
    ));
    let outer_tail = format!(
        "#[refusal(family = \"testpak.demo\", shape = single_cause, \
         order(A = \"a\", B = \"b\"))] \
         #[threadpak_mutations({VALID_MUTATION_BODY}) trailing] enum DemoFamily {{ A, B }}"
    );
    assert!(mutation_refused_as(
        &outer_tail,
        MutationDeclarationCause::NotAClause,
        ObservedClassification::ContractDisagreement,
    ));

    let cases = [
        (
            mutation_source(&format!(",{VALID_MUTATION_BODY}")),
            MutationDeclarationCause::NotAClause,
            ObservedClassification::ContractDisagreement,
        ),
        (
            mutation_source(
                r#"
                    support = demo_mutations,
                    family = named("testpak", "demo-evaluation"),
                    map declared_order = named("testpak", "demo-order"),
                    permit named("testpak", "demo-order") = ["declared-order-permutation"],
                "#,
            ),
            MutationDeclarationCause::NotCovered,
            ObservedClassification::SeatAbsent,
        ),
        (
            mutation_source(
                r#"
                    support = demo_mutations,
                    module = generated_demo_mutations,
                    family = "demo-evaluation",
                    map declared_order = named("testpak", "demo-order"),
                    permit named("testpak", "demo-order") = ["declared-order-permutation"],
                "#,
            ),
            MutationDeclarationCause::NotANamedReference,
            ObservedClassification::ContractDisagreement,
        ),
        (
            mutation_source(&VALID_MUTATION_BODY.replace("map declared_order =", "map declared_order")),
            MutationDeclarationCause::NotAMapping,
            ObservedClassification::ContractDisagreement,
        ),
        (
            mutation_source(&VALID_MUTATION_BODY.replace("map declared_order", "map invented")),
            MutationDeclarationCause::UnknownOwnerFact,
            ObservedClassification::ContractDisagreement,
        ),
        (
            mutation_source(&VALID_MUTATION_BODY.replace(
                "map declared_order = named(\"testpak\", \"demo-order\"),",
                "map declared_order = named(\"testpak\", \"demo-order\"), \
                 map declared_order = named(\"testpak\", \"other-order\"),",
            )),
            MutationDeclarationCause::DuplicateOwnerFact,
            ObservedClassification::IdentityDisagreement,
        ),
        (
            mutation_source(&VALID_MUTATION_BODY.replace(
                "permit named(\"testpak\", \"demo-order\") =",
                "permit named(\"testpak\", \"demo-order\")",
            )),
            MutationDeclarationCause::NotAPermission,
            ObservedClassification::ContractDisagreement,
        ),
        (
            mutation_source(&VALID_MUTATION_BODY.replace(
                "[\"declared-order-permutation\"],",
                "[\"declared-order-permutation\"], \
                 permit named(\"testpak\", \"demo-order\") = [\"declared-order-permutation\"],",
            )),
            MutationDeclarationCause::DuplicatePermissionClaim,
            ObservedClassification::IdentityDisagreement,
        ),
        (
            mutation_source(&VALID_MUTATION_BODY.replace(
                "[\"declared-order-permutation\"]",
                "[]",
            )),
            MutationDeclarationCause::EmptyOperatorFamilies,
            ObservedClassification::ContractDisagreement,
        ),
        (
            mutation_source(&VALID_MUTATION_BODY.replace(
                "[\"declared-order-permutation\"]",
                "[\"declared-order-permutation\", \"declared-order-permutation\"]",
            )),
            MutationDeclarationCause::DuplicateOperatorFamily,
            ObservedClassification::IdentityDisagreement,
        ),
        (
            mutation_source(&VALID_MUTATION_BODY.replace(
                "declared-order-permutation",
                "invented-operator",
            )),
            MutationDeclarationCause::UnknownOperatorFamily,
            ObservedClassification::ContractDisagreement,
        ),
    ];
    for (source, cause, observed) in cases {
        assert!(mutation_refused_as(&source, cause, observed), "{cause:?}");
    }

    let unavailable_fact = format!(
        "#[refusal(family = \"testpak.demo\", shape = issue_collection)] \
         #[threadpak_mutations({VALID_MUTATION_BODY})] enum DemoFamily {{ A, B }}"
    );
    assert!(mutation_refused_as(
        &unavailable_fact,
        MutationDeclarationCause::OwnerFactNotAvailable,
        ObservedClassification::ContractDisagreement,
    ));

    let module_collision = r#"
        #[refusal(family = "testpak.demo", shape = single_cause, order(A = "a", B = "b"))]
        #[threadpak_trials(
            support = demo_trials,
            module = generated_demo,
            table = named("testpak", "demo-trials"),
            suite construction = named("testpak", "construction") {
                observes_a {
                    claim = named("testpak", "demo-order"),
                    roles = [named("testpak", "regression")],
                    tags = [named("testpak", "generated")],
                    subject = named("testpak", "demo-subject"),
                    check = named("testpak", "demo-check"),
                    population = named("testpak", "demo-population"),
                },
            },
        )]
        #[threadpak_mutations(
            module = generated_demo,
            family = named("testpak", "demo-evaluation"),
            map declared_order = named("testpak", "demo-order"),
            permit named("testpak", "demo-order") = ["declared-order-permutation"],
        )]
        enum DemoFamily { A, B }
    "#;
    assert!(mutation_refused_as(
        module_collision,
        MutationDeclarationCause::ModuleAlreadyDeclared,
        ObservedClassification::IdentityDisagreement,
    ));
}

/// Mutation declaration meaning changes only the mutation member and the carrier that delivers it.
#[test]
fn mutation_commitment_moves_only_its_member_and_delivery() -> Result<(), ()> {
    let first = r#"
        #[refusal(family = "testpak.demo", shape = single_cause, order(A = "a", B = "b"))]
        #[threadpak_mutations(
            support = demo_mutations,
            module = generated_demo_mutations,
            family = named("testpak", "demo-evaluation"),
            map declared_order = named("testpak", "first-order-claim"),
            permit named("testpak", "first-order-claim") = ["declared-order-permutation"],
        )]
        enum DemoFamily { A, B }
    "#;
    let second = r#"
        #[refusal(family = "testpak.demo", shape = single_cause, order(A = "a", B = "b"))]
        #[threadpak_mutations(
            support = demo_mutations,
            module = generated_demo_mutations,
            family = named("testpak", "demo-evaluation"),
            map declared_order = named("testpak", "second-order-claim"),
            permit named("testpak", "second-order-claim") = ["declared-order-permutation"],
        )]
        enum DemoFamily { A, B }
    "#;
    let first = delivered(first)?;
    let second = delivered(second)?;
    let first_projected = first.joined().projected();
    let second_projected = second.joined().projected();

    assert_eq!(first_projected.surface().identity(), second_projected.surface().identity());
    assert_eq!(
        first_projected.surface().documentation_identity(),
        second_projected.surface().documentation_identity(),
    );
    assert_eq!(
        first_projected.plan().account().commitment(),
        second_projected.plan().account().commitment(),
    );
    assert_ne!(
        first_projected.plan().account().addressing(),
        second_projected.plan().account().addressing(),
    );

    for role in [
        RenderedImplementation::RenderedFamilyImpl,
        RenderedImplementation::RenderedCauseOrderImpl,
    ] {
        let first_unit = first_projected.rendered().under(role).ok_or(())?;
        let second_unit = second_projected.rendered().under(role).ok_or(())?;
        assert_eq!(first_unit.semantic_key(), second_unit.semantic_key());
        assert_eq!(first_unit.identity(), second_unit.identity());
        assert_eq!(first_unit.digest(), second_unit.digest());
        assert_eq!(first_unit.tree().canonical_bytes(), second_unit.tree().canonical_bytes());
    }
    let mutation = RenderedImplementation::RenderedMutationEvaluation;
    let first_mutation = first_projected.rendered().under(mutation).ok_or(())?;
    let second_mutation = second_projected.rendered().under(mutation).ok_or(())?;
    assert_ne!(first_mutation.semantic_key(), second_mutation.semantic_key());
    assert_ne!(first_mutation.identity(), second_mutation.identity());
    assert_ne!(first_mutation.digest(), second_mutation.digest());
    assert_ne!(
        first_mutation.tree().canonical_bytes(),
        second_mutation.tree().canonical_bytes(),
    );

    let first_carrier = first.joined().carrier();
    let second_carrier = second.joined().carrier();
    assert_ne!(
        first_carrier.plan().account().addressing(),
        second_carrier.plan().account().addressing(),
    );
    let first_unit = first_carrier
        .closure()
        .rendered()
        .under(SoleRenderedUnit::Sole)
        .ok_or(())?;
    let second_unit = second_carrier
        .closure()
        .rendered()
        .under(SoleRenderedUnit::Sole)
        .ok_or(())?;
    assert_ne!(first_unit.semantic_key(), second_unit.semantic_key());
    assert_ne!(first_unit.identity(), second_unit.identity());
    assert_ne!(first_unit.digest(), second_unit.digest());
    Ok(())
}

/// A mutation-policy edit leaves the independent trial reading fixed while moving the mutation member and shared carrier.
#[test]
fn trial_and_mutation_commitments_remain_independent_where_both_are_delivered(
) -> Result<(), ()> {
    let first = delivered_with_trials("first-order-claim")?;
    let second = delivered_with_trials("second-order-claim")?;
    let first_projected = first.joined().projected();
    let second_projected = second.joined().projected();

    assert_eq!(first_projected.surface().identity(), second_projected.surface().identity());
    assert_eq!(
        first_projected.surface().documentation_identity(),
        second_projected.surface().documentation_identity(),
    );
    let (
        TrialDeclarationPosture::Declared(first_trials),
        TrialDeclarationPosture::Declared(second_trials),
    ) = (
        first_projected.surface().trials(),
        second_projected.surface().trials(),
    ) else {
        return Err(());
    };
    assert_eq!(first_trials.commitment(), second_trials.commitment());
    assert_eq!(first_trials.payload(), second_trials.payload());

    let (
        MutationDeclarationPosture::Declared(first_mutations),
        MutationDeclarationPosture::Declared(second_mutations),
    ) = (
        first_projected.surface().mutations(),
        second_projected.surface().mutations(),
    ) else {
        return Err(());
    };
    assert_ne!(first_mutations.commitment(), second_mutations.commitment());

    for role in [
        RenderedImplementation::RenderedFamilyImpl,
        RenderedImplementation::RenderedCauseOrderImpl,
    ] {
        let first_unit = first_projected.rendered().under(role).ok_or(())?;
        let second_unit = second_projected.rendered().under(role).ok_or(())?;
        assert_eq!(first_unit.identity(), second_unit.identity());
        assert_eq!(first_unit.tree().canonical_bytes(), second_unit.tree().canonical_bytes());
    }
    let mutation = RenderedImplementation::RenderedMutationEvaluation;
    let first_mutation = first_projected.rendered().under(mutation).ok_or(())?;
    let second_mutation = second_projected.rendered().under(mutation).ok_or(())?;
    assert_ne!(first_mutation.identity(), second_mutation.identity());
    assert_ne!(
        first_mutation.tree().canonical_bytes(),
        second_mutation.tree().canonical_bytes(),
    );
    assert_eq!(
        first.joined().assembly().trial(),
        second.joined().assembly().trial(),
    );
    assert_ne!(
        first.joined().carrier().plan().account().addressing(),
        second.joined().carrier().plan().account().addressing(),
    );
    Ok(())
}

/// Same-semantic mutation deliveries from two helper readings cannot cross at either assembly join.
#[test]
fn mutation_dependencies_refuse_crossed_delivery_joins() -> Result<(), ()> {
    let first = r#"
        #[refusal(family = "testpak.demo", shape = single_cause, order(A = "a", B = "b"))]
        #[threadpak_mutations(
            support = demo_mutations,
            module = generated_demo_mutations,
            family = named("testpak", "demo-evaluation"),
            map declared_order = named("testpak", "first-order-claim"),
            permit named("testpak", "first-order-claim") = ["declared-order-permutation"],
        )]
        enum DemoFamily { A, B }
    "#;
    let second = r#"
        #[refusal(family = "testpak.demo", shape = single_cause, order(A = "a", B = "b"))]
        #[threadpak_mutations(
            support = demo_mutations,
            module = generated_demo_mutations,
            family = named("testpak", "demo-evaluation"),
            map declared_order = named("testpak", "second-order-claim"),
            permit named("testpak", "second-order-claim") = ["declared-order-permutation"],
        )]
        enum DemoFamily { A, B }
    "#;
    let first = delivered(first)?;
    let second = delivered(second)?;
    let second_draft = second.joined().projected().surface().clone().planned();

    let refusal = threadpak_macroc::derive_refusal::assembly(
        &second_draft,
        first.joined().projected().expansion(),
    )
    .err()
    .ok_or(())?;
    assert!(matches!(
        refusal,
        threadpak_macroc::derive_refusal::CarrierRoadRefusal::Assembled(body)
            if body.body().carried().iter().any(|issue| matches!(
                issue,
                AssemblyIssue::RootsDisagree { .. }
                    | AssemblyIssue::CarrierRootIsNotTheAssemblys { .. }
            ))
    ));

    let second_plan = threadpak_macroc::derive_refusal::carrier_plan(&second_draft)
        .map_err(|_| ())?;
    let refusal = threadpak_macroc::derive_refusal::carrier_expansion(
        &second_draft,
        second_plan,
        first.joined().assembly(),
    )
    .err()
    .ok_or(())?;
    assert!(matches!(
        refusal,
        threadpak_macroc::derive_refusal::CarrierRoadRefusal::Composed(body)
            if matches!(
                &*body,
                threadpak_macroc::ShellComposition::NotOneDeclarations(body)
                    if body.body().carried().iter().any(|issue| matches!(
                        issue,
                        AssemblyIssue::CarrierRootIsNotTheAssemblys { .. }
                    ))
            )
    ));
    Ok(())
}
