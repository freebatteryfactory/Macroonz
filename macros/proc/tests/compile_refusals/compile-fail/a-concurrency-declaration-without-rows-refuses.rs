//! A concurrency declaration with no exploration rows refuses through the actual proc entry.

macroonz_macros::concurrency! {
    harness = mh,
    module = explorations,
    namespace = "proc",
}

fn main() {}
