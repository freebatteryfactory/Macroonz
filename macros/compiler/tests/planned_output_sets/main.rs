//! What a plan declares, observed from outside the crate that plans it.
//!
//! A plan names the complete output set of one request before a token of Rust exists, and everything a reader can ask about that set is asked here through public roads: which seats it fills, which build each seat's unit lands in, what invalidates it, and what the request MEANT.
//!
//! Nothing below renders anything.
//! Planning is the step under judgement, so every value this lane hands the planner it also authored — the commitments, the origin nodes, the seats, the decisions — and the plan's own answers are the only thing read back.
//!
//! # Reversals
//!
//! A plan that admitted anything would satisfy every positive assertion here, so each is paired with the shape that must refuse: a seat declared twice, a seat outside the kind's declared roster, a set past its magnitude, an account past its magnitude, and a one-trigger reading of a cause set that names more than one declaration.

use macroonz::identity::{self, Identity, Transcript};
use macroonz::{
    Account, BoundAxis, Bounded, ContentBinding, Context, CrateBinding, DEPENDENCY_LIMIT,
    DecisionTrace, Destination, DigestContract, Door, InvalidationTrigger, Kind, MEMBERSHIP_LIMIT,
    Membership, NoQuestions, OriginEdge, OriginRelation, OriginTrail, OwnerFact, Plan,
    PlanDecisions, PlanIssue, PlannedMember, PlannedOutput, Producer, RUST_DECLARATION_PROFILE,
    Role, TextCapture, TraceDecision, TraceEntry, bound_content,
};

const DOOR: Door = Door::declared(
    "lane",
    "lane grammar",
    "lane::planned",
    CrateBinding::declared("macroonz"),
    Producer {
        namespace: "lane",
        name: "planned-output-sets",
    },
);

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

/// One content binding over captured text this lane authored.
fn binding<K: Kind>(material: &str, content: K::Content) -> Option<ContentBinding<K>> {
    let read = TextCapture::read(material).ok()?;
    Some(bound_content::<K>(read.input(), content, &DOOR))
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

/// A role type whose lawful values outnumber its declared roster: `Head` is in `ALL`, `Ghost` is not.
///
/// This is the shape the open [`Role`] contract permits any external kind to write, and the shape membership admission must refuse — a `Ghost` member would be held, rendered, and dropped from every roster-quantified walk.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Haunted {
    /// The one seat the roster declares.
    Head,
    /// A lawful value the roster omits.
    Ghost,
}

impl Role for Haunted {
    const ALL: &'static [Self] = &[Self::Head];

    fn name(self) -> &'static str {
        match self {
            Self::Head => "head",
            Self::Ghost => "ghost",
        }
    }

    fn destination(self) -> Destination {
        match self {
            Self::Head => Destination::DeclarationSite,
            Self::Ghost => Destination::TestCarrier,
        }
    }
}

/// One planned member at one haunted seat, over material derived from that seat's own name.
fn haunted_member(seat: Haunted) -> PlannedMember<Haunted> {
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
    let first = planned(
        Account::over(binding::<Pair>("one declaration", "pair").ok_or(())?),
        one,
    )
    .ok_or(())?;
    let second = planned(
        Account::over(binding::<Pair>("one declaration", "pair").ok_or(())?),
        other,
    )
    .ok_or(())?;
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
    let shortened = Membership::from_member(member(Seat::Head)).map_err(|_| ())?;
    let first = planned(
        Account::over(binding::<Pair>("one declaration", "pair").ok_or(())?),
        whole,
    )
    .ok_or(())?;
    let second = planned(
        Account::over(binding::<Pair>("one declaration", "pair").ok_or(())?),
        shortened,
    )
    .ok_or(())?;
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

/// A member whose seat the roster does not declare refuses at the declaration, naming the seat.
///
/// The roster is the denominator of every downstream walk, so a member outside it would render and then vanish from the encoding, the proof, and the delivery — a closure would prove a set it never examined whole.
/// This lane's role type carries a lawful value its own `ALL` omits, which is exactly the shape an open roster permits an external kind to write.
#[test]
fn a_member_outside_the_roster_refuses_at_the_declaration() -> Result<(), ()> {
    let refusal = Membership::declared(
        haunted_member(Haunted::Head),
        vec![haunted_member(Haunted::Ghost)],
    )
    .err()
    .ok_or(())?;
    assert_eq!(
        refusal.first_issue(),
        &PlanIssue::MembershipForeign { seat: "ghost" }
    );
    Ok(())
}

/// The one-member road refuses the same foreign seat, so no membership holding one is constructible at all.
#[test]
fn a_one_member_set_refuses_a_seat_outside_the_roster() -> Result<(), ()> {
    let refusal = Membership::from_member(haunted_member(Haunted::Ghost))
        .err()
        .ok_or(())?;
    assert_eq!(
        refusal.first_issue(),
        &PlanIssue::MembershipForeign { seat: "ghost" }
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
    let refusal = Account::<Pair>::standing_on(
        binding::<Pair>("one declaration", "pair").ok_or(())?,
        dependencies,
    )
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
    let stands_over = binding::<Pair>("one declaration", "pair").ok_or(())?;
    let first = commitment(b"first dependency");
    let second = commitment(b"second dependency");
    let one =
        Account::<Pair>::standing_on(stands_over.clone(), vec![first, second]).map_err(|_| ())?;
    let other =
        Account::<Pair>::standing_on(stands_over, vec![second, first, second]).map_err(|_| ())?;
    assert_eq!(one, other);
    assert_eq!(one.dependencies().len(), 2);
    Ok(())
}

/// The narrow one-trigger reading covers an account with no dependencies and refuses where another capture is named beside it.
///
/// The content commitment is already bound under the account's own captured declaration, so restating that capture would add no cause; a dependency is an independent cause and cannot be dropped.
#[test]
fn a_one_trigger_reading_refuses_where_the_account_names_an_independent_dependency()
-> Result<(), ()> {
    let stands_over = binding::<Pair>("one declaration", "pair").ok_or(())?;
    let alone = Account::<Pair>::over(stands_over.clone());
    assert!(matches!(
        alone.cause_trigger().map_err(|_| ())?,
        InvalidationTrigger::ProjectionContent { .. }
    ));

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

/// The watch set watches the content binding, every independent declaration it depends on, and the two facts the context declares.
///
/// The shared half of any plan's invalidation is derived from the context's own seats rather than listed at a plan site, so a context that grew a seat and a watch set that did not cannot drift apart.
#[test]
fn the_watch_set_watches_the_content_binding_and_every_declared_dependency() -> Result<(), ()> {
    let stands_over = binding::<Pair>("one declaration", "pair").ok_or(())?;
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
    assert_eq!(named.len(), 2);
    assert_eq!(
        watched
            .iter()
            .filter(|trigger| matches!(trigger, InvalidationTrigger::ProjectionContent { .. }))
            .count(),
        1
    );
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
    let stands_over = binding::<Pair>("one declaration", "pair").ok_or(())?;
    let alone = Account::<Pair>::over(stands_over.clone());
    let standing_on = Account::<Pair>::standing_on(stands_over, vec![commitment(b"a dependency")])
        .map_err(|_| ())?;
    assert_eq!(alone.intent(), standing_on.intent());

    let another_kind =
        Account::<Other>::over(binding::<Other>("one declaration", "pair").ok_or(())?);
    assert_ne!(alone.intent(), another_kind.intent());

    let another_declaration =
        Account::<Pair>::over(binding::<Pair>("another declaration", "pair").ok_or(())?);
    assert_ne!(alone.intent(), another_declaration.intent());
    Ok(())
}
