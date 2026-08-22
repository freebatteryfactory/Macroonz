//! The generation contract's public types: the generation dispositions and
//! their census, the two owning plans, the deterministic byte source, the
//! sequence driver's seams, and the minimization vocabulary.
//!
//! Declarations only. Every road that reaches a private field is in this
//! module's own child `type_guard.rs`; the driver and the reducer are their own
//! pure-function modules.
//!
//! # The borrowed vocabularies
//!
//! The generation profile, the minimization profile, the case and byte budgets,
//! the failure fingerprint, and the replay capsule belong to the record
//! instrument ([`crate::report`]) and arrive already made. The population
//! reference and the namespaced name belong to the descriptor vocabulary
//! ([`crate::descriptor`]). Nothing here restates either home's contract: a plan
//! BINDS those values, and what they mean is written where they are declared.

use crate::descriptor::{NamespacedName, PopulationRef};
use crate::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use crate::report::{ByteBudget, CaseBudget, Fingerprint, GenerationProfile, MinimizationProfile};
use arbitrary::Unstructured;
use std::collections::BTreeSet;

// ---------------------------------------------------------------------------
// The generation axis.
// ---------------------------------------------------------------------------

macro_rules! with_generation_dispositions {
    ($callback:ident) => {
        $callback! {
            /// A case was produced and the population's precondition admitted it.
            Generated => generated,
            /// The byte source held less than the width the plan's ramp asked for.
            BytesInsufficient => bytes_insufficient,
            /// A case was produced and the population's declared precondition rejected it.
            ///
            /// COUNTED, always. A rejection that silently burned budget would shrink the denominator without anybody being able to read that it had.
            PreconditionRejected => precondition_rejected,
            /// The generator declined the bytes it was handed and produced no command at all.
            GeneratorRefused => generator_refused,
            /// The generator broke the contract the driver drives it under: it reported a decoded command while consuming none of the case's bytes.
            GeneratorContractViolated => generator_contract_violated,
            /// The plan reached this case with one of its declared bounds already spent, so no draw was attempted.
            GenerationBudgetExhausted => budget_exhausted,
        }
    };
}

