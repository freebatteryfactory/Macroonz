//! The property vocabulary's invariant nucleus: every road that reaches a
//! private field, and every reader that hands one back.
//!
//! Declared inside `types.rs` as its own child, so it sees the fields the
//! declarations keep private and no sibling module does. A parity suite that
//! never stated what its two roads share, a shared-substrate roster naming no
//! substrate at all, a roster naming one substrate twice, and a transition
//! contract demanding nothing of its histories are all refused HERE, which is
//! what makes those claims structural rather than remembered.
//!
//! The empty-roster refusal is the one that separates two claims. A roster
//! naming nothing and the declaration that two roads share nothing are opposite
//! statements, so no constructor here turns the first into the second.

use super::{
    ComposedRoads, ContractRefusal, Equivalence, ParityReading, ParitySuite, Road, RoadPairing,
    SharedSubstrate, SubstrateRef, SubstrateRefusal, SubstrateRoster, TemporalClaim,
    TemporalDemand, TemporalDriveReading, TemporalDriveStanding, TransitionContract,
};
use crate::descriptor::{NameRefusal, NamespacedName};
use crate::generate::GeneratedSequences;
use crate::report::{FindingCause, TrialConclusion};
use std::collections::BTreeSet;

// ---------------------------------------------------------------------------
// The parity suite.
// ---------------------------------------------------------------------------

impl SubstrateRef {
    /// This substrate, parsed from the owner that declares it and the spelling
    /// it carries.
    ///
    /// # Errors
    ///
    /// Refuses an empty namespace, then an empty stem.
    pub fn named(namespace: &'static str, stem: &'static str) -> Result<Self, NameRefusal> {
        NamespacedName::named(namespace, stem).map(Self)
    }

    /// This substrate, over a name already parsed.
    #[must_use]
    pub const fn over(name: NamespacedName) -> Self {
        Self(name)
    }

    /// The namespaced name this substrate carries.
    #[must_use]
    pub const fn name(self) -> NamespacedName {
        self.0
    }
}

impl SubstrateRoster {
    /// The substrates two roads both stand on.
    ///
    /// # Errors
    ///
    /// Refuses an empty roster, then a substrate the roster names more than
    /// once. The empty refusal is what keeps the two meanings apart: an owner
    /// whose roads stand on nothing in common writes
    /// [`SharedSubstrate::DeclaredIndependent`], and there is no road from this
    /// constructor to that claim.
    pub fn declared(standing: &[SubstrateRef]) -> Result<Self, SubstrateRefusal> {
        if standing.is_empty() {
            return Err(SubstrateRefusal::EmptyRoster);
        }
        let mut roster: BTreeSet<SubstrateRef> = BTreeSet::new();
        for substrate in standing {
            if !roster.insert(*substrate) {
                return Err(SubstrateRefusal::DuplicateSubstrate(*substrate));
            }
        }
        Ok(Self { standing: roster })
    }

    /// Every substrate the pair stands on, in the roster's storage order.
    #[must_use]
    pub const fn standing(&self) -> &BTreeSet<SubstrateRef> {
        &self.standing
    }
}

impl<Input, Meaning> ParitySuite<Input, Meaning> {
    /// The suite two roads to one meaning are judged by.
    #[must_use]
    pub fn over(
        pairing: RoadPairing,
        left: Road<Input, Meaning>,
        right: Road<Input, Meaning>,
        same: Equivalence<Meaning>,
        substrate: SharedSubstrate,
    ) -> Self {
        Self {
            pairing,
            left,
            right,
            same,
            substrate,
        }
    }

    /// The suite a fused implementation and the composition of the separate
    /// steps it fuses are judged by.
    ///
    /// The fused road is the left one, the separate composition the right.
    #[must_use]
    pub fn fused_versus_separate(
        fused: Road<Input, Meaning>,
        separate: Road<Input, Meaning>,
        same: Equivalence<Meaning>,
        substrate: SharedSubstrate,
    ) -> Self {
        Self::over(
            RoadPairing::FusedVersusSeparate,
            fused,
            separate,
            same,
            substrate,
        )
    }

    /// The suite a live run and its reproduction are judged by.
    ///
    /// The live road is the left one, the reproduction the right.
    #[must_use]
    pub fn replay_equivalence(
        live: Road<Input, Meaning>,
        replayed: Road<Input, Meaning>,
        same: Equivalence<Meaning>,
        substrate: SharedSubstrate,
    ) -> Self {
        Self::over(
            RoadPairing::LiveVersusReplayed,
            live,
            replayed,
            same,
            substrate,
        )
    }

    /// Which two roads this suite stands over.
    #[must_use]
    pub const fn pairing(&self) -> RoadPairing {
        self.pairing
    }

    /// The left road.
    #[must_use]
    pub const fn left(&self) -> Road<Input, Meaning> {
        self.left
    }

    /// The right road.
    #[must_use]
    pub const fn right(&self) -> Road<Input, Meaning> {
        self.right
    }

    /// The equivalence the two meanings are compared under.
    #[must_use]
    pub const fn same(&self) -> Equivalence<Meaning> {
        self.same
    }

