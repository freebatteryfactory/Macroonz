//! The render home's declarations: the ceiling one rendered unit's bytes stand under, one materialized unit, the whole rendering, the sink a renderer writes into, and how rendering says no.
//!
//! Declarations only.
//! Every road that reaches a private field lives in `type_guard.rs`, this file's own child, which is what makes the digest structural: it is taken there over the tree's own canonical bytes, and no caller supplies one.

use crate::bounded::NonEmpty;
use crate::identity::{self, Identity, OwnerIdentity, Profile};
use crate::kind::{Kind, Role};
use crate::origin::OriginTrail;
use crate::plan::{MEMBERSHIP_LIMIT, Plan};
use crate::token::GeneratedTree;

#[path = "type_guard.rs"]
mod guard;

/// Bytes one rendered unit may carry.
///
/// A renderer that would emit past this refuses rather than materializing part of a unit.
pub const RENDERED_BYTE_LIMIT: usize = 65_536;

/// One unit a renderer actually materialized.
///
/// Its seat and every fact that seat's planned member states are carried, so a proof can rebuild a membership out of a rendering and compare it against the declared one; the tree, the identity, and the digest are the rendering's own.
///
/// # Nonclaims
///
/// The Rust source text is not a member of the unit.
/// It is [`GeneratedTree::inspected`](crate::token::GeneratedTree::inspected) — a projection of the tree, for a person.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedUnit<R: Role> {
    role: R,
    identity: Identity<identity::RenderedUnit>,
    semantic_key: Identity<identity::GeneratedUnit>,
    profile: Profile,
    origin: OriginTrail,
    address: Option<OwnerIdentity>,
    tree: GeneratedTree,
    digest: Identity<identity::OutputBytes>,
}

/// Everything one renderer produced for one plan.
///
/// Structurally non-empty: a rendering that materialized nothing is not a rendering, and no plan can ever close over one.
/// Bounded by the magnitude a plan declares its membership inside, because a rendering wider than any plan could declare has no plan to close over.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedProjection<R: Role> {
    units: NonEmpty<RenderedUnit<R>, MEMBERSHIP_LIMIT>,
}

/// The value a renderer writes its units into.
///
/// It holds the plan, so a renderer names a seat and hands over tokens: everything else one unit carries is that seat's planned member, read here.
/// It holds no proof and makes none — a seat left unfilled and a seat filled twice are written into it as freely as an honest rendering, because those are disagreements between a rendering and a plan and the proof that compares the two is what settles them.
pub struct Output<'plan, K: Kind> {
    plan: &'plan Plan<K>,
    units: Vec<RenderedUnit<K::Role>>,
}

/// How rendering says no.
///
/// One refusal, at the first thing that goes wrong: a unit that cannot be materialized is not a unit, and the units after it were never written.
/// Three rows name a declared magnitude and the two counts that passed it; the other two say a rendering and a plan's seats do not line up at all.
#[must_use = "a rendering refusal names the seat or the magnitude the renderer would have passed"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RenderError {
    /// The renderer wrote no unit at all.
    NothingRendered,
    /// A unit was written under a seat this plan declares no member for.
    ///
    /// Refused where it is written rather than where a proof would notice it, because a unit answers to its member's semantic key and a seat with no member offers none.
    SeatUnplanned {
        /// The seat's declared name.
        role: &'static str,
    },
    /// One unit's canonical bytes pass the declared magnitude.
    BytesUnbounded {
        /// The seat the unit was written under, by its declared name.
        role: &'static str,
        /// The declared bound.
        bound: usize,
        /// The observed count.
        observed: usize,
    },
    /// The rendering carries more units than the declared magnitude admits.
    UnitsUnbounded {
        /// The declared bound.
        bound: usize,
        /// The observed count.
        observed: usize,
    },
    /// A generated tree passed the declared per-level magnitude while a unit was being composed.
    ///
    /// The one overflow a renderer meets: the composition helpers and the tree assembler are what it builds with, and they are the only roads it takes that bound anything.
    TokensUnbounded {
        /// The declared bound.
        bound: usize,
        /// The observed count.
        observed: usize,
    },
}
