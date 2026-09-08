//! Neither source-specific transcript reader omits independent decoder limits.

use macroonz_harness::network::{
    NetworkSchedule, Topology, TranscriptPack, TranscriptRefusal, read_recorded_live, read_simulated,
};

fn main() {
    let _live: fn(&Topology, &[u8]) -> Result<TranscriptPack, TranscriptRefusal> = read_recorded_live;
    let _simulation: fn(&Topology, &NetworkSchedule, &[u8]) -> Result<TranscriptPack, TranscriptRefusal> = read_simulated;
}
