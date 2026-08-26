//! Source-specific transcript writing from live rows or a retained deterministic simulation.

use super::admit::lawful_entries;
use super::encode::encode_body;
use super::project::{encoded_actions, encoded_delivery};
use super::reproduce_run::reproduce;
use super::{
    SimulationManifest, SimulationReproduction, TRANSCRIPT_TAG, TranscriptAddress, TranscriptEntry,
    TranscriptMaterial, TranscriptPack, TranscriptRefusal,
};
use crate::identity::ContentAddress;
use crate::network::simulation::{SimNet, Topology};

/// Write one content-addressed live-recorded transcript in delivery order.
///
/// The address is derived here rather than accepted from anyone, and every row is judged against the topology before a byte is written.
/// This road records an adopter's source claim and mints no simulation standing.
///
/// # Errors
///
/// Refuses a transcript with no delivery, then the first entry on a link the topology never declared, then the first entry delivered before its own send, then the first entry stamped earlier than the entry before it.
pub fn recorded_live(
    topology: &Topology,
    entries: Vec<TranscriptEntry>,
) -> Result<TranscriptPack, TranscriptRefusal> {
    write_pack(topology.clone(), TranscriptMaterial::RecordedLive, entries)
}

/// Write one content-addressed simulated transcript from the sim that retained its whole drive.
///
/// The payload encoder supplies the byte projection used both by the input actions and by the resulting delivery rows.
/// The writer executes the completed byte-valued manifest again before returning its separately minted reproduction standing.
///
/// # Errors
///
/// Refuses everything the ordinary transcript writer refuses, then any failure to re-open, drive, or exactly reproduce the retained manifest.
pub fn simulated<Payload>(
    sim: &SimNet<Payload>,
    encode_payload: fn(&Payload) -> Vec<u8>,
) -> Result<(TranscriptPack, SimulationReproduction), TranscriptRefusal> {
    let actions = encoded_actions(sim.retained_actions(), encode_payload);
    let entries = sim
        .retained_history()
        .iter()
        .map(|delivery| encoded_delivery(delivery, encode_payload))
        .collect();
    let material = TranscriptMaterial::Simulated(SimulationManifest::captured(
        sim.retained_schedule().clone(),
        actions,
    ));
    let pack = write_pack(sim.retained_topology().clone(), material, entries)?;
    let reproduction = reproduce(&pack)?;
    Ok((pack, reproduction))
}

/// Build and address a pack after its source-specific caller has assembled complete material.
fn write_pack(
    topology: Topology,
    material: TranscriptMaterial,
    entries: Vec<TranscriptEntry>,
) -> Result<TranscriptPack, TranscriptRefusal> {
    lawful_entries(&topology, &entries)?;
    let body = encode_body(&material, &topology, &entries);
    let address = TranscriptAddress::derived(ContentAddress::derived(TRANSCRIPT_TAG, &body));
    let claim = address.address();
    let mut encoded = Vec::with_capacity(claim.as_bytes().len().saturating_add(body.len()));
    encoded.extend_from_slice(claim.as_bytes());
    encoded.extend_from_slice(&body);
    Ok(TranscriptPack::assembled(
        topology, material, address, entries, encoded,
    ))
}
