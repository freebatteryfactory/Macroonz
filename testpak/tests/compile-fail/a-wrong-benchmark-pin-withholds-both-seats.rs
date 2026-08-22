//! A wrong generated-support pin withholds both benchmark and reporter cargo.

threadpak_testpak::generated_support! {
    expected: [0],
    harness: threadpak_testpak,
    benches: { this is not a benchmark table },
    reporter: { neither is this reporter cargo },
}

fn main() {}
