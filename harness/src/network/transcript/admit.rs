//! Admission of a delivery roster against its owner-built topology and temporal order.

use super::{TranscriptEntry, TranscriptRefusal};
use crate::network::simulation::{Tick, Topology};

/// Whether every row belongs to the topology, no delivery precedes its own send, and the stamps never step backward.
pub(super) fn lawful_entries(
    topology: &Topology,
    entries: &[TranscriptEntry],
) -> Result<(), TranscriptRefusal> {
    if entries.is_empty() {
        return Err(TranscriptRefusal::NoDelivery);
    }
    let mut latest = Tick::at(0u64);
    for (at, entry) in entries.iter().enumerate() {
        if !topology.links().contains(&entry.link()) {
            return Err(TranscriptRefusal::ForeignLink { at });
        }
        if entry.delivered_at() < entry.sent_at() {
            return Err(TranscriptRefusal::DeliveryBeforeSend { at });
        }
        if entry.delivered_at() < latest {
            return Err(TranscriptRefusal::DeliveryOrderBroken { at });
        }
        latest = entry.delivered_at();
    }
    Ok(())
}
