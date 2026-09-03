//! Every public type of the reduction home, declared and nothing else.
//!
//! Construction and reading live in this module's own child `type_guard.rs`, which is where the private fields are reachable.

use crate::descriptor::{GeneratedSupportSchemaId, NamespacedName, RevisionBinding};
use crate::report::{
    Fingerprint, GenerationProfile, MinimizationProfile, ReplayPosture, TrialRunStanding,
};

// The reduction plan.

/// Which generic byte reducer a reduction plan binds.
///
/// A closed roster rather than an open name, because the generic byte reducers are this home's own realizations, so a plan cannot name one that nothing implements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ByteReducerId {
    /// Chunk removal and chunk zeroing over the byte input, at halving window widths, run to a fixed point under the reduction budget.
    ChunkRemovalAndZeroing,
}

/// One owner-declared semantic reducer.
///
/// Open and namespaced, because a semantic reducer knows what the bytes mean and that knowledge is the owner's.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SemanticReducerId(NamespacedName);

/// A semantic reducer's ordered candidate sequence for one input.
///
/// Each candidate must be shorter than the input or candidate immediately before it.
/// An owner may use any semantic knowledge to propose the bytes; the strict descent is what keeps the shared engine terminating, and [`shrink_verdict`](crate::generate::shrink_verdict) still decides what is admitted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticCandidates {
    candidates: Vec<Vec<u8>>,
}

/// Why one semantic reducer's candidate sequence was refused.
#[must_use = "a refusal is the reason semantic candidates were not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticCandidateRefusal {
    /// One candidate was not strictly smaller than the value before it.
    NotStrictlySmaller {
        /// The candidate's position in the offered sequence.
        position: usize,
        /// The byte length it had to be smaller than.
        predecessor_bytes: usize,
        /// The candidate's byte length.
        candidate_bytes: usize,
    },
}

/// The owner-supplied semantic candidate producer.
///
/// A function pointer excludes captured closure state; it does not establish purity, stability, or the absence of ambient effects.
/// Its typed result does establish the strict descent, before the shared engine probes any candidate.
pub type SemanticReducerCall = fn(&[u8]) -> Result<SemanticCandidates, SemanticCandidateRefusal>;

/// One semantic reducer's name, revision posture, and executable candidate producer.
///
/// The callable and its revision travel in one value, so a plan cannot name a reducer apart from the function it will run.
/// The revision posture reaches the replay posture only when this binding is actually invoked.
#[derive(Debug, Clone, Copy)]
pub struct SemanticReducerBinding {
    reducer: SemanticReducerId,
    revision: RevisionBinding,
    call: SemanticReducerCall,
}

/// One semantic reducer invocation, as reduction evidence retains it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticReducerExecution {
    reducer: SemanticReducerId,
    revision: RevisionBinding,
    candidates: usize,
    probes: usize,
}

/// Whether the generic byte reducer was reached by one reduction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ByteReducerExecution {
    /// The generic reducer ran under this closed implementation identity.
    Executed(ByteReducerId),
    /// Semantic candidates spent the probe budget before the generic reducer.
    NotReachedBecauseBudgetSpent,
}

/// That a reduction preserves the failure fingerprint.
///
/// A plan field rather than an option, which is what makes preservation non-optional in the shape of a plan: there is no reduction plan anybody can build that does not require it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FingerprintPreservation {
    /// The shrunk input must carry the same failure fingerprint.
    Required,
}

/// How many candidate probes one reduction admits.
///
/// The baseline probe is not a candidate and does not spend this budget; the bound is over the shrinks a reduction offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ReductionBudget(u32);

/// One plan's complete statement of how a find is minimized.
#[derive(Debug, Clone)]
pub struct ReductionPlan {
    profile: MinimizationProfile,
    byte_reducer: ByteReducerId,
    semantic_reducers: Vec<SemanticReducerBinding>,
    preservation: FingerprintPreservation,
    budget: ReductionBudget,
}

/// Why one reduction plan was refused.
///
/// The budget is read before the semantic reducers.
#[must_use = "a refusal is the reason a reduction plan was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReductionPlanRefusal {
    /// The budget admits no probe, so the plan states a reduction that could never offer a shrink.
    ZeroReductionBudget,
    /// The semantic reducer roster names this reducer more than once, which is refused rather than folded away.
    DuplicateSemanticReducer(SemanticReducerId),
}

/// The exact report standing and re-execution probe one reduction runs under.
///
/// A binding opens only from a real refused [`crate::report::TrialReport`], so the fingerprint it preserves comes from that report rather than from a caller beside it.
/// A function pointer does not prove the bound adapter is the original subject callable, and the bound revision posture is what states that ceiling.
pub struct ReductionProbeBinding {
    standing: TrialRunStanding,
    preserved: Fingerprint,
    generation: GenerationProfile,
    schema: GeneratedSupportSchemaId,
    revision: RevisionBinding,
    probe: FingerprintProbe,
}

