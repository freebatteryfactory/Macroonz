//! The proof that what was rendered is what was planned, and the seal that makes it reachable.
//!
//! A closure is the whole agreement between a plan's declared output set and the units a renderer produced, and an expansion is the one value tokens come out of.
//! Every claim below is asked from outside, through the road a consumer walks: a request, a renderer, and whatever the compiler hands back.
//!
//! # Reversals
//!
//! A road that closed over anything would satisfy every positive assertion here, so each is paired with the shape that must refuse: a set the rendering outgrew, a rendering short of the set, a renderer that wrote nothing, and each of the three ways three separately produced values can disagree about their parentage.

mod hostile;

use macroonz_compiler::support::{
    AssemblyIssue, AxisCargo, CargoAxis, DeclaredCargo, DeferredCargo, EXPECTED_SCHEMA_ID,
    ProvedCargo, SupportAssembly, SupportAxes,
};
use macroonz_compiler::{
    BindError, Bounded, CanonicalContent, Closure, ClosureIssue, CrateBinding, Destination,
    Disposition, Door, Expansion, GeneratedToken, GeneratedTree, InvalidationTrigger, Kind,
    Membership, NoQuestions, Observed, Overflow, OwnerFact, OwnerIdentity, PartitionCargo, Phase,
    Plan, PlanDecisions, Producer, RefusalClass, Refused, RenderedProjection, RenderedUnit,
    Request, Role, TextCapture, UNIVERSAL_QUESTION_COUNT, encode_bytes,
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

/// The outside observer's reason for leaving an unrelated support axis empty.
const SUPPORT_AXIS_ABSENT: OwnerFact = OwnerFact {
    home: "lane",
    name: "support-axis-not-part-of-this-reversal",
};

/// A kind whose one seat reaches only the benchmark carrier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BenchCargo;

impl Kind for BenchCargo {
    const NAME: &'static str = "lane.bench-cargo";
    type Content = &'static str;
    type Role = BenchSeat;
    type Question = NoQuestions;
}

/// The one seat the benchmark-cargo fixture fills.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BenchSeat {
    /// Cargo proved for a benchmark target.
    Cargo,
}

impl Role for BenchSeat {
    const ALL: &'static [Self] = &[Self::Cargo];

    fn name(self) -> &'static str {
        "cargo"
    }

    fn destination(self) -> Destination {
        Destination::BenchCarrier
    }
}

/// One generated tree spelling one word.
fn spelled(word: &str) -> Result<GeneratedTree, Overflow> {
    GeneratedTree::assembled(vec![GeneratedToken::word(word)])
}

/// The expansion one lawful request over this source produces, or nothing where a step refused.
fn expansion(source: &str) -> Option<Expansion<Pair>> {
    expansion_rendered(source, "head", "tail")
}

/// One expansion with caller-chosen output bytes, so two terminals can share one declaration root without sharing one closed identity.
fn expansion_rendered(source: &str, head: &str, tail: &str) -> Option<Expansion<Pair>> {
    let read = TextCapture::read(source).ok()?;
    Request::<Pair>::over(read.input().clone(), "pair", &DOOR)
        .render(|_plan, out| {
            out.unit(Seat::Head, spelled(head)?)?;
            out.unit(Seat::Tail, spelled(tail)?)
        })
        .ok()
}

/// The expansion whose one proved delivery belongs to the benchmark carrier.
fn bench_expansion(source: &str) -> Option<Expansion<BenchCargo>> {
    let read = TextCapture::read(source).ok()?;
    Request::<BenchCargo>::over(read.input().clone(), "bench-cargo", &DOOR)
        .render(|_plan, out| out.unit(BenchSeat::Cargo, spelled("bench")?))
        .ok()
}

/// One axis whose absence is explicit and irrelevant to the active reversal.
fn absent_axis<Material>() -> AxisCargo<Material> {
    AxisCargo::Absent {
        because: Disposition::NotApplicable {
            because: SUPPORT_AXIS_ABSENT,
        },
    }
}

