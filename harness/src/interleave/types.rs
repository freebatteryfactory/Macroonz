//! Every public type of the interleaving home, declared and nothing else.
//!
//! Construction and reading live in this module's own child `type_guard.rs`, which is where the private fields are reachable.
//!
//! Some of the vocabulary arrives already made: the names are [`crate::descriptor`]'s, the census, halt, plan refusal, and case index are [`crate::generate`]'s, and a counterexample's finding is [`crate::report`]'s.
//! What is declared here is the schedule vocabulary alone: parties, merge orders, the bound an exploration runs under, and what a walk of the space leaves behind.

use crate::descriptor::NamespacedName;
use crate::generate::{
    CaseIndex, CaseWidth, GenerationCensus, GenerationHalt, GenerationPlanRefusal,
};
use crate::report::{FindingCause, TrialFinding};

#[path = "type_guard.rs"]
mod guard;

/// The owner every cause this home cites is declared under.
///
/// Qualified with the harness's own name, like every sibling family, so a consumer declaring a bare `interleave` family cannot alias a fingerprint of this home's.
const CAUSE_FAMILY: &str = "macroonz.interleave";

/// The cause a sampling drive that stopped short of its declared samples is cited under.
///
/// An all-pass over fewer schedules than the bound declared is unexercised evidence, not a pass, on the same ground [`crate::properties::NO_SEQUENCE_DRIVEN`] refuses an empty drive.
pub const EXPLORATION_STARVED: FindingCause =
    FindingCause::named(CAUSE_FAMILY, "exploration-starved");

/// How many strands one choice byte can address.
///
/// A canonical interleaving spells one strand ordinal per step in a single byte, so a set larger than this could declare a party no choice string can name.
pub const ADDRESSABLE_STRANDS: usize = 256;

// The parties.

/// One named concurrent party: the commands it will issue, in its own program order.
///
/// The word is Cilk's, for a serial chain of steps, and no operating-system thread is implied by it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Strand<Command> {
    name: NamespacedName,
    commands: Vec<Command>,
}

/// Why one strand was refused.
#[must_use = "a refusal is the reason a strand was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrandRefusal {
    /// The strand declares no command, so it names a party that never acts.
    EmptyStrand(NamespacedName),
}

/// The concurrent parties together: at least two, uniquely named, addressable by one choice byte.
///
/// The step total and the case width every sampled draw uses are established here, so no road downstream re-derives either.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrandSet<Command> {
    strands: Vec<Strand<Command>>,
    steps: usize,
    width: CaseWidth,
}

/// Why one strand set was refused.
#[must_use = "a refusal is the reason a strand set was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrandSetRefusal {
    /// Two strands declare the same name.
    DuplicateStrand(NamespacedName),
    /// The set declares more strands than [`ADDRESSABLE_STRANDS`].
    MoreStrandsThanAddressable {
        /// How many strands the set declared.
        strands: usize,
    },
    /// The strands' step total is past what an address can hold.
    StepsUnaddressable,
    /// The set declares fewer than two strands, so there is nothing to reorder.
    FewerThanTwoStrands {
        /// How many strands the set declared.
        strands: usize,
    },
}

// The merge orders.

/// One merge order, spelled as the canonical choice string: which strand stepped next, one ordinal per step.
///
/// A spelling rather than a checked fact — whether it belongs to a particular strand set is judged where the two meet, at [`encoded`](crate::interleave::encoded), the way a scheduled position is judged at injection.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Interleaving {
    choices: Vec<u8>,
}

/// One realized merge: the canonical interleaving, and the commands in the order it merged them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterleavedSequence<Command> {
    interleaving: Interleaving,
    commands: Vec<Command>,
}

