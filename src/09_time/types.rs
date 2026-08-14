//! Time: the typed temporal algebra. Four laws govern every physical-time
//! value; these roles are one closed register, never a runtime clock enum.
//!
//! # T1 — Clocks are untrusted peripherals
//!
//! A clock reading is foreign input: no `now()` exists in the semantic layer. A
//! reading enters only as an admitted [`ClockObservation`] through the ordinary
//! firewall discipline, classified once at the boundary. Once an injected clock
//! is supplied it owns every wall and monotonic observation on the complete
//! operation path — open, append, durability handoff, close, runtime,
//! diagnostic, and evidence code cannot fall back to ambient host time.
//! Deterministic replay and virtual time follow for free: time is just another
//! admitted input.
//!
//! ## Decision (2026-08-09) — "tick" is the clock's tick
//!
//! "Tick" means the clock's tick: a `ClockObservation` entering through
//! validated admission under T1's discipline — the clock separated from the
//! machine, the reading arriving as data, everything downstream stateless pure
//! functions over typed values. The word is reserved for this and is never the
//! name of the Stitch (the runtime transition), a scheduler step, or any host
//! method.
//!
//! # T2 — Instants don't exist
//!
//! Every observation is an interval; a point value is the degenerate interval
//! and must say so. Unknown uncertainty is never zero uncertainty: it is the
//! configured maximum, or refusal.
//!
//! # T3 — Comparisons speak K3
//!
//! Temporal comparison routes through the numeric home's interval-comparison
//! family; the decided truth is the canonical `Truth` — never a bool. What
//! `Pending` means at a site is decided by fail-closed narrowing: for a
//! deadline, `Pending` narrows to "budget exhausted unless proven otherwise."
//!
//! # T4 — Domain crossings are named morphisms
//!
//! There are no free conversions between temporal roles. Crossing a clock
//! domain or a durability boundary requires a named conversion that consumes
//! explicit evidence, may refuse, and monotonically WIDENS uncertainty —
//! uncertainty shrinks only via new observations, never via arithmetic. The
//! nonexistent morphisms are law too: none from a live monotonic value to any
//! durable form, and none from wall time or HLC to order, retry, or deadline
//! authority.

use crate::bounds::{Dimension, DimensionId};
use crate::identity::{CreationLaw, IdentityClass, IdentityRole, Occurrence};
use crate::refusal::{FamilyShape, RefusalFamily};
use crate::types::{Bounded, EvidenceRef, Limit, UnstatedMagnitude};
use crate::value::BoundedText;
use core::marker::PhantomData;

// ---------------------------------------------------------------------------
// Clock observations — the tick.
// ---------------------------------------------------------------------------

/// The identity role marker for clock domains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClockDomainRole;

/// One clock lineage: this boot × this clock kind — a fresh occurrence
/// identity, not content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClockDomainId(Occurrence<ClockDomainRole>);

impl IdentityRole for ClockDomainId {
    const CLASS: IdentityClass = IdentityClass::Occurrence;
    const CREATION: CreationLaw = CreationLaw::FreshOpaque;
}

/// An observed wall reading — an interval with the uncertainty INSIDE (one
/// fact, one home: a second uncertainty member beside the reading would store
/// the same fact twice, and two copies of one fact can disagree). A point value
/// is the degenerate interval `earliest == latest`, and says so structurally.
/// Carrier (nanoseconds, domain-relative) is realization, never value law.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObservedWallTime {
    /// The earliest bound.
    pub earliest_nanos: u64,
    /// The latest bound.
    pub latest_nanos: u64,
}

/// A signed, interval-valued wall-clock difference — diagnostic evidence ONLY:
/// not duration authority, HLC, order, cut, cursor, checkpoint, causal
/// evidence, work, or budget, and there is no implicit conversion to a timeout
/// or progress claim. A negative value is lawful evidence of clock regression
/// and is never normalized away.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimeDelta {
    /// The earliest signed difference.
    pub earliest_nanos: i64,
    /// The latest signed difference.
    pub latest_nanos: i64,
}

