//! What a plan hangs off, and what it therefore watches.
//!
//! Both answers are read off the same typed postures rather than chosen at a call
//! site.
//! A plan caused by the machine's declaration fragments anchors on the first of
//! them and watches it; a plan caused by captured token material anchors on the
//! capture and watches THAT.
//! Neither posture is dressed up as the other, and neither is an absence, so an
//! expansion-time plan states its own footing instead of borrowing a linked
//! artifact that does not exist yet.
//!
//! [`ProjectionContext::watch_set`] is the one road to a shared watch set, and it
//! reads the context's own seats rather than taking a roster listed beside them.
//! It destructures the context exhaustively, so a seat added to
//! [`ProjectionContext`] stops compiling here until somebody decides whether it
//! is a dependency key; and it deduplicates, so the roster's own cardinality —
//! which is what [`InvalidationLimit`](crate::plane::InvalidationLimit) is
//! declared as — stays the honest bound.
//!
//! # Bounds
//!
//! A watch set covers the identities the SHARED CONTEXT carries, which is what a
//! shared derivation is entitled to know about.
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
//! roster seat watches one.
//!
//! A cause set names up to the declared source magnitude, and one roster seat
//! carries one identity.
//! Where the two disagree the derivation fails closed, because a set watching the
//! FIRST declaration and no other is byte-for-byte the shape of a complete watch
//! set: a plan committed to three declarations, watching one, reads as CURRENT
//! after the other two changed.
//! Partial invalidation is not a narrower claim than the roster supports; it is a
//! false one, so a multi-declaration cause set refuses with the planning family
//! naming
//! [`ProjectionPlanningIssue::CauseSetUnwatchable`](crate::refusal::ProjectionPlanningIssue::CauseSetUnwatchable),
//! carrying both counts.
//! The wider dependency-key roster that would watch them all opens when its own
//! magnitude is declared.
//!
//! HOW MANY a seat watches is the roster's own fact and is declared beside the
//! roster:
//! [`InvalidationTrigger::WATCHED_SOURCE_DECLARATIONS`](crate::planning::InvalidationTrigger::WATCHED_SOURCE_DECLARATIONS),
//! next to the road that consumes exactly that many identities and destructures
//! them into the seat.
//! This file reads that capacity and holds no second copy of it, so the threshold
//! moves with the variant instead of staying behind while the variant grows.

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
    /// machine's own identities.
    /// The remaining fragments are inside the transcript's content rather than at
    /// its anchor, because an anchor names one thing.
    ///
    /// The first fragment here is a POSITION and never a stand-in.
    /// [`CauseAnchoring::encode_into`](super::CauseAnchoring::encode_into) writes
    /// the set's length and every declaration it names into the content the
    /// identity is derived over, so two plans caused by different sets reach
    /// different identities whatever they anchor at.
    /// An anchor naming one member of a committed set is a spelling rule; a WATCH
    /// naming one member of a committed set is a claim about the others.
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
/// [`ProjectionPlanningIssue::CauseSetUnwatchable`] when the cause set names more
/// declarations than the roster can watch.
/// It refuses rather than watching the first, because a set that watches one of
/// three declarations is not a smaller watch set — it is one that reads as
/// current after two of the three changed.
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
            // member elected out of it. Its arity is the seat's own, so a roster
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
/// context contributes none rather than a trigger over a placeholder.
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
    /// more declarations than the roster can watch.
    /// The refusal is on this road and not only on
    /// [`ProjectionContext::watch_set`]: a caller reading one seat is asking the
    /// same question the whole set asks, and a road that answered it with the
    /// first declaration would be the partial claim surviving beside the road
    /// that refuses it.
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
    /// context declares rather than listed at a plan site.
    /// A kind adds whatever its own anchors require — where the roster has a seat
    /// for them — on top of this.
    ///
    /// It is a SET, and the deduplication is why.
    /// Two of these seats can name the same thing: an expansion-time context is
    /// decided against one captured declaration and CAUSED by that same capture,
    /// so its cause trigger and its graph trigger are the same trigger.
    /// Listed, that is one kind stated twice — which is what
    /// [`InvalidationLimit`](crate::plane::InvalidationLimit) is declared to
    /// exclude, since its magnitude IS the trigger roster's cardinality.
    /// A duplicate would also be written twice by the plan transcript's set
    /// encoding, so two contexts watching the same identities would carry two
    /// plan identities depending only on whether a call site remembered to skip
    /// the repeat.
    ///
    /// # Errors
    ///
    /// The two refusals are different facts: one says a derived set outgrew a
    /// declared magnitude, the other says the watch PROFILE cannot represent this
    /// context.
    ///
    /// Returns the planning family naming
    /// [`BoundAxis::Declarations`](crate::refusal::BoundAxis::Declarations) when
    /// the admitted magnitude does not hold the derived set.
    /// The set is admitted under the authoring profile, which claims the declared
    /// magnitude passed admission rather than merely that these items fit it, and
    /// that road cannot overrun from here: the derivation yields at most one
    /// trigger per kind, and the magnitude IS the roster's cardinality.
    /// Returns the family naming [`ProjectionPlanningIssue::CauseSetUnwatchable`]
    /// when the context's cause set names more declarations than the roster can
    /// watch: that set has no representable watch set at all, and the derivation
    /// says so rather than emitting one that covers the first declaration.
    pub fn watch_set(&self) -> Result<InvalidationSet, ProjectionPlanning> {
        // Exhaustive on purpose: a seat added to the context stops compiling
        // HERE until somebody decides whether it is a dependency key, so the
        // watch set cannot fall a seat behind the context it is derived from.
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
            // partially: a set watching the first declaration alone would read
            // exactly like a complete one. Watching the rest needs a wider
            // roster with its own declared magnitude.
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
