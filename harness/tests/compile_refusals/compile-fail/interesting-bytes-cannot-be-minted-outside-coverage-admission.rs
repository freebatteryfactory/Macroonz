//! A caller cannot mint bytes as coverage-interesting without a novel joined execution reading.

use macroonz_harness::fuzz::InterestingBytes;

fn main() {
    let _ = InterestingBytes { bytes: vec![1] };
}
