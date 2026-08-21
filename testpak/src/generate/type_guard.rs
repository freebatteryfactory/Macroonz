//! The generation contract's invariant nucleus: every road that reaches a
//! private field, and every reader that hands one back.
//!
//! Declared inside `types.rs` as its own child, so it sees the fields the
//! declarations keep private and no sibling module does. A width of zero, a
//! plan that admits no case, a cursor pointing past its own chunk, and a
//! reduction plan naming one semantic reducer twice are all refused HERE, which
//! is what makes those claims structural rather than remembered.
//!
//! # The derivations
//!
//! Two identities are minted in this file: the address one derived byte stream
//! is counted from, and the chunk at one counter of that stream. Both are
//! content addresses over preimages written here, framed through the record
//! instrument's published length framing ([`crate::report::encode_bytes`])
//! rather than a second framing invented locally.

use super::{
    ByteDraw, ByteReducerId, ByteSource, ByteSourceAddress, CaseIndex, CaseWidth, CaseWidthRefusal,
    CommandSequence, FingerprintPreservation, GENERATION_CHUNK_TAG, GENERATION_DISPOSITION_SEATS,
    GENERATION_SOURCE_TAG, GeneratedSequences, GenerationCensus, GenerationDisposition,
    GenerationHalt, GenerationPlan, GenerationPlanRefusal, InputOrigin, ReductionBudget,
    ReductionCensus, ReductionHalt, ReductionOutcome, ReductionPlan, ReductionPlanRefusal,
    RejectionBudget, RootSeed, SOURCE_CHUNK_BYTES, SemanticReducerId, ShrinkVerdict,
    SizeProgression, StreamCursor, StreamCursorRefusal,
};
use crate::descriptor::{NameRefusal, NamespacedName, PopulationRef};
use crate::identity::ContentAddress;
use crate::report::{
    ByteBudget, CaseBudget, Fingerprint, GenerationProfile, MinimizationProfile, encode_bytes,
};
use std::collections::BTreeSet;

// ---------------------------------------------------------------------------
// The generation axis.
// ---------------------------------------------------------------------------

impl GenerationCensus {
    /// An accounting opened over one population, with every seat at zero.
    #[must_use]
    pub const fn over(population: PopulationRef) -> Self {
        Self {
            population,
            generated: 0,
            bytes_insufficient: 0,
            precondition_rejected: 0,
            generator_refused: 0,
            generator_contract_violated: 0,
            budget_exhausted: 0,
        }
    }

    /// The population this accounting stands over.
    #[must_use]
    pub const fn population(&self) -> PopulationRef {
        self.population
    }

    /// Count one case under its disposition.
    ///
    /// Saturating rather than wrapping: a count that rolled over would read as
    /// a smaller denominator than the one that was actually reached.
    pub fn count(&mut self, disposition: GenerationDisposition) {
        match disposition {
            GenerationDisposition::Generated => {
                self.generated = self.generated.saturating_add(1);
            }
            GenerationDisposition::BytesInsufficient => {
                self.bytes_insufficient = self.bytes_insufficient.saturating_add(1);
            }
            GenerationDisposition::PreconditionRejected => {
                self.precondition_rejected = self.precondition_rejected.saturating_add(1);
            }
            GenerationDisposition::GeneratorRefused => {
                self.generator_refused = self.generator_refused.saturating_add(1);
            }
            GenerationDisposition::GeneratorContractViolated => {
                self.generator_contract_violated =
                    self.generator_contract_violated.saturating_add(1);
            }
            GenerationDisposition::GenerationBudgetExhausted => {
                self.budget_exhausted = self.budget_exhausted.saturating_add(1);
            }
        }
    }

    /// How many cases fell under one disposition.
    #[must_use]
    pub const fn count_of(&self, disposition: GenerationDisposition) -> u32 {
        match disposition {
            GenerationDisposition::Generated => self.generated,
            GenerationDisposition::BytesInsufficient => self.bytes_insufficient,
            GenerationDisposition::PreconditionRejected => self.precondition_rejected,
            GenerationDisposition::GeneratorRefused => self.generator_refused,
            GenerationDisposition::GeneratorContractViolated => self.generator_contract_violated,
            GenerationDisposition::GenerationBudgetExhausted => self.budget_exhausted,
        }
    }

