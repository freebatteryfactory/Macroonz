//! What was actually rendered, and the proof that it is what was planned.
//!
//! # Why a plan and a rendering are two values
//!
//! A plan is made before anything exists. It states what WILL be materialized:
//! under which roles, with which semantic keys, landing where, coming from where,
//! and whose digests will be anchored to what. A rendering is what a renderer
//! actually produced: token trees, their bytes, and the digests over those bytes.
//!
//! Collapsing the two is the defect this module exists to make unrepresentable.
//! A plan that carried its own rendered-byte digest would either be carrying a
//! placeholder or carrying a digest from a rendering that already happened — and
//! in the second case, any later "check" compares the value against itself and
//! passes on every input.
//!
//! # The closure is a reconstruction, not an assertion
//!
//! [`ProjectionClosure::proved`] does not ask the renderer whether it obeyed the
//! plan. It **rebuilds the membership out of the rendered units** — role by role,
//! reading each unit's own semantic key, destination, profile, origin, and
//! recomputed digest — and then compares that reconstruction against the
//! membership the plan declared. Every way the two can disagree is a typed
//! refusal naming the role it disagreed at:
//!
//! - a planned role nothing rendered ([`ClosureIssue::MemberMissing`]);
//! - a rendered role nothing planned ([`ClosureIssue::MemberUnplanned`]);
//! - one role rendered twice ([`ClosureIssue::MemberDuplicated`]);
//! - a rendered unit whose origin is not the planned one
//!   ([`ClosureIssue::OriginOrphan`]);
//! - a digest that is not the digest of the bytes actually rendered, under the
//!   contract the plan stated ([`ClosureIssue::DigestMismatch`]);
//! - a unit standing under the right role and answering to a different semantic
//!   key ([`ClosureIssue::SemanticKeyMismatch`]);
//! - a unit rendered under a profile or to a destination the plan did not name
//!   ([`ClosureIssue::MaterializationMismatch`]);
//! - a role the PLAN ITSELF declared twice ([`ClosureIssue::MemberPlannedTwice`]);
//! - a rebuild that is not the planned membership as a complete SET
//!   ([`ClosureIssue::MembershipDisagreement`]).
//!
//! **Tokens are emitted only FROM a closure.** The closure joins the rendered
//! units in role-roster order, keeps the resulting tree, and commits to its
//! digest inside its own identity — so the exact byte stream a caller emits is
//! part of what was proved rather than something assembled afterwards. Holding a
//! closure is the proof; there is no partial closure and no closure with a
//! warning attached.
//!
//! # The seats
//!
//! `types.rs` declares; its own child `type_guard.rs` takes the digests, owns
//! the join, and builds the proof, which is what keeps every one of those roads
//! unreachable from anywhere else. `prove.rs` is the per-role pass the proof
//! consumes, and `type_contract.rs` states the refusal family and the issue
//! roster's own table.

mod prove;
mod type_contract;
mod types;

pub use types::{
    ClosureIssue, ProjectionClosure, ProjectionClosureRefusal, RenderedProjection, RenderedUnit,
    RenderingRefusal,
};
