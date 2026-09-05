//! Source-specific transcript reading against owner-built topology and schedule values.

use super::admit::lawful_entries;
use super::encode::{copy_of, source_of};
use super::{
    SimulationAction, SimulationManifest, TRANSCRIPT_FORMAT_VERSION, TRANSCRIPT_TAG,
    TranscriptAddress, TranscriptEntry, TranscriptMaterial, TranscriptPack, TranscriptRefusal,
    TranscriptSourceClaim,
};
use crate::descriptor::NamespacedName;
use crate::identity::{BodyReader as IdentityBodyReader, addressed_body};
use crate::network::simulation::{
    Link, LinkDiscipline, LinkFault, NetworkSchedule, NodeRef, SendOrdinal, Tick, Topology,
};

type BodyReader<'body> = IdentityBodyReader<'body, TranscriptRefusal>;

/// Read one live-recorded transcript envelope for the topology the caller expects.
///
/// The leading claim is settled before a member of the body is interpreted, and the body must declare the live-recorded source posture.
///
/// # Errors
///
/// Refuses every malformed envelope in reading order and a body whose source claim is not live-recorded.
pub fn read_recorded_live(
    expected: &Topology,
    encoded: &[u8],
) -> Result<TranscriptPack, TranscriptRefusal> {
    read_as(expected, None, TranscriptSourceClaim::RecordedLive, encoded)
}

/// Read one simulated transcript envelope for the topology and selected schedule the caller expects.
///
/// The static names remain owner-built values: foreign bytes are compared with the supplied schedule and never mint names.
/// Reading admits addressed declaration material only; [`crate::network::reproduce`] is the separate operation that can mint simulation standing.
///
/// # Errors
///
/// Refuses every malformed envelope in reading order, a body whose source claim is not simulated, or a schedule different from the owner-built expected value.
pub fn read_simulated(
    expected: &Topology,
    schedule: &NetworkSchedule,
    encoded: &[u8],
) -> Result<TranscriptPack, TranscriptRefusal> {
    read_as(
        expected,
        Some(schedule),
        TranscriptSourceClaim::Simulated,
        encoded,
    )
}

/// Read one envelope under the source-specific public road that called here.
fn read_as(
    expected: &Topology,
    schedule: Option<&NetworkSchedule>,
    source: TranscriptSourceClaim,
    encoded: &[u8],
) -> Result<TranscriptPack, TranscriptRefusal> {
    let (address, body) = addressed_body(
        encoded,
        TRANSCRIPT_TAG,
        TranscriptAddress::derived,
        TranscriptRefusal::Truncated,
        |derived| TranscriptRefusal::AddressMismatch { derived },
    )?;
    let (material, entries) = read_body(expected, schedule, source, body)?;
    lawful_entries(expected, &entries)?;
    Ok(TranscriptPack::assembled(
        expected.clone(),
        material,
        address,
        entries,
        encoded.to_vec(),
    ))
}

/// Read every member the body declares under the source-specific public road.
fn read_body(
    expected: &Topology,
    expected_schedule: Option<&NetworkSchedule>,
    expected_source: TranscriptSourceClaim,
    body: &[u8],
) -> Result<(TranscriptMaterial, Vec<TranscriptEntry>), TranscriptRefusal> {
    let mut reader = BodyReader::over(body, TranscriptRefusal::Truncated, |declared| {
        TranscriptRefusal::LengthOutsidePlatform { declared }
    });
    let found = reader.u32()?;
    if found != TRANSCRIPT_FORMAT_VERSION {
        return Err(TranscriptRefusal::UnsupportedFormat { found });
    }
    let source = source_of(reader.u32()?)?;
    if source != expected_source {
        return Err(TranscriptRefusal::SourceClaimMismatch {
            expected: expected_source,
            found: source,
        });
    }
    read_topology(expected, &mut reader)?;
    let material = read_material(source, expected, expected_schedule, &mut reader)?;
    let entries = read_entries(expected, &mut reader)?;
    let trailing = reader.remaining();
    if trailing != 0usize {
        return Err(TranscriptRefusal::TrailingBytes { count: trailing });
    }
    Ok((material, entries))
}

