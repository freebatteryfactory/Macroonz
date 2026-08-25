//! What one request decides before a token of Rust exists, and the identity chain it mints doing so.
//!
//! Pure functions over values their types already inform.
//! No caller supplies the primary capture, kind, content, member, or plan identity: each is derived from the informed values this road receives.
//! Dependency captures and publication addresses cross as typed citations because their owners are independent declarations, and they never substitute for an identity this request mints.

use super::SELECTION_FACT;
use crate::bounded::{Bounded, Overflow};
use crate::diagnostic::Door;
use crate::identity::{
    self, Identity, OwnerFact, OwnerIdentity, Profile, Transcript, encode_bytes,
};
use crate::kind::{Destination, Kind, Role};
use crate::origin::{
    DecisionTrace, OriginEdge, OriginRelation, OriginTrail, TRACE_ENTRY_LIMIT, TraceDecision,
    TraceEntry,
};
use crate::plan::{
    Account, BoundAxis, ContentBinding, Context, DigestContract, Membership, Plan, PlanDecisions,
    PlanError, PlanIssue, PlannedMember, PlannedOutput,
};
use crate::request::Producer;
use crate::token::CapturedInput;

/// Plan one request: the account it stands on, the context it is decided under, one member per declared seat, and the record of why.
///
/// The watch set travels inside the plan and nowhere beside it: the plan owns the value, and every later reading — the explanation's included — is read off that one seat, so no second copy exists for a later normalization to disagree with.
///
/// # Errors
///
/// Returns the planning refusal where the declared dependency set, the watch set, the output set, or the decision trace outgrows its magnitude, where the kind's roster declares no seat at all, and one [`PlanIssue::AddressInert`] per stated address whose seat no publication act consumes.
pub(super) fn planned<K: Kind>(
    capture: &CapturedInput,
    content: K::Content,
    door: &Door,
    dependencies: Vec<Identity<identity::CapturedDeclaration>>,
    profile: Profile,
    assumptions: &[OwnerFact],
    addresses: &[(K::Role, OwnerIdentity)],
) -> Result<Plan<K>, PlanError> {
    consumable::<K::Role>(addresses)?;
    let account = Account::standing_on(bound_content(capture, content, door), dependencies)?;
    let stands_over = account.commitment();
    let content_commitment = account.content_commitment();
    let kind = account.kind();
    let decided_under = Context::under(profile);
    let invalidation = decided_under.watch_set(&account)?;
    let authored = account.origin_node();
    let membership = membership(
        stands_over,
        content_commitment,
        authored,
        profile,
        addresses,
        kind,
    )?;
    let origin = OriginTrail::from_edge(OriginEdge {
        from: authored,
        relation: OriginRelation::AuthoredDeclaration,
        to: seat_node(
            kind,
            content_commitment,
            stands_over,
            membership.first().role,
        ),
    });
    let trace = trace(traced(kind, content_commitment, stands_over), assumptions)?;
    Ok(Plan::planned(
        account,
        decided_under,
        PlanDecisions {
            membership,
            invalidation,
            trace,
            origin,
            nonclaims: Bounded::empty(),
        },
    ))
}

/// The complete output set: one member per row of the kind's roster, in roster order.
///
/// # Errors
///
/// Returns [`PlanIssue::UnknownKind`] where the roster declares no seat — a kind with nothing to render is a kind this door was handed no implementation of — and the output magnitude where it declares more seats than a plan admits.
fn membership<R: Role>(
    stands_over: Identity<identity::CapturedDeclaration>,
    content: Identity<identity::ProjectionContent>,
    authored: Identity<identity::OriginNode>,
    profile: Profile,
    addresses: &[(R, OwnerIdentity)],
    kind: Identity<identity::ProjectionKind>,
) -> Result<Membership<R>, PlanError> {
    let mut seats = R::ALL.iter().copied();
    let Some(head) = seats.next() else {
        return Err(PlanError::of(PlanIssue::UnknownKind { named: kind }));
    };
    let rest = seats
        .map(|role| {
            member(
                kind,
                content,
                stands_over,
                authored,
                profile,
                role,
                addresses,
            )
        })
        .collect();
    Membership::declared(
        member(
            kind,
            content,
            stands_over,
            authored,
            profile,
            head,
            addresses,
        ),
        rest,
    )
}

/// One planned member: what the seat's unit will be, where it came from, who renders it, and what its digest must satisfy.
fn member<R: Role>(
    kind: Identity<identity::ProjectionKind>,
    content: Identity<identity::ProjectionContent>,
    stands_over: Identity<identity::CapturedDeclaration>,
    authored: Identity<identity::OriginNode>,
    profile: Profile,
    role: R,
    addresses: &[(R, OwnerIdentity)],
) -> PlannedMember<R> {
    let key = semantic_key(kind, content, stands_over, role);
    PlannedMember {
        role,
        output: PlannedOutput {
            semantic_key: key,
            origin: OriginTrail::from_edge(OriginEdge {
                from: authored,
                relation: OriginRelation::SemanticDerivation,
                to: seat_node(kind, content, stands_over, role),
            }),
            expected_profile: profile,
            address: addressed(role, addresses),
            digest_contract: DigestContract { anchored_to: key },
        },
    }
}

/// Whether every stated address names a seat some publication act will consume.
///
/// An address enters the plan's, the rendering's, and the closure's identities, so one that nothing consumes is not loose metadata — it is a claim with no act.
/// A seat consumes an address only where the roster declares it and its delivery is a publication artifact; an address stated anywhere else refuses here, before any identity commits to it.
///
/// # Errors
///
/// Returns one [`PlanIssue::AddressInert`] per address whose seat never publishes.
fn consumable<R: Role>(addresses: &[(R, OwnerIdentity)]) -> Result<(), PlanError> {
    let mut inert = addresses
        .iter()
        .filter(|(seat, _)| {
            !R::ALL.contains(seat) || seat.destination() != Destination::PublicationArtifact
        })
        .map(|(seat, _)| PlanIssue::AddressInert { seat: seat.name() });
    match inert.next() {
        Some(issue) => Err(PlanError::over(issue, inert.collect())),
        None => Ok(()),
    }
}

