//! The fault instrument's invariant nucleus: constructors, validated selection, and readers over private fields.

use super::{
    CampaignSelection, FaultAdapter, FaultCampaign, FaultCampaignRefusal, FaultSchedule,
    FaultSelectionRefusal, InjectedCommand, InjectedSequence, ScheduledFault, SequencePosition,
};
use crate::descriptor::NamespacedName;
use std::collections::BTreeSet;

impl<Behavior, Postcondition> FaultAdapter<Behavior, Postcondition> {
    /// One concrete behavior joined to the postcondition its adopter declares.
    #[must_use]
    pub const fn declared(behavior: Behavior, postcondition: Postcondition) -> Self {
        Self {
            behavior,
            postcondition,
        }
    }

    /// The adopter-owned behavior.
    #[must_use]
    pub const fn behavior(&self) -> &Behavior {
        &self.behavior
    }

    /// The adopter-owned postcondition.
    #[must_use]
    pub const fn postcondition(&self) -> &Postcondition {
        &self.postcondition
    }
}

impl SequencePosition {
    /// The zero-based position the schedule declares.
    #[must_use]
    pub const fn at(ordinal: u32) -> Self {
        Self(ordinal)
    }

    /// The zero-based ordinal this position carries.
    #[must_use]
    pub const fn ordinal(self) -> u32 {
        self.0
    }
}

impl<Behavior, Postcondition> ScheduledFault<Behavior, Postcondition> {
    /// One typed adapter placed at one sequence position.
    #[must_use]
    pub const fn at(
        position: SequencePosition,
        adapter: FaultAdapter<Behavior, Postcondition>,
    ) -> Self {
        Self { position, adapter }
    }

    /// The position this adapter is scheduled at.
    #[must_use]
    pub const fn position(&self) -> SequencePosition {
        self.position
    }

    /// The adopter-owned adapter value.
    #[must_use]
    pub const fn adapter(&self) -> &FaultAdapter<Behavior, Postcondition> {
        &self.adapter
    }
}

impl<Behavior, Postcondition> FaultSchedule<Behavior, Postcondition> {
    /// One named schedule in authored adapter order.
    #[must_use]
    pub fn declared(
        name: NamespacedName,
        faults: Vec<ScheduledFault<Behavior, Postcondition>>,
    ) -> Self {
        Self { name, faults }
    }

    /// The schedule's campaign-local name.
    #[must_use]
    pub const fn name(&self) -> NamespacedName {
        self.name
    }

    /// The scheduled adapters in authored order.
    #[must_use]
    pub fn faults(&self) -> &[ScheduledFault<Behavior, Postcondition>] {
        &self.faults
    }
}

impl<Behavior, Postcondition> FaultCampaign<Behavior, Postcondition> {
    /// One campaign over uniquely named schedules in authored order.
    ///
    /// # Errors
    ///
    /// Refuses an empty campaign, then the first schedule name repeated in authored order, then a campaign whose schedules are all empty controls and therefore declare no fault.
    pub fn declared(
        schedules: Vec<FaultSchedule<Behavior, Postcondition>>,
    ) -> Result<Self, FaultCampaignRefusal> {
        if schedules.is_empty() {
            return Err(FaultCampaignRefusal::NoSchedule);
        }
        let mut names = BTreeSet::new();
        for schedule in &schedules {
            if !names.insert(schedule.name()) {
                return Err(FaultCampaignRefusal::DuplicateSchedule(schedule.name()));
            }
        }
        if schedules
            .iter()
            .all(|schedule| schedule.faults().is_empty())
        {
            return Err(FaultCampaignRefusal::NoFaultDeclared);
        }
        Ok(Self { schedules })
    }

    /// The campaign's schedules in authored order.
    #[must_use]
    pub fn schedules(&self) -> &[FaultSchedule<Behavior, Postcondition>] {
        &self.schedules
    }

    /// Select the schedule this campaign declares under `name`.
    ///
    /// # Errors
    ///
    /// Refuses a name no schedule in this campaign declares.
    pub fn select(
        &self,
        name: NamespacedName,
    ) -> Result<CampaignSelection<'_, Behavior, Postcondition>, FaultSelectionRefusal> {
        self.schedules
            .iter()
            .find(|schedule| schedule.name() == name)
            .map(|schedule| CampaignSelection { schedule })
            .ok_or(FaultSelectionRefusal::ScheduleAbsent(name))
    }
}

impl<'campaign, Behavior, Postcondition> CampaignSelection<'campaign, Behavior, Postcondition> {
    /// The schedule this campaign selection validated.
    #[must_use]
    pub const fn schedule(&self) -> &'campaign FaultSchedule<Behavior, Postcondition> {
        self.schedule
    }
}

impl<Command, Behavior, Postcondition> InjectedCommand<Command, Behavior, Postcondition> {
    /// One command and the typed adapters placed beside it.
    #[must_use]
    pub(crate) const fn injected(
        command: Command,
        faults: Vec<FaultAdapter<Behavior, Postcondition>>,
    ) -> Self {
        Self { command, faults }
    }

    /// The original command.
    #[must_use]
    pub const fn command(&self) -> &Command {
        &self.command
    }

    /// The typed adapters placed at this command, in authored schedule order.
    #[must_use]
    pub fn faults(&self) -> &[FaultAdapter<Behavior, Postcondition>] {
        &self.faults
    }
}

impl<Command, Behavior, Postcondition> InjectedSequence<Command, Behavior, Postcondition> {
    /// One fully injected sequence, minted only by the injection operation.
    #[must_use]
    pub(crate) const fn injected(
        schedule: NamespacedName,
        commands: Vec<InjectedCommand<Command, Behavior, Postcondition>>,
    ) -> Self {
        Self { schedule, commands }
    }

    /// The selected schedule's name.
    #[must_use]
    pub const fn schedule(&self) -> NamespacedName {
        self.schedule
    }

    /// The commands and the adapters placed beside each one.
    #[must_use]
    pub fn commands(&self) -> &[InjectedCommand<Command, Behavior, Postcondition>] {
        &self.commands
    }

    /// How many typed adapter values were injected, derived from the schedule rather than declared beside it.
    #[must_use]
    pub fn fault_count(&self) -> usize {
        self.commands
            .iter()
            .map(|command| command.faults().len())
            .sum()
    }
}
