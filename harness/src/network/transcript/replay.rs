//! Playback of addressed delivery rows and exhaustion evidence.

use super::{Replay, ReplayExhaustion, ReplayIncomplete, TranscriptPack};
use crate::network::simulation::{Delivery, Tick};

impl Replay {
    /// Open one admitted pack for playback at tick zero, handing back every delivery already due at the opening tick.
    ///
    /// A live recording whose epoch starts at zero lawfully carries a delivery stamped at tick zero, and the opening is where it plays — never shifted onto a later tick.
    /// The simulator's own transcripts never carry one, because its time law places every delivery at least one tick after its send; for them the opening hand is empty and playback is unchanged.
    /// The drain rides the constructor rather than a second call, so there is no road to a replay value that has not already surrendered its opening hand — what a caller does with a hand it was dealt is its own affair, at this tick as at every later one.
    #[must_use]
    pub fn opened(pack: &TranscriptPack) -> (Self, Vec<Delivery<Vec<u8>>>) {
        let mut replay = Self {
            address: pack.address(),
            entries: pack.entries().to_vec(),
            total: pack.entries().len(),
            at: 0usize,
            tick: Tick::at(0u64),
        };
        let opening = replay.due_now();
        (replay, opening)
    }

    /// The current logical tick.
    #[must_use]
    pub const fn tick(&self) -> Tick {
        self.tick
    }

    /// How many recorded deliveries have not yet been handed out.
    #[must_use]
    pub fn remaining(&self) -> usize {
        self.entries.len().saturating_sub(self.at)
    }

    /// Advance one tick and hand back every recorded delivery due by it, stamps included.
    ///
    /// The deliveries are exactly the record's, in the record's order — a replay invents nothing and reorders nothing.
    #[must_use]
    pub fn advance(&mut self) -> Vec<Delivery<Vec<u8>>> {
        self.tick = self.tick.next();
        self.due_now()
    }

    /// Consume playback and mint evidence that every addressed row was handed out.
    ///
    /// # Errors
    ///
    /// Refuses while any recorded row remains, naming exactly how many were never handed out.
    pub fn exhaust(self) -> Result<ReplayExhaustion, ReplayIncomplete> {
        let remaining = self.remaining();
        if remaining != 0usize {
            return Err(ReplayIncomplete::witnessed(self.address, remaining));
        }
        Ok(ReplayExhaustion::witnessed(
            self.address,
            self.total,
            self.tick,
        ))
    }

    /// Every recorded delivery due by the current tick and not yet handed out, stamps included.
    fn due_now(&mut self) -> Vec<Delivery<Vec<u8>>> {
        let now = self.tick;
        let mut played = Vec::new();
        while let Some(entry) = self.entries.get(self.at) {
            if entry.delivered_at() > now {
                break;
            }
            played.push(Delivery::delivered(
                entry.link(),
                entry.ordinal(),
                entry.payload().to_vec(),
                entry.sent_at(),
                entry.delivered_at(),
                entry.copy(),
            ));
            self.at = self.at.saturating_add(1usize);
        }
        played
    }
}
