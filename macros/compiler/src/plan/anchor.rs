//! What a plan hangs off, what its account therefore watches, and where that account's content stands in the origin graph.
//!
//! Three readings of one value, so a plan's anchor, its triggers, and its origin edges cannot disagree about what the request walked in with.
//!
//! # Bounds
//!
//! An anchor names ONE address, and that is a spelling rule: what the content stands on is written into the material the identity is derived over, so two plans over different dependency sets reach different identities whatever they anchor at.
//! A watch naming one address out of several would be a claim about the others, which is why the watch reading covers the whole account and the narrow one-trigger reading refuses instead of electing.

use super::{Account, InvalidationTrigger, PlanError, PlanIssue};
use crate::identity::{self, Anchoring, Identity, Transcript};
use crate::kind::Kind;
use crate::origin::{OriginEdge, OriginRelation};

/// The material every origin node over a request's content is derived from, beside the content's own commitment at the anchor.
///
/// A declared constant rather than a per-call spelling: the node one piece of content stands at must not depend on which reading asked for it, or one piece of content would stand at two nodes and the graph would carry two answers to one question.
const CONTENT_NODE: &[u8] = b"content";

/// The origin node one content address stands at.
fn content_node(
    commitment: Identity<identity::CapturedDeclaration>,
) -> Identity<identity::OriginNode> {
    Identity::derived(Transcript::under_projection(
        identity::Role::OriginNode,
        &commitment,
        CONTENT_NODE,
        0,
    ))
}

impl<K: Kind> Account<K> {
    /// What a transcript derived over this account hangs off.
    ///
    /// Read off the account's own commitment, so a plan's anchor and a plan's stated cause are one fact rather than two that could disagree.
    #[must_use]
    pub fn anchoring(&self) -> Anchoring {
        Anchoring::UnderProjection(*self.commitment().as_bytes())
    }

    /// The origin node this account's content stands at.
    ///
    /// Derived from the commitment alone, so content reached from two directions is one node: what a plan is over, and what another content declares it stands on, are the same thing named the same way.
    #[must_use]
    pub fn origin_node(&self) -> Identity<identity::OriginNode> {
        content_node(self.commitment())
    }

    /// The origin edges this account contributes: one per declared dependency, each running from what the content stands on to the content itself.
    ///
    /// The relation is [`OriginRelation::ExplicitLink`] because that is what happened — an author supplied this dependency set at the door.
    /// It is no semantic derivation: nothing here derived meaning from a dependency.
    ///
    /// # Ordering
    ///
    /// The edges are a FAN-IN and not a walk: every one of them ends at [`Account::origin_node`], so consecutive edges do not join and the set is not a trail.
    /// A caller draws trails through these edges one at a time rather than handing the set to a trail constructor, which would refuse the discontinuity — correctly.
    #[must_use]
    pub fn dependency_edges(&self) -> Vec<OriginEdge> {
        let to = self.origin_node();
        self.dependencies()
            .iter()
            .map(|dependency| OriginEdge {
                from: content_node(*dependency),
                relation: OriginRelation::ExplicitLink,
                to,
            })
            .collect()
    }

    /// The triggers that watch this account's commitment and every dependency it declares.
    #[must_use]
    pub fn cause_triggers(&self) -> Vec<InvalidationTrigger> {
        let (first, rest) = self.caused_by();
        core::iter::once(first).chain(rest).collect()
    }

    /// The single trigger that watches this account's own content.
    ///
    /// The deliberately narrow reading, for a caller that can carry one trigger and no more.
    ///
    /// # Errors
    ///
    /// Returns [`PlanIssue::CauseSetUnwatchable`] where the account also names dependencies: one trigger cannot state that cause set, and a watch covering the commitment alone would read exactly like a complete one.
    pub fn cause_trigger(&self) -> Result<InvalidationTrigger, PlanError> {
        let (first, rest) = self.caused_by();
        if rest.is_empty() {
            return Ok(first);
        }
        Err(PlanError::of(PlanIssue::CauseSetUnwatchable {
            named: u32::try_from(rest.len().saturating_add(1)).unwrap_or(u32::MAX),
            watchable: 1,
        }))
    }

    /// The commitment's trigger, and one per declared dependency beside it.
    ///
    /// The one spelling both readings above take, and the one the shared watch derivation opens with.
    pub(super) fn caused_by(&self) -> (InvalidationTrigger, Vec<InvalidationTrigger>) {
        (
            InvalidationTrigger::CapturedDeclaration {
                watched: self.commitment(),
            },
            self.dependencies()
                .iter()
                .map(|watched| InvalidationTrigger::CapturedDeclaration { watched: *watched })
                .collect(),
        )
    }
}