macro_rules! declare_generation_dispositions {
    ($($(#[$variant_meta:meta])* $variant:ident => $seat:ident),+ $(,)?) => {
        /// What became of one case the generator was asked for.
        ///
        /// # Authority
        ///
        /// This is the generation axis and only the generation axis.
        /// What an execution did with a generated case is [`crate::report::RunAttempt`]'s roster, and what a check concluded is [`crate::report::TrialConclusion`]'s: three owned axes, never one status blob.
        ///
        /// # Nonclaims
        ///
        /// A disposition says what became of one request for a case.
        /// It says nothing about whether the case that was produced found anything.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum GenerationDisposition {
            $($(#[$variant_meta])* $variant),+
        }

        /// How many disposition seats one census carries.
        ///
        /// The width is derived from the disposition roster, so a seat cannot be added to [`GenerationDisposition`] without the census growing with it.
        pub const GENERATION_DISPOSITION_SEATS: usize = [
            $(GenerationDisposition::$variant),+
        ]
        .len();

        /// The honest per-population accounting over generation dispositions.
        ///
        /// # Authority
        ///
        /// One count seat exists per arm of [`GenerationDisposition`], always, so the denominator cannot silently shrink.
        /// Every case a plan reached is counted exactly once under exactly one arm, and [`GenerationCensus::attempted`] is the sum rather than a separately maintained total.
        ///
        /// # Nonclaims
        ///
        /// It counts one population's cases under one drive.
        /// It is not the trial census, the selected-trial census, the mutant census, or the bench-sample census.
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

// ---------------------------------------------------------------------------
// The generation plan.
// ---------------------------------------------------------------------------

/// The seed one derived byte stream is addressed from.
///
/// # Nonclaims
///
/// A seed is not a replay account. [`crate::report::ReplayCapsule`] is the run-bound output shape that can carry the exact input and execution standing once their custody is established; a seed alone names a stream and nothing about what was run over it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RootSeed(u64);

/// Where one plan's input bytes come from.
///
/// # Authority
///
/// The two arms are the whole roster, and the union is the point: a plan states
/// a seed or states exact bytes, and neither is a naked integer standing alone
/// in a field somebody has to remember the meaning of.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputOrigin {
    /// A root seed, from which the paved byte source derives its stream.
    Seeded(RootSeed),
    /// Exact caller-supplied bytes, including a corpus warm start or replay material whose authority is owned elsewhere.
    Supplied(Vec<u8>),
}

/// How many bytes one generated case is drawn from.
///
/// # Construction
///
/// Zero is refused: every case under a zero width would be the empty input, and
/// a ramp that produced it would be spending a plan's whole case budget on one
/// input repeated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CaseWidth(usize);

/// Why one case width was refused.
#[must_use = "a refusal is the reason a case width was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CaseWidthRefusal {
    /// The width is zero, so every case drawn at it would be the empty input.
    ZeroBytes,
}

/// How a plan's case widths progress across its sequence of cases.
///
/// # Authority
///
/// The roster is closed at three ramps, and each one is a declaration a reader
/// can compute from the case ordinal alone — there is no ramp here that reads a
/// previous case's outcome, so widths are a function of the plan and never of
/// what the subject did.
///
/// # Bounds
///
/// Every ramp saturates rather than overflowing, and the driver caps each draw
/// at the plan's remaining byte budget — so the declared byte budget, not the
/// ramp, is the ceiling on how wide any one case gets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SizeProgression {
    /// Every case is drawn at one declared width.
    Constant {
        /// The width every case is drawn at.
        width: CaseWidth,
    },
    /// The first case is the base, and each case adds one step to the one
    /// before it.
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
/// The ordinal counts from zero, so the first case's width is the ramp's base.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CaseIndex(u32);

/// How many draws one plan admits that produce no case.
///
/// # Authority
///
/// The bound is over draws that came back empty-handed —
/// [`GenerationDisposition::PreconditionRejected`] and
/// [`GenerationDisposition::GeneratorRefused`] — because those are the two
/// outcomes that spend a case seat without filling it. The census still counts
/// them apart; this is one bound over their sum, not a place they are
/// flattened.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RejectionBudget(u32);

/// One plan's complete statement of what to generate and under which bounds.
///
/// It binds the population identity, the generation profile and its version,
/// the root seed or the exact supplied bytes, the case budget, the byte budget,
/// the rejection budget, and the size progression.
///
/// # Authority
///
/// The budgets here are the generator's own. There is no per-trial budget field
/// anywhere in the harness: the invocation's budgets suffice, and a row field
/// would be a second budget authority answering the same question.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerationPlan {
    population: PopulationRef,
    profile: GenerationProfile,
    origin: InputOrigin,
    cases: CaseBudget,
    bytes: ByteBudget,
    rejections: RejectionBudget,
    progression: SizeProgression,
}

/// Why one generation plan was refused.
///
/// Dependent checks in a declared order — the case budget is read, then the
/// byte budget, then the origin — so exactly one cause is true of any refused
/// plan.
#[must_use = "a refusal is the reason a generation plan was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GenerationPlanRefusal {
    /// The case budget admits no case, so the plan states a population it will
    /// never draw from.
    ZeroCaseBudget,
    /// The byte budget admits no byte, so the plan's first case could never be
    /// drawn.
    ZeroByteBudget,
    /// The origin supplies bytes and the supplied material is empty, so there is nothing to draw.
    EmptySuppliedBytes,
}

// ---------------------------------------------------------------------------
// The deterministic byte source.
// ---------------------------------------------------------------------------

/// The domain tag every generation byte-source address is derived under.
pub const GENERATION_SOURCE_TAG: DomainTag = DomainTag::declared(
    "generation-byte-source",
    IdentityProfileVersion::declared(1),
);

/// The domain tag every derived stream chunk is derived under.
///
/// Its own tag rather than the address's: an address and a chunk are different
/// kinds, and two kinds derived over preimages that could coincide must not
/// share a derivation context.
pub const GENERATION_CHUNK_TAG: DomainTag = DomainTag::declared(
    "generation-stream-chunk",
    IdentityProfileVersion::declared(1),
);

/// How many bytes one chunk of a byte stream carries.
///
/// The width is the identity substrate's address width, because a derived chunk
/// IS one address's bytes. The supplied arm reads the same grid, so one cursor
/// vocabulary addresses both arms.
pub const SOURCE_CHUNK_BYTES: usize = 32;

/// The address one plan's derived byte stream is counted from.
///
/// # Authority
///
/// It is derived from the plan's population, generation profile, and input
/// origin — and from nothing else. Growing the case budget or changing the size
/// progression re-windows the same stream rather than renaming it, so a longer
/// run reproduces every case a shorter one produced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ByteSourceAddress(ContentAddress);

/// One position in a byte stream: which chunk, and how far into it.
///
/// # Construction
///
/// A cursor is seekable: any position is nameable directly through
/// [`StreamCursor::at`] without drawing the positions before it, which is what
/// makes the derived stream addressable rather than merely repeatable.
///
/// # Bounds
///
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
/// # Authority
///
/// Both arms are deterministic, seekable, and free of ambient entropy: nothing
/// here reads a clock, an environment, or an operating-system random source, so
/// two runs of one plan on two machines draw identical bytes.
///
/// The derived arm is the paved road the generation contract recommends: chunk
/// N of the stream is the identity substrate's address of the source address
/// followed by the counter N, under [`GENERATION_CHUNK_TAG`]. No state carries
/// between chunks, which is exactly why any position is directly addressable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ByteSource {
    /// A counter-addressed stream over the identity substrate.
    Derived(ByteSourceAddress),
    /// The exact bytes a plan supplied, read over the same chunk grid.
    Supplied(Vec<u8>),
}

