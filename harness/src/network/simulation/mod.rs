#![doc = include_str!("README.md")]

mod types;

pub(in crate::network) use types::Action;
pub use types::{
    Delivery, DeliveryCopy, Link, LinkDiscipline, LinkFault, NetworkCampaign,
    NetworkCampaignRefusal, NetworkCensus, NetworkSchedule, NetworkScheduleRefusal,
    NetworkSelection, NetworkSelectionRefusal, NodeRef, SendFate, SendOrdinal, SendReceipt,
    SendRefusal, SimNet, SimNetRefusal, Tick, TickSpan, TickSpanRefusal, Topology, TopologyRefusal,
};
