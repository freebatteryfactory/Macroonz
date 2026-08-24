#![doc = include_str!("README.md")]

mod inject;
mod types;

pub use inject::inject;
pub use types::{
    CampaignSelection, FaultAdapter, FaultCampaign, FaultCampaignRefusal, FaultInjectionRefusal,
    FaultSchedule, FaultSelectionRefusal, InjectedCommand, InjectedSequence, ScheduledFault,
    SequencePosition,
};
