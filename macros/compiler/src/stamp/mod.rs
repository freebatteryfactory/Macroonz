#![doc = include_str!("README.md")]

mod plan;
mod render;
mod type_contract;
mod types;

pub use plan::planned;
pub use render::{
    DECLARED_REACH, OPAQUE_REACH_REFUSAL, TRANSCRIBE_ARM, TRANSPORTED_REACH, declared_reach,
    definition, forwarded, invocation, transported_reach,
};
pub use types::{
    Fragment, Landing, PART_LIMIT, PATH_SEGMENT_LIMIT, Part, Pattern, PublicationGround,
    PublicationRecord, PublishedStamp, SITE_LIMIT, Seat, Seating, Site, SiteRoot, Stamp,
    StampError, StampName, StampedPlan, TransportedReach, Visibility,
};