    /// Every seat with its count, in the disposition roster's declared order.
    ///
    /// A renderer walks this rather than reading the seats it happens to know
    /// about, so a disposition added to the roster cannot be silently left out
    /// of a report.
    #[must_use]
    pub const fn entries(&self) -> [(GenerationDisposition, u32); GENERATION_DISPOSITION_SEATS] {
        [
            (GenerationDisposition::Generated, self.generated),
            (
                GenerationDisposition::BytesInsufficient,
                self.bytes_insufficient,
            ),
            (
                GenerationDisposition::PreconditionRejected,
                self.precondition_rejected,
            ),
            (
                GenerationDisposition::GeneratorRefused,
                self.generator_refused,
            ),
            (
                GenerationDisposition::GeneratorContractViolated,
                self.generator_contract_violated,
            ),
            (
                GenerationDisposition::GenerationBudgetExhausted,
                self.budget_exhausted,
            ),
        ]
    }

    /// How many cases the drive reached, over every seat.
    ///
    /// The sum of the parts rather than a total kept beside them, so there is
    /// no second number that could disagree with the seats it is made of.
    #[must_use]
    pub fn attempted(&self) -> u32 {
        self.entries()
            .into_iter()
            .map(|(_, count)| count)
            .fold(0u32, u32::saturating_add)
    }
}

// ---------------------------------------------------------------------------
// The generation plan.
// ---------------------------------------------------------------------------

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
    /// A slot rather than the Rust spelling: renaming the variant leaves every
    /// address derived under it with its name. No slot is zero, so a zeroed
    /// buffer never reads back as a lawful arm.
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
    /// # Bounds
    ///
    /// Total and saturating: an ordinal that would carry a ramp past the widest
    /// value a width can hold yields that widest value rather than wrapping,
    /// and the driver's own cap at the plan's remaining byte budget is what
    /// keeps a saturated width from being drawn.
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
    /// Every ramp parameter is already non-zero, so the floor is the encoding
    /// of that fact rather than a repair of anything a caller could state.
    const fn at_least_one_byte(width: usize) -> CaseWidth {
        if width == 0 {
            CaseWidth(1)
        } else {
            CaseWidth(width)
        }
    }
}

impl RejectionBudget {
    /// The rejection budget the plan's author declared.
    #[must_use]
    pub const fn declared(draws: u32) -> Self {
        Self(draws)
    }

    /// How many empty-handed draws the plan admits.
    #[must_use]
    pub const fn draws(self) -> u32 {
        self.0
    }
}

