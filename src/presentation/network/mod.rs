#![doc = include_str!("README.md")]

mod declaration;
mod record;
mod refusal;
mod standing;

pub use record::network_transcript;
pub use refusal::network_transcript_refusal;
pub use standing::{
    network_replay_exhaustion, network_replay_incomplete, network_replay_join_refusal,
    network_reproduced_replay, network_reproduction,
};
