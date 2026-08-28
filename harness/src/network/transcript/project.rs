//! Projection of retained simulation actions and deliveries into transcript byte values.

use super::{SimulationAction, TranscriptEntry};
use crate::network::simulation::{Action, Delivery};

/// Encode every retained sim action through the caller's declared payload projection.
pub(super) fn encoded_actions<Payload>(
    actions: &[Action<Payload>],
    encode_payload: fn(&Payload) -> Vec<u8>,
) -> Vec<SimulationAction> {
    actions
        .iter()
        .map(|action| match action {
            Action::Send { link, payload } => SimulationAction::Send {
                link: *link,
                payload: encode_payload(payload),
            },
            Action::Advance => SimulationAction::Advance,
        })
        .collect()
}

/// Encode one delivery without changing its lineage.
pub(super) fn encoded_delivery<Payload>(
    delivery: &Delivery<Payload>,
    encode_payload: fn(&Payload) -> Vec<u8>,
) -> TranscriptEntry {
    TranscriptEntry::witnessed(
        delivery.link(),
        delivery.ordinal(),
        encode_payload(delivery.payload()),
        delivery.sent_at(),
        delivery.delivered_at(),
        delivery.copy(),
    )
}
