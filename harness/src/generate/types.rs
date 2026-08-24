//! Every public type of the generation home, declared and nothing else.
//!
//! Construction and reading live in this module's own child `type_guard.rs`, which is where the private fields are reachable.
//!
//! Some of the vocabulary arrives already made: the profiles, budgets, fingerprints, postures, and standings are [`crate::report`]'s, and the names, schema identity, and revision bindings are [`crate::descriptor`]'s.
//! A plan binds those values; what they mean is written where they are declared.

use crate::descriptor::{GeneratedSupportSchemaId, NamespacedName, PopulationRef, RevisionBinding};
use crate::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use crate::report::{
    ByteBudget, CaseBudget, Fingerprint, GenerationProfile, MinimizationProfile, ReplayPosture,
    TrialRunStanding,
};
use arbitrary::Unstructured;
use std::num::NonZeroU32;

// The generation axis and its census.

macro_rules! with_generation_dispositions {
    ($callback:ident) => {
        $callback! {
            /// A case was produced and the population's precondition admitted it.
            Generated => generated,
            /// The byte source held less than the width the plan's ramp asked for.
            BytesInsufficient => bytes_insufficient,
            /// A case was produced and the population's declared precondition rejected it.
            PreconditionRejected => precondition_rejected,
            /// The generator declined the bytes it was handed and produced no command at all.
            GeneratorRefused => generator_refused,
            /// The generator reported a command while consuming none of the case's bytes.
            GeneratorContractViolated => generator_contract_violated,
            /// The plan reached this case with its byte budget already spent, so no draw was attempted.
            GenerationBudgetExhausted => budget_exhausted,
        }
    };
}

macro_rules! declare_generation_dispositions {
    ($($(#[$variant_meta:meta])* $variant:ident => $seat:ident),+ $(,)?) => {
        /// What became of one case the generator was asked for.
        ///
        /// This is the generation axis alone.
        /// What an execution did with a case is [`crate::report::RunAttempt`]'s roster, and what a check concluded is [`crate::report::TrialConclusion`]'s.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum GenerationDisposition {
            $($(#[$variant_meta])* $variant),+
        }

        /// How many disposition seats one census carries.
        ///
        /// The width comes from the roster, so a seat cannot be added to [`GenerationDisposition`] without the census growing with it.
        pub const GENERATION_DISPOSITION_SEATS: usize = [
            $(GenerationDisposition::$variant),+
        ]
        .len();

        /// The per-population accounting over generation dispositions.
        ///
        /// One seat exists per arm of [`GenerationDisposition`], always, so the denominator cannot silently shrink.
        /// Every case a plan reached is counted once, and [`GenerationCensus::attempted`] is the sum of the seats rather than a total kept beside them.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct GenerationCensus {
            population: PopulationRef,
            counts: [u32; GENERATION_DISPOSITION_SEATS],
        }
    };
}

with_generation_dispositions!(declare_generation_dispositions);

#[path = "type_guard.rs"]
mod guard;

// The generation plan.

/// The seed one derived byte stream is addressed from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RootSeed(u64);

/// Where one plan's input bytes come from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputOrigin {
    /// A root seed, from which the paved byte source derives its stream.
    Seeded(RootSeed),
    /// Exact caller-supplied bytes, such as a corpus warm start or replay material whose authority is owned elsewhere.
    Supplied(Vec<u8>),
}

/// How many bytes one generated case is drawn from.
///
/// Zero is refused: every case drawn at it would be the empty input, and a ramp built on it would spend a whole case budget on one input repeated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CaseWidth(usize);

/// Why one case width was refused.
#[must_use = "a refusal is the reason a case width was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CaseWidthRefusal {
    /// The width is zero, so every case drawn at it would be the empty input.
    ZeroBytes,
}

/// How a plan's case widths progress across its cases.
///
/// Every ramp is a function of the case ordinal alone, so a width never depends on what the subject did with an earlier case, and the plan's byte budget is the ceiling on any one draw rather than the ramp.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SizeProgression {
    /// Every case is drawn at one declared width.
    Constant {
        /// The width every case is drawn at.
        width: CaseWidth,
    },
    /// The first case is the base, and each case adds one step to the one before it.
    Linear {
        /// The first case's width.
        base: CaseWidth,
        /// What each case adds to the case before it.
        step: CaseWidth,
    },
    /// The first case is the base, and each case doubles the one before it.
    Doubling {
        /// The first case's width.
        base: CaseWidth,
    },
}