/// What one draw against a byte source yielded.
///
/// # Authority
///
/// A draw yields the width it was asked for or it yields nothing. A partial
/// tail is [`ByteDraw::Insufficient`] with both counts kept, because a case
/// drawn at less than its ramp's width is not the case the plan declared, and
/// recording it as generated would let the ramp quietly lie about what was
/// exercised.
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

// ---------------------------------------------------------------------------
// The sequence driver's seams.
// ---------------------------------------------------------------------------

/// The owner-supplied road from a case's bytes to one command.
///
/// # Authority
///
/// A function pointer rather than a closure or a trait object, so a decoder
/// carries no captured state and nothing ambient rides in with it. An owner
/// whose command type derives the generation vocabulary's `Arbitrary` reaches
/// this seam through
/// [`decode_arbitrary`](crate::generate::decode_arbitrary); an owner with a
/// hand-written decoder writes one directly. Both roads are the same seam, which
/// is why the temporal suites, sequence mutation, and chaos scheduling all drive
/// through one driver.
///
/// # Bounds
///
/// The contract the driver holds a decoder to: a decoder that reports a command
/// must have consumed at least one byte of the case. A decoder that reports a
/// command while consuming nothing is recorded as
/// [`GenerationDisposition::GeneratorContractViolated`], which is also what
/// makes the driver's decode loop provably terminate.
pub type CommandDecode<Command> = fn(&mut Unstructured<'_>) -> arbitrary::Result<Command>;

/// The owner-supplied precondition one decoded sequence is judged by.
///
/// A population without a precondition drives under
/// [`admit_every_sequence`](crate::generate::admit_every_sequence) rather than
/// under an absent one, so no road here has to decide what a missing
/// precondition would have meant.
pub type SequencePrecondition<Command> = fn(&[Command]) -> PreconditionVerdict;

/// Whether a population's declared precondition admits one sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PreconditionVerdict {
    /// The sequence satisfies the population's declared precondition.
    Admitted,
    /// The sequence does not, and the case is counted as rejected.
    Rejected,
}

