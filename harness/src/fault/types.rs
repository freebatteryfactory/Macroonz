//! What a fault is, where it is scheduled, and what a selection and an injection hand back.

#[path = "type_guard.rs"]
mod guard;

use crate::descriptor::NamespacedName;

/// One behavior the owner wrote, joined to the postcondition the owner promises it leaves standing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FaultAdapter<Behavior, Postcondition> {
    behavior: Behavior,
    postcondition: Postcondition,
}

/// One zero-based position in a command sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SequencePosition(u32);

/// One adapter placed at one command position.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduledFault<Behavior, Postcondition> {
    position: SequencePosition,
    adapter: FaultAdapter<Behavior, Postcondition>,
}

/// One named course of adversity, from an empty control to a stack of adapters.
///
/// Adapters keep their authored order, and two may name one position, because stacking two faults on one command is a statement rather than a duplicate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FaultSchedule<Behavior, Postcondition> {
    name: NamespacedName,
    faults: Vec<ScheduledFault<Behavior, Postcondition>>,
}

/// The uniquely named schedules one run chooses among.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FaultCampaign<Behavior, Postcondition> {
    schedules: Vec<FaultSchedule<Behavior, Postcondition>>,
}

/// Why a campaign was refused.
#[must_use = "a refusal is the reason a fault campaign was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaultCampaignRefusal {
    /// The campaign declares no schedule, so no selection could be satisfied.
    NoSchedule,
    /// Two schedules declare the same name.
    DuplicateSchedule(NamespacedName),
    /// Every schedule is an empty control, so the campaign declares no pressure at all.
    NoFaultDeclared,
}

/// One schedule, handed back by the campaign that declares it.
///
/// The selection borrows its campaign member, so injection can never be given one campaign's schedule beside another campaign's name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CampaignSelection<'campaign, Behavior, Postcondition> {
    schedule: &'campaign FaultSchedule<Behavior, Postcondition>,
}

/// Why a selection was refused.
#[must_use = "a refusal is the reason a fault schedule was not selected"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaultSelectionRefusal {
    /// The campaign declares no schedule under this name.
    ScheduleAbsent(NamespacedName),
}

/// One command and the adapters scheduled at its position, in schedule order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InjectedCommand<Command, Behavior, Postcondition> {
    command: Command,
    faults: Vec<FaultAdapter<Behavior, Postcondition>>,
}

/// A command sequence with one selected schedule placed into it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InjectedSequence<Command, Behavior, Postcondition> {
    schedule: NamespacedName,
    commands: Vec<InjectedCommand<Command, Behavior, Postcondition>>,
}

/// Why a selected schedule could not be injected.
#[must_use = "a refusal is the reason a selected fault schedule was not injected"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaultInjectionRefusal {
    /// The sequence's last coordinate is past what a [`SequencePosition`] can spell.
    SequenceTooLong {
        /// How many commands the sequence carries.
        commands: usize,
    },
    /// A scheduled position lies outside the sequence.
    PositionOutsideSequence {
        /// The position the schedule declared.
        position: SequencePosition,
        /// How many commands the sequence carries.
        commands: usize,
    },
}
