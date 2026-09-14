//! An independent harness binding refuses before either raw cargo seat can expand.

macroonz_harness::generated_support! {
    expected: [
        185, 251, 251, 45, 168, 146, 85, 42, 248, 177, 196, 48, 117, 229, 207, 5,
        84, 120, 104, 25, 150, 41, 202, 2, 243, 73, 31, 148, 241, 22, 122, 34,
    ],
    harness: macroonz_compiler,
    trials: { compile_error!("trial seat was released"); },
    deferred: { compile_error!("deferred seat was released"); },
}

fn main() {}
