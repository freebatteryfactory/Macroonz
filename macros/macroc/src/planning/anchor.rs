//! What a plan hangs off, what it therefore watches, and where its content
//! stands in the origin graph.
//!
//! All three answers are read off the ONE entry account and the context's own
//! typed postures, never chosen at a call site.
//! A plan over content the machine minted a fragment identity for anchors on
//! that fragment and watches it; a plan over captured token material anchors on
//! the capture and watches THAT.
//! Neither posture is dressed up as the other, and neither is an absence, so an
//! expansion-time plan states its own footing instead of borrowing a linked
//! artifact that does not exist yet.
//!
//! [`ProjectionContext::watch_set`] is the one road to a shared watch set, and it
//! reads the context's own seats and the account it is handed rather than a
//! roster listed beside them.
//! It destructures the context exhaustively, so a seat added to
//! [`ProjectionContext`] stops compiling here until somebody decides whether it
//! is a dependency key; it reads the account through the account's own readings,
//! so the dependency set has exactly one holder; and it deduplicates exact
//! triggers while preserving distinct identities carried by the same variant.
//!
//! # Bounds
//!
//! A watch set covers the identities the SHARED CONTEXT carries and the
//! commitments the ENTRY ACCOUNT names, which is what a shared derivation is
//! entitled to know about.
//! It does not cover the anchors a kind supplies beside its context: those are
//! named at the kind's own plan site, where the compiler forces each one to be
//! accounted for.
//! They carry no trigger because the trigger roster declares no seat any of them
//! could be watched through — every roster seat is one thirty-two-byte identity
//! of a declared kind — and minting a kind per anchor would rebuild a
//! hand-maintained roster one level down and push the set past the cardinality
//! that bounds it.
//!
//! One seat inside the context is a named boundary for the same reason and is
//! stated at its binding below: a profile VERSION is not an identity, and every
//! trigger watches one identity.
//!
//! An account names one commitment and up to the declared dependency magnitude
//! beside it. The complete watch set carries one trigger per distinct identity,
//! so no dependency is elected or silently omitted. The separate
//! [`OwnerContentAccount::cause_trigger`] reader remains the deliberately narrow
//! one-trigger view and refuses accounts that cannot be represented by one
//! trigger.
//!
//! # The origin footing
//!
//! The same account answers the origin-graph question, and it answers it from
//! the same seats: the node one piece of content stands at is derived from its
//! commitment alone, so one piece of content is one node wherever it is read —
//! as the content a plan is over, or as a commitment some other content declares
//! it stands on.
//! That is the reason the node's material is a declared constant rather than a
//! per-call spelling.

use super::{
    CauseAnchoring, ContentAddressing, GraphAnchoring, InvalidationSet, InvalidationTrigger,
    OwnerContentAccount, ProjectionContext, ProjectionKind, TargetBinding,
};
use crate::origin_graph::{OriginEdge, OriginRelation};
use crate::plane::{
    OriginNodeSubject, ProjectionIdentity, ProjectionRole, ProjectionTranscript,
    TranscriptAnchoring,
};
use crate::refusal::ProjectionPlanning;

impl CauseAnchoring {
    /// What a transcript derived over this content is anchored to.
    ///
    /// A plan hangs off the address its content walked in with: the captured
    /// declaration where the content IS the capture, and the declaration
    /// fragment where a caller holds the machine's own identity.
    ///
    /// The address is the WHOLE of what the anchor names, and nothing is elected
    /// out of a set to fill it: what the content stands on lives in the entry
    /// account's dependency seat and is written into the content the identity is
    /// derived over, so two plans over different dependency sets reach different
    /// identities whatever they anchor at.
    /// An anchor naming one address is a spelling rule; a WATCH naming one
    /// address out of several is a claim about the others.
    #[must_use]
    pub const fn anchoring(&self) -> TranscriptAnchoring {
        match self {
            Self::Declaration(fragment) => {
                TranscriptAnchoring::UnderOwnerIdentity(*fragment.as_bytes())
            }
            Self::CapturedDeclaration(captured) => {
                TranscriptAnchoring::UnderProjectionIdentity(*captured.as_bytes())
            }
        }
    }
}

/// The material every origin node over owner content is derived from, beside the
/// content's own commitment at the anchor.
///
/// A declared constant and not a per-call spelling: the node one piece of content
/// stands at must not depend on which reading asked for it, or one piece of
/// content would stand at two nodes — one where it is the content a plan is over,
/// and one where another content declares it stands on it — and the origin graph
/// would carry two answers to one question.
const OWNER_CONTENT_NODE: &[u8] = b"owner-content";