/// An address stated for a seat that never publishes refuses at planning, before any identity commits to it.
///
/// Both of this lane's seats deliver to builds rather than to a publication artifact, so an address on either is a claim no act would consume — admitted, it would ride the plan's, the rendering's, and the closure's identities while nothing ever wrote to it.
#[test]
fn an_address_on_an_unpublished_seat_refuses_at_planning() -> Result<(), ()> {
    let read = TextCapture::read(DECLARATION).map_err(|_| ())?;
    let refusal = Request::<Pair>::over(read.input().clone(), "pair", &DOOR)
        .publishing_at(
            Seat::Tail,
            OwnerIdentity {
                subject: "lane.address",
                bytes: [7u8; 32],
            },
        )
        .render(|_plan, out| {
            out.unit(Seat::Head, spelled("head")?)?;
            out.unit(Seat::Tail, spelled("tail")?)
        })
        .err()
        .ok_or(())?;
    assert_eq!(refusal.phase(), Phase::Planning);
    Ok(())
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

/// Declaration-site cargo proved under another declaration cannot enter this assembly's declared axis.
#[test]
fn declared_cargo_from_another_declaration_refuses_the_checked_join() -> Result<(), ()> {
    let stated = expansion(DECLARATION).ok_or(())?;
    let foreign = expansion(OTHER_DECLARATION).ok_or(())?;
    let declared = DeclaredCargo::stamped_from(&foreign, spelled("matcher").map_err(|_| ())?)
        .map_err(|_| ())?;
    let stated_root = stated.plan().account().commitment();
    let foreign_root = foreign.plan().account().commitment();
    let refusal = SupportAssembly::assembled(
        stated_root,
        None,
        SupportAxes {
            declared: AxisCargo::Carried(declared),
            deferred: absent_axis(),
            bench: absent_axis(),
        },
    )
    .err()
    .ok_or(())?;
    assert_eq!(
        refusal.first_issue(),
        &AssemblyIssue::RootsDisagree {
            axis: CargoAxis::Declared,
            stated: stated_root,
            carried: foreign_root,
        }
    );
    Ok(())
}

/// The proving-terminal roster includes declared cargo first, followed by the one occupied deferred form.
///
/// Two terminals stand over the same declaration but render different bytes, so their closed identities make the axis order independently observable.
#[test]
fn assembly_sources_include_declared_parentage_in_axis_order() -> Result<(), ()> {
    let declaring = expansion(DECLARATION).ok_or(())?;
    let testing = expansion_rendered(DECLARATION, "other_head", "other_tail").ok_or(())?;
    assert_ne!(declaring.identity(), testing.identity());
    let declared = DeclaredCargo::stamped_from(&declaring, spelled("matcher").map_err(|_| ())?)
        .map_err(|_| ())?;
    let deferred = DeferredCargo::deferred(testing.test_carrier().tokens().ok_or(())?.clone());
    let proved = ProvedCargo::carried(
        &testing,
        CargoAxis::Deferred,
        Destination::TestCarrier,
        deferred,
    )
    .map_err(|_| ())?;
    let assembly = SupportAssembly::assembled(
        declaring.plan().account().commitment(),
        None,
        SupportAxes {
            declared: AxisCargo::Carried(declared),
            deferred: AxisCargo::Carried(proved),
            bench: absent_axis(),
        },
    )
    .map_err(|_| ())?;
    assert_eq!(
        assembly.sources().collect::<Vec<_>>(),
        [declaring.identity(), testing.identity()]
    );
    Ok(())
}

/// Declared-cargo provenance guards the join without changing the accepted canonical assembly bytes.
///
/// The independent expected encoding writes the outer declaration root and the exact declared matcher/stamped payload, but no second source or root inside the declared axis.
#[test]
fn declared_cargo_parentage_does_not_change_its_canonical_axis_encoding() -> Result<(), ()> {
    let bound = expansion(DECLARATION).ok_or(())?;
    let declared =
        DeclaredCargo::stamped_from(&bound, spelled("matcher").map_err(|_| ())?).map_err(|_| ())?;
    let root = bound.plan().account().commitment();

    let mut expected = Vec::new();
    encode_bytes(root.as_bytes(), &mut expected);
    encode_bytes(EXPECTED_SCHEMA_ID.as_bytes(), &mut expected);
    expected.push(0);

    expected.push(1);
    let mut declared_axis = Vec::new();
    encode_bytes(&declared.matched().canonical_bytes(), &mut declared_axis);
    encode_bytes(&declared.stamped().canonical_bytes(), &mut declared_axis);
    encode_bytes(&declared_axis, &mut expected);

    for _axis in [CargoAxis::Deferred, CargoAxis::Bench] {
        expected.push(0);
        expected.push(1);
        encode_bytes(&SUPPORT_AXIS_ABSENT.citation_bytes(), &mut expected);
    }

    let assembly = SupportAssembly::assembled(
        root,
        None,
        SupportAxes {
            declared: AxisCargo::Carried(declared),
            deferred: absent_axis(),
            bench: absent_axis(),
        },
    )
    .map_err(|_| ())?;
    assert_eq!(assembly.canonical_content_bytes(), expected);
    Ok(())
}

/// Opaque deferred cargo cannot be promoted for the stamped declaration axis, even from that axis's own delivery.
#[test]
fn the_declared_axis_accepts_only_stamped_cargo() -> Result<(), ()> {
    let bound = expansion(DECLARATION).ok_or(())?;
    let cargo = DeferredCargo::deferred(bound.emit().tokens().ok_or(())?.clone());
    let refusal = ProvedCargo::carried(
        &bound,
        CargoAxis::Declared,
        Destination::DeclarationSite,
        cargo,
    )
    .err()
    .ok_or(())?;
    let issue = AssemblyIssue::DeclaredAxisRequiresStampedCargo;
    assert_eq!(refusal.first_issue(), &issue);
    assert_eq!(issue.slot(), 7);
    assert_eq!(issue.canonical_bytes(), [7]);
    assert_eq!(issue.axis(), Some(CargoAxis::Declared));
    assert_eq!(issue.observed(), Observed::ContractDisagreement);
    assert_eq!(refusal.class(), RefusalClass::CarrierNotAssembled);
    Ok(())
}

/// Cargo proved for a test target cannot be reseated in the public benchmark field after promotion.
#[test]
fn proved_test_cargo_cannot_be_reseated_as_benchmark_cargo() -> Result<(), ()> {
    let bound = expansion(DECLARATION).ok_or(())?;
    let cargo = DeferredCargo::deferred(bound.test_carrier().tokens().ok_or(())?.clone());
    let proved = ProvedCargo::carried(&bound, CargoAxis::Deferred, Destination::TestCarrier, cargo)
        .map_err(|_| ())?;
    let root = bound.plan().account().commitment();
    let refusal = SupportAssembly::assembled(
        root,
        None,
        SupportAxes {
            declared: absent_axis(),
            deferred: absent_axis(),
            bench: AxisCargo::Carried(proved),
        },
    )
    .err()
    .ok_or(())?;
    assert_eq!(
        refusal.first_issue(),
        &AssemblyIssue::CargoReachesASecondDestination {
            axis: CargoAxis::Bench,
            destination: Destination::TestCarrier,
        }
    );
    Ok(())
}

/// Cargo proved for a benchmark target cannot be reseated in the public deferred field after promotion.
#[test]
fn proved_benchmark_cargo_cannot_be_reseated_as_test_cargo() -> Result<(), ()> {
    let bound = bench_expansion(OTHER_DECLARATION).ok_or(())?;
    let cargo = DeferredCargo::deferred(bound.bench_carrier().tokens().ok_or(())?.clone());
    let proved = ProvedCargo::carried(&bound, CargoAxis::Bench, Destination::BenchCarrier, cargo)
        .map_err(|_| ())?;
    let root = bound.plan().account().commitment();
    let refusal = SupportAssembly::assembled(
        root,
        None,
        SupportAxes {
            declared: absent_axis(),
            deferred: AxisCargo::Carried(proved),
            bench: absent_axis(),
        },
    )
    .err()
    .ok_or(())?;
    assert_eq!(
        refusal.first_issue(),
        &AssemblyIssue::CargoReachesASecondDestination {
            axis: CargoAxis::Deferred,
            destination: Destination::BenchCarrier,
        }
    );
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
    let smaller = replanned(plan, Membership::from_member(head).map_err(|_| ())?).ok_or(())?;
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