/// Limit family for provenance text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProvenanceLimit;
impl Limit for ProvenanceLimit {
    type Authority = UnstatedMagnitude;
}

/// Where a reading came from: source, route, admission context. Lost
/// provenance defaults to refusal.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClockObservationProvenance {
    /// The named source.
    pub source: BoundedText<ProvenanceLimit>,
}

/// THE tick: one clock reading admitted as data. Everything downstream is pure
/// functions over typed values.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClockObservation {
    /// The interval reading, uncertainty inside.
    pub reading: ObservedWallTime,
    /// The clock lineage.
    pub clock_domain: ClockDomainId,
    /// The reading's provenance.
    pub provenance: ClockObservationProvenance,
}

/// Limit family for clock-source policies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClockPolicyLimit;
impl Limit for ClockPolicyLimit {
    type Authority = UnstatedMagnitude;
}

/// Admitted clock domains and their admission requirements — policy, never a
/// live clock.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClockSourcePolicy {
    /// The admitted clock domains.
    pub admitted: Bounded<ClockDomainId, ClockPolicyLimit>,
}

/// The skew disposition of an observed remote reading under the admission
/// policy (AUTHORED roster: the role carries a shape here, never a bare name).
#[must_use = "a skew disposition is what the admission policy concluded about the reading"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClockSkewDisposition {
    /// Within the policy's tolerance.
    WithinTolerance,
    /// Excessive — preserved and marked, never clamped into a false source
    /// value, never advancing accepted chronology unlawfully.
    Excessive,
}

// ---------------------------------------------------------------------------
// The deadline split. The retired fused monotonic-deadline was asked to be
// durable (survive restart) and monotonic (meaningful only in one boot) at
// once; no type can be both.
// ---------------------------------------------------------------------------

/// The deadline-budget authority role: finite, exact, nonnegative, canonical;
/// ZERO IS LAWFUL (immediately exhausted). Deliberately not [`TimeDelta`]
/// (observation and authority never share one type) and deliberately not the
/// numeric home's `Duration` (a general exact quantity value). On the typed
/// route every property is bound by the carrier, so the construction causes are
/// reachable only on the decoded route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DurationLimit(u64);

impl DurationLimit {
    /// The typed-route construction: total, because the carrier is finite,
    /// exact, nonnegative, and canonical by construction. Zero is lawful.
    #[must_use]
    pub const fn admitted(nanos: u64) -> Self {
        Self(nanos)
    }

    /// The limit in carrier units.
    #[must_use]
    pub fn nanos(&self) -> u64 {
        self.0
    }
}

/// `DurationLimit` decoded-route construction: single-cause, six causes, all
/// unit, in the declared order — representation before value, value before
/// provenance, provenance before arithmetic. Zero is lawful and never appears
/// as a cause; no cause judges whether a budget SUFFICES.
#[must_use = "a construction refusal carries the lawful reason the limit was not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DurationLimitConstruction {
    /// The decoded value is not finite.
    NonFinite,
    /// The decoded value is approximate.
    Approximate,
    /// The decoded form is not canonical.
    NonCanonical,
    /// The decoded value is negative.
    Negative,
    /// The value carries wall-clock provenance — names the offending fact,
    /// never a mechanism.
    WallClockProvenance,
    /// Arithmetic over already-admitted limits overflowed.
    ArithmeticOverflow,
}

impl RefusalFamily for DurationLimitConstruction {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] = &[
        "NonFinite",
        "Approximate",
        "NonCanonical",
        "Negative",
        "WallClockProvenance",
        "ArithmeticOverflow",
    ];
}