/// The origin node one content address stands at.
fn content_node(address: &CauseAnchoring) -> ProjectionIdentity<OriginNodeSubject> {
    ProjectionIdentity::derived(ProjectionTranscript::under(
        ProjectionRole::OriginNode,
        address.anchoring(),
        OWNER_CONTENT_NODE,
        0,
    ))
}

/// The trigger that watches whatever an account's content is.
///
fn caused_by<K: ProjectionKind>(
    content: &OwnerContentAccount<K>,
) -> (InvalidationTrigger, Vec<InvalidationTrigger>) {
    match content.addressing() {
        ContentAddressing::Linked {
            commitment,
            dependencies,
        } => (
            InvalidationTrigger::SourceDeclarationChanged {
                watched: *commitment,
            },
            dependencies
                .iter()
                .map(|watched| InvalidationTrigger::SourceDeclarationChanged {
                    watched: *watched,
                })
                .collect(),
        ),
        ContentAddressing::Captured {
            commitment,
            dependencies,
        } => (
            InvalidationTrigger::CapturedDeclarationChanged {
                watched: *commitment,
            },
            dependencies
                .iter()
                .map(|watched| InvalidationTrigger::CapturedDeclarationChanged {
                    watched: *watched,
                })
                .collect(),
        ),
    }
}

/// The trigger that watches whatever a context was decided against.
const fn decided_against(graph: &GraphAnchoring) -> InvalidationTrigger {
    match *graph {
        GraphAnchoring::ClosedGraph(graph) => {
            InvalidationTrigger::GraphIdentityChanged { watched: graph }
        }
        GraphAnchoring::CapturedDeclarationOnly(captured) => {
            InvalidationTrigger::CapturedDeclarationChanged { watched: captured }
        }
    }
}

/// The trigger that watches a context's target end, where it has one to watch.
///
/// Target-free is a stated posture rather than an absent contract, and a posture
/// is not an identity: there is nothing for a trigger to name, so a target-free
/// context contributes none rather than a trigger over a placeholder.
const fn bound_to(target: &TargetBinding) -> Option<InvalidationTrigger> {
    match *target {
        TargetBinding::HostContract(contract) => {
            Some(InvalidationTrigger::TargetContractChanged { watched: contract })
        }
        TargetBinding::TargetFree => None,
    }
}

impl<K: ProjectionKind> OwnerContentAccount<K> {
    /// What a transcript derived over this account is anchored to.
    ///
    /// Read off the account's own commitment, so a plan's anchor and a plan's
    /// stated cause are one fact rather than two that could disagree.
    #[must_use]
    pub const fn anchoring(&self) -> TranscriptAnchoring {
        self.commitment().anchoring()
    }

    /// The invalidation triggers that watch the commitment and every declared dependency.
    #[must_use]
    pub fn cause_triggers(&self) -> Vec<InvalidationTrigger> {
        let (first, rest) = caused_by(self);
        core::iter::once(first).chain(rest).collect()
    }

    /// The single invalidation trigger that watches this account's own content.
    ///
    /// # Errors
    ///
    /// Returns [`ProjectionPlanningIssue::CauseSetUnwatchable`](crate::refusal::ProjectionPlanningIssue::CauseSetUnwatchable)
    /// when the account also names dependencies. One trigger cannot state that
    /// complete cause set, so this narrow reader refuses instead of electing the
    /// content commitment and hiding the dependencies. Use
    /// [`ProjectionContext::watch_set`] for the complete multi-trigger reading.
    pub fn cause_trigger(&self) -> Result<InvalidationTrigger, ProjectionPlanning> {
        let (first, rest) = caused_by(self);
        if rest.is_empty() {
            return Ok(first);
        }
        Err(ProjectionPlanning::established(
            crate::refusal::ProjectionPlanningIssue::CauseSetUnwatchable {
                named: u32::try_from(rest.len().saturating_add(1)).unwrap_or(u32::MAX),
                watchable: 1,
            },
        ))
    }

    /// The origin-graph node this account's content stands at.
    ///
    /// Derived from the commitment alone, so the same content reached from two
    /// directions is one node: the content a plan is over, and the commitment
    /// another content declares it stands on, are the same thing and are named
    /// the same way.
    #[must_use]
    pub fn origin_node(&self) -> ProjectionIdentity<OriginNodeSubject> {
        content_node(&self.commitment())
    }

