//! The proof that what was rendered is what was planned, and the seal that makes it reachable.
//!
//! A closure is the whole agreement between a plan's declared output set and the units a renderer produced, and an expansion is the one value tokens come out of.
//! Every claim below is asked from outside, through the road a consumer walks: a request, a renderer, and whatever the compiler hands back.
//!
//! # Reversals
//!
//! A road that closed over anything would satisfy every positive assertion here, so each is paired with the shape that must refuse: a set the rendering outgrew, a rendering short of the set, a renderer that wrote nothing, and each of the three ways three separately produced values can disagree about their parentage.

use macroonz::{
    BindError, Bounded, Closure, ClosureIssue, CrateBinding, Destination, Door, Expansion,
    GeneratedToken, GeneratedTree, InvalidationTrigger, Kind, Membership, NoQuestions, Observed,
    Overflow, PartitionCargo, Phase, Plan, PlanDecisions, Producer, RenderedProjection,
    RenderedUnit, Request, Role, TextCapture, UNIVERSAL_QUESTION_COUNT,
};

/// The kind this lane renders: two seats, delivered to two different builds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pair;

impl Kind for Pair {
    const NAME: &'static str = "lane.pair";
    type Content = &'static str;
    type Role = Seat;
    type Question = NoQuestions;
}

/// The two seats a pair fills.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Seat {
    /// The unit the consumer's normal build compiles.
    Head,
    /// The unit the consumer's test target invokes.
    Tail,
}

impl Role for Seat {
    const ALL: &'static [Self] = &[Self::Head, Self::Tail];

    fn name(self) -> &'static str {
        match self {
            Self::Head => "head",
            Self::Tail => "tail",
        }
    }

    fn destination(self) -> Destination {
        match self {
            Self::Head => Destination::DeclarationSite,
            Self::Tail => Destination::TestCarrier,
        }
    }
}

/// The one value that says who is asking.
const DOOR: Door = Door::declared(
    "lane",
    "lane.pair.grammar",
    "lane::pair",
    CrateBinding::declared("demo"),
    Producer {
        namespace: "lane",
        name: "pair",
    },
);

/// One declared input this lane hands the compiler.
const DECLARATION: &str = "struct Greeting { line: Line }";

/// A second declared input, so a lane needing two expansions has two.
const OTHER_DECLARATION: &str = "struct Farewell { line: Line }";

/// One generated tree spelling one word.
fn spelled(word: &str) -> Result<GeneratedTree, Overflow> {
    GeneratedTree::assembled(vec![GeneratedToken::word(word)])
}

/// The expansion one lawful request over this source produces, or nothing where a step refused.
fn expansion(source: &str) -> Option<Expansion<Pair>> {
    let read = TextCapture::read(source).ok()?;
    Request::<Pair>::over(read.input().clone(), "pair", &DOOR)
        .render(|_plan, out| {
            out.unit(Seat::Head, spelled("head")?)?;
            out.unit(Seat::Tail, spelled("tail")?)
        })
        .ok()
}

/// One rendering of a plan's own members, spelled with the words this lane names.
///
/// The road a renderer takes, reached directly, so this lane can produce a second rendering of one plan without a second request.
fn rendered(plan: &Plan<Pair>, head: &str, tail: &str) -> Option<RenderedProjection<Seat>> {
    let members = plan.membership();
    let units = vec![
        RenderedUnit::materialized(members.under(Seat::Head)?, spelled(head).ok()?).ok()?,
        RenderedUnit::materialized(members.under(Seat::Tail)?, spelled(tail).ok()?).ok()?,
    ];
    RenderedProjection::materialized(units).ok()
}

/// The same plan, re-planned over an output set this lane declares.
///
/// Every other seat is read off the lawful plan and moved across unchanged, so what differs between the two is the declared set and nothing beside it.
fn replanned(plan: &Plan<Pair>, membership: Membership<Seat>) -> Option<Plan<Pair>> {
    let (first, rest) = plan.invalidation().split();
    let invalidation = InvalidationTrigger::watched(*first, rest.to_vec()).ok()?;
    let nonclaims = Bounded::new(plan.nonclaims().to_vec()).ok()?;
    Some(Plan::planned(
        plan.account().clone(),
        *plan.context(),
        *plan.content(),
        PlanDecisions {
            membership,
            invalidation,
            trace: plan.trace().clone(),
            origin: plan.origin().clone(),
            nonclaims,
        },
    ))
}