/// A durable chronology coordinate serving as a deadline anchor — forward under
/// the owning chronology law; no wall time, HLC-summary shortcut, route, or
/// live instant substitutes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChronologyAnchor {
    /// An accepted chronology position.
    AcceptedPosition(AcceptedHlc),
    /// A summary-derived bound under one declared chronology authority — a
    /// bound, never a timestamp (an envelope component can never acquire
    /// timestamp meaning).
    SummaryBound {
        /// The bound's physical component.
        physical: u64,
        /// The bound's logical component.
        logical: u32,
    },
}

/// The durable, portable deadline commitment. Opaque over a private closed
/// triple, because a public enum would grant public construction; the posture
/// view reveals the selected posture without granting it. `DurationBudget` is
/// the paved road; `ChronologyBound` anchors a durable coordinate, never a live
/// clock; `WallTimeBound` is expressible but explicit, never a default (its
/// effective allowance narrows by the reading's uncertainty, clock-domain
/// crossing requires a named rebase, and lost provenance refuses).
///
/// The narrowing law: uncertainty narrows remaining work, never extends it —
/// remaining = policy − (consumed widened by ±u) — so a crash-restart loop
/// monotonically loses budget; reconnect, retry, rescheduling, and adapter
/// conversion never reset it; adapters receive derived allowances and never
/// hold the policy. The rebase morphism (policy × observation × consumed
/// evidence → live deadline | refusal) is the machinery seam and lands with
/// implementation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeadlinePolicy {
    posture: DeadlinePosture,
}

/// The private closed triple.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum DeadlinePosture {
    /// The paved road: a pure duration budget.
    DurationBudget {
        /// The budget.
        limit: DurationLimit,
    },
    /// Anchored to a durable chronology coordinate.
    ChronologyBound {
        /// The anchor.
        anchor: ChronologyAnchor,
    },
    /// Wall-anchored with tolerance — explicit, never a default.
    WallTimeBound {
        /// The anchoring observation.
        observation: ClockObservation,
        /// The tolerance.
        tolerance: DurationLimit,
        /// The governing source policy.
        source_policy: ClockSourcePolicy,
    },
}

/// The neutral posture view — inspection without construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeadlinePostureView {
    /// A duration budget.
    DurationBudget,
    /// A chronology-bound anchor.
    ChronologyBound,
    /// A wall-time bound.
    WallTimeBound,
}

impl DeadlinePolicy {
    /// The paved road: author a pure duration budget. Total on the typed
    /// route — the carrier admits every value; zero is immediately exhausted.
    #[must_use]
    pub const fn duration_budget(limit: DurationLimit) -> Self {
        Self {
            posture: DeadlinePosture::DurationBudget { limit },
        }
    }

    /// Author a chronology-bound policy against a durable coordinate — never a
    /// live clock.
    #[must_use]
    pub const fn chronology_bound(anchor: ChronologyAnchor) -> Self {
        Self {
            posture: DeadlinePosture::ChronologyBound { anchor },
        }
    }

    /// Author a wall-anchored policy — expressible but explicit, never a
    /// default. Provenance and uncertainty presence are structural (the
    /// observation carries both by type); admission against the source policy
    /// is the boundary's judgment, not this constructor's.
    #[must_use]
    pub const fn wall_time_bound(
        observation: ClockObservation,
        tolerance: DurationLimit,
        source_policy: ClockSourcePolicy,
    ) -> Self {
        Self {
            posture: DeadlinePosture::WallTimeBound {
                observation,
                tolerance,
                source_policy,
            },
        }
    }

    /// The selected posture, revealed without granting variant access.
    #[must_use]
    pub fn posture(&self) -> DeadlinePostureView {
        match &self.posture {
            DeadlinePosture::DurationBudget { .. } => DeadlinePostureView::DurationBudget,
            DeadlinePosture::ChronologyBound { .. } => DeadlinePostureView::ChronologyBound,
            DeadlinePosture::WallTimeBound { .. } => DeadlinePostureView::WallTimeBound,
        }
    }
}

