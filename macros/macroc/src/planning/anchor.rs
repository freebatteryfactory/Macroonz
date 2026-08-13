//! What a plan hangs off, and what it therefore watches.
//!
//! Both answers are read off the same typed postures rather than chosen at a
//! call site. A plan caused by the machine's declaration fragments anchors on
//! the first of them and watches it; a plan caused by captured token material
//! anchors on the capture and watches THAT. Neither posture is dressed up as
//! the other, and neither is an absence, so an expansion-time plan states its
//! own footing instead of borrowing a linked artifact that does not exist yet.
//!
//! # The watch set is derived from the context, not listed beside it
//!
//! [`ProjectionContext::watch_set`] is the one road to a shared watch set, and
//! it reads the context's own seats. Before it, each plan site wrote a
//! `vec![…]` of triggers by hand — a roster standing beside the context and
//! maintained in step with it by whoever remembered. The two rosters that
//! existed had already drifted differently: the pattern-stamp site listed four
//! triggers and never watched the target binding, and the derivation site
//! listed three and omitted the graph trigger, because at that site the graph
//! and the cause are the same capture and listing both would have stated one
//! kind twice. Neither fact was wrong; both were knowledge held at a call site
//! about a value declared elsewhere, which is the thing that goes stale.
//!
//! The derivation holds both facts once. It destructures the context
//! exhaustively, so a seat added to [`ProjectionContext`] stops compiling here
//! until somebody decides whether it is a dependency key; and it deduplicates,
//! so the roster's own cardinality — which is what
//! [`InvalidationLimit`](crate::plane::InvalidationLimit) is declared as —
//! stays the honest bound rather than an accident of which sites remembered.
//!
//! # What a watch set covers, and what it cannot
//!
//! It covers the identities the SHARED CONTEXT carries, which is what a shared
//! derivation is entitled to know about. It does not cover the anchors a kind
//! supplies beside its context. Those are named at the kind's own plan site,
//! where the compiler forces each one to be accounted for, and the reason they
//! carry no trigger is not a judgment that they do not matter: the trigger
//! roster declares no seat any of them could be watched through, and every
//! roster seat is one thirty-two-byte identity of a declared kind. Minting nine
//! more kinds so that nine more anchors could be listed would rebuild the
//! hand-maintained roster one level down and push the set past the cardinality
//! that bounds it.
//!
//! Two seats inside the context are named boundaries for the same reason and
//! are stated at their bindings below: a cause set names up to sixty-four
//! declarations and the roster can watch one, and a profile VERSION is not an
//! identity and no seat watches it.

use super::{
    CauseAnchoring, GraphAnchoring, InvalidationSet, InvalidationTrigger, ProjectionContext,
    TargetBinding,
};
use crate::plane::TranscriptAnchoring;
use crate::refusal::ProjectionPlanning;

impl CauseAnchoring {
    /// What a transcript derived under this cause is anchored to.
    ///
    /// A plan hangs off what caused it: the captured declaration where the cause
    /// IS the capture, and the first declared fragment where a caller holds the
    /// machine's own identities. The remaining fragments are inside the
    /// transcript's content rather than at its anchor, because an anchor names
    /// one thing.
    #[must_use]
    pub fn anchoring(&self) -> TranscriptAnchoring {
        match self {
            Self::Declarations(sources) => {
                TranscriptAnchoring::UnderOwnerIdentity(*sources.first().as_bytes())
            }
            Self::CapturedDeclaration(captured) => {
                TranscriptAnchoring::UnderProjectionIdentity(*captured.as_bytes())
            }
        }
    }
}