/// One generated case: which case it is, the commands decoded from it, and the
/// exact bytes it was drawn from.
///
/// # Authority
///
/// The input bytes are the whole draw, including any tail the decoder did not
/// consume, because the draw is what the case was HANDED — which is what a
/// [`crate::report::ReplayCapsule`] carries and what a reduction shrinks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandSequence<Command> {
    case: CaseIndex,
    commands: Vec<Command>,
    input: Vec<u8>,
}

/// Why one plan's generation stopped.
///
/// # Authority
///
/// The halt names the bound that ended the drive; the census counts what became
/// of every case the drive reached. They answer different questions and neither
/// is derivable from the other: two plans can record one
/// [`GenerationDisposition::GenerationBudgetExhausted`] and have stopped on
/// different bounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GenerationHalt {
    /// The plan reached every case its case budget declared. This is the one
    /// arm that means the plan finished rather than stopped.
    CaseBudgetMet,
    /// The plan reached a case with its byte budget already spent.
    ByteBudgetExhausted,
    /// The plan reached a case with its rejection budget already spent.
    RejectionBudgetExhausted,
    /// The byte source held less than a case's ramp width asked for.
    SourceExhausted,
    /// A decoder reported a command while consuming none of the case's bytes.
    GeneratorContractViolated,
}

/// What one drive of one plan produced: the admitted sequences, the census over
/// every case the drive reached, and the bound that ended it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedSequences<Command> {
    sequences: Vec<CommandSequence<Command>>,
    census: GenerationCensus,
    halt: GenerationHalt,
}

// ---------------------------------------------------------------------------
// The reduction plan.
// ---------------------------------------------------------------------------

/// Which generic byte reducer a reduction plan binds.
///
/// # Authority
///
/// A closed roster rather than an open name, because the generic byte reducers
/// are this home's own realizations: the sole lawful value states what is true
/// of every reduction the harness runs. A plan therefore cannot name a byte
/// reducer nothing implements, so no refusal is owed for one.
///
/// # Bounds
///
/// A second generic reducer adds a variant beside this one, and adding it is a
/// law change rather than a new argument.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ByteReducerId {
    /// Chunk removal and chunk zeroing over the byte input, at halving window
    /// widths, run to a fixed point under the reduction budget.
    ChunkRemovalAndZeroing,
}

/// One owner-declared semantic reducer.
///
/// Open and namespaced, because a semantic reducer knows what the bytes MEAN
/// and that knowledge is the owner's. The plan names which ones a reduction ran
/// under; the law they run under is
/// [`shrink_verdict`](crate::generate::shrink_verdict), so a semantic pass and
/// the generic byte pass admit a candidate on exactly the same ground.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SemanticReducerId(NamespacedName);

/// That a reduction preserves the failure fingerprint.
///
/// # Authority
///
/// The sole lawful value, carried as a plan field rather than as an option, is
/// what makes preservation non-optional in the shape of a plan: there is no
/// reduction plan anybody can build that does not require it.
///
/// # Bounds
///
/// A second variant would be a law change, and the generation contract's
/// sentence about minimization would have to move first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FingerprintPreservation {
    /// The shrunk input must carry the same failure fingerprint.
    Required,
}

/// How many candidate probes one reduction admits.
///
/// The baseline probe is not a candidate and does not spend this budget; the
/// bound is over the shrinks a reduction offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ReductionBudget(u32);

/// One plan's complete statement of how a find is minimized.
///
/// It binds the minimization profile and its version, the generic byte reducer,
/// the semantic reducers, the required fingerprint preservation, and the
/// reduction budget.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReductionPlan {
    profile: MinimizationProfile,
    byte_reducer: ByteReducerId,
    semantic_reducers: BTreeSet<SemanticReducerId>,
    preservation: FingerprintPreservation,
    budget: ReductionBudget,
}