/// Which case of a plan's sequence one draw serves.
///
/// The ordinal counts from zero, so the first case is drawn at the ramp's base.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CaseIndex(u32);

/// How many empty-handed draws one plan admits before another draw is withheld.
///
/// Empty-handed means [`GenerationDisposition::PreconditionRejected`] or [`GenerationDisposition::GeneratorRefused`] — the two outcomes that spend a case seat without filling it.
/// The census still counts them apart; this is one allowance over their sum, not a place they are flattened.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RejectionAllowance {
    /// Successful cases may proceed, but the first empty-handed draw spends the allowance.
    NoRejections,
    /// This non-zero number of empty-handed draws may be counted before another draw is withheld.
    AtMost(NonZeroU32),
}

/// One plan's complete statement of what to generate and under which bounds.
///
/// The budgets are the plan's own, and they are the only budgets a drive reads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerationPlan {
    population: PopulationRef,
    profile: GenerationProfile,
    origin: InputOrigin,
    cases: CaseBudget,
    bytes: ByteBudget,
    rejection_allowance: RejectionAllowance,
    progression: SizeProgression,
}

/// Why one generation plan was refused.
///
/// The checks run in the order the arms are declared, so exactly one of them is true of any refused plan.
#[must_use = "a refusal is the reason a generation plan was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GenerationPlanRefusal {
    /// The case budget admits no case, so the plan names a population it would never draw from.
    ZeroCaseBudget,
    /// The byte budget admits no byte, so the plan's first case could never be drawn.
    ZeroByteBudget,
    /// The origin supplies bytes and the supplied material is empty, so there is nothing to draw.
    EmptySuppliedBytes,
}

// The deterministic byte source.

/// The domain tag every generation byte-source address is derived under.
pub const GENERATION_SOURCE_TAG: DomainTag = DomainTag::declared(
    "generation-byte-source",
    IdentityProfileVersion::declared(1),
);

/// The domain tag every derived stream chunk is derived under.
///
/// Its own tag rather than the address's, because an address and a chunk are different kinds and two kinds whose preimages could coincide must not share a derivation context.
pub const GENERATION_CHUNK_TAG: DomainTag = DomainTag::declared(
    "generation-stream-chunk",
    IdentityProfileVersion::declared(1),
);

/// How many bytes one chunk of a byte stream carries.
///
/// The width is the identity substrate's address width, because a derived chunk is one address's bytes, and the supplied arm is read over the same grid.
pub const SOURCE_CHUNK_BYTES: usize = 32;

/// The address one plan's derived byte stream is counted from.
///
/// It is derived from the plan's population, generation profile, and input origin, and from nothing else.
/// Growing the case budget or changing the size progression re-windows the same stream rather than renaming it, so a longer run reproduces every case a shorter one produced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ByteSourceAddress(ContentAddress);

/// One position in a byte stream: which chunk, and how far into it.
///
/// Any position is nameable directly through [`StreamCursor::at`] without drawing the positions before it, which is what makes the derived stream addressable rather than merely repeatable.
/// The offset within a chunk is always less than [`SOURCE_CHUNK_BYTES`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StreamCursor {
    chunk: u64,
    within: usize,
}

/// Why one stream cursor was refused.
#[must_use = "a refusal is the reason a stream cursor was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StreamCursorRefusal {
    /// The offset within the chunk is not inside the chunk.
    WithinPastChunk {
        /// The offset that was offered.
        within: usize,
    },
}

/// Where one plan's cases are drawn from.
///
/// Both arms are deterministic and seekable, and neither reads a clock, an environment, or an operating-system random source.
/// The derived arm is the paved road: chunk N of the stream is the identity substrate's address of the source address followed by the counter N, under [`GENERATION_CHUNK_TAG`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ByteSource {
    /// A counter-addressed stream over the identity substrate.
    Derived(ByteSourceAddress),
    /// The exact bytes a plan supplied, read over the same chunk grid.
    Supplied(Vec<u8>),
}