/// Why one report could not open a reduction probe binding.
#[must_use = "a refusal is the reason a reduction probe binding was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReductionProbeRefusal {
    /// The report did not execute to a conclusion, so it carries no finding to preserve.
    TrialDidNotConclude,
    /// The report passed, so it carries no failure fingerprint to preserve.
    TrialPassed,
}

// Minimization.

/// What one probe of one candidate input concluded.
///
/// A candidate that stopped failing is not a candidate that found a different bug, and a reduction that folded the two together could not say which of them ended a pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProbeOutcome {
    /// The candidate failed, under this fingerprint.
    Reproduced(Fingerprint),
    /// The candidate did not fail at all.
    NoFailure,
}

/// The owner-supplied re-check one candidate input is judged by.
///
/// The pointer shape excludes captured state; it does not establish semantic purity or outcome stability, and a probe's effects and readings stay its owner's.
pub type FingerprintProbe = fn(&[u8]) -> ProbeOutcome;

macro_rules! with_shrink_verdicts {
    ($callback:ident) => {
        $callback! {
            /// The candidate carries the same failure fingerprint, so it stands in for the input it shrinks.
            Accepted => accepted,
            /// The candidate failed under a different fingerprint.
            RejectedFingerprintMoved {
                /// The fingerprint the candidate reached instead.
                found: Fingerprint,
            } => fingerprint_moved,
            /// The candidate did not fail at all.
            RejectedNoFailure => no_failure,
        }
    };
}

macro_rules! declare_reduction_census {
    (
        $(
            $(#[$variant_meta:meta])*
            $variant:ident
            $( { $( $(#[$field_meta:meta])* $field:ident: $field_type:ty, )+ } )?
            => $seat:ident,
        )+
    ) => {
        /// Whether one candidate shrink is admitted.
        ///
        /// Acceptance is fingerprint equality and nothing else.
        /// A candidate that reaches a different fingerprint is refused and the fingerprint it reached is carried out, so a reduction never wanders from the bug it was minimizing and a reader can see where it tried to wander to.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum ShrinkVerdict {
            $(
                $(#[$variant_meta])*
                $variant
                $( { $( $(#[$field_meta])* $field: $field_type, )+ } )?,
            )+
        }

        crate::census::declare_census! {
            /// The accounting over one reduction's candidate probes.
            ///
            /// It counts candidates, never bytes: how far an input actually shrank is the outcome's input, not a number here.
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct ReductionCensus {
                count: u32,
                seat: ReductionCensusSeat,
                context {}
                fields {
                    $( $variant => $seat, )+
                }
            }
        }
    };
}

with_shrink_verdicts!(declare_reduction_census);

#[path = "type_guard.rs"]
mod guard;

/// Why one reduction stopped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReductionHalt {
    /// A whole round of passes admitted no candidate, so the input is at the reducer's fixed point.
    FixedPointReached,
    /// The reduction budget was spent before a fixed point was reached.
    BudgetExhausted,
}

/// What one reduction produced: the smallest input that kept the fingerprint, that fingerprint, the census over every candidate, and the reason it stopped.
///
/// The input is the smallest one this reduction reached under its budget and its reducer, which is not the same as the smallest input that reproduces the failure.
/// [`ReductionHalt::BudgetExhausted`] is what says so.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReductionOutcome {
    input: Vec<u8>,
    fingerprint: Fingerprint,
    census: ReductionCensus,
    halt: ReductionHalt,
}

/// What one complete reduction leaves behind.
///
/// It is the only input [`capture_replay`](crate::generate::capture_replay) accepts, and the replay posture it carries is the meet of everything that actually ran.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReductionEvidence {
    standing: TrialRunStanding,
    generation: GenerationProfile,
    schema: GeneratedSupportSchemaId,
    probe_revision: RevisionBinding,
    minimization: MinimizationProfile,
    semantic_reducers: Vec<SemanticReducerExecution>,
    byte_reducer: ByteReducerExecution,
    outcome: ReductionOutcome,
    replay: ReplayPosture,
}

/// Why one reduction was refused before any candidate was offered.
///
/// The first two arms come from the baseline probe: a reduction that skipped it could shrink a passing input forever, or minimize a find into a bug it was never asked about.
#[must_use = "a refusal is the reason a reduction did not run"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReductionRefusal {
    /// The input handed in does not fail at all, so there is no find to minimize.
    BaselineDidNotFail,
    /// The input handed in fails under a different fingerprint than the one it was told to preserve.
    BaselineFingerprintDiffers {
        /// The fingerprint the baseline actually reached.
        found: Fingerprint,
    },
    /// An invoked semantic reducer refused the candidate sequence it authored.
    SemanticReducerRefused {
        /// The reducer whose typed output refused.
        reducer: SemanticReducerId,
        /// Why its candidate sequence was not lawful.
        cause: SemanticCandidateRefusal,
    },
}
