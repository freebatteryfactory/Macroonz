//! Constructors, readers, and invariant joins for transcript custody, reproduction, and replay.

use super::{
    ReplayExhaustion, ReplayIncomplete, ReproducedReplay, ReproducedReplayRefusal,
    SimulationAction, SimulationManifest, SimulationReproduction, TranscriptAddress,
    TranscriptEntry, TranscriptMaterial, TranscriptPack, TranscriptSourceClaim,
};
use crate::identity::ContentAddress;
use crate::network::simulation::{
    DeliveryCopy, Link, NetworkSchedule, SendOrdinal, Tick, Topology,
};

impl TranscriptEntry {
    /// One delivery as somebody witnessed it — the sim's own, or a live adapter's observation.
    #[must_use]
    pub const fn witnessed(
        link: Link,
        ordinal: SendOrdinal,
        payload: Vec<u8>,
        sent_at: Tick,
        delivered_at: Tick,
        copy: DeliveryCopy,
    ) -> Self {
        Self {
            link,
            ordinal,
            payload,
            sent_at,
            delivered_at,
            copy,
        }
    }

    /// The link the payload traveled.
    #[must_use]
    pub const fn link(&self) -> Link {
        self.link
    }

    /// The send's zero-based ordinal on that link.
    #[must_use]
    pub const fn ordinal(&self) -> SendOrdinal {
        self.ordinal
    }

    /// The payload, in bytes.
    #[must_use]
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    /// The tick the send was placed at.
    #[must_use]
    pub const fn sent_at(&self) -> Tick {
        self.sent_at
    }

    /// The tick the delivery came due.
    #[must_use]
    pub const fn delivered_at(&self) -> Tick {
        self.delivered_at
    }

    /// Whether this is the send's original or a duplicate.
    #[must_use]
    pub const fn copy(&self) -> DeliveryCopy {
        self.copy
    }
}

impl TranscriptAddress {
    /// The address the writer or the reader derived, minted nowhere else.
    #[must_use]
    pub(super) const fn derived(address: ContentAddress) -> Self {
        Self(address)
    }

    /// The derivation this address carries.
    #[must_use]
    pub const fn address(self) -> ContentAddress {
        self.0
    }
}

impl TranscriptPack {
    /// One admitted pack, minted only by the write and read roads.
    #[must_use]
    pub(super) const fn assembled(
        topology: Topology,
        material: TranscriptMaterial,
        address: TranscriptAddress,
        entries: Vec<TranscriptEntry>,
        encoded: Vec<u8>,
    ) -> Self {
        Self {
            topology,
            material,
            address,
            entries,
            encoded,
        }
    }

    /// The topology retained in this pack's addressed body.
    #[must_use]
    pub const fn topology(&self) -> &Topology {
        &self.topology
    }

    /// What this pack's addressed body claims about its source.
    ///
    /// The reading carries no reproduction standing.
    #[must_use]
    pub const fn source_claim(&self) -> TranscriptSourceClaim {
        match self.material {
            TranscriptMaterial::Simulated(_) => TranscriptSourceClaim::Simulated,
            TranscriptMaterial::RecordedLive => TranscriptSourceClaim::RecordedLive,
        }
    }

    /// The simulation inputs this pack retains, where its body claims a simulated source.
    #[must_use]
    pub const fn simulation_manifest(&self) -> Option<&SimulationManifest> {
        match &self.material {
            TranscriptMaterial::Simulated(manifest) => Some(manifest),
            TranscriptMaterial::RecordedLive => None,
        }
    }

    /// The address the whole body derives.
    #[must_use]
    pub const fn address(&self) -> TranscriptAddress {
        self.address
    }

    /// The deliveries, in delivery order.
    #[must_use]
    pub fn entries(&self) -> &[TranscriptEntry] {
        &self.entries
    }

    /// The complete envelope, exactly as derived: the address, then the body it covers.
    #[must_use]
    pub fn encoded(&self) -> &[u8] {
        &self.encoded
    }
}

impl SimulationManifest {
    /// One selected schedule beside the complete ordered actions driven through it.
    #[must_use]
    pub(super) const fn captured(
        schedule: NetworkSchedule,
        actions: Vec<SimulationAction>,
    ) -> Self {
        Self { schedule, actions }
    }

    /// The selected schedule the manifest declares.
    #[must_use]
    pub const fn schedule(&self) -> &NetworkSchedule {
        &self.schedule
    }

    /// Every send and advance, in exact drive order.
    #[must_use]
    pub fn actions(&self) -> &[SimulationAction] {
        &self.actions
    }
}

impl SimulationReproduction {
    /// One exact transcript address whose manifest reproduced all addressed rows.
    #[must_use]
    pub(super) const fn witnessed(
        address: TranscriptAddress,
        actions: usize,
        rows: usize,
        final_tick: Tick,
    ) -> Self {
        Self {
            address,
            actions,
            rows,
            final_tick,
        }
    }

    /// The transcript whose exact manifest and rows were reproduced.
    #[must_use]
    pub const fn address(self) -> TranscriptAddress {
        self.address
    }

    /// How many manifest actions were executed.
    #[must_use]
    pub const fn actions(self) -> usize {
        self.actions
    }

    /// How many delivery rows the execution reproduced.
    #[must_use]
    pub const fn rows(self) -> usize {
        self.rows
    }

    /// The reproduced sim's logical tick after the final manifest action.
    #[must_use]
    pub const fn final_tick(self) -> Tick {
        self.final_tick
    }
}

impl ReplayExhaustion {
    /// One exhausted replay, minted only by consuming that replay.
    #[must_use]
    pub(super) const fn witnessed(
        address: TranscriptAddress,
        total: usize,
        final_tick: Tick,
    ) -> Self {
        Self {
            address,
            total,
            final_tick,
        }
    }

    /// The transcript whose rows were all handed out.
    #[must_use]
    pub const fn address(self) -> TranscriptAddress {
        self.address
    }

    /// How many addressed rows were handed out.
    #[must_use]
    pub const fn total(self) -> usize {
        self.total
    }

    /// The replay tick at exhaustion.
    #[must_use]
    pub const fn final_tick(self) -> Tick {
        self.final_tick
    }
}

impl ReplayIncomplete {
    /// One replay that still retained rows when exhaustion was requested.
    pub(super) const fn witnessed(address: TranscriptAddress, remaining: usize) -> Self {
        Self { address, remaining }
    }

    /// The transcript whose replay remains incomplete.
    #[must_use]
    pub const fn address(self) -> TranscriptAddress {
        self.address
    }

    /// How many recorded rows were never handed out.
    #[must_use]
    pub const fn remaining(self) -> usize {
        self.remaining
    }
}

impl ReproducedReplay {
    /// Join exact simulation reproduction with exhausted playback over the same address.
    ///
    /// # Errors
    ///
    /// Refuses values that name different transcript addresses.
    pub fn joined(
        reproduction: SimulationReproduction,
        exhaustion: ReplayExhaustion,
    ) -> Result<Self, ReproducedReplayRefusal> {
        if reproduction.address() != exhaustion.address() {
            return Err(ReproducedReplayRefusal::AddressMismatch {
                reproduction: reproduction.address(),
                replay: exhaustion.address(),
            });
        }
        Ok(Self {
            reproduction,
            exhaustion,
        })
    }

    /// The simulation reproduction this join retains.
    #[must_use]
    pub const fn reproduction(self) -> SimulationReproduction {
        self.reproduction
    }

    /// The replay exhaustion this join retains.
    #[must_use]
    pub const fn exhaustion(self) -> ReplayExhaustion {
        self.exhaustion
    }
}
