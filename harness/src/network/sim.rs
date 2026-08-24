//! The sim's two moves: placing a send under its link's discipline, and advancing the tick to collect what came due.
//!
//! Determinism is structural on both roads.
//! A send's fate follows from the declared faults at its exact link and ordinal, with a partition read first because a dead wire outranks any shaping of live traffic; a delivery's place in an advance follows from its due tick and its scheduling sequence, and from nothing else.

use super::{
    Delivery, DeliveryCopy, Link, LinkFault, SendFate, SendOrdinal, SendReceipt, SendRefusal,
    SimNet, Tick,
};
use std::mem;

/// One scheduled delivery, waiting for its tick.
#[derive(Debug, Clone)]
pub(super) struct InFlight<Payload> {
    /// The tick this delivery comes due.
    due: Tick,
    /// The global scheduling sequence, which orders same-tick deliveries.
    sequence: u64,
    /// The link the payload travels.
    link: Link,
    /// The send's ordinal on that link.
    ordinal: SendOrdinal,
    /// The payload itself.
    payload: Payload,
    /// The tick the send was placed at.
    sent_at: Tick,
    /// Whether this copy is the original or a duplicate.
    copy: DeliveryCopy,
}

/// What one send's declared faults add up to, before anything is scheduled.
enum Shaping {
    /// The send travels, carrying this much declared delay and this many copies.
    Travels {
        /// How many ticks of declared delay the send carries.
        delay: u64,
        /// One for the original, plus one per duplicate fault.
        copies: u32,
    },
    /// An open partition took the send.
    TakenByPartition,
    /// A drop fault took the send.
    TakenByDiscipline,
}

impl<Payload: Clone> SimNet<Payload> {
    /// Place one payload on a link at the current tick, under the link's discipline.
    ///
    /// The receipt is the experimenter's record of the send's fate; whether the subject under test learns any of it is the adopter's port's decision.
    ///
    /// # Errors
    ///
    /// Refuses a link the topology never declared.
    pub fn send(&mut self, link: Link, payload: Payload) -> Result<SendReceipt, SendRefusal> {
        if !self.topology.links().contains(&link) {
            return Err(SendRefusal::LinkUndeclared(link));
        }
        let count = self.placed.entry(link).or_insert(0u32);
        let ordinal = SendOrdinal::at(*count);
        *count = count.saturating_add(1u32);
        self.census.sends = self.census.sends.saturating_add(1u64);
        let shaping = shaped(
            self.schedule
                .discipline_of(link)
                .map_or(&[][..], super::LinkDiscipline::faults),
            ordinal,
            self.tick,
        );
        let (delay, copies) = match shaping {
            Shaping::TakenByPartition => {
                self.census.dropped_by_partition =
                    self.census.dropped_by_partition.saturating_add(1u64);
                return Ok(SendReceipt {
                    link,
                    ordinal,
                    fate: SendFate::DroppedByPartition,
                });
            }
            Shaping::TakenByDiscipline => {
                self.census.dropped_by_discipline =
                    self.census.dropped_by_discipline.saturating_add(1u64);
                return Ok(SendReceipt {
                    link,
                    ordinal,
                    fate: SendFate::DroppedByDiscipline,
                });
            }
            Shaping::Travels { delay, copies } => (delay, copies),
        };
        let due = self.tick.later_by(delay.saturating_add(1u64));
        for placed in 0u32..copies {
            let copy = if placed == 0u32 {
                DeliveryCopy::Original
            } else {
                DeliveryCopy::Duplicate
            };
            self.in_flight.push(InFlight {
                due,
                sequence: self.sequence,
                link,
                ordinal,
                payload: payload.clone(),
                sent_at: self.tick,
                copy,
            });
            self.sequence = self.sequence.saturating_add(1u64);
            self.census.scheduled_deliveries =
                self.census.scheduled_deliveries.saturating_add(1u64);
        }
        Ok(SendReceipt {
            link,
            ordinal,
            fate: SendFate::Scheduled { copies, due },
        })
    }

    /// Advance one tick and hand back every delivery that came due, in deterministic order.
    ///
    /// Deliveries are ordered by due tick, then by scheduling sequence, so two identically driven sims hand back identical histories.
    #[must_use]
    pub fn advance(&mut self) -> Vec<Delivery<Payload>> {
        self.tick = self.tick.next();
        let now = self.tick;
        let (mut due, waiting): (Vec<_>, Vec<_>) = mem::take(&mut self.in_flight)
            .into_iter()
            .partition(|flight| flight.due <= now);
        self.in_flight = waiting;
        due.sort_unstable_by_key(|flight| (flight.due, flight.sequence));
        self.census.delivered = self
            .census
            .delivered
            .saturating_add(u64::try_from(due.len()).unwrap_or(u64::MAX));
        due.into_iter()
            .map(|flight| {
                Delivery::delivered(
                    flight.link,
                    flight.ordinal,
                    flight.payload,
                    flight.sent_at,
                    now,
                    flight.copy,
                )
            })
            .collect()
    }
}

/// Read one send's declared faults into their joint effect.
///
/// The partition is read first against the placement tick, because a dead wire outranks any shaping of live traffic; a drop is read next; every remaining positional fault composes.
/// Faults on other ordinals say nothing about this send.
fn shaped(faults: &[LinkFault], ordinal: SendOrdinal, placed_at: Tick) -> Shaping {
    let partitioned = faults.iter().any(|fault| {
        matches!(*fault, LinkFault::Partition { opens, heals } if placed_at >= opens && placed_at < heals)
    });
    if partitioned {
        return Shaping::TakenByPartition;
    }
    let dropped = faults
        .iter()
        .any(|fault| matches!(*fault, LinkFault::DropAt { position } if position == ordinal));
    if dropped {
        return Shaping::TakenByDiscipline;
    }
    let mut delay = 0u64;
    let mut copies = 1u32;
    for fault in faults {
        match *fault {
            LinkFault::DelayAt { position, ticks } if position == ordinal => {
                delay = delay.saturating_add(u64::from(ticks.ticks()));
            }
            LinkFault::DuplicateAt { position } if position == ordinal => {
                copies = copies.saturating_add(1u32);
            }
            LinkFault::DropAt { .. }
            | LinkFault::DelayAt { .. }
            | LinkFault::DuplicateAt { .. }
            | LinkFault::Partition { .. } => {}
        }
    }
    Shaping::Travels { delay, copies }
}