/// The trigger that watches whatever a context was caused by.
fn caused_by(sources: &CauseAnchoring) -> InvalidationTrigger {
    match sources {
        CauseAnchoring::Declarations(declared) => InvalidationTrigger::SourceDeclarationChanged {
            watched: *declared.first(),
        },
        CauseAnchoring::CapturedDeclaration(captured) => {
            InvalidationTrigger::CapturedDeclarationChanged { watched: *captured }
        }
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
/// context contributes none rather than contributing a trigger over a placeholder.
const fn bound_to(target: &TargetBinding) -> Option<InvalidationTrigger> {
    match *target {
        TargetBinding::HostContract(contract) => {
            Some(InvalidationTrigger::TargetContractChanged { watched: contract })
        }
        TargetBinding::TargetFree => None,
    }
}

impl ProjectionContext {
    /// The invalidation trigger that watches whatever this context was caused
    /// by — the fragment where a caller holds one, and the captured declaration
    /// where the cause IS the capture.
    #[must_use]
    pub fn cause_trigger(&self) -> InvalidationTrigger {
        caused_by(&self.sources)
    }

    /// The invalidation trigger that watches whatever this context was decided
    /// against.
    #[must_use]
    pub const fn graph_trigger(&self) -> InvalidationTrigger {
        decided_against(&self.graph)
    }

    /// Every trigger this context's own identities require, as a SET.
    ///
    /// The shared half of any plan's invalidation, derived from the seats the
    /// context declares rather than listed at a plan site. A kind adds whatever
    /// its own anchors require — where the roster has a seat for them — on top
    /// of this.
    ///
    /// # It is a set, and the deduplication is the reason
    ///
    /// Two of these seats can name the same thing. An expansion-time context is
    /// decided against one captured declaration and CAUSED by that same capture,
    /// so its cause trigger and its graph trigger are the same trigger. Listed,
    /// that is one kind stated twice — which is precisely what
    /// [`InvalidationLimit`](crate::plane::InvalidationLimit) is declared to
    /// exclude, since its magnitude IS the trigger roster's cardinality. A
    /// duplicate would also be written twice by the plan transcript's set
    /// encoding, so two contexts watching the same identities would carry two
    /// plan identities depending only on whether a call site remembered to skip
    /// the repeat.
    ///
    /// # Why this road can refuse at all
    ///
    /// It admits the set under the authoring profile, which is the claim that
    /// the declared magnitude was admitted rather than merely that these items
    /// fit it. The derivation itself cannot overrun — it yields at most one
    /// trigger per kind and reaches five of the nine kinds — so the refusal is
    /// the admission road's, not a case this derivation produces. Both plan
    /// sites already carry that road and propagate it; nothing here invents a
    /// branch a caller has to find a value for.
    ///
    /// # Errors
    ///
    /// Returns the planning family naming
    /// [`BoundAxis::Declarations`](crate::refusal::BoundAxis::Declarations) when
    /// the admitted magnitude does not hold the derived set.
    pub fn watch_set(&self) -> Result<InvalidationSet, ProjectionPlanning> {
        // Exhaustive on purpose, and that is the mechanism rather than a
        // style: a seat added to the context stops compiling HERE until
        // somebody decides whether it is a dependency key, so the watch set
        // cannot fall a seat behind the context it is derived from.
        let Self {
            graph,
            profile,
            // A version is not an identity, and every roster seat watches one.
            // A profile that moved to a new version under the same identity is
            // therefore a change no trigger names — a named boundary rather
            // than an oversight, and it closes when a seat exists that can
            // carry something other than thirty-two bytes.
            profile_version: _,
            // A cause set names up to sixty-four declarations and the roster
            // seat names one, so the second and later declarations of a
            // multi-cause plan are unwatched. Also a named boundary: watching
            // them needs a magnitude wider than the roster's cardinality, which
            // is a declared-limit decision with its own controls and not one
            // this derivation may take on its own.
            sources,
            generator,
            target,
        } = self;

        let first = caused_by(sources);
        let others = [
            decided_against(graph),
            InvalidationTrigger::ProjectionProfileChanged { watched: *profile },
            InvalidationTrigger::GeneratorVersionChanged {
                watched: *generator,
            },
        ];
        let mut rest: Vec<InvalidationTrigger> = Vec::new();
        for trigger in others.into_iter().chain(bound_to(target)) {
            if trigger != first && !rest.contains(&trigger) {
                rest.push(trigger);
            }
        }
        InvalidationTrigger::watched(first, rest)
    }
}
