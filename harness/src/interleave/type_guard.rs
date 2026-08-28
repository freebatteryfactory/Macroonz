//! Constructors, readers, and the checks that decide whether a party, a set, and a bound exist at all.

use super::{
    ADDRESSABLE_STRANDS, Counterexample, ExplorationBound, ExplorationBoundRefusal,
    ExplorationMode, ExplorationReading, ExplorationSite, ExplorationStanding, InterleavedSequence,
    Interleaving, InterleavingSpace, Strand, StrandRefusal, StrandSet, StrandSetRefusal,
};
use crate::descriptor::NamespacedName;
use crate::generate::{CaseWidth, CaseWidthRefusal};
use crate::report::TrialFinding;
use std::collections::BTreeSet;

impl<Command> Strand<Command> {
    /// A named party and the commands it will issue, in its own program order.
    ///
    /// # Errors
    ///
    /// Refuses a strand with no commands, because a declared party that never acts is vacuous.
    pub fn declared(name: NamespacedName, commands: Vec<Command>) -> Result<Self, StrandRefusal> {
        if commands.is_empty() {
            return Err(StrandRefusal::EmptyStrand(name));
        }
        Ok(Self { name, commands })
    }

    /// The name this party is told apart by.
    #[must_use]
    pub const fn name(&self) -> NamespacedName {
        self.name
    }

    /// The commands, in this party's own program order.
    #[must_use]
    pub fn commands(&self) -> &[Command] {
        &self.commands
    }
}

impl<Command> StrandSet<Command> {
    /// The concurrent parties together, in authored ordinal order.
    ///
    /// # Errors
    ///
    /// Refuses a repeated name, then a set larger than [`ADDRESSABLE_STRANDS`], then a step total past addressing, then a set with fewer than two strands.
    pub fn declared(strands: Vec<Strand<Command>>) -> Result<Self, StrandSetRefusal> {
        unique_names(&strands)?;
        if strands.len() > ADDRESSABLE_STRANDS {
            return Err(StrandSetRefusal::MoreStrandsThanAddressable {
                strands: strands.len(),
            });
        }
        let (steps, width) = steps_and_width(&strands)?;
        if strands.len() < 2 {
            return Err(StrandSetRefusal::FewerThanTwoStrands {
                strands: strands.len(),
            });
        }
        Ok(Self {
            strands,
            steps,
            width,
        })
    }

    /// The parties, in ordinal order.
    #[must_use]
    pub fn strands(&self) -> &[Strand<Command>] {
        &self.strands
    }

    /// How many steps every interleaving of this set holds.
    #[must_use]
    pub const fn steps(&self) -> usize {
        self.steps
    }

    /// The step total as the case width every sampled draw uses.
    #[must_use]
    pub(crate) const fn width(&self) -> CaseWidth {
        self.width
    }
}

/// Establish that every strand has one distinct name.
fn unique_names<Command>(strands: &[Strand<Command>]) -> Result<(), StrandSetRefusal> {
    let mut seen = BTreeSet::new();
    for strand in strands {
        if !seen.insert(strand.name()) {
            return Err(StrandSetRefusal::DuplicateStrand(strand.name()));
        }
    }
    Ok(())
}

/// Establish the addressable step total and the case width sampled material uses.
fn steps_and_width<Command>(
    strands: &[Strand<Command>],
) -> Result<(usize, CaseWidth), StrandSetRefusal> {
    let mut steps = 0usize;
    for strand in strands {
        steps = steps
            .checked_add(strand.commands().len())
            .ok_or(StrandSetRefusal::StepsUnaddressable)?;
    }
    match CaseWidth::declared(steps) {
        Ok(width) => Ok((steps, width)),
        Err(CaseWidthRefusal::ZeroBytes) => Err(StrandSetRefusal::FewerThanTwoStrands {
            strands: strands.len(),
        }),
    }
}

impl Interleaving {
    /// The choice string its author spelled: one strand ordinal per step.
    #[must_use]
    pub const fn declared(choices: Vec<u8>) -> Self {
        Self { choices }
    }

    /// The choices, one strand ordinal per step.
    #[must_use]
    pub fn choices(&self) -> &[u8] {
        &self.choices
    }
}

impl<Command> InterleavedSequence<Command> {
    /// One realized merge, minted only by interpretation.
    #[must_use]
    pub(crate) const fn realized(interleaving: Interleaving, commands: Vec<Command>) -> Self {
        Self {
            interleaving,
            commands,
        }
    }

    /// The canonical merge order this sequence realizes.
    #[must_use]
    pub const fn interleaving(&self) -> &Interleaving {
        &self.interleaving
    }

    /// The commands, in the order the interleaving merged them.
    #[must_use]
    pub fn commands(&self) -> &[Command] {
        &self.commands
    }
}

impl ExplorationBound {
    /// The budget its author declared: the exhaustive ceiling, and the sample count beyond it.
    ///
    /// # Errors
    ///
    /// Refuses a bound with no interleaving seat, then one with no sample seat.
    pub const fn declared(
        interleavings: u32,
        samples: u32,
    ) -> Result<Self, ExplorationBoundRefusal> {
        if interleavings == 0u32 {
            return Err(ExplorationBoundRefusal::ZeroInterleavings);
        }
        if samples == 0u32 {
            return Err(ExplorationBoundRefusal::ZeroSamples);
        }
        Ok(Self {
            interleavings,
            samples,
        })
    }

    /// The ceiling under which the space is walked exhaustively.
    #[must_use]
    pub const fn interleavings(self) -> u32 {
        self.interleavings
    }

    /// How many schedules are drawn when the space is beyond the ceiling.
    #[must_use]
    pub const fn samples(self) -> u32 {
        self.samples
    }
}

impl Counterexample {
    /// One found break, minted only by exploration.
    #[must_use]
    pub(crate) const fn found(
        site: ExplorationSite,
        interleaving: Interleaving,
        finding: TrialFinding,
    ) -> Self {
        Self {
            site,
            interleaving,
            finding,
        }
    }

    /// Where this counterexample was found.
    #[must_use]
    pub const fn site(&self) -> ExplorationSite {
        self.site
    }

    /// The merge order whose history broke the claim.
    #[must_use]
    pub const fn interleaving(&self) -> &Interleaving {
        &self.interleaving
    }

    /// The typed finding the broken claim concluded with.
    #[must_use]
    pub const fn finding(&self) -> &TrialFinding {
        &self.finding
    }
}

impl ExplorationReading {
    /// One exploration's product, minted only by exploration.
    #[must_use]
    pub(crate) const fn read(
        space: InterleavingSpace,
        mode: ExplorationMode,
        explored: u64,
        standing: ExplorationStanding,
    ) -> Self {
        Self {
            space,
            mode,
            explored,
            standing,
        }
    }

    /// How many interleavings the strand set admits.
    #[must_use]
    pub const fn space(&self) -> InterleavingSpace {
        self.space
    }

    /// Which way the space was walked, with that walk's evidence.
    #[must_use]
    pub const fn mode(&self) -> ExplorationMode {
        self.mode
    }

    /// How many interleavings were judged.
    #[must_use]
    pub const fn explored(&self) -> u64 {
        self.explored
    }

    /// What the walked evidence establishes.
    #[must_use]
    pub const fn standing(&self) -> &ExplorationStanding {
        &self.standing
    }
}