/// Read source-specific material after the common header and topology.
fn read_material(
    source: TranscriptSourceClaim,
    expected: &Topology,
    expected_schedule: Option<&NetworkSchedule>,
    reader: &mut BodyReader<'_>,
) -> Result<TranscriptMaterial, TranscriptRefusal> {
    match source {
        TranscriptSourceClaim::RecordedLive => Ok(TranscriptMaterial::RecordedLive),
        TranscriptSourceClaim::Simulated => {
            let Some(schedule) = expected_schedule else {
                return Err(TranscriptRefusal::ScheduleMismatch);
            };
            read_schedule(schedule, reader)?;
            let actions = read_actions(expected, reader)?;
            Ok(TranscriptMaterial::Simulated(SimulationManifest::captured(
                schedule.clone(),
                actions,
            )))
        }
    }
}

/// Compare the encoded topology section with the owner-built expected value.
fn read_topology(
    expected: &Topology,
    reader: &mut BodyReader<'_>,
) -> Result<(), TranscriptRefusal> {
    let nodes = reader.count()?;
    if nodes != expected.nodes().len() {
        return Err(TranscriptRefusal::TopologyMismatch);
    }
    for node in expected.nodes() {
        read_expected_name(node.name(), reader, TranscriptRefusal::TopologyMismatch)?;
    }
    let links = reader.count()?;
    if links != expected.links().len() {
        return Err(TranscriptRefusal::TopologyMismatch);
    }
    for link in expected.links() {
        read_expected_link(*link, reader, TranscriptRefusal::TopologyMismatch)?;
    }
    Ok(())
}

/// Compare the encoded schedule section with the owner-built expected value.
fn read_schedule(
    expected: &NetworkSchedule,
    reader: &mut BodyReader<'_>,
) -> Result<(), TranscriptRefusal> {
    read_expected_name(expected.name(), reader, TranscriptRefusal::ScheduleMismatch)?;
    let count = reader.count()?;
    if count != expected.disciplines().len() {
        return Err(TranscriptRefusal::ScheduleMismatch);
    }
    for discipline in expected.disciplines() {
        read_discipline(discipline, reader)?;
    }
    Ok(())
}

/// Compare one encoded discipline and fault roster with the expected value.
fn read_discipline(
    expected: &LinkDiscipline,
    reader: &mut BodyReader<'_>,
) -> Result<(), TranscriptRefusal> {
    read_expected_link(expected.link(), reader, TranscriptRefusal::ScheduleMismatch)?;
    let count = reader.count()?;
    if count != expected.faults().len() {
        return Err(TranscriptRefusal::ScheduleMismatch);
    }
    for fault in expected.faults() {
        read_fault(*fault, reader)?;
    }
    Ok(())
}

/// Compare one encoded fault with its expected typed value.
fn read_fault(expected: LinkFault, reader: &mut BodyReader<'_>) -> Result<(), TranscriptRefusal> {
    let slot = reader.u32()?;
    let matches = match slot {
        0u32 => {
            expected
                == LinkFault::DropAt {
                    position: SendOrdinal::at(reader.u32()?),
                }
        }
        1u32 => {
            let position = SendOrdinal::at(reader.u32()?);
            let ticks = reader.u32()?;
            matches!(expected, LinkFault::DelayAt { position: expected_position, ticks: expected_ticks } if expected_position == position && expected_ticks.ticks() == ticks)
        }
        2u32 => {
            expected
                == LinkFault::DuplicateAt {
                    position: SendOrdinal::at(reader.u32()?),
                }
        }
        3u32 => {
            expected
                == LinkFault::Partition {
                    opens: Tick::at(reader.u64()?),
                    heals: Tick::at(reader.u64()?),
                }
        }
        found => return Err(TranscriptRefusal::UnknownFault { found }),
    };
    if !matches {
        return Err(TranscriptRefusal::ScheduleMismatch);
    }
    Ok(())
}

