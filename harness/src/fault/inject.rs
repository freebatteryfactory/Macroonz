use super::{
    CampaignSelection, FaultInjectionRefusal, InjectedCommand, InjectedSequence, ScheduledFault,
};

/// Inject a selected schedule's adapters into a command sequence.
///
/// Command order is kept, and adapters stacked on one command keep their authored order.
/// Every scheduled position is resolved before any adapter is cloned, so a refused schedule places nothing at all.
///
/// # Errors
///
/// Refuses a sequence too long to have positions, then the first scheduled position that lies outside it.
pub fn inject<Command, Behavior: Clone, Postcondition: Clone>(
    selection: &CampaignSelection<'_, Behavior, Postcondition>,
    commands: Vec<Command>,
) -> Result<InjectedSequence<Command, Behavior, Postcondition>, FaultInjectionRefusal> {
    let schedule = selection.schedule();
    let places = places_within(schedule.faults(), commands.len())?;
    let injected = commands
        .into_iter()
        .enumerate()
        .map(|(index, command)| {
            let faults = places
                .iter()
                .zip(schedule.faults())
                .filter(|(place, _)| **place == index)
                .map(|(_, scheduled)| scheduled.adapter().clone())
                .collect();
            InjectedCommand::injected(command, faults)
        })
        .collect();
    Ok(InjectedSequence::injected(schedule.name(), injected))
}

/// Where each scheduled adapter lands in a sequence of this many commands, or the first position that lands nowhere.
fn places_within<Behavior, Postcondition>(
    faults: &[ScheduledFault<Behavior, Postcondition>],
    commands: usize,
) -> Result<Vec<usize>, FaultInjectionRefusal> {
    if u32::try_from(commands.saturating_sub(1)).is_err() {
        return Err(FaultInjectionRefusal::SequenceTooLong { commands });
    }
    faults
        .iter()
        .map(|scheduled| {
            usize::try_from(scheduled.position().ordinal())
                .ok()
                .filter(|place| *place < commands)
                .ok_or(FaultInjectionRefusal::PositionOutsideSequence {
                    position: scheduled.position(),
                    commands,
                })
        })
        .collect()
}
