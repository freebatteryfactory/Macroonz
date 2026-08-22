#![doc = include_str!("README.md")]
//!
//! # The files
//!
//! `types.rs` declares positions, schedules, campaigns, selections, injected sequences, and their refusal families. Its child `type_guard.rs` owns every constructor and reader over their private fields. `inject.rs` is the one operation: it joins a validated selection to an ordinary command sequence.

mod inject;
mod types;

pub use inject::inject;
pub use types::{
    CampaignSelection, FaultAdapter, FaultCampaign, FaultCampaignRefusal, FaultInjectionRefusal,
    FaultSchedule, FaultSelectionRefusal, InjectedCommand, InjectedSequence, ScheduledFault,
    SequencePosition,
};