impl GenerationPlan {
    /// The plan its author declared.
    ///
    /// # Errors
    ///
    /// Refuses a zero case budget, then a zero byte budget, then an origin that
    /// supplies no bytes.
    pub fn declared(
        population: PopulationRef,
        profile: GenerationProfile,
        origin: InputOrigin,
        cases: CaseBudget,
        bytes: ByteBudget,
        rejections: RejectionBudget,
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
            rejections,
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
    pub const fn rejections(&self) -> RejectionBudget {
        self.rejections
    }

    /// How the plan's case widths progress.
    #[must_use]
    pub const fn progression(&self) -> SizeProgression {
        self.progression
    }
}

// ---------------------------------------------------------------------------
// The deterministic byte source.
// ---------------------------------------------------------------------------

/// The COMPLETE preimage one [`ByteSourceAddress`] is derived from.
///
/// Two primitives, both the record instrument's:
///
/// - `u32be(n)` / `u64be(n)` — the integer in four or eight big-endian bytes.
/// - `bytes(x)` — `u64be(x.len())` followed by the bytes of `x`.
///
/// The members, in exactly this order, with no separators and no padding:
///
/// | # | member | encoding |
/// | - | ------ | -------- |
/// | 1 | population namespace | `bytes(utf8)` |
/// | 2 | population stem | `bytes(utf8)` |
/// | 3 | generation profile name | `bytes(utf8)` |
/// | 4 | generation profile version | `u32be` |
/// | 5 | origin arm | one byte, [`InputOrigin::slot`] |
/// | 6 | origin payload | `u64be` of the seed, or `bytes(…)` of the supplied material |
///
/// The budgets and the size progression are deliberately absent. A stream is
/// what the population, the profile, and the origin name; how much of it one
/// run draws and how it is cut into cases are the plan's windowing, so growing
/// a budget or changing a ramp re-windows the same stream instead of renaming
/// it.
fn source_preimage(plan: &GenerationPlan) -> Vec<u8> {
    let mut preimage: Vec<u8> = Vec::new();
    let population = plan.population().name();
    encode_bytes(population.namespace().written().as_bytes(), &mut preimage);
    encode_bytes(population.stem().written().as_bytes(), &mut preimage);
    encode_bytes(plan.profile().name().as_bytes(), &mut preimage);
    preimage.extend_from_slice(&plan.profile().version().to_be_bytes());
    preimage.push(plan.origin().slot());
    match plan.origin() {
        InputOrigin::Seeded(seed) => preimage.extend_from_slice(&seed.value().to_be_bytes()),
        InputOrigin::Supplied(material) => encode_bytes(material, &mut preimage),
    }
    preimage
}

/// The COMPLETE preimage one derived chunk is addressed by.
///
/// | # | member | encoding |
/// | - | ------ | -------- |
/// | 1 | source address | `bytes(…)` of the full thirty-two |
/// | 2 | counter | `u64be` |
///
/// Nothing carries between chunks: chunk N is a function of the address and N
/// alone, which is what makes any position directly addressable and what keeps
/// two machines drawing identical bytes.
fn chunk_material(address: ByteSourceAddress, counter: u64) -> [u8; SOURCE_CHUNK_BYTES] {
    let mut preimage: Vec<u8> = Vec::new();
    encode_bytes(address.address().as_bytes(), &mut preimage);
    preimage.extend_from_slice(&counter.to_be_bytes());
    *ContentAddress::derived(GENERATION_CHUNK_TAG, &preimage).as_bytes()
}

impl ByteSourceAddress {
    /// The address one plan's derived stream is counted from.
    ///
    /// Deterministic and total: every plan names a stream, on any machine, with
    /// no ambient fact anywhere in the derivation.
    #[must_use]
    pub fn of_plan(plan: &GenerationPlan) -> Self {
        Self(ContentAddress::derived(
            GENERATION_SOURCE_TAG,
            &source_preimage(plan),
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
}

/// The position one draw of a given width ends at.
///
/// Saturating at the last addressable chunk. The ceiling is unreachable under
/// any lawful plan — a plan would have to draw thirty-two times more bytes than
/// its byte budget can even count — and it is stated so that no road here has a
/// panic in it.
fn advanced(cursor: StreamCursor, width: usize) -> StreamCursor {
    let mut chunk = cursor.chunk();
    let mut within = cursor.within();
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
    StreamCursor { chunk, within }
}

/// One draw against the counter-addressed stream.
///
/// The stream is unbounded, so this arm always yields the width it was asked
/// for: byte insufficiency is the supplied arm's fact, not this one's.
fn derived_draw(address: ByteSourceAddress, cursor: StreamCursor, width: usize) -> ByteDraw {
    let mut bytes: Vec<u8> = Vec::new();
    let mut position = cursor;
    while bytes.len() < width {
        let wanted = width.saturating_sub(bytes.len());
        let available = SOURCE_CHUNK_BYTES.saturating_sub(position.within());
        let taken = wanted.min(available);
        let material = chunk_material(address, position.chunk());
        bytes.extend(material.iter().copied().skip(position.within()).take(taken));
        position = advanced(position, taken);
    }
    ByteDraw::Drawn {
        bytes,
        next: position,
    }
}

/// One draw against supplied bytes, read over the same chunk grid.
fn supplied_draw(material: &[u8], cursor: StreamCursor, width: usize) -> ByteDraw {
    let addressed = usize::try_from(cursor.chunk())
        .ok()
        .and_then(|chunk| chunk.checked_mul(SOURCE_CHUNK_BYTES))
        .and_then(|base| base.checked_add(cursor.within()));
    let Some(offset) = addressed else {
        return ByteDraw::Insufficient {
            requested: width,
            available: 0,
        };
    };
    let available = material.len().saturating_sub(offset);
    if available < width {
        return ByteDraw::Insufficient {
            requested: width,
            available,
        };
    }
    ByteDraw::Drawn {
        bytes: material.iter().copied().skip(offset).take(width).collect(),
        next: advanced(cursor, width),
    }
}

impl ByteSource {
    /// The source one plan draws from.
    ///
    /// A seeded origin builds the paved counter-addressed stream over the
    /// plan's own address; a supplied origin builds the exact bytes.
    #[must_use]
    pub fn of_plan(plan: &GenerationPlan) -> Self {
        match plan.origin() {
            InputOrigin::Seeded(_) => Self::Derived(ByteSourceAddress::of_plan(plan)),
            InputOrigin::Supplied(material) => Self::Supplied(material.clone()),
        }
    }

    /// One draw of the requested width, beginning at the cursor.
    ///
    /// Deterministic and free of ambient entropy on both arms: the same source,
    /// cursor, and width yield the same bytes on any machine, and the cursor
    /// handed back is where the next draw begins.
    #[must_use]
    pub fn draw(&self, cursor: StreamCursor, width: usize) -> ByteDraw {
        match self {
            Self::Derived(address) => derived_draw(*address, cursor, width),
            Self::Supplied(material) => supplied_draw(material, cursor, width),
        }
    }
}

// ---------------------------------------------------------------------------
// The driver's products.
// ---------------------------------------------------------------------------

impl<Command> CommandSequence<Command> {
    /// One admitted case: the ordinal, the commands decoded from it, and the
    /// exact bytes it was drawn from.
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

// ---------------------------------------------------------------------------
// The reduction plan.
// ---------------------------------------------------------------------------

impl SemanticReducerId {
    /// This reducer, parsed from the owner that declares it and the spelling it
    /// carries.
    ///
    /// # Errors
    ///
    /// Refuses an empty namespace, then an empty stem.
    pub fn named(namespace: &'static str, stem: &'static str) -> Result<Self, NameRefusal> {
        NamespacedName::named(namespace, stem).map(Self)
    }

    /// This reducer, over a name already parsed.
    #[must_use]
    pub const fn over(name: NamespacedName) -> Self {
        Self(name)
    }

    /// The namespaced name this reducer carries.
    #[must_use]
    pub const fn name(self) -> NamespacedName {
        self.0
    }
}

impl ReductionBudget {
    /// The probe budget the plan's author declared.
    #[must_use]
    pub const fn declared(probes: u32) -> Self {
        Self(probes)
    }

    /// How many candidate probes the plan admits.
    #[must_use]
    pub const fn probes(self) -> u32 {
        self.0
    }
}

impl ReductionPlan {
    /// The plan its author declared.
    ///
    /// # Errors
    ///
    /// Refuses a zero budget, then a semantic reducer named more than once.
    pub fn declared(
        profile: MinimizationProfile,
        byte_reducer: ByteReducerId,
        semantic_reducers: &[SemanticReducerId],
        preservation: FingerprintPreservation,
        budget: ReductionBudget,
    ) -> Result<Self, ReductionPlanRefusal> {
        if budget.probes() == 0 {
            return Err(ReductionPlanRefusal::ZeroReductionBudget);
        }
        let mut roster: BTreeSet<SemanticReducerId> = BTreeSet::new();
        for reducer in semantic_reducers {
            if !roster.insert(*reducer) {
                return Err(ReductionPlanRefusal::DuplicateSemanticReducer(*reducer));
            }
        }
        Ok(Self {
            profile,
            byte_reducer,
            semantic_reducers: roster,
            preservation,
            budget,
        })
    }

    /// The minimization profile and version the reduction runs under.
    #[must_use]
    pub const fn profile(&self) -> MinimizationProfile {
        self.profile
    }

    /// The generic byte reducer the plan binds.
    #[must_use]
    pub const fn byte_reducer(&self) -> ByteReducerId {
        self.byte_reducer
    }

    /// The semantic reducers the plan binds, in their storage order.
    #[must_use]
    pub const fn semantic_reducers(&self) -> &BTreeSet<SemanticReducerId> {
        &self.semantic_reducers
    }

    /// That the reduction preserves the failure fingerprint.
    #[must_use]
    pub const fn preservation(&self) -> FingerprintPreservation {
        self.preservation
    }

    /// How many candidate probes the reduction admits.
    #[must_use]
    pub const fn budget(&self) -> ReductionBudget {
        self.budget
    }
}

// ---------------------------------------------------------------------------
// Minimization.
// ---------------------------------------------------------------------------

impl ReductionCensus {
    /// An accounting opened with every seat at zero.
    #[must_use]
    pub const fn opening() -> Self {
        Self {
            accepted: 0,
            fingerprint_moved: 0,
            no_failure: 0,
        }
    }

    /// Count one candidate under the verdict it earned.
    pub fn count(&mut self, verdict: ShrinkVerdict) {
        match verdict {
            ShrinkVerdict::Accepted => self.accepted = self.accepted.saturating_add(1),
            ShrinkVerdict::RejectedFingerprintMoved { found: _ } => {
                self.fingerprint_moved = self.fingerprint_moved.saturating_add(1);
            }
            ShrinkVerdict::RejectedNoFailure => {
                self.no_failure = self.no_failure.saturating_add(1);
            }
        }
    }

    /// How many candidates carried the fingerprint through.
    #[must_use]
    pub const fn accepted(self) -> u32 {
        self.accepted
    }

    /// How many candidates failed under a different fingerprint.
    ///
    /// Every one of these is a shrink the reduction REFUSED: the count is the
    /// evidence that minimization stayed on the bug it started from.
    #[must_use]
    pub const fn fingerprint_moved(self) -> u32 {
        self.fingerprint_moved
    }

    /// How many candidates stopped failing.
    #[must_use]
    pub const fn no_failure(self) -> u32 {
        self.no_failure
    }

    /// How many candidate probes were spent, over every seat.
    #[must_use]
    pub const fn probes(self) -> u32 {
        self.accepted
            .saturating_add(self.fingerprint_moved)
            .saturating_add(self.no_failure)
    }
}

impl ReductionOutcome {
    /// What one reduction produced.
    #[must_use]
    pub fn reduced(
        input: Vec<u8>,
        fingerprint: Fingerprint,
        census: ReductionCensus,
        halt: ReductionHalt,
    ) -> Self {
        Self {
            input,
            fingerprint,
            census,
            halt,
        }
    }

    /// The smallest input the reduction reached.
    #[must_use]
    pub fn input(&self) -> &[u8] {
        &self.input
    }

    /// The fingerprint the reduced input still carries.
    ///
    /// It is the one the caller required, carried here so an outcome is
    /// readable without the call that produced it.
    #[must_use]
    pub const fn fingerprint(&self) -> Fingerprint {
        self.fingerprint
    }

    /// The accounting over every candidate.
    #[must_use]
    pub const fn census(&self) -> ReductionCensus {
        self.census
    }

    /// Why the reduction stopped.
    #[must_use]
    pub const fn halt(&self) -> ReductionHalt {
        self.halt
    }

    /// The reduced input, taken by a caller that consumes it — the exact bytes
    /// a [`crate::report::ReplayCapsule`] records.
    #[must_use]
    pub fn into_input(self) -> Vec<u8> {
        self.input
    }
}