/// The lawful road binds every required seat and closes over what it rendered.
///
/// Load-bearing in its own right: every refusal below is only evidence because the same road, on an ordinary declaration, produces a bound expansion with each seat occupied.
#[test]
fn the_lawful_road_binds_every_required_seat() -> Result<(), ()> {
    let bound = expansion(DECLARATION).ok_or(())?;
    let declared = bound.plan().membership().count();
    assert_eq!(declared, Seat::ALL.len());
    assert_eq!(bound.closure().rendered().count(), declared);
    assert_eq!(bound.closure().reconstructed().count(), declared);
    assert_eq!(bound.explain().seats(), UNIVERSAL_QUESTION_COUNT);
    assert_eq!(bound.closure().plan(), bound.plan().identity());
    Ok(())
}

/// Each delivery carries what its own seats declared, and a delivery nothing was planned into says so.
///
/// "This delivery receives no cargo" and "this delivery receives a cargo of no tokens" are answers to different questions, and the publication delivery is not joined at all.
#[test]
fn each_delivery_carries_what_its_own_seats_declared() -> Result<(), ()> {
    let bound = expansion(DECLARATION).ok_or(())?;
    let emitted = bound.emit().tokens().ok_or(())?;
    let carried = bound.test_carrier().tokens().ok_or(())?;
    assert_eq!(emitted.inspected().trim(), "head");
    assert_eq!(carried.inspected().trim(), "tail");
    assert!(matches!(
        bound.bench_carrier(),
        PartitionCargo::NothingPlanned
    ));
    assert!(
        bound
            .emission()
            .joined(Destination::PublicationArtifact)
            .is_none()
    );
    assert_eq!(bound.published().count(), 0);
    Ok(())
}

/// A shortened output set leaves a rendered seat unplanned, and the proof names it.
///
/// The repair this reverses hands back the first member of a set whose construction failed, which is a well-formed membership about a smaller claim.
/// The two plans carry two identities, because a plan's transcript commits to the set it declared.
#[test]
fn a_shortened_output_set_leaves_a_rendered_seat_unplanned() -> Result<(), ()> {
    let bound = expansion(DECLARATION).ok_or(())?;
    let plan = bound.plan();
    let head = plan.membership().under(Seat::Head).ok_or(())?.clone();
    let smaller = replanned(plan, Membership::from_member(head)).ok_or(())?;
    assert_ne!(smaller.identity(), plan.identity());

    let refusal = Closure::proved(&smaller, bound.closure().rendered().clone())
        .err()
        .ok_or(())?;
    assert_eq!(
        refusal.first_issue(),
        &ClosureIssue::MemberUnplanned { role: Seat::Tail }
    );
    Ok(())
}

/// A rendering short of the declared set names the seat nothing materialized.
#[test]
fn a_rendering_short_of_the_declared_set_names_the_seat() -> Result<(), ()> {
    let bound = expansion(DECLARATION).ok_or(())?;
    let head = bound
        .closure()
        .rendered()
        .under(Seat::Head)
        .ok_or(())?
        .clone();
    let refusal = Closure::proved(bound.plan(), RenderedProjection::of_one(head))
        .err()
        .ok_or(())?;
    assert_eq!(
        refusal.first_issue(),
        &ClosureIssue::MemberMissing { role: Seat::Tail }
    );
    Ok(())
}

