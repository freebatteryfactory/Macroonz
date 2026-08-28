//! Re-execution of an admitted simulation manifest against its addressed delivery roster.

use super::project::encoded_delivery;
use super::{
    SimulationAction, SimulationReproduction, TranscriptEntry, TranscriptMaterial, TranscriptPack,
    TranscriptRefusal,
};
use crate::network::simulation::{NetworkSelection, SimNet};

/// Execute one admitted simulation manifest and compare its complete delivery roster with the addressed rows.
///
/// # Errors
///
/// Refuses recorded-live material, a schedule that cannot open over the retained topology, a send the reproduced sim refuses, or the first reproduced row that differs or is absent.
pub fn reproduce(pack: &TranscriptPack) -> Result<SimulationReproduction, TranscriptRefusal> {
    let TranscriptMaterial::Simulated(manifest) = &pack.material else {
        return Err(TranscriptRefusal::RecordedLiveCannotReproduce);
    };
    let selection = NetworkSelection::retained(manifest.schedule());
    let mut sim = SimNet::declared(pack.topology.clone(), selection)
        .map_err(TranscriptRefusal::SimulationNotOpened)?;
    let mut deliveries = Vec::new();
    for (at, action) in manifest.actions().iter().enumerate() {
        match action {
            SimulationAction::Send { link, payload } => {
                sim.send(*link, payload.clone())
                    .map_err(|refusal| TranscriptRefusal::SimulationSendRefused { at, refusal })?;
            }
            SimulationAction::Advance => deliveries.extend(sim.advance()),
        }
    }
    let reproduced: Vec<_> = deliveries
        .iter()
        .map(|delivery| encoded_delivery(delivery, Vec::clone))
        .collect();
    if let Some(at) = first_divergence(&reproduced, pack.entries()) {
        return Err(TranscriptRefusal::SimulationRowsDiverge { at });
    }
    Ok(SimulationReproduction::witnessed(
        pack.address(),
        manifest.actions().len(),
        reproduced.len(),
        sim.tick(),
    ))
}

/// The first roster position where two exact delivery accounts differ.
fn first_divergence(left: &[TranscriptEntry], right: &[TranscriptEntry]) -> Option<usize> {
    let common = left.len().min(right.len());
    left.iter()
        .zip(right)
        .position(|(left, right)| left != right)
        .or_else(|| (left.len() != right.len()).then_some(common))
}
