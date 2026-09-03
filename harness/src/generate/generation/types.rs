//! Every public type of the input-generation home, declared and nothing else.
//!
//! Construction and reading live in this module's own child `type_guard.rs`, which is where the private fields are reachable.
//!
//! Some of the vocabulary arrives already made: the profiles and budgets are [`crate::report`]'s, and the population reference is [`crate::descriptor`]'s.
//! A plan binds those values; what they mean is written where they are declared.

use crate::descriptor::PopulationRef;
use crate::identity::{DomainTag, IdentityProfileVersion};
use crate::report::{ByteBudget, CaseBudget, GenerationProfile};
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

        crate::census::declare_census! {
            /// The per-population accounting over generation dispositions.
            ///
            /// One seat exists per arm of [`GenerationDisposition`], always, so the denominator cannot silently shrink.
            /// Every case a plan reached is counted once, and [`GenerationCensus::attempted`] is the sum of the seats rather than a total kept beside them.
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub struct GenerationCensus {
                count: u32,
                seat: GenerationCensusSeat,
                context { population: PopulationRef, }
                array counts [GENERATION_DISPOSITION_SEATS] {
                    $( $variant => $seat, )+
                }
            }
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

crate::identity::content_address_reference! {
    /// The address one plan's derived byte stream is counted from.
    ///
    /// It is derived from the plan's population, generation profile, and input origin, and from nothing else.
    /// Growing the case budget or changing the size progression re-windows the same stream rather than renaming it, so a longer run reproduces every case a shorter one produced.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ByteSourceAddress;
}

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
