//! An independently supplied harness cannot accompany a matching staged delivery.

#[macro_export]
macro_rules! forbidden_delivery {
    ($($input:tt)*) => {
        compile_error!("delivery was released");
    };
}

macroonz_harness::generated_support! {
    expected: [
        185, 251, 251, 45, 168, 146, 85, 42, 248, 177, 196, 48, 117, 229, 207, 5,
        84, 120, 104, 25, 150, 41, 202, 2, 243, 73, 31, 148, 241, 22, 122, 34,
    ],
    harness: macroonz_compiler,
    trials: forbidden_delivery,
    with: { opaque input must remain unparsed },
}

fn main() {}