/// Why one interleaving could not be written as material over a strand set.
#[must_use = "a refusal is the reason an interleaving was not encoded"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncodingRefusal {
    /// The interleaving spells a different number of steps than the set holds.
    StepsMismatch {
        /// How many steps the interleaving spells.
        declared: usize,
        /// How many steps the set holds.
        steps: usize,
    },
    /// A choice names an ordinal no strand in the set owns.
    ChoiceOutsideStrands {
        /// The zero-based step the choice sits at.
        at: usize,
        /// The ordinal the choice spelled.
        choice: u8,
    },
    /// A choice draws a strand past its own length.
    StrandExhausted {
        /// The zero-based step the choice sits at.
        at: usize,
        /// The ordinal the choice spelled.
        choice: u8,
    },
}

// The bound and the space.

/// The declared budget one exploration runs under.
///
/// The interleaving seat is the ceiling under which the space is walked exhaustively; the sample seat is how many schedules are drawn when the space is beyond it.
/// Both are the author's statement, never runner-tuned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExplorationBound {
    interleavings: u32,
    samples: u32,
}

/// Why one exploration bound was refused.
#[must_use = "a refusal is the reason an exploration bound was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExplorationBoundRefusal {
    /// The bound admits no interleaving, so no space could ever be walked under it.
    ZeroInterleavings,
    /// The bound admits no sample, so a space beyond the ceiling could never be drawn from.
    ZeroSamples,
}

/// How many interleavings the strand set admits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InterleavingSpace {
    /// The space holds exactly this many interleavings.
    Counted(u128),
    /// The count left the counter's range before it was established.
    BeyondCount,
}

// What one walk of the space leaves behind.

/// Which way the space was walked, with the evidence that walk produced.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExplorationMode {
    /// Every interleaving in the space was enumerated, in ascending position order.
    Exhaustive,
    /// Choice bytes were drawn through the one shared sequence driver, and this is what generation reported.
    Sampled {
        /// What became of every case the sampling drive reached.
        census: GenerationCensus,
        /// Why the sampling drive stopped.
        halt: GenerationHalt,
    },
}

/// Where one counterexample was found.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExplorationSite {
    /// At this zero-based position of the exhaustive enumeration.
    Enumerated {
        /// The interleaving's position in ascending enumeration order.
        ordinal: u64,
    },
    /// At this case of the sampling drive.
    Sampled {
        /// The generated case the failing schedule was drawn at.
        case: CaseIndex,
    },
}

/// One interleaving whose merged history broke a claim: where it was found, the order itself, and the typed finding.
///
/// The merged history is not carried, because the interleaving re-derives it: [`encoded`](crate::interleave::encoded) writes the order as material and [`interpreted`](crate::interleave::interpreted) realizes it, which is the replay.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Counterexample {
    site: ExplorationSite,
    interleaving: Interleaving,
    finding: TrialFinding,
}

/// What the walked evidence establishes.
///
/// The two all-hold arms are kept apart because they claim different worlds: an exhausted space is a statement about every interleaving, and a clean sample is a statement about the sampled schedules and nothing more.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExplorationStanding {
    /// Every interleaving in the space was driven, and every claim held of every history.
    SpaceExhaustedAllHold,
    /// Every sampled schedule held; the space itself was not exhausted, and this standing cannot say more.
    SampledAllHold,
    /// This interleaving's history broke a claim.
    CounterexampleFound(Counterexample),
}

/// What one exploration produced: the counted space, the mode with its evidence, how many interleavings were judged, and the standing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExplorationReading {
    space: InterleavingSpace,
    mode: ExplorationMode,
    explored: u64,
    standing: ExplorationStanding,
}

/// Why one exploration was refused.
///
/// Both arms belong to the sampling road; an exhaustive walk refuses nothing beyond what the set and the bound already refused at construction.
#[must_use = "a refusal is the reason an exploration did not run"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExplorationRefusal {
    /// The sample seat times the step total is past what a byte budget can hold.
    SampleBytesOverflow {
        /// The bound's sample seat.
        samples: u32,
        /// The set's step total.
        steps: usize,
    },
    /// The sampling plan was refused, carried rather than unwrapped.
    ///
    /// Unreachable while the bound refuses zero seats and the set refuses zero steps, and carried anyway because this home does not decide for the plan.
    SamplingPlanRefused(GenerationPlanRefusal),
}