/// The address a seat's unit is written to, where the caller stated one.
fn addressed<R: Role>(role: R, addresses: &[(R, OwnerIdentity)]) -> Option<OwnerIdentity> {
    addresses
        .iter()
        .find(|(seat, _)| *seat == role)
        .map(|(_, address)| *address)
}

/// The decisions that produced the plan: this home's selection rule, then every fact the caller says the projection rests on.
///
/// # Errors
///
/// Returns the planning refusal naming [`BoundAxis::TraceEntries`] where the assumed facts outrun what one trace records.
fn trace(
    subject: Identity<identity::Traced>,
    assumptions: &[OwnerFact],
) -> Result<DecisionTrace, PlanError> {
    let mut entries = vec![TraceEntry {
        subject,
        decision: TraceDecision::SelectedBecause(SELECTION_FACT),
    }];
    entries.extend(assumptions.iter().map(|fact| TraceEntry {
        subject,
        decision: TraceDecision::SelectedBecause(*fact),
    }));
    let offered = entries.len();
    DecisionTrace::recorded(entries).map_err(|_| {
        PlanError::bounded(
            BoundAxis::TraceEntries,
            Overflow {
                capacity: TRACE_ENTRY_LIMIT,
                offered,
            },
        )
    })
}

/// The identity of the material one request walked in with.
///
/// Over the capture's own canonical bytes exactly as they were handed over: a consumer that names a narrower reading of its declaration hands the narrower capture.
///
/// # Authority
///
/// **This is the one derivation of a captured declaration's identity**, and it is public for exactly one further caller: a door stating a request's DEPENDENCIES hands over the identities of the further captures it read content from, and those identities must be this derivation over those captures — a second spelling of the rule beside this one would agree until one of them was edited.
#[must_use]
pub fn committed(capture: &CapturedInput) -> Identity<identity::CapturedDeclaration> {
    Identity::derived(Transcript::rooted(
        identity::Role::CapturedDeclaration,
        &capture.canonical_bytes(),
        0,
    ))
}

/// What one seat's identities are derived over: the owner-qualified kind, the content commitment, and the seat's own name, each framed.
///
/// Framed rather than raw, which is what keeps a seat named `content` at position zero from deriving the origin node an account already stands at.
/// The owner-qualified kind identity is an ancestor on purpose: roles are open and [`SoleRole`](crate::kind::SoleRole) is reusable by any one-unit kind, so two kinds sharing one capture and one roster would otherwise share a semantic key — and if their bytes agreed, a rendered-unit identity too — while the public contract calls them different generation kinds.
fn seat_material<R: Role>(
    kind: Identity<identity::ProjectionKind>,
    content: Identity<identity::ProjectionContent>,
    role: R,
) -> Vec<u8> {
    let mut material = Vec::new();
    encode_bytes(kind.as_bytes(), &mut material);
    encode_bytes(content.as_bytes(), &mut material);
    encode_bytes(role.name().as_bytes(), &mut material);
    material
}

/// What the unit under one seat IS, independently of any bytes.
fn semantic_key<R: Role>(
    kind: Identity<identity::ProjectionKind>,
    content: Identity<identity::ProjectionContent>,
    stands_over: Identity<identity::CapturedDeclaration>,
    role: R,
) -> Identity<identity::GeneratedUnit> {
    Identity::derived(Transcript::under_projection(
        identity::Role::GeneratedUnit,
        &stands_over,
        &seat_material(kind, content, role),
        u32::from(role.slot()),
    ))
}

/// The origin node one seat's unit stands at.
fn seat_node<R: Role>(
    kind: Identity<identity::ProjectionKind>,
    content: Identity<identity::ProjectionContent>,
    stands_over: Identity<identity::CapturedDeclaration>,
    role: R,
) -> Identity<identity::OriginNode> {
    Identity::derived(Transcript::under_projection(
        identity::Role::OriginNode,
        &stands_over,
        &seat_material(kind, content, role),
        u32::from(role.slot()),
    ))
}

/// The subject every decision of one request is recorded against.
fn traced(
    kind: Identity<identity::ProjectionKind>,
    content: Identity<identity::ProjectionContent>,
    stands_over: Identity<identity::CapturedDeclaration>,
) -> Identity<identity::Traced> {
    let mut material = Vec::new();
    encode_bytes(kind.as_bytes(), &mut material);
    encode_bytes(content.as_bytes(), &mut material);
    Identity::derived(Transcript::under_projection(
        identity::Role::Plan,
        &stands_over,
        &material,
        0,
    ))
}

/// The kind a request names, by the one fact of a kind that reaches an identity.
fn named<K: Kind>(producer: Producer) -> Identity<identity::ProjectionKind> {
    let mut material = Vec::new();
    encode_bytes(producer.namespace.as_bytes(), &mut material);
    encode_bytes(producer.name.as_bytes(), &mut material);
    encode_bytes(K::NAME.as_bytes(), &mut material);
    Identity::derived(Transcript::rooted(
        identity::Role::ProjectionKind,
        &material,
        0,
    ))
}

/// Bind one kind's content to the exact captured declaration and door-qualified kind it was presented under.
pub fn bound_content<K: Kind>(
    capture: &CapturedInput,
    content: K::Content,
    door: &Door,
) -> ContentBinding<K> {
    ContentBinding::bound(committed(capture), named::<K>(door.producer()), content)
}
