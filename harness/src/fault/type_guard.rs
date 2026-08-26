use super::{
    CampaignSelection, FaultAdapter, FaultCampaign, FaultCampaignRefusal, FaultSchedule,
    FaultSelectionRefusal, InjectedCommand, InjectedSequence, ScheduledFault, SequencePosition,
};
use crate::descriptor::NamespacedName;
use std::collections::BTreeSet;

impl<Behavior, Postcondition> FaultAdapter<Behavior, Postcondition> {
    /// A behavior joined to the postcondition its owner declares for it.
    #[must_use]
    pub const fn declared(behavior: Behavior, postcondition: Postcondition) -> Self {
        Self {
            behavior,
            postcondition,
        }
    }

    /// The behavior half.
    #[must_use]
    pub const fn behavior(&self) -> &Behavior {
        &self.behavior
    }

    /// The postcondition half.
    #[must_use]
    pub const fn postcondition(&self) -> &Postcondition {
        &self.postcondition
    }
}

impl SequencePosition {
    /// The position a schedule declares, counted from zero.
    #[must_use]
    pub const fn at(ordinal: u32) -> Self {
        Self(ordinal)
    }

    /// The ordinal this position carries.
    #[must_use]
    pub const fn ordinal(self) -> u32 {
        self.0
    }
}

impl<Behavior, Postcondition> ScheduledFault<Behavior, Postcondition> {
    /// One adapter placed at one position.
    #[must_use]
    pub const fn at(
        position: SequencePosition,
        adapter: FaultAdapter<Behavior, Postcondition>,
    ) -> Self {
        Self { position, adapter }
    }

    /// Where this adapter is placed.
    #[must_use]
    pub const fn position(&self) -> SequencePosition {
        self.position
    }

    /// The adapter itself.
    #[must_use]
    pub const fn adapter(&self) -> &FaultAdapter<Behavior, Postcondition> {
        &self.adapter
    }
}

impl<Behavior, Postcondition> FaultSchedule<Behavior, Postcondition> {
    /// A named schedule, in authored adapter order.
    #[must_use]
    pub fn declared(
        name: NamespacedName,
        faults: Vec<ScheduledFault<Behavior, Postcondition>>,
    ) -> Self {
        Self { name, faults }
    }

    /// The name this schedule is selected by.
    #[must_use]
    pub const fn name(&self) -> NamespacedName {
        self.name
    }

    /// The scheduled adapters, in authored order.
    #[must_use]
    pub fn faults(&self) -> &[ScheduledFault<Behavior, Postcondition>] {
        &self.faults
    }
}

impl<Behavior, Postcondition> FaultCampaign<Behavior, Postcondition> {
    /// A campaign over uniquely named schedules, in authored order.
    ///
    /// # Errors
    ///
    /// Refuses an empty campaign, then the first repeated name, then a campaign whose schedules are all empty controls.
    pub fn declared(
        schedules: Vec<FaultSchedule<Behavior, Postcondition>>,
    ) -> Result<Self, FaultCampaignRefusal> {
        if schedules.is_empty() {
            return Err(FaultCampaignRefusal::NoSchedule);
        }
        let mut seen = BTreeSet::new();
        for schedule in &schedules {
            if !seen.insert(schedule.name()) {
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

    /// The campaign's schedules, in authored order.
    #[must_use]
    pub fn schedules(&self) -> &[FaultSchedule<Behavior, Postcondition>] {
        &self.schedules
    }

    /// The schedule this campaign declares under `name`.
    ///
    /// # Errors
    ///
    /// Refuses a name no schedule here declares.
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
    /// The schedule the campaign handed back.
    #[must_use]
    pub const fn schedule(&self) -> &'campaign FaultSchedule<Behavior, Postcondition> {
        self.schedule
    }
}

impl<Command, Behavior, Postcondition> InjectedCommand<Command, Behavior, Postcondition> {
    /// One command and the adapters placed beside it, minted only by injection.
    #[must_use]
    pub(crate) const fn injected(
        command: Command,
        faults: Vec<FaultAdapter<Behavior, Postcondition>>,
    ) -> Self {
        Self { command, faults }
    }

    /// The command as the caller wrote it.
    #[must_use]
    pub const fn command(&self) -> &Command {
        &self.command
    }

    /// The adapters placed here, in schedule order.
    #[must_use]
    pub fn faults(&self) -> &[FaultAdapter<Behavior, Postcondition>] {
        &self.faults
    }
}

impl<Command, Behavior, Postcondition> InjectedSequence<Command, Behavior, Postcondition> {
    /// A whole injected sequence, minted only by injection.
    #[must_use]
    pub(crate) const fn injected(
        schedule: NamespacedName,
        commands: Vec<InjectedCommand<Command, Behavior, Postcondition>>,
    ) -> Self {
        Self { schedule, commands }
    }

    /// The name of the schedule that was injected.
    #[must_use]
    pub const fn schedule(&self) -> NamespacedName {
        self.schedule
    }

    /// The commands, each with the adapters placed at it.
    #[must_use]
    pub fn commands(&self) -> &[InjectedCommand<Command, Behavior, Postcondition>] {
        &self.commands
    }

    /// How many adapters landed, counted from the sequence rather than declared beside it.
    #[must_use]
    pub fn fault_count(&self) -> usize {
        self.commands
            .iter()
            .map(|command| command.faults().len())
            .sum()
    }
}
