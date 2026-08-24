//! A wrong generated-support pin withholds both benchmark and reporter cargo.

macroonz_harness::generated_support! {
    expected: [0],
    harness: macroonz_harness,
    benches: { this is not a benchmark table },
    reporter: { neither is this reporter cargo },
}

fn main() {}
