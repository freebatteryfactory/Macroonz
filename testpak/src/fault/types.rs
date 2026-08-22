//! The fault instrument's declarations: where typed adapters are scheduled, how schedules are selected, and what injection returns.
//!
//! Declarations only. Every road that reaches a private field lives in this file's own child, `type_guard.rs`; joining a selected schedule to commands is `inject.rs`.

#[path = "type_guard.rs"]
mod guard;

use crate::descriptor::NamespacedName;

/// One adopter-owned fault behavior joined to the adopter-owned postcondition it promises.
///
/// `TestPak` carries the two typed values together and interprets neither one. The behavior may implement a concrete product port contract, while the postcondition remains available to the consumer that executes and observes it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FaultAdapter<Behavior, Postcondition> {
    behavior: Behavior,
    postcondition: Postcondition,
}

/// One zero-based position in a command sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SequencePosition(u32);

/// One typed adapter scheduled at one command position.
///
/// The generic parameters are the adopter's concrete behavior and postcondition types. `TestPak` stores and places the joined value without deciding which port contract the behavior implements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduledFault<Behavior, Postcondition> {
    position: SequencePosition,
    adapter: FaultAdapter<Behavior, Postcondition>,
}

/// One named schedule of typed adapters, including an empty lawful-control schedule.
///
/// # Ordering
///
/// Scheduled values retain authored order. Several values may name one position because stacking two distinct faults is a campaign statement, not a duplicate declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FaultSchedule<Behavior, Postcondition> {
    name: NamespacedName,
    faults: Vec<ScheduledFault<Behavior, Postcondition>>,
}

/// A nonempty set of uniquely named fault schedules.
///
/// A campaign is selection authority only. It does not execute adapters or interpret their postconditions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FaultCampaign<Behavior, Postcondition> {
    schedules: Vec<FaultSchedule<Behavior, Postcondition>>,
}

/// Why one fault campaign was refused.
#[must_use = "a refusal is the reason a fault campaign was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaultCampaignRefusal {
    /// The campaign declares no schedule, so no selection could be satisfied.
    NoSchedule,
    /// Two schedules declare the same name.
    DuplicateSchedule(NamespacedName),
    /// Every schedule is an empty control, so the campaign declares no fault pressure.
    NoFaultDeclared,
}

/// One campaign-validated schedule selection.
///
/// The selection borrows the campaign member it names, so injection cannot be handed a schedule from another campaign beside a separately asserted name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CampaignSelection<'campaign, Behavior, Postcondition> {
    schedule: &'campaign FaultSchedule<Behavior, Postcondition>,
}

/// Why one campaign selection was refused.
#[must_use = "a refusal is the reason a fault schedule was not selected"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaultSelectionRefusal {
    /// The campaign declares no schedule under this name.
    ScheduleAbsent(NamespacedName),
}

/// One command with the typed adapters injected at its position.
///
/// The command remains the caller's value, and faults retain their schedule order. Executing either is outside this type's authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InjectedCommand<Command, Behavior, Postcondition> {
    command: Command,
    faults: Vec<FaultAdapter<Behavior, Postcondition>>,
}

/// One command sequence after a validated fault schedule was injected.
///
/// # Authority
///
/// The retained schedule name and derived injected count describe only placement. A consumer earns behavior and postcondition conclusions by executing its own adapter and observing the result through the ordinary `TestPak` roads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InjectedSequence<Command, Behavior, Postcondition> {
    schedule: NamespacedName,
    commands: Vec<InjectedCommand<Command, Behavior, Postcondition>>,
}

/// Why one selected schedule could not be injected into a command sequence.
#[must_use = "a refusal is the reason a selected fault schedule was not injected"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaultInjectionRefusal {
    /// The sequence has a last coordinate that cannot be represented by [`SequencePosition`].
    SequenceTooLong {
        /// The number of commands the sequence carries.
        commands: usize,
    },
    /// A scheduled position lies outside the supplied sequence.
    PositionOutsideSequence {
        /// The invalid scheduled position.
        position: SequencePosition,
        /// The number of commands the sequence carries.
        commands: usize,
    },
}