/// `DeadlinePolicy` construction: single-cause, six causes, all unit, declared
/// order — an unadmitted profile is settled before any member is interpreted; a
/// reading with lost provenance refuses before its numbers are read; anchor and
/// duration validity precede the arithmetic over them; policy arithmetic runs
/// last, over members already admitted. Never a package-wide deadline error;
/// the nested `DurationLimit` family is never shadowed; the rebase refusal is
/// not here; and no cause judges whether a budget SUFFICES.
#[must_use = "a construction refusal carries the lawful reason the policy was not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeadlinePolicyConstruction {
    /// A selected profile this boundary does not admit — refuses rather than
    /// silently substituting a weaker one.
    UnsupportedProfile,
    /// The reading's provenance is lost.
    LostProvenance,
    /// Unknown uncertainty is never zero uncertainty.
    MissingWallUncertainty,
    /// The chronology anchor is invalid.
    InvalidChronologyAnchor,
    /// The duration is invalid (an overflowing `DurationLimit` refuses through
    /// its own family and reaches this boundary as this cause).
    InvalidDuration,
    /// Policy-level arithmetic overflowed.
    ArithmeticOverflow,
}

impl RefusalFamily for DeadlinePolicyConstruction {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] = &[
        "UnsupportedProfile",
        "LostProvenance",
        "MissingWallUncertainty",
        "InvalidChronologyAnchor",
        "InvalidDuration",
        "ArithmeticOverflow",
    ];
}

/// Where a spend observation was durably recorded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecordingSite {
    /// At admission.
    Admission,
    /// At a checkpoint.
    Checkpoint,
    /// At an effect attempt.
    EffectAttempt,
}

/// One measured spend in one bound dimension, with its uncertainty.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpendRecord {
    /// The dimension spent in.
    pub dimension: DimensionId,
    /// The measured magnitude.
    pub magnitude: u64,
    /// The measurement uncertainty (±).
    pub uncertainty: u64,
}

/// Limit family for spend collections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpendLimit;
impl Limit for SpendLimit {
    type Authority = UnstatedMagnitude;
}

/// The claim marker for the durable coordinate a spend was recorded at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpendCoordinateClaim;

/// Persisted observations of spend at named durable points — never a raw
/// instant. The durable coordinate rides a typed reference (the coordinate
/// itself is the history home's value, above this band).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConsumedBudgetEvidence {
    /// The recording site.
    pub site: RecordingSite,
    /// The durable coordinate reference.
    pub coordinate: EvidenceRef<SpendCoordinateClaim>,
    /// The per-dimension spends.
    pub spends: Bounded<SpendRecord, SpendLimit>,
}

/// The per-clock-domain-life enforcement point: unserializable by
/// construction, dead with the clock domain that produced it, derived only by
/// the rebase morphism. The raw-pointer phantom makes it structurally `!Send`
/// and `!Sync`; no morphism leads from a live monotonic value to any durable
/// form.
#[derive(Debug)]
pub struct LiveMonotonicDeadline {
    remaining: DurationLimit,
    _clock_domain_local: PhantomData<*const ()>,
}

impl LiveMonotonicDeadline {
    /// The remaining allowance at derivation.
    #[must_use]
    pub fn remaining(&self) -> DurationLimit {
        self.remaining
    }
}

/// The deadline dimension marker — the time home's promised rider on the
/// bounds home's affine budget shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeadlineDimension;
impl Dimension for DeadlineDimension {}

// ---------------------------------------------------------------------------
// HLC chronology. HLC carries admitted chronology evidence — not durable
// order, causal topology, cursor, checkpoint, retry authority, deadline,
// partition authority, or a global total order.
// ---------------------------------------------------------------------------

