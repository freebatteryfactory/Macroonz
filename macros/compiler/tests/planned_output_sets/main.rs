//! What a plan declares, observed from outside the crate that plans it.
//!
//! A plan names the complete output set of one request before a token of Rust exists, and everything a reader can ask about that set is asked here through public roads: which seats it fills, which build each seat's unit lands in, what invalidates it, and what the request MEANT.
//!
//! Nothing below renders anything.
//! Planning is the step under judgement, so every value this lane hands the planner it also authored — the commitments, the origin nodes, the seats, the decisions — and the plan's own answers are the only thing read back.
//!
//! # Reversals
//!
//! A plan that admitted anything would satisfy every positive assertion here, so each is paired with the shape that must refuse: a seat declared twice, a set past its magnitude, an account past its magnitude, and a one-trigger reading of a cause set that names more than one declaration.

use macroonz::identity::{self, Identity, Transcript};
use macroonz::{
    Account, BoundAxis, Bounded, Context, DEPENDENCY_LIMIT, DecisionTrace, Destination,
    DigestContract, InvalidationTrigger, Kind, MEMBERSHIP_LIMIT, Membership, NoQuestions,
    OriginEdge, OriginRelation, OriginTrail, OwnerFact, Plan, PlanDecisions, PlanIssue,
    PlannedMember, PlannedOutput, RUST_DECLARATION_PROFILE, Role, TraceDecision, TraceEntry,
};

/// The kind this lane plans: two seats, delivered to two different builds.
///
/// Two rather than one, because every claim below about which seat holds what is vacuous over a roster of one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pair;

impl Kind for Pair {
    const NAME: &'static str = "lane.pair";
    type Content = &'static str;
    type Role = Seat;
    type Question = NoQuestions;
}

/// A second kind over the same seats, for the one claim that is about the kind's name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Other;