/// Why one reduction plan was refused.
///
/// Dependent checks in a declared order — the budget is read before the
/// semantic reducers.
#[must_use = "a refusal is the reason a reduction plan was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReductionPlanRefusal {
    /// The budget admits no probe, so the plan states a reduction that could
    /// never offer a shrink.
    ZeroReductionBudget,
    /// The semantic reducer roster names this reducer more than once.
    ///
    /// Refused rather than folded away, because collapsing a duplicate silently
    /// would be the harness normalizing an authoring defect out of sight.
    DuplicateSemanticReducer(SemanticReducerId),
}

// ---------------------------------------------------------------------------
// Minimization.
// ---------------------------------------------------------------------------

/// What one probe of one candidate input concluded.
///
/// # Authority
///
/// The two arms are different facts and the reduction treats them differently:
/// a candidate that stopped failing is not a candidate that found a different
/// bug, and a reduction that folded them together could not say which of the
/// two ended a pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProbeOutcome {
    /// The candidate failed, under this fingerprint.
    Reproduced(Fingerprint),
    /// The candidate did not fail at all.
    NoFailure,
}

/// The owner-supplied re-check one candidate input is judged by.
///
/// A function pointer for the same reason a decoder is one: a probe carries no
/// captured closure state. That shape does not establish semantic purity or
/// outcome stability; the probe's effects and readings remain its owner's
/// responsibility.
pub type FingerprintProbe = fn(&[u8]) -> ProbeOutcome;

/// Whether one candidate shrink is admitted.
///
/// # Authority
///
/// Acceptance is fingerprint equality and nothing else. A shrink that reaches a
/// different fingerprint is REJECTED and the fingerprint it reached is carried,
/// so a reduction never wanders from the bug it was minimizing and a reader can
/// see where it tried to wander to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShrinkVerdict {
    /// The candidate carries the same failure fingerprint, so it stands in for
    /// the input it shrinks.
    Accepted,
    /// The candidate failed under a different fingerprint.
    RejectedFingerprintMoved {
        /// The fingerprint the candidate reached instead.
        found: Fingerprint,
    },
    /// The candidate did not fail at all.
    RejectedNoFailure,
}

/// The honest accounting over one reduction's candidate probes.
///
/// # Nonclaims
///
/// It counts candidates, never bytes: how far an input actually shrank is the
/// outcome's input, not a number here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReductionCensus {
    accepted: u32,
    fingerprint_moved: u32,
    no_failure: u32,
}

/// Why one reduction stopped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReductionHalt {
    /// A whole round of passes admitted no candidate, so the input is at the
    /// reducer's fixed point.
    FixedPointReached,
    /// The reduction budget was spent before a fixed point was reached.
    BudgetExhausted,
}

/// What one reduction produced: the smallest input that kept the fingerprint,
/// that fingerprint, the census over every candidate, and the reason it
/// stopped.
///
/// # Nonclaims
///
/// The input is the smallest one this reduction REACHED under its budget and
/// its reducer. It is not the smallest input that reproduces the failure, and
/// [`ReductionHalt::BudgetExhausted`] is what says so.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReductionOutcome {
    input: Vec<u8>,
    fingerprint: Fingerprint,
    census: ReductionCensus,
    halt: ReductionHalt,
}

/// Why one reduction was refused before any candidate was offered.
///
/// # Authority
///
/// Both arms are read from the BASELINE probe — the reduction's first act is to
/// confirm that the input it was handed still carries the fingerprint it was
/// told to preserve. A reduction that skipped that step could shrink a passing
/// input forever, or minimize a find into a bug it was never asked about.
#[must_use = "a refusal is the reason a reduction did not run"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReductionRefusal {
    /// The input handed in does not fail at all, so there is no find to
    /// minimize.
    BaselineDidNotFail,
    /// The input handed in fails under a different fingerprint than the one it
    /// was told to preserve.
    BaselineFingerprintDiffers {
        /// The fingerprint the baseline actually reached.
        found: Fingerprint,
    },
}