    /// What the two roads share, and what the suite is therefore silent about.
    #[must_use]
    pub const fn substrate(&self) -> &SharedSubstrate {
        &self.substrate
    }
}

impl<'suite, 'input, Input, Meaning> ParityReading<'suite, 'input, Input, Meaning> {
    /// One comparison result over the exact suite and input that produced it.
    #[must_use]
    pub(crate) fn from_run(
        suite: &'suite ParitySuite<Input, Meaning>,
        input: &'input Input,
        left: Meaning,
        right: Meaning,
        conclusion: TrialConclusion,
    ) -> Self {
        Self {
            suite,
            input,
            left,
            right,
            conclusion,
        }
    }

    /// The suite that owns the road pair, equivalence, and shared-substrate ceiling.
    #[must_use]
    pub const fn suite(&self) -> &'suite ParitySuite<Input, Meaning> {
        self.suite
    }

    /// The exact input supplied to both roads.
    #[must_use]
    pub const fn input(&self) -> &'input Input {
        self.input
    }

    /// The left road's result.
    #[must_use]
    pub const fn left(&self) -> &Meaning {
        &self.left
    }

    /// The right road's result.
    #[must_use]
    pub const fn right(&self) -> &Meaning {
        &self.right
    }

    /// The conclusion reached under the suite's declared equivalence.
    #[must_use]
    pub const fn conclusion(&self) -> &TrialConclusion {
        &self.conclusion
    }
}

// ---------------------------------------------------------------------------
// The temporal suite.
// ---------------------------------------------------------------------------

impl<State> TemporalClaim<State> {
    /// The claim its owner declared, under the cause a break is cited by.
    #[must_use]
    pub const fn declared(cause: FindingCause, demand: TemporalDemand<State>) -> Self {
        Self { cause, demand }
    }

    /// The cause a break in this claim is cited under.
    #[must_use]
    pub const fn cause(&self) -> FindingCause {
        self.cause
    }

    /// What this claim demands of a history.
    #[must_use]
    pub const fn demand(&self) -> &TemporalDemand<State> {
        &self.demand
    }
}

impl<State, Command> TransitionContract<State, Command> {
    /// The transition system its owner declared.
    ///
    /// # Errors
    ///
    /// Refuses a contract carrying no claim, because every history driven
    /// through one would read as a pass with nothing demanded of it.
    pub fn declared(
        opening: fn() -> State,
        apply: fn(&State, &Command) -> State,
        claims: Vec<TemporalClaim<State>>,
    ) -> Result<Self, ContractRefusal> {
        if claims.is_empty() {
            return Err(ContractRefusal::NoClaimDeclared);
        }
        Ok(Self {
            opening,
            apply,
            claims,
        })
    }

    /// The road to the state every history opens at.
    #[must_use]
    pub const fn opening(&self) -> fn() -> State {
        self.opening
    }

    /// The transition one command moves the state by.
    #[must_use]
    pub const fn apply(&self) -> fn(&State, &Command) -> State {
        self.apply
    }

    /// The claims every history of this system owes.
    #[must_use]
    pub fn claims(&self) -> &[TemporalClaim<State>] {
        &self.claims
    }
}

impl<Command> TemporalDriveReading<Command> {
    /// One temporal reading over the exact generation result it evaluated.
    #[must_use]
    pub(crate) const fn from_drive(
        generated: GeneratedSequences<Command>,
        evaluated: usize,
        standing: TemporalDriveStanding,
    ) -> Self {
        Self {
            generated,
            evaluated,
            standing,
        }
    }

    /// The admitted sequences, complete generation census, and halt supplied to temporal evaluation.
    #[must_use]
    pub const fn generated(&self) -> &GeneratedSequences<Command> {
        &self.generated
    }

    /// How many admitted sequences temporal evaluation actually read.
    #[must_use]
    pub const fn evaluated(&self) -> usize {
        self.evaluated
    }

    /// What the retained generation and evaluation evidence can establish.
    #[must_use]
    pub const fn standing(&self) -> &TemporalDriveStanding {
        &self.standing
    }
}

// ---------------------------------------------------------------------------
// The composed-roads suite.
// ---------------------------------------------------------------------------

impl<Entry, Middle, Exit> ComposedRoads<Entry, Middle, Exit> {
    /// The two steps its owner wired, in the order they run.
    #[must_use]
    pub const fn wired(
        first: Road<Entry, Middle>,
        second: Road<Middle, Exit>,
        same: Equivalence<Exit>,
    ) -> Self {
        Self {
            first,
            second,
            same,
        }
    }

    /// The step that runs first.
    #[must_use]
    pub const fn first(&self) -> Road<Entry, Middle> {
        self.first
    }

    /// The step that runs over the first step's image.
    #[must_use]
    pub const fn second(&self) -> Road<Middle, Exit> {
        self.second
    }

    /// The equivalence the composition's images are compared under.
    #[must_use]
    pub const fn same(&self) -> Equivalence<Exit> {
        self.same
    }
}
