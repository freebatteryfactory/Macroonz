#![doc = include_str!("README.md")]

mod simulation;
mod transcript;
mod types;

pub use transcript::{read_recorded_live, read_simulated, recorded_live, reproduce, simulated};
pub use types::{
    Delivery, DeliveryCopy, Link, LinkDiscipline, LinkFault, NetworkCampaign,
    NetworkCampaignRefusal, NetworkCensus, NetworkSchedule, NetworkScheduleRefusal,
    NetworkSelection, NetworkSelectionRefusal, NodeRef, Replay, ReplayExhaustion, ReplayIncomplete,
    ReproducedReplay, ReproducedReplayRefusal, SendFate, SendOrdinal, SendReceipt, SendRefusal,
    SimNet, SimNetRefusal, SimulationAction, SimulationManifest, SimulationReproduction,
    TRANSCRIPT_FORMAT_VERSION, TRANSCRIPT_TAG, Tick, TickSpan, TickSpanRefusal, Topology,
    TopologyRefusal, TranscriptAddress, TranscriptEntry, TranscriptPack, TranscriptRefusal,
    TranscriptSourceClaim,
};
