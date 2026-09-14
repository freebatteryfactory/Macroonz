//! Reading foreign seeds requires independently supplied decoder limits.

use macroonz_harness::corpus::{SeedPack, SeedPackRefusal, read};
use macroonz_harness::descriptor::PopulationRef;

fn main() {
    let _unbounded: fn(PopulationRef, &[u8]) -> Result<SeedPack, SeedPackRefusal> = read;
}