/// The order a renderer happened to produce its units in does not change what was proved.
///
/// The claim walks the kind's roster rather than the rendering's sequence, so two renderings carrying one set of units reach one closure identity — which is what lets a caller compare two proofs without knowing how either renderer was written.
#[test]
fn rendering_order_does_not_change_what_was_proved() -> Result<(), ()> {
    let bound = expansion(DECLARATION).ok_or(())?;
    let plan = bound.plan();
    let head = plan.membership().under(Seat::Head).ok_or(())?;
    let tail = plan.membership().under(Seat::Tail).ok_or(())?;
    let reversed = RenderedProjection::materialized(vec![
        RenderedUnit::materialized(tail, spelled("tail").map_err(|_| ())?).map_err(|_| ())?,
        RenderedUnit::materialized(head, spelled("head").map_err(|_| ())?).map_err(|_| ())?,
    ])
    .map_err(|_| ())?;
    let proved = Closure::proved(plan, reversed).map_err(|_| ())?;
    assert_eq!(proved.identity(), bound.closure().identity());
    Ok(())
}

/// A rendered unit answers for its own bytes, so two units of one derivation are never interchangeable.
///
/// This is the fact every neighbouring-value repair depends on being false: both units are rendered from one declaration under one profile, and they still carry different semantic keys and different digests, because the key is derived over the SEAT and the digest over the bytes that seat produced.
#[test]
fn the_two_rendered_units_are_never_interchangeable() -> Result<(), ()> {
    let bound = expansion(DECLARATION).ok_or(())?;
    let rendering = bound.closure().rendered();
    let head = rendering.under(Seat::Head).ok_or(())?;
    let tail = rendering.under(Seat::Tail).ok_or(())?;
    assert_ne!(head.semantic_key(), tail.semantic_key());
    assert_ne!(head.digest(), tail.digest());
    assert_ne!(head.identity(), tail.identity());
    assert_ne!(head.destination(), tail.destination());
    Ok(())
}

/// A renderer that wrote no unit at all refuses, and the refusal reaches the caller as this door's diagnostic.
#[test]
fn a_renderer_that_wrote_nothing_refuses() -> Result<(), ()> {
    let read = TextCapture::read(DECLARATION).map_err(|_| ())?;
    let refused = Request::<Pair>::over(read.input().clone(), "pair", &DOOR)
        .render(|_plan, _out| Ok(()))
        .err()
        .ok_or(())?;
    assert_eq!(refused.phase(), Phase::Rendering);
    assert_eq!(refused.observed(), Observed::SeatAbsent);
    assert!(refused.summary().starts_with("lane: "));
    Ok(())
}

/// Three values produced separately disagree about their parentage in exactly three places, and each names both identities it holds.
///
/// The type parameter cannot catch any of them: two requests of one kind admit the same seats and the same questions, so a proof or an explanation about the other one is complete, well formed, and about something else.
#[test]
fn three_separately_produced_values_bind_only_where_they_name_one_another() -> Result<(), ()> {
    let first = expansion(DECLARATION).ok_or(())?;
    let second = expansion(OTHER_DECLARATION).ok_or(())?;
    assert_ne!(first.plan().identity(), second.plan().identity());

    let foreign_proof = Expansion::bound(
        first.plan().clone(),
        second.closure().clone(),
        first.explain().clone(),
    )
    .err()
    .ok_or(())?;
    assert_eq!(
        foreign_proof,
        BindError::ClosureProvedAgainstAnotherPlan {
            planned: first.plan().identity(),
            proved: second.plan().identity(),
        }
    );

    let foreign_plan = Expansion::bound(
        first.plan().clone(),
        first.closure().clone(),
        second.explain().clone(),
    )
    .err()
    .ok_or(())?;
    assert_eq!(
        foreign_plan,
        BindError::ExplanationAnsweredOverAnotherPlan {
            planned: first.plan().identity(),
            answered: second.plan().identity(),
        }
    );

    let other_proof = Closure::proved(
        first.plan(),
        rendered(first.plan(), "another head", "another tail").ok_or(())?,
    )
    .map_err(|_| ())?;
    assert_ne!(other_proof.identity(), first.closure().identity());
    let foreign_closure =
        Expansion::bound(first.plan().clone(), other_proof, first.explain().clone())
            .err()
            .ok_or(())?;
    assert!(matches!(
        foreign_closure,
        BindError::ExplanationAnsweredOverAnotherClosure { .. }
    ));
    Ok(())
}
