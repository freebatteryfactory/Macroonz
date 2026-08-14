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
//! One seat inside the context is a named boundary for the same reason and is
//! stated at its binding below: a profile VERSION is not an identity, and every
//! roster seat watches one.
//!
//! # A cause set the roster cannot watch is refused, not partially watched
//!
//! A cause set names up to the declared source magnitude and one roster seat
//! carries one identity. That gap used to be written down as a named boundary
//! too, and a boundary is the wrong shape for it. The other boundaries cost a
//! reader nothing: a profile version nobody watches leaves a plan that is
//! honestly silent about profile versions. This one cost a reader the answer —
//! a set watching the FIRST declaration and no other is byte-for-byte the shape
//! of a complete watch set, so a plan committed to three declarations, watching
//! one, reads as CURRENT after the other two changed. Partial invalidation is
//! not a narrower claim than the roster supports; it is a false one.
//!
//! So the derivation fails closed. A multi-declaration cause set refuses with
//! the planning family naming
//! [`ProjectionPlanningIssue::CauseSetUnwatchable`](crate::refusal::ProjectionPlanningIssue::CauseSetUnwatchable),
//! carrying both counts, and no plan stands over a context this profile cannot
//! represent. The complete dependency-key watch set is a wider roster with its
//! own declared magnitude, and it is owed; refusing is what the plane does until
//! it exists, rather than issuing a freshness claim it cannot support.
//!
//! HOW MANY a seat watches is the roster's own fact and is declared beside the
//! roster:
//! [`InvalidationTrigger::WATCHED_SOURCE_DECLARATIONS`](crate::planning::InvalidationTrigger::WATCHED_SOURCE_DECLARATIONS),
//! next to the road that consumes exactly that many identities and destructures
//! them into the seat. This file reads the capacity and holds no second copy of
//! it, which is the difference between a threshold that moves with the variant
//! and one that quietly stays behind while the variant grows.

use super::{
    CauseAnchoring, GraphAnchoring, InvalidationSet, InvalidationTrigger, ProjectionContext,
    TargetBinding,
};
use crate::plane::TranscriptAnchoring;
use crate::refusal::{ProjectionPlanning, ProjectionPlanningIssue};

impl CauseAnchoring {
    /// What a transcript derived under this cause is anchored to.
    ///
    /// A plan hangs off what caused it: the captured declaration where the cause
    /// IS the capture, and the first declared fragment where a caller holds the
    /// machine's own identities. The remaining fragments are inside the
    /// transcript's content rather than at its anchor, because an anchor names
    /// one thing.
    ///
    /// The first fragment here is a POSITION and never a stand-in, and the
    /// difference is what keeps this road out of the defect the watch set had.
    /// [`CauseAnchoring::encode_into`](super::CauseAnchoring::encode_into)
    /// writes the set's length and every declaration it names into the content
    /// the identity is derived over, so two plans caused by different sets reach
    /// different identities whatever they anchor at. An anchor naming one member
    /// of a committed set is a spelling rule; a WATCH naming one member of a
    /// committed set is a claim about the others.
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
///
/// # Errors
///
/// Returns the planning family naming
/// [`ProjectionPlanningIssue::CauseSetUnwatchable`] when the cause set names
/// more declarations than the roster can watch. It refuses rather than watching
/// the first, because a set that watches one of three declarations is not a
/// smaller watch set — it is one that reads as current after two of the three
/// changed.
fn caused_by(sources: &CauseAnchoring) -> Result<InvalidationTrigger, ProjectionPlanning> {
    match sources {
        CauseAnchoring::Declarations(declared) => {
            let named = declared.len();
            let watchable = InvalidationTrigger::WATCHED_SOURCE_DECLARATIONS;
            if named > watchable {
                return Err(ProjectionPlanning::established(
                    ProjectionPlanningIssue::CauseSetUnwatchable {
                        named: u32::try_from(named).unwrap_or(u32::MAX),
                        watchable: u32::try_from(watchable).unwrap_or(u32::MAX),
                    },
                ));
            }
            // Past the refusal the set holds exactly as many declarations as the
            // seat watches, so the array below is the whole set rather than a
            // member elected out of it: the same call was a stand-in one line ago
            // and is a total read here. Its arity is the seat's own, so a roster
            // that began watching two would stop this line compiling rather than
            // leaving it quietly reporting one.
            Ok(InvalidationTrigger::watching_source_declarations([
                *declared.first(),
            ]))
        }
        CauseAnchoring::CapturedDeclaration(captured) => {
            Ok(InvalidationTrigger::CapturedDeclarationChanged { watched: *captured })
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
    ///
    /// # Errors
    ///
    /// Returns the planning family naming
    /// [`ProjectionPlanningIssue::CauseSetUnwatchable`] when the cause set names
    /// more declarations than the roster can watch. The refusal is on this road
    /// and not only on [`ProjectionContext::watch_set`]: a caller reading one
    /// seat is asking the same question the whole set asks, and a road that
    /// answered it with the first declaration would be the partial claim
    /// surviving beside the road that refuses it.
    pub fn cause_trigger(&self) -> Result<InvalidationTrigger, ProjectionPlanning> {
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
    /// # The two roads this can refuse on, and they are different facts
    ///
    /// It admits the set under the authoring profile, which is the claim that
    /// the declared magnitude was admitted rather than merely that these items
    /// fit it. That admission cannot overrun here — the derivation yields at
    /// most one trigger per kind and reaches five of the nine kinds — so the
    /// magnitude half of the refusal is the admission road's rather than a case
    /// this derivation produces.
    ///
    /// The other half IS this derivation's. A cause set naming more
    /// declarations than the roster can watch has no representable set at all,
    /// and the derivation says so rather than emitting one that covers the
    /// first. The two are not the same refusal wearing two payloads: the first
    /// says a derived set outgrew a declared magnitude, and the second says the
    /// watch PROFILE cannot represent this context. Both plan sites already
    /// carry the road and propagate it; nothing here invents a branch a caller
    /// has to find a value for.
    ///
    /// # Errors
    ///
    /// Returns the planning family naming
    /// [`BoundAxis::Declarations`](crate::refusal::BoundAxis::Declarations) when
    /// the admitted magnitude does not hold the derived set, and naming
    /// [`ProjectionPlanningIssue::CauseSetUnwatchable`] when the context's cause
    /// set names more declarations than the roster can watch.
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
            // A cause set names up to the declared source magnitude and the
            // roster seat names one. That gap is REFUSED rather than watched
            // partially: watching the second and later declarations needs a
            // wider roster with its own declared magnitude, which is a
            // declared-limit decision with its own controls and not one this
            // derivation may take on its own — and until it is taken, a set
            // watching the first alone would read exactly like a complete one.
            sources,
            generator,
            target,
        } = self;

        let first = caused_by(sources)?;
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
