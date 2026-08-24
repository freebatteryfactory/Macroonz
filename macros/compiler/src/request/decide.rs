//! What one request decides before a token of Rust exists, and the five identities it mints doing so.
//!
//! Pure functions over values their types already inform.
//! Nothing here is handed an identity: the commitment is derived from the bytes the caller walked in with, and every seat's identity hangs off that commitment, so a plan is a deterministic function of the declaration and cannot be told it stands over material it was not planned for.

use super::SELECTION_FACT;
use crate::bounded::{Bounded, Overflow};
use crate::identity::{
    self, Identity, OwnerFact, OwnerIdentity, Profile, Transcript, encode_bytes,
};
use crate::kind::{Kind, Role};
use crate::origin::{
    DecisionTrace, OriginEdge, OriginRelation, OriginTrail, TRACE_ENTRY_LIMIT, TraceDecision,
    TraceEntry,
};
use crate::plan::{
    Account, BoundAxis, Context, DigestContract, InvalidationSet, Membership, Plan, PlanDecisions,
    PlanError, PlanIssue, PlannedMember, PlannedOutput,
};
use crate::token::CapturedInput;

/// Plan one request: the account it stands on, the context it is decided under, one member per declared seat, and the record of why.
///
/// The watch set is handed back beside the plan because the explanation answers with it, and rebuilding it from the plan's own reading would be a second construction of one derived value.
///
/// # Errors
///
/// Returns the planning refusal where the declared dependency set, the watch set, the output set, or the decision trace outgrows its magnitude, and where the kind's roster declares no seat at all.
pub(super) fn planned<K: Kind>(
    capture: &CapturedInput,
    content: K::Content,
    dependencies: Vec<Identity<identity::CapturedDeclaration>>,
    profile: Profile,
    assumptions: &[OwnerFact],
    addresses: &[(K::Role, OwnerIdentity)],
) -> Result<(Plan<K>, InvalidationSet), PlanError> {
    let stands_over = committed(capture);
    let account = Account::standing_on(stands_over, dependencies)?;
    let decided_under = Context::under(profile);
    let invalidation = decided_under.watch_set(&account)?;
    let authored = account.origin_node();
    let membership = membership(stands_over, authored, profile, addresses, named::<K>())?;
    let origin = OriginTrail::from_edge(OriginEdge {
        from: authored,
        relation: OriginRelation::AuthoredDeclaration,
        to: seat_node(stands_over, membership.first().role),
    });
    let trace = trace(traced::<K>(stands_over), assumptions)?;
    let plan = Plan::planned(
        account,
        decided_under,
        content,
        PlanDecisions {
            membership,
            invalidation: invalidation.clone(),
            trace,
            origin,
            nonclaims: Bounded::empty(),
        },
    );
    Ok((plan, invalidation))
}

/// The complete output set: one member per row of the kind's roster, in roster order.
///
/// # Errors
///
/// Returns [`PlanIssue::UnknownKind`] where the roster declares no seat — a kind with nothing to render is a kind this door was handed no implementation of — and the output magnitude where it declares more seats than a plan admits.
fn membership<R: Role>(
    stands_over: Identity<identity::CapturedDeclaration>,
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
        .map(|role| member(stands_over, authored, profile, role, addresses))
        .collect();
    Membership::declared(
        member(stands_over, authored, profile, head, addresses),
        rest,
    )
}

/// One planned member: what the seat's unit will be, where it came from, who renders it, and what its digest must satisfy.
fn member<R: Role>(
    stands_over: Identity<identity::CapturedDeclaration>,
    authored: Identity<identity::OriginNode>,
    profile: Profile,
    role: R,
    addresses: &[(R, OwnerIdentity)],
) -> PlannedMember<R> {
    let key = semantic_key(stands_over, role);
    PlannedMember {
        role,
        output: PlannedOutput {
            semantic_key: key,
            origin: OriginTrail::from_edge(OriginEdge {
                from: authored,
                relation: OriginRelation::SemanticDerivation,
                to: seat_node(stands_over, role),
            }),
            expected_profile: profile,
            address: addressed(role, addresses),
            digest_contract: DigestContract { anchored_to: key },
        },
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
fn committed(capture: &CapturedInput) -> Identity<identity::CapturedDeclaration> {
    Identity::derived(Transcript::rooted(
        identity::Role::CapturedDeclaration,
        &capture.canonical_bytes(),
        0,
    ))
}

/// What one seat's identities are derived over: its declared name, framed.
///
/// Framed rather than raw, which is what keeps a seat named `content` at position zero from deriving the origin node an account already stands at.
fn seat_material<R: Role>(role: R) -> Vec<u8> {
    let mut material = Vec::new();
    encode_bytes(role.name().as_bytes(), &mut material);
    material
}

/// What the unit under one seat IS, independently of any bytes.
fn semantic_key<R: Role>(
    stands_over: Identity<identity::CapturedDeclaration>,
    role: R,
) -> Identity<identity::GeneratedUnit> {
    Identity::derived(Transcript::under_projection(
        identity::Role::GeneratedUnit,
        &stands_over,
        &seat_material(role),
        u32::from(role.slot()),
    ))
}

/// The origin node one seat's unit stands at.
fn seat_node<R: Role>(
    stands_over: Identity<identity::CapturedDeclaration>,
    role: R,
) -> Identity<identity::OriginNode> {
    Identity::derived(Transcript::under_projection(
        identity::Role::OriginNode,
        &stands_over,
        &seat_material(role),
        u32::from(role.slot()),
    ))
}

/// The subject every decision of one request is recorded against.
fn traced<K: Kind>(
    stands_over: Identity<identity::CapturedDeclaration>,
) -> Identity<identity::Traced> {
    let mut material = Vec::new();
    encode_bytes(K::NAME.as_bytes(), &mut material);
    Identity::derived(Transcript::under_projection(
        identity::Role::Plan,
        &stands_over,
        &material,
        0,
    ))
}

/// The kind a request names, by the one fact of a kind that reaches an identity.
fn named<K: Kind>() -> Identity<identity::ProjectionKind> {
    Identity::derived(Transcript::rooted(
        identity::Role::DeclaredName,
        K::NAME.as_bytes(),
        0,
    ))
}
