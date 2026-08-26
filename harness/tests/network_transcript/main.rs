//! Transcript custody, source posture, identity, reproduction, and replay claims exercised from outside the library.
//!
//! Claim modules retain one Cargo integration target while separating exact-byte custody from source/refusal and playback claims.

mod custody;
mod identity;
mod refusals;
mod source_posture;
mod support;