/// The HLC coordinate shape: physical u64 + logical u32. The logical counter
/// overflow refuses, never wraps — u32 is the smallest width whose overflow can
/// only mean broken clock physics, never legitimate load.
///
/// A bare coordinate carries no ROLE. It is the payload the two roles are made
/// of and is never itself observed chronology or admitted chronology; which of
/// those a coordinate stands for is [`SourceHlc`]'s and [`AcceptedHlc`]'s
/// question, and those two are separate types precisely so the payload cannot
/// answer it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HlcCoordinate {
    physical: u64,
    logical: u32,
}

impl HlcCoordinate {
    /// State one coordinate from its two components.
    #[must_use]
    pub const fn at(physical: u64, logical: u32) -> Self {
        Self { physical, logical }
    }

    /// The physical component.
    #[must_use]
    pub const fn physical(self) -> u64 {
        self.physical
    }

    /// The logical counter.
    #[must_use]
    pub const fn logical(self) -> u32 {
        self.logical
    }
}

/// Chronology supplied or observed from a source — always preserved, never
/// clamped into a false source value.
///
/// # Observation is the open end of the crossing
///
/// [`observed`](Self::observed) is public because an observation genuinely
/// arrives from anywhere: a peer's envelope, a decoded record, a host's reading.
/// Admitting one is the closed end — see [`AcceptedHlc`] — and the two ends do
/// not meet through a shared payload: reading this value back yields a
/// role-free [`HlcCoordinate`], and no mint takes a coordinate into admitted
/// chronology.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceHlc(HlcCoordinate);

impl SourceHlc {
    /// Record one observation of chronology from a source.
    #[must_use]
    pub const fn observed(coordinate: HlcCoordinate) -> Self {
        Self(coordinate)
    }

    /// The observed coordinate, preserved exactly as it arrived.
    #[must_use]
    pub const fn coordinate(self) -> HlcCoordinate {
        self.0
    }
}

/// Chronology lawfully admitted into local state — yielded only by the
/// stateful chronology admission clock.
///
/// # There is no mint here, and that absence IS the law
///
/// "Yielded only by the admission clock" was prose while the payload seat was
/// public: anyone holding a coordinate could write `AcceptedHlc(coordinate)` and
/// have a value the whole machine reads as admitted. The seat is private now, so
/// the sentence is the type's shape rather than a rule a reader has to remember.
/// The one road that produces this value is [`ChronologyAdmission::admit`], and
/// it consumes a [`SourceHlc`] to do it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AcceptedHlc(HlcCoordinate);

impl AcceptedHlc {
    /// In-crate mint for laws. Test-gated until the admission clock's
    /// advancement rule exists — the gate comes off when a lawful minter does,
    /// never before.
    #[cfg(test)]
    pub(crate) const fn for_laws(coordinate: HlcCoordinate) -> Self {
        Self(coordinate)
    }

    /// The admitted coordinate.
    #[must_use]
    pub const fn coordinate(self) -> HlcCoordinate {
        self.0
    }
}

/// The one lawful crossing from observed chronology into admitted chronology.
///
/// # Why this is a contract and not a written body
///
/// The crossing's SHAPE is settled and is stated here: the observation is
/// CONSUMED, exactly one admitted position comes out, the admitting clock is
/// mutated by the act, and no road runs the other way — an admitted position
/// cannot be turned back into an observation, and an observation cannot become
/// admitted by any route that skips a clock.
///
/// The crossing's RULE is not settled and is not stated here. Counter
/// advancement, clock-regression behavior, excessive-future classification, and
/// the overflow refusal are the admission clock's declared machinery, and this
/// phase carries no machinery. Writing a body would mean choosing that rule
/// here, in the seat that is supposed to receive it — so the seat is declared
/// empty and the rule lands in it when the time home opens for implementation.
/// An implementor is core's own clock: nothing outside this crate can satisfy
/// this contract, because nothing outside can mint an [`AcceptedHlc`].
pub trait ChronologyAdmission {
    /// The typed refusal family this admission road speaks — the clock's
    /// overflow refusal and its regression posture are named there, by the
    /// implementor, in its own vocabulary.
    type Refusal;