    /// The origin edges this account contributes: one per declared dependency,
    /// each running from what the content stands on to the content itself.
    ///
    /// The relation is [`OriginRelation::ExplicitLink`], because that is what
    /// happened: an author supplied this dependency set at the door.
    /// It is not a semantic derivation — nothing here derived meaning from the
    /// dependency — and it is not a fragment construction, which is a claim about
    /// how a fragment was built that only the fragment's own owner can make.
    ///
    /// # Ordering
    ///
    /// The edges are a FAN-IN and not a walk: every one of them ends at
    /// [`OwnerContentAccount::origin_node`], so consecutive edges do not join and
    /// the set is not a trail. An [`OriginTrail`](crate::origin_graph::OriginTrail)
    /// is a walk by law; a caller draws trails through these edges one at a time
    /// rather than handing the set to a trail constructor, which would refuse the
    /// discontinuity — correctly.
    ///
    /// # Bounds
    ///
    /// At most the declared dependency magnitude, because that is what the
    /// account admitted; the count is the account's, not this road's.
    #[must_use]
    pub fn dependency_edges(&self) -> Vec<OriginEdge> {
        let to = self.origin_node();
        let edge = |from| OriginEdge {
            from,
            relation: OriginRelation::ExplicitLink,
            to,
        };
        match self.addressing() {
            ContentAddressing::Linked { dependencies, .. } => dependencies
                .iter()
                .map(|dependency| edge(content_node(&CauseAnchoring::Declaration(*dependency))))
                .collect(),
            ContentAddressing::Captured { dependencies, .. } => dependencies
                .iter()
                .map(|dependency| {
                    edge(content_node(&CauseAnchoring::CapturedDeclaration(
                        *dependency,
                    )))
                })
                .collect(),
        }
    }
}

impl ProjectionContext {
    /// The invalidation trigger that watches whatever this context was decided
    /// against.
    #[must_use]
    pub const fn graph_trigger(&self) -> InvalidationTrigger {
        decided_against(&self.graph)
    }

    /// Every trigger one plan's own identities require, as a SET.
    ///
    /// The shared half of any plan's invalidation, derived from the seats this
    /// context declares and the commitments the entry account names, rather than
    /// listed at a plan site.
    /// A kind adds whatever its own anchors require — where the roster has a seat
    /// for them — on top of this.
    ///
    /// It is a SET, and the deduplication is why.
    /// Two of these seats can name the same thing: an expansion-time context is
    /// decided against one captured declaration and its account's content IS that
    /// same capture, so its cause trigger and its graph trigger are the same
    /// trigger.
    /// A duplicate would be written twice by the plan transcript's set
    /// encoding, so two plans watching the same identities would carry two plan
    /// identities depending only on whether a call site remembered to skip the
    /// repeat.
    ///
    /// # Errors
    ///
    /// Returns the planning family naming
    /// [`BoundAxis::Declarations`](crate::refusal::BoundAxis::Declarations) when
    /// the admitted magnitude does not hold the derived set.
    /// The set is admitted under the authoring profile, which claims the declared
    /// magnitude passed admission rather than merely that these items fit it.
    pub fn watch_set<K: ProjectionKind>(
        &self,
        content: &OwnerContentAccount<K>,
    ) -> Result<InvalidationSet, ProjectionPlanning> {
        // Exhaustive on purpose: a seat added to the context stops compiling
        // HERE until somebody decides whether it is a dependency key, so the
        // watch set cannot fall a seat behind the context it is derived from.
        // The account's seats are read through the account's own readings for
        // the same reason in the other direction: there is one holder of what
        // the content stands on, and this road is a reader of it.
        let Self {
            graph,
            profile,
            // A version is not an identity, and every roster seat watches one.
            // A profile that moved to a new version under the same identity is
            // therefore a change no trigger names — a named boundary rather
            // than an oversight, and it closes when a seat exists that can
            // carry something other than thirty-two bytes.
            profile_version: _,
            generator,
            target,
        } = self;

        let (first, mut rest) = caused_by(content);
        let others = [
            decided_against(graph),
            InvalidationTrigger::ProjectionProfileChanged { watched: *profile },
            InvalidationTrigger::GeneratorVersionChanged {
                watched: *generator,
            },
        ];
        for trigger in others.into_iter().chain(bound_to(target)) {
            if trigger != first && !rest.contains(&trigger) {
                rest.push(trigger);
            }
        }
        InvalidationTrigger::watched(first, rest)
    }
}
