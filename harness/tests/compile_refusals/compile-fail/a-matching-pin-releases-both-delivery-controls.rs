//! Matching pins release the same expansion-time sentinels withheld by the wrong-pin fixtures.

#[macro_export]
macro_rules! delivery_control {
    (@trials $($input:tt)*) => {
        compile_error!("trial seat was released");
        compile_error!("deferred seat was released");
    };
    (@benches $($input:tt)*) => {
        compile_error!("benchmark seat was released");
        compile_error!("reporter seat was released");
    };
}

macroonz_harness::generated_support! {
    expected: [
        185, 251, 251, 45, 168, 146, 85, 42, 248, 177, 196, 48, 117, 229, 207, 5,
        84, 120, 104, 25, 150, 41, 202, 2, 243, 73, 31, 148, 241, 22, 122, 34,
    ],
    trials: delivery_control,
    with: { opaque input must remain unparsed },
}

macroonz_harness::generated_support! {
    expected: [
        185, 251, 251, 45, 168, 146, 85, 42, 248, 177, 196, 48, 117, 229, 207, 5,
        84, 120, 104, 25, 150, 41, 202, 2, 243, 73, 31, 148, 241, 22, 122, 34,
    ],
    benches: delivery_control,
    with: { opaque input must remain unparsed },
}

fn main() {}