/// What one draw against a byte source yielded.
///
/// A draw yields the width it was asked for or it yields nothing.
/// A partial tail is [`ByteDraw::Insufficient`] with both counts kept, because a case drawn narrower than its ramp is not the case the plan declared.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ByteDraw {
    /// The requested width, and the position the next draw begins at.
    Drawn {
        /// The exact bytes drawn.
        bytes: Vec<u8>,
        /// Where the next draw begins.
        next: StreamCursor,
    },
    /// The source held less than the draw asked for, with both counts kept.
    Insufficient {
        /// How many bytes the draw asked for.
        requested: usize,
        /// How many bytes the source still held at the cursor.
        available: usize,
    },
}

// The sequence driver's seams.

/// The owner-supplied road from a case's bytes to one command.
///
/// A function pointer rather than a closure or a trait object, so a decoder carries no captured state and nothing ambient rides in with it.
/// A command type that derives the generation vocabulary's `Arbitrary` reaches this seam through [`decode_arbitrary`](crate::generate::decode_arbitrary); a hand-written decoder stands here directly.
///
/// The contract the driver holds a decoder to: a decoder that reports a command must have consumed at least one byte of the case.
/// One that reports a command while consuming nothing is counted as [`GenerationDisposition::GeneratorContractViolated`], which is also what makes the decode loop terminate.
pub type CommandDecode<Command> = fn(&mut Unstructured<'_>) -> arbitrary::Result<Command>;

/// The owner-supplied precondition one decoded sequence is judged by.
///
/// A population without a precondition drives under [`admit_every_sequence`](crate::generate::admit_every_sequence) rather than under an absent one, so no road in the driver has to decide what a missing precondition would have meant.
pub type SequencePrecondition<Command> = fn(&[Command]) -> PreconditionVerdict;

/// Whether a population's declared precondition admits one sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PreconditionVerdict {
    /// The sequence satisfies the population's declared precondition.
    Admitted,
    /// The sequence does not, and the case is counted as rejected.
    Rejected,
}

/// One generated case: which case it is, the commands decoded from it, and the exact bytes it was drawn from.
///
/// The input is the whole draw, including any tail the decoder did not consume, because the draw is what the case was handed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandSequence<Command> {
    case: CaseIndex,
    commands: Vec<Command>,
    input: Vec<u8>,
}

/// Why one plan's generation stopped.
///
/// The halt names the event that ended the drive and the census counts what became of every case the drive reached, and neither is derivable from the other.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GenerationHalt {
    /// The plan reached every case its case budget declared, which is the one arm that means the plan finished rather than stopped.
    CaseBudgetMet,
    /// The plan reached a case with its byte budget already spent.
    ByteBudgetExhausted,
    /// An empty-handed draw spent the plan's rejection allowance.
    RejectionAllowanceSpent,
    /// The byte source held less than a case's ramp width asked for.
    SourceExhausted,
    /// A decoder reported a command while consuming none of the case's bytes.
    GeneratorContractViolated,
}

/// What one drive of one plan produced: the admitted sequences, the census over every case it reached, and the bound that ended it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedSequences<Command> {
    sequences: Vec<CommandSequence<Command>>,
    census: GenerationCensus,
    halt: GenerationHalt,
}

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

/// Whether one candidate shrink is admitted.
///
/// Acceptance is fingerprint equality and nothing else.
/// A candidate that reaches a different fingerprint is refused and the fingerprint it reached is carried out, so a reduction never wanders from the bug it was minimizing and a reader can see where it tried to wander to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShrinkVerdict {
    /// The candidate carries the same failure fingerprint, so it stands in for the input it shrinks.
    Accepted,
    /// The candidate failed under a different fingerprint.
    RejectedFingerprintMoved {
        /// The fingerprint the candidate reached instead.
        found: Fingerprint,
    },
    /// The candidate did not fail at all.
    RejectedNoFailure,
}

/// The accounting over one reduction's candidate probes.
///
/// It counts candidates, never bytes: how far an input actually shrank is the outcome's input, not a number here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReductionCensus {
    accepted: u32,
    fingerprint_moved: u32,
    no_failure: u32,
}

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
