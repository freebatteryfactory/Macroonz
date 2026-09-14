//! A mismatched schema withholds a benchmark carrier whose expansion would emit independent errors.

#[macro_export]
macro_rules! forbidden_delivery {
    ($($input:tt)*) => {
        compile_error!("benchmark seat was released");
        compile_error!("reporter seat was released");
    };
}

macroonz_harness::generated_support! {
    expected: [0],
    benches: forbidden_delivery,
    with: { opaque input must remain unparsed },
}

fn main() {}