impl Kind for Other {
    const NAME: &'static str = "lane.other";
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

/// One captured-declaration commitment over material this lane names.
fn commitment(material: &[u8]) -> Identity<identity::CapturedDeclaration> {
    Identity::derived(Transcript::rooted(
        identity::Role::CapturedDeclaration,
        material,
        0,
    ))
}

/// One origin node over material this lane names.
fn node(material: &[u8]) -> Identity<identity::OriginNode> {
    Identity::derived(Transcript::rooted(identity::Role::OriginNode, material, 0))
}

/// One generated-unit key over material this lane names.
fn key(material: &[u8]) -> Identity<identity::GeneratedUnit> {
    Identity::derived(Transcript::rooted(
        identity::Role::GeneratedUnit,
        material,
        0,
    ))
}

/// The subject this lane records its decisions against.
fn subject() -> Identity<identity::Traced> {
    Identity::derived(Transcript::rooted(identity::Role::Plan, b"lane", 0))
}

/// One planned member at one seat, over material derived from that seat's own name.
fn member(seat: Seat) -> PlannedMember<Seat> {
    let semantic_key = key(seat.name().as_bytes());
    PlannedMember {
        role: seat,
        output: PlannedOutput {
            semantic_key,
            origin: OriginTrail::from_edge(OriginEdge {
                from: node(b"authored"),
                relation: OriginRelation::AuthoredDeclaration,
                to: node(seat.name().as_bytes()),
            }),
            expected_profile: RUST_DECLARATION_PROFILE,
            address: None,
            digest_contract: DigestContract {
                anchored_to: semantic_key,
            },
        },
    }
}

/// One plan over the account and the output set this lane declares.
fn planned(account: Account<Pair>, membership: Membership<Seat>) -> Option<Plan<Pair>> {
    let decided_under = Context::under(RUST_DECLARATION_PROFILE);
    let invalidation = decided_under.watch_set(&account).ok()?;
    Some(Plan::planned(
        account,
        decided_under,
        "pair",
        PlanDecisions {
            membership,
            invalidation,
            trace: DecisionTrace::from_entry(TraceEntry {
                subject: subject(),
                decision: TraceDecision::SelectedBecause(OwnerFact {
                    home: "lane",
                    name: "the-lane-declares-its-own-set",
                }),
            }),
            origin: OriginTrail::from_edge(OriginEdge {
                from: node(b"authored"),
                relation: OriginRelation::AuthoredDeclaration,
                to: node(b"plan"),
            }),
            nonclaims: Bounded::empty(),
        },
    ))
}

/// A plan's declared set is whatever its declaration admitted, and each seat routes where the seat says.
///
/// The delivery is read off the seat rather than off the member, so a plan cannot disagree with its own roster about which build compiles a unit — and a delivery nothing was declared into answers with zero rather than with nothing.
#[test]
fn a_plans_declared_set_is_whatever_its_declaration_admitted() -> Result<(), ()> {
    let declared =
        Membership::declared(member(Seat::Head), vec![member(Seat::Tail)]).map_err(|_| ())?;
    assert_eq!(declared.count(), Seat::ALL.len());
    assert!(declared.under(Seat::Head).is_some() && declared.under(Seat::Tail).is_some());
    assert_eq!(declared.count_to(Destination::DeclarationSite), 1);
    assert_eq!(declared.count_to(Destination::TestCarrier), 1);
    assert_eq!(declared.count_to(Destination::BenchCarrier), 0);
    Ok(())
}

/// One declared set reaches one plan whichever order it was declared in.
///
/// A declared output set is order-insensitive and the canonical encoding walks the kind's roster rather than the caller's sequence, so this is a claim about the plan's own identity rather than about how the members happen to be held.
#[test]
fn a_declared_set_reaches_one_plan_whichever_order_it_was_declared_in() -> Result<(), ()> {
    let one = Membership::declared(member(Seat::Head), vec![member(Seat::Tail)]).map_err(|_| ())?;
    let other =
        Membership::declared(member(Seat::Tail), vec![member(Seat::Head)]).map_err(|_| ())?;
    let first = planned(Account::over(commitment(b"one declaration")), one).ok_or(())?;
    let second = planned(Account::over(commitment(b"one declaration")), other).ok_or(())?;
    assert_eq!(first.identity(), second.identity());
    Ok(())
}

/// A plan commits to the set it declared.
///
/// The reversal for the test above: two plans over one account and one context, differing only in which seats their set names, are two plans.
#[test]
fn a_plan_commits_to_the_set_it_declared() -> Result<(), ()> {
    let whole =
        Membership::declared(member(Seat::Head), vec![member(Seat::Tail)]).map_err(|_| ())?;
    let shortened = Membership::from_member(member(Seat::Head));
    let first = planned(Account::over(commitment(b"one declaration")), whole).ok_or(())?;
    let second = planned(Account::over(commitment(b"one declaration")), shortened).ok_or(())?;
    assert_ne!(first.identity(), second.identity());
    Ok(())
}

/// Two members standing under one seat refuse at the declaration, naming the seat and the count.
///
/// The check is here rather than downstream because a doubled seat is a defect in the DECLARATION of the set: a proof matches a rendered unit to a planned member by seat, so a membership that reached one doubled would have made that match elect a member and prove nothing about the other.
#[test]
fn two_members_under_one_seat_refuse_at_the_declaration() -> Result<(), ()> {
    let refusal = Membership::declared(member(Seat::Head), vec![member(Seat::Head)])
        .err()
        .ok_or(())?;
    assert_eq!(
        refusal.first_issue(),
        &PlanIssue::MembershipDoubled {
            role_slot: Seat::Head.slot(),
            observed: 2,
        }
    );
    Ok(())
}

/// A declared set past its magnitude names the output axis, and the two counts are the collection's own.
///
/// The magnitude is settled before the doubling walk runs, so a set that is both too wide and doubled reports the width — which is the repair that has to happen first.
#[test]
fn a_declared_set_past_its_magnitude_names_the_output_axis() -> Result<(), ()> {
    let offered = MEMBERSHIP_LIMIT.saturating_add(1);
    let rest = vec![member(Seat::Head); MEMBERSHIP_LIMIT];
    let refusal = Membership::declared(member(Seat::Head), rest)
        .err()
        .ok_or(())?;
    assert_eq!(
        refusal.first_issue(),
        &PlanIssue::BoundExceeded {
            axis: BoundAxis::Outputs,
            bound: u64::try_from(MEMBERSHIP_LIMIT).map_err(|_| ())?,
            observed: u64::try_from(offered).map_err(|_| ())?,
        }
    );
    Ok(())
}

/// An account naming more captures than its magnitude admits names the declaration axis.
///
/// A cause list cut to fit is byte for byte the shape of a complete one, so the account refuses rather than narrating a partial cause.
#[test]
fn an_account_past_the_dependency_magnitude_names_the_declaration_axis() -> Result<(), ()> {
    let offered = DEPENDENCY_LIMIT.saturating_add(1);
    let dependencies: Vec<Identity<identity::CapturedDeclaration>> = (0..offered)
        .map(|position| commitment(&position.to_be_bytes()))
        .collect();
    let refusal = Account::<Pair>::standing_on(commitment(b"one declaration"), dependencies)
        .err()
        .ok_or(())?;
    assert_eq!(
        refusal.first_issue(),
        &PlanIssue::BoundExceeded {
            axis: BoundAxis::Declarations,
            bound: u64::try_from(DEPENDENCY_LIMIT).map_err(|_| ())?,
            observed: u64::try_from(offered).map_err(|_| ())?,
        }
    );
    Ok(())
}

/// One dependency set declared in two orders, with a repeat, reaches one account.
///
/// The set is canonicalized where the account is built, so two callers who declared the same captures reach one plan rather than two that differ by the order somebody wrote them in.
#[test]
fn one_dependency_set_declared_in_two_orders_reaches_one_account() -> Result<(), ()> {
    let stands_over = commitment(b"one declaration");
    let first = commitment(b"first dependency");
    let second = commitment(b"second dependency");
    let one = Account::<Pair>::standing_on(stands_over, vec![first, second]).map_err(|_| ())?;
    let other =
        Account::<Pair>::standing_on(stands_over, vec![second, first, second]).map_err(|_| ())?;
    assert_eq!(one, other);
    assert_eq!(one.dependencies().len(), 2);
    Ok(())
}

/// The narrow one-trigger reading refuses where the account names more than one declaration.
///
/// A watch covering the commitment and none of the dependencies reads exactly like a complete one, so the reading refuses rather than issuing a claim about the declarations it dropped.
#[test]
fn a_one_trigger_reading_refuses_where_the_account_names_more_than_one() -> Result<(), ()> {
    let stands_over = commitment(b"one declaration");
    let alone = Account::<Pair>::over(stands_over);
    assert!(alone.cause_trigger().is_ok());

    let standing_on = Account::<Pair>::standing_on(stands_over, vec![commitment(b"a dependency")])
        .map_err(|_| ())?;
    let refusal = standing_on.cause_trigger().err().ok_or(())?;
    assert_eq!(
        refusal.first_issue(),
        &PlanIssue::CauseSetUnwatchable {
            named: 2,
            watchable: 1,
        }
    );
    Ok(())
}

/// The watch set watches every declaration the account names, and the two facts the context declares.
///
/// The shared half of any plan's invalidation is derived from the context's own seats rather than listed at a plan site, so a context that grew a seat and a watch set that did not cannot drift apart.
#[test]
fn the_watch_set_watches_every_declaration_the_account_names() -> Result<(), ()> {
    let stands_over = commitment(b"one declaration");
    let account = Account::<Pair>::standing_on(
        stands_over,
        vec![commitment(b"first"), commitment(b"second")],
    )
    .map_err(|_| ())?;
    let watched = Context::under(RUST_DECLARATION_PROFILE)
        .watch_set(&account)
        .map_err(|_| ())?;
    let named: Vec<&InvalidationTrigger> = watched
        .iter()
        .filter(|trigger| matches!(trigger, InvalidationTrigger::CapturedDeclaration { .. }))
        .collect();
    assert_eq!(named.len(), 3);
    assert_eq!(
        watched
            .iter()
            .filter(|trigger| matches!(trigger, InvalidationTrigger::Profile { .. }))
            .count(),
        1
    );
    assert_eq!(
        watched
            .iter()
            .filter(|trigger| matches!(trigger, InvalidationTrigger::Generator { .. }))
            .count(),
        1
    );
    Ok(())
}

/// What a request MEANT is its kind's name over its content commitment, and nothing else.
///
/// The layer exists so two distinct requests may agree at it, so the claim needs both directions: what a request stands on does not change what it meant, and the kind and the content each do.
#[test]
fn what_a_request_meant_is_its_kind_over_its_content_and_nothing_else() -> Result<(), ()> {
    let stands_over = commitment(b"one declaration");
    let alone = Account::<Pair>::over(stands_over);
    let standing_on = Account::<Pair>::standing_on(stands_over, vec![commitment(b"a dependency")])
        .map_err(|_| ())?;
    assert_eq!(alone.intent(), standing_on.intent());

    let another_kind = Account::<Other>::over(stands_over);
    assert_ne!(alone.intent(), another_kind.intent());

    let another_declaration = Account::<Pair>::over(commitment(b"another declaration"));
    assert_ne!(alone.intent(), another_declaration.intent());
    Ok(())
}
