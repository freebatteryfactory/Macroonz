//! A mismatched schema withholds a trial carrier whose expansion would emit independent errors.

#[macro_export]
macro_rules! forbidden_delivery {
    ($($input:tt)*) => {
        compile_error!("trial seat was released");
        compile_error!("deferred seat was released");
    };
}

macroonz_harness::generated_support! {
    expected: [0],
    trials: forbidden_delivery,
    with: { opaque input must remain unparsed },
}

fn main() {}
