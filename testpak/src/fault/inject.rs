//! Joining one campaign-validated schedule to one ordinary command sequence.

use super::{
    CampaignSelection, FaultAdapter, FaultInjectionRefusal, InjectedCommand, InjectedSequence,
};

/// Inject the selected schedule's typed adapter values into a command sequence.
///
/// The command order and the authored order of faults stacked at one command are retained exactly. Every scheduled position is validated before any adopter-owned adapter is cloned; nothing is silently dropped.
///
/// # Errors
///
/// Refuses the first scheduled position outside the sequence, in authored schedule order.
pub fn inject<Command, Behavior: Clone, Postcondition: Clone>(
    selection: &CampaignSelection<'_, Behavior, Postcondition>,
    commands: Vec<Command>,
) -> Result<InjectedSequence<Command, Behavior, Postcondition>, FaultInjectionRefusal> {
    let command_count = commands.len();
    if command_count > 0 && u32::try_from(command_count.saturating_sub(1)).is_err() {
        return Err(FaultInjectionRefusal::SequenceTooLong {
            commands: command_count,
        });
    }
    let mut positions = Vec::with_capacity(selection.schedule().faults().len());
    for scheduled in selection.schedule().faults() {
        let Ok(position) = usize::try_from(scheduled.position().ordinal()) else {
            return Err(FaultInjectionRefusal::PositionOutsideSequence {
                position: scheduled.position(),
                commands: command_count,
            });
        };
        if position >= command_count {
            return Err(FaultInjectionRefusal::PositionOutsideSequence {
                position: scheduled.position(),
                commands: command_count,
            });
        }
        positions.push(position);
    }
    let mut placements: Vec<Vec<FaultAdapter<Behavior, Postcondition>>> =
        (0..command_count).map(|_| Vec::new()).collect();
    for (scheduled, position) in selection.schedule().faults().iter().zip(positions) {
        let Some(at) = placements.get_mut(position) else {
            return Err(FaultInjectionRefusal::PositionOutsideSequence {
                position: scheduled.position(),
                commands: command_count,
            });
        };
        at.push(scheduled.adapter().clone());
    }
    let commands = commands
        .into_iter()
        .zip(placements)
        .map(|(command, faults)| InjectedCommand::injected(command, faults))
        .collect();
    Ok(InjectedSequence::injected(
        selection.schedule().name(),
        commands,
    ))
}
