//! Every road that reaches a private field of the input-generation home, and every reader that hands one back.
//!
//! Declared inside `types.rs` as its own child, so it sees fields no sibling module does.
//! A width of zero, a plan that admits no case, and a cursor pointing past its own chunk are all refused here, which is what makes those claims structural rather than remembered.

use super::super::{draw, encode};
use super::{
    ByteDraw, ByteSource, ByteSourceAddress, CaseIndex, CaseWidth, CaseWidthRefusal,
    CommandSequence, GENERATION_DISPOSITION_SEATS, GENERATION_SOURCE_TAG, GeneratedSequences,
    GenerationCensus, GenerationCensusSeat, GenerationDisposition, GenerationHalt, GenerationPlan,
    GenerationPlanRefusal, InputOrigin, RejectionAllowance, RootSeed, SOURCE_CHUNK_BYTES,
    SizeProgression, StreamCursor, StreamCursorRefusal,
};
use crate::descriptor::PopulationRef;
use crate::identity::ContentAddress;
use crate::report::{ByteBudget, CaseBudget, GenerationProfile};
use std::num::NonZeroU32;

// The generation census, one seat per disposition.

macro_rules! implement_generation_census {
    ($($(#[$variant_meta:meta])* $variant:ident => $seat:ident),+ $(,)?) => {
        crate::census::implement_census! {
            impl GenerationCensus {
                count: u32,
                zero: 0u32,
                seat: GenerationCensusSeat,
                context { population: PopulationRef, }
                array counts [GENERATION_DISPOSITION_SEATS] {
                    $( $variant => $seat, )+
                }
            }
        }

        impl GenerationCensus {
            /// An accounting opened over one population, with every seat at zero.
            #[must_use]
            pub const fn over(population: PopulationRef) -> Self {
                Self::empty(population)
            }

            /// The population this accounting stands over.
            #[must_use]
            pub const fn population(&self) -> PopulationRef {
                self.population
            }

            /// Count one case under its disposition.
            ///
            /// Saturating rather than wrapping: a count that rolled over would read as a smaller denominator than the one that was reached.
            pub fn count(&mut self, disposition: GenerationDisposition) {
                let seat = match disposition {
                    $(GenerationDisposition::$variant => GenerationCensusSeat::$variant),+
                };
                self.increment(seat, 1u32);
            }

            /// How many cases fell under one disposition.
            #[must_use]
            pub const fn count_of(&self, disposition: GenerationDisposition) -> u32 {
                let seat = match disposition {
                    $(GenerationDisposition::$variant => GenerationCensusSeat::$variant),+
                };
                self.count_at(seat)
            }

            /// Every seat with its count, in the roster's declared order.
            ///
            /// A renderer walks this rather than the seats it happens to know about, so a new disposition cannot be silently left out of a report.
            #[must_use]
            pub const fn entries(&self) -> [(GenerationDisposition, u32); GENERATION_DISPOSITION_SEATS] {
                [$(
                    (
                        GenerationDisposition::$variant,
                        self.count_at(GenerationCensusSeat::$variant),
                    )
                ),+]
            }

            /// How many cases the drive reached, over every seat.
            ///
            /// The sum of the parts rather than a total kept beside them, so no second number can disagree with the seats it is made of.
            #[must_use]
            pub fn attempted(&self) -> u32 {
                self.entries()
                    .into_iter()
                    .map(|(_, count)| count)
                    .fold(0u32, u32::saturating_add)
            }
        }
    }
}

with_generation_dispositions!(implement_generation_census);

// The generation plan and the values it binds.

impl RootSeed {
    /// The seed the plan's author declared.
    #[must_use]
    pub const fn declared(seed: u64) -> Self {
        Self(seed)
    }

    /// The declared seed.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl InputOrigin {
    /// The discriminant byte a preimage carries for this arm.
    ///
    /// A slot rather than the Rust spelling, so renaming the variant leaves every address derived under it with its name.
    /// No slot is zero, so a zeroed buffer never reads back as a lawful arm.
    #[must_use]
    pub fn slot(&self) -> u8 {
        match self {
            Self::Seeded(_) => 1u8,
            Self::Supplied(_) => 2u8,
        }
    }
}

impl CaseWidth {
    /// The width the plan's author declared.
    ///
    /// # Errors
    ///
    /// Refuses zero.
    pub const fn declared(bytes: usize) -> Result<Self, CaseWidthRefusal> {
        if bytes == 0 {
            return Err(CaseWidthRefusal::ZeroBytes);
        }
        Ok(Self(bytes))
    }

    /// The declared width, in bytes.
    #[must_use]
    pub const fn bytes(self) -> usize {
        self.0
    }
}

impl CaseIndex {
    /// The case at one ordinal, counting from zero.
    #[must_use]
    pub const fn at(ordinal: u32) -> Self {
        Self(ordinal)
    }

    /// The ordinal.
    #[must_use]
    pub const fn ordinal(self) -> u32 {
        self.0
    }
}

impl SizeProgression {
    /// The width one case is drawn at under this ramp.
    ///
    /// Total and saturating: an ordinal that would carry a ramp past the widest value a width can hold yields that widest value, and the driver's cap at the plan's remaining byte budget is what keeps a saturated width from being drawn.
    #[must_use]
    pub fn width_at(&self, case: CaseIndex) -> CaseWidth {
        let ordinal = usize::try_from(case.ordinal()).unwrap_or(usize::MAX);
        let width = match *self {
            Self::Constant { width } => width.bytes(),
            Self::Linear { base, step } => step
                .bytes()
                .checked_mul(ordinal)
                .and_then(|grown| base.bytes().checked_add(grown))
                .unwrap_or(usize::MAX),
            Self::Doubling { base } => 1usize
                .checked_shl(case.ordinal())
                .and_then(|factor| base.bytes().checked_mul(factor))
                .unwrap_or(usize::MAX),
        };
        Self::at_least_one_byte(width)
    }

    /// One width, floored at a byte.
    ///
    /// Every ramp parameter is already non-zero, so the floor encodes that fact rather than repairing anything a caller could state.
    const fn at_least_one_byte(width: usize) -> CaseWidth {
        if width == 0 {
            CaseWidth(1)
        } else {
            CaseWidth(width)
        }
    }
}

impl RejectionAllowance {
    /// The rejection allowance the plan's author declared.
    ///
    /// Zero becomes [`RejectionAllowance::NoRejections`]; a positive value becomes [`RejectionAllowance::AtMost`] and is non-zero inside that arm.
    #[must_use]
    pub const fn declared(draws: u32) -> Self {
        match NonZeroU32::new(draws) {
            Some(draws) => Self::AtMost(draws),
            None => Self::NoRejections,
        }
    }

    /// How many empty-handed draws the plan admits.
    #[must_use]
    pub const fn draws(self) -> u32 {
        match self {
            Self::NoRejections => 0,
            Self::AtMost(draws) => draws.get(),
        }
    }
}

impl GenerationPlan {
    /// The plan its author declared.
    ///
    /// # Errors
    ///
    /// Refuses a zero case budget, then a zero byte budget, then an origin that supplies no bytes.
    pub fn declared(
        population: PopulationRef,
        profile: GenerationProfile,
        origin: InputOrigin,
        cases: CaseBudget,
        bytes: ByteBudget,
        rejection_allowance: RejectionAllowance,
        progression: SizeProgression,
    ) -> Result<Self, GenerationPlanRefusal> {
        if cases.cases() == 0 {
            return Err(GenerationPlanRefusal::ZeroCaseBudget);
        }
        if bytes.bytes() == 0 {
            return Err(GenerationPlanRefusal::ZeroByteBudget);
        }
        match &origin {
            InputOrigin::Seeded(_) => {}
            InputOrigin::Supplied(material) => {
                if material.is_empty() {
                    return Err(GenerationPlanRefusal::EmptySuppliedBytes);
                }
            }
        }
        Ok(Self {
            population,
            profile,
            origin,
            cases,
            bytes,
            rejection_allowance,
            progression,
        })
    }

    /// The population this plan supplies inputs for.
    #[must_use]
    pub const fn population(&self) -> PopulationRef {
        self.population
    }

    /// The generation profile and version the plan generates under.
    #[must_use]
    pub const fn profile(&self) -> GenerationProfile {
        self.profile
    }

    /// Where the plan's input bytes come from.
    #[must_use]
    pub const fn origin(&self) -> &InputOrigin {
        &self.origin
    }

    /// How many cases the plan admits.
    #[must_use]
    pub const fn cases(&self) -> CaseBudget {
        self.cases
    }

    /// How many input bytes the plan admits.
    #[must_use]
    pub const fn bytes(&self) -> ByteBudget {
        self.bytes
    }

    /// How many empty-handed draws the plan admits.
    #[must_use]
    pub const fn rejection_allowance(&self) -> RejectionAllowance {
        self.rejection_allowance
    }

    /// How the plan's case widths progress.
    #[must_use]
    pub const fn progression(&self) -> SizeProgression {
        self.progression
    }
}

// The byte source, its address, and its cursor.

impl ByteSourceAddress {
    /// The address one plan's derived stream is counted from.
    ///
    /// Deterministic and total: every plan names a stream, with no ambient fact anywhere in the derivation.
    #[must_use]
    pub fn of_plan(plan: &GenerationPlan) -> Self {
        Self(ContentAddress::derived(
            GENERATION_SOURCE_TAG,
            &encode::source_preimage(plan),
        ))
    }

    /// The address, over a content address already minted.
    #[must_use]
    pub const fn over(address: ContentAddress) -> Self {
        Self(address)
    }

    /// The content address this value carries.
    #[must_use]
    pub const fn address(self) -> ContentAddress {
        self.0
    }
}

impl StreamCursor {
    /// The position a stream is read from before anything is drawn.
    #[must_use]
    pub const fn opening() -> Self {
        Self {
            chunk: 0,
            within: 0,
        }
    }

    /// The position at one chunk and one offset inside it.
    ///
    /// # Errors
    ///
    /// Refuses an offset that is not inside the chunk.
    pub const fn at(chunk: u64, within: usize) -> Result<Self, StreamCursorRefusal> {
        if within >= SOURCE_CHUNK_BYTES {
            return Err(StreamCursorRefusal::WithinPastChunk { within });
        }
        Ok(Self { chunk, within })
    }

    /// Which chunk the position is in.
    #[must_use]
    pub const fn chunk(self) -> u64 {
        self.chunk
    }

    /// How far into that chunk the position is.
    #[must_use]
    pub const fn within(self) -> usize {
        self.within
    }

    /// The position one draw of a given width ends at.
    ///
    /// Saturating at the last addressable chunk, a ceiling no lawful plan reaches — a plan would have to draw thirty-two times more bytes than its byte budget can count — and stated so that no road here has a panic in it.
    pub(crate) fn advanced(self, width: usize) -> Self {
        let mut chunk = self.chunk;
        let mut within = self.within;
        let mut left = width;
        while left > 0 {
            let available = SOURCE_CHUNK_BYTES.saturating_sub(within);
            let taken = left.min(available);
            left = left.saturating_sub(taken);
            if taken < available {
                within = within.saturating_add(taken);
            } else {
                within = 0;
                chunk = chunk.saturating_add(1);
            }
        }
        Self { chunk, within }
    }
}

impl ByteSource {
    /// The source one plan draws from.
    ///
    /// A seeded origin builds the paved counter-addressed stream over the plan's own address; a supplied origin builds the exact caller-supplied bytes without assigning them replay authority.
    #[must_use]
    pub fn of_plan(plan: &GenerationPlan) -> Self {
        match plan.origin() {
            InputOrigin::Seeded(_) => Self::Derived(ByteSourceAddress::of_plan(plan)),
            InputOrigin::Supplied(material) => Self::Supplied(material.clone()),
        }
    }

    /// One draw of the requested width, beginning at the cursor.
    ///
    /// Deterministic on both arms: the same source, cursor, and width yield the same bytes every time, and the cursor handed back is where the next draw begins.
    #[must_use]
    pub fn draw(&self, cursor: StreamCursor, width: usize) -> ByteDraw {
        match self {
            Self::Derived(address) => draw::from_stream(*address, cursor, width),
            Self::Supplied(material) => draw::from_material(material, cursor, width),
        }
    }
}

// What the driver produces.

impl<Command> CommandSequence<Command> {
    /// One admitted case: the ordinal, the commands decoded from it, and the exact bytes it was drawn from.
    #[must_use]
    pub fn generated(case: CaseIndex, commands: Vec<Command>, input: Vec<u8>) -> Self {
        Self {
            case,
            commands,
            input,
        }
    }

    /// Which case of the plan's sequence this is.
    #[must_use]
    pub const fn case(&self) -> CaseIndex {
        self.case
    }

    /// The commands, for a caller that drives them.
    #[must_use]
    pub fn commands(&self) -> &[Command] {
        &self.commands
    }

    /// The exact bytes the case was handed.
    #[must_use]
    pub fn input(&self) -> &[u8] {
        &self.input
    }

    /// The commands, taken by a caller that consumes them.
    #[must_use]
    pub fn into_commands(self) -> Vec<Command> {
        self.commands
    }
}

impl<Command> GeneratedSequences<Command> {
    /// What one drive produced.
    #[must_use]
    pub fn produced(
        sequences: Vec<CommandSequence<Command>>,
        census: GenerationCensus,
        halt: GenerationHalt,
    ) -> Self {
        Self {
            sequences,
            census,
            halt,
        }
    }

    /// The sequences the precondition admitted.
    #[must_use]
    pub fn sequences(&self) -> &[CommandSequence<Command>] {
        &self.sequences
    }

    /// The accounting over every case the drive reached.
    #[must_use]
    pub const fn census(&self) -> GenerationCensus {
        self.census
    }

    /// The bound that ended the drive.
    #[must_use]
    pub const fn halt(&self) -> GenerationHalt {
        self.halt
    }

    /// The sequences, taken by a caller that consumes them.
    #[must_use]
    pub fn into_sequences(self) -> Vec<CommandSequence<Command>> {
        self.sequences
    }
}