/// Read the complete action roster, resolving every send link against the expected topology.
fn read_actions(
    expected: &Topology,
    reader: &mut BodyReader<'_>,
) -> Result<Vec<SimulationAction>, TranscriptRefusal> {
    let count = reader.count()?;
    let mut actions = Vec::new();
    for at in 0..count {
        match reader.u32()? {
            0u32 => {
                let link = read_action_link(expected, at, reader)?;
                let payload = reader.bytes()?.to_vec();
                actions.push(SimulationAction::Send { link, payload });
            }
            1u32 => actions.push(SimulationAction::Advance),
            found => return Err(TranscriptRefusal::UnknownAction { found }),
        }
    }
    Ok(actions)
}

/// Read the entry roster, resolving each row's link against the expected topology.
fn read_entries(
    expected: &Topology,
    reader: &mut BodyReader<'_>,
) -> Result<Vec<TranscriptEntry>, TranscriptRefusal> {
    let count = reader.count()?;
    let mut entries = Vec::new();
    for at in 0..count {
        let link = read_entry_link(expected, at, reader)?;
        let ordinal = SendOrdinal::at(reader.u32()?);
        let payload = reader.bytes()?.to_vec();
        let sent_at = Tick::at(reader.u64()?);
        let delivered_at = Tick::at(reader.u64()?);
        let copy = copy_of(reader.u32()?)?;
        entries.push(TranscriptEntry::witnessed(
            link,
            ordinal,
            payload,
            sent_at,
            delivered_at,
            copy,
        ));
    }
    Ok(entries)
}

/// Read one expected name without minting names from foreign bytes.
fn read_expected_name(
    expected: NamespacedName,
    reader: &mut BodyReader<'_>,
    mismatch: TranscriptRefusal,
) -> Result<(), TranscriptRefusal> {
    let namespace = reader.bytes()?;
    let stem = reader.bytes()?;
    if namespace != expected.namespace().written().as_bytes()
        || stem != expected.stem().written().as_bytes()
    {
        return Err(mismatch);
    }
    Ok(())
}

/// Read one expected link without minting node names from foreign bytes.
fn read_expected_link(
    expected: Link,
    reader: &mut BodyReader<'_>,
    mismatch: TranscriptRefusal,
) -> Result<(), TranscriptRefusal> {
    read_expected_name(expected.from().name(), reader, mismatch)?;
    read_expected_name(expected.to().name(), reader, mismatch)
}

/// Resolve one action link from encoded name bytes.
fn read_action_link(
    expected: &Topology,
    at: usize,
    reader: &mut BodyReader<'_>,
) -> Result<Link, TranscriptRefusal> {
    read_link(expected, reader)?.ok_or(TranscriptRefusal::SimulationActionForeignLink { at })
}

/// Resolve one delivery-entry link from encoded name bytes.
fn read_entry_link(
    expected: &Topology,
    at: usize,
    reader: &mut BodyReader<'_>,
) -> Result<Link, TranscriptRefusal> {
    read_link(expected, reader)?.ok_or(TranscriptRefusal::ForeignLink { at })
}

/// Read four name parts and resolve them against the expected topology's links.
fn read_link(
    expected: &Topology,
    reader: &mut BodyReader<'_>,
) -> Result<Option<Link>, TranscriptRefusal> {
    let from_namespace = reader.bytes()?.to_vec();
    let from_stem = reader.bytes()?.to_vec();
    let to_namespace = reader.bytes()?.to_vec();
    let to_stem = reader.bytes()?.to_vec();
    Ok(expected
        .links()
        .iter()
        .find(|link| {
            spells(link.from(), &from_namespace, &from_stem)
                && spells(link.to(), &to_namespace, &to_stem)
        })
        .copied())
}

/// Whether this node's name is spelled by these two byte strings.
fn spells(node: NodeRef, namespace: &[u8], stem: &[u8]) -> bool {
    let name = node.name();
    name.namespace().written().as_bytes() == namespace && name.stem().written().as_bytes() == stem
}
