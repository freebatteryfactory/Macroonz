//! An outside caller cannot choose the replay posture of a capsule through a loose recorder.
//!
//! This refusal claims only that the old independent key/input/posture mint is closed. It does not claim that the later run-bound replay-admission road is complete.

use threadpak_testpak::descriptor::GeneratedSupportSchemaId;
use threadpak_testpak::report::{
    ExecutionKey, GenerationProfile, MinimizationProfile, ReplayCapsule, ReplayPosture,
};

fn loose_capsule(
    key: ExecutionKey,
    input: Vec<u8>,
    generation: GenerationProfile,
    minimization: MinimizationProfile,
    schema: GeneratedSupportSchemaId,
) -> ReplayCapsule {
    ReplayCapsule::recorded(
        key,
        input,
        generation,
        minimization,
        schema,
        ReplayPosture::ExactDerived,
    )
}

fn main() {}
