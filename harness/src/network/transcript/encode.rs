//! The exact addressed body encoding for transcript material.

use super::{
    SimulationAction, SimulationManifest, TRANSCRIPT_FORMAT_VERSION, TranscriptEntry,
    TranscriptMaterial, TranscriptRefusal, TranscriptSourceClaim,
};
use crate::network::simulation::{
    DeliveryCopy, Link, LinkDiscipline, LinkFault, NodeRef, Topology,
};
use crate::report::{encode_bytes, encode_length};

/// The complete address preimage of one transcript.
pub(super) fn encode_body(
    material: &TranscriptMaterial,
    topology: &Topology,
    entries: &[TranscriptEntry],
) -> Vec<u8> {
    let mut body = Vec::new();
    body.extend_from_slice(&TRANSCRIPT_FORMAT_VERSION.to_be_bytes());
    body.extend_from_slice(&source_slot(source_claim(material)).to_be_bytes());
    encode_topology(topology, &mut body);
    if let TranscriptMaterial::Simulated(manifest) = material {
        encode_manifest(manifest, &mut body);
    }
    encode_entries(entries, &mut body);
    body
}

/// Append one topology in authored order.
fn encode_topology(topology: &Topology, into: &mut Vec<u8>) {
    encode_length(topology.nodes().len(), into);
    for node in topology.nodes() {
        encode_node(*node, into);
    }
    encode_length(topology.links().len(), into);
    for link in topology.links() {
        encode_link(*link, into);
    }
}

/// Append one complete selected-schedule and action manifest.
fn encode_manifest(manifest: &SimulationManifest, into: &mut Vec<u8>) {
    let schedule = manifest.schedule();
    schedule.name().encode_into(into);
    encode_length(schedule.disciplines().len(), into);
    for discipline in schedule.disciplines() {
        encode_discipline(discipline, into);
    }
    encode_length(manifest.actions().len(), into);
    for action in manifest.actions() {
        encode_action(action, into);
    }
}

/// Append one link discipline and its ordered fault roster.
fn encode_discipline(discipline: &LinkDiscipline, into: &mut Vec<u8>) {
    encode_link(discipline.link(), into);
    encode_length(discipline.faults().len(), into);
    for fault in discipline.faults() {
        encode_fault(*fault, into);
    }
}

/// Append one fault under its stable slot.
fn encode_fault(fault: LinkFault, into: &mut Vec<u8>) {
    match fault {
        LinkFault::DropAt { position } => {
            into.extend_from_slice(&0u32.to_be_bytes());
            into.extend_from_slice(&position.ordinal().to_be_bytes());
        }
        LinkFault::DelayAt { position, ticks } => {
            into.extend_from_slice(&1u32.to_be_bytes());
            into.extend_from_slice(&position.ordinal().to_be_bytes());
            into.extend_from_slice(&ticks.ticks().to_be_bytes());
        }
        LinkFault::DuplicateAt { position } => {
            into.extend_from_slice(&2u32.to_be_bytes());
            into.extend_from_slice(&position.ordinal().to_be_bytes());
        }
        LinkFault::Partition { opens, heals } => {
            into.extend_from_slice(&3u32.to_be_bytes());
            into.extend_from_slice(&opens.ordinal().to_be_bytes());
            into.extend_from_slice(&heals.ordinal().to_be_bytes());
        }
    }
}

/// Append one simulation action under its stable slot.
fn encode_action(action: &SimulationAction, into: &mut Vec<u8>) {
    match action {
        SimulationAction::Send { link, payload } => {
            into.extend_from_slice(&0u32.to_be_bytes());
            encode_link(*link, into);
            encode_bytes(payload, into);
        }
        SimulationAction::Advance => into.extend_from_slice(&1u32.to_be_bytes()),
    }
}

/// Append the ordered delivery roster.
fn encode_entries(entries: &[TranscriptEntry], into: &mut Vec<u8>) {
    encode_length(entries.len(), into);
    for entry in entries {
        encode_link(entry.link(), into);
        into.extend_from_slice(&entry.ordinal().ordinal().to_be_bytes());
        encode_bytes(entry.payload(), into);
        into.extend_from_slice(&entry.sent_at().ordinal().to_be_bytes());
        into.extend_from_slice(&entry.delivered_at().ordinal().to_be_bytes());
        into.extend_from_slice(&copy_slot(entry.copy()).to_be_bytes());
    }
}

/// Append one node's name through the type's own seated spelling.
fn encode_node(node: NodeRef, into: &mut Vec<u8>) {
    node.name().encode_into(into);
}

/// Append one link's four name parts.
fn encode_link(link: Link, into: &mut Vec<u8>) {
    encode_node(link.from(), into);
    encode_node(link.to(), into);
}

/// The source claim retained by one internal material arm.
const fn source_claim(material: &TranscriptMaterial) -> TranscriptSourceClaim {
    match material {
        TranscriptMaterial::Simulated(_) => TranscriptSourceClaim::Simulated,
        TranscriptMaterial::RecordedLive => TranscriptSourceClaim::RecordedLive,
    }
}

/// The stable slot one source claim occupies.
const fn source_slot(source: TranscriptSourceClaim) -> u32 {
    match source {
        TranscriptSourceClaim::Simulated => 0u32,
        TranscriptSourceClaim::RecordedLive => 1u32,
    }
}

/// The source claim one stable slot spells.
pub(super) const fn source_of(slot: u32) -> Result<TranscriptSourceClaim, TranscriptRefusal> {
    match slot {
        0u32 => Ok(TranscriptSourceClaim::Simulated),
        1u32 => Ok(TranscriptSourceClaim::RecordedLive),
        found => Err(TranscriptRefusal::UnknownSourceClaim { found }),
    }
}

/// The stable slot one delivery-copy posture occupies.
const fn copy_slot(copy: DeliveryCopy) -> u32 {
    match copy {
        DeliveryCopy::Original => 0u32,
        DeliveryCopy::Duplicate => 1u32,
    }
}

/// The delivery-copy posture one stable slot spells.
pub(super) const fn copy_of(slot: u32) -> Result<DeliveryCopy, TranscriptRefusal> {
    match slot {
        0u32 => Ok(DeliveryCopy::Original),
        1u32 => Ok(DeliveryCopy::Duplicate),
        found => Err(TranscriptRefusal::UnknownCopy { found }),
    }
}