    /// Consume one observation and yield the position this clock admitted.
    ///
    /// # Errors
    ///
    /// Returns the implementor's refusal family when the observation is not
    /// admissible under the clock's declared profile.
    fn admit(&mut self, observed: SourceHlc) -> Result<AcceptedHlc, Self::Refusal>;
}

/// A registered chronology profile identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChronologyProfileId(u16);

impl ChronologyProfileId {
    /// In-crate mint for laws. Test-gated until the profile register exists.
    #[cfg(test)]
    pub(crate) const fn registered(id: u16) -> Self {
        Self(id)
    }

    /// The registered identity.
    #[must_use]
    pub fn value(&self) -> u16 {
        self.0
    }
}

/// The immutable chronology envelope: componentwise extrema over
/// already-admitted chronology whose components need never have co-occurred in
/// one admitted observation. It BOUNDS admitted chronology without being an
/// admitted `SourceHlc` or `AcceptedHlc` — no morphism leads from the envelope
/// back to either, so an envelope component can never acquire timestamp
/// meaning. The name is always in full — never abbreviated. Deliberately NOT
/// carrying an [`HlcCoordinate`]: the extrema are independent fields so the
/// envelope cannot even look like an HLC value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChronologySummary {
    /// The bound profile.
    pub profile: ChronologyProfileId,
    /// The maximum admitted physical component.
    pub max_physical: u64,
    /// The maximum admitted logical component.
    pub max_logical: u32,
}

/// The merge refusal: exactly one cause. The merge's own totality clause IS the
/// roster — total over validated same-profile summaries names exactly one
/// guard. Profile identity subsumes profile version, so no second version cause
/// exists. One inhabited cause, so no cause-selection rule is owed. A summary
/// is an envelope carrying no chronology value, so it has no chronology
/// authority to mismatch; the join performs no arithmetic, so no overflow cause
/// exists — counter advancement, regression, excessive-future classification,
/// and overflow refusal belong to the stateful admission clock, with which this
/// operation shares no surface.
#[must_use = "a merge refusal carries the lawful reason two chronologies were not joined"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChronologyMerge {
    /// The two summaries bind different profiles.
    ProfileMismatch,
}

impl RefusalFamily for ChronologyMerge {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] = &["ProfileMismatch"];
}

impl ChronologySummary {
    /// The immutable summary merge: total over validated same-profile
    /// summaries; commutative, associative, and idempotent under its stated
    /// domain (proven in the laws). It consults no ambient wall time, mutates
    /// no admission clock, evaluates no source trust, clamps no observation,
    /// and stamps no event. It produces no order, cut, cursor, checkpoint,
    /// retry right, or deadline.
    ///
    /// # Errors
    ///
    /// Returns [`ChronologyMerge::ProfileMismatch`] when the summaries bind
    /// different profiles.
    pub fn try_merge(self, other: Self) -> Result<Self, ChronologyMerge> {
        if self.profile == other.profile {
            Ok(Self {
                profile: self.profile,
                max_physical: self.max_physical.max(other.max_physical),
                max_logical: self.max_logical.max(other.max_logical),
            })
        } else {
            Err(ChronologyMerge::ProfileMismatch)
        }
    }
}

/// The stateful chronology admission clock — a distinct owned object sharing NO
/// surface with the summary merge (AUTHORED name: the role and its nine-item
/// ownership roster are law, the spelling is this home's). It owns: local
/// tick (the clock's tick), remote observation, wall-clock input, counter
/// advancement, clock-regression behavior, excessive-future classification,
/// overflow refusal (or another selected safe behavior), `SourceHlc`
/// preservation, and the policy's required evidence. Its ticking is machinery
/// and lands with implementation; the shape binds its domain and current
/// position.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChronologyAdmissionClock {
    /// The clock domain this admission clock serves.
    pub domain: ClockDomainId,
    /// The current accepted position.
    pub current: AcceptedHlc,
}
