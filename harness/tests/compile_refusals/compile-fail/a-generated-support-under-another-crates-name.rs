//! A delivery that names the wrong crate as the harness refuses AT THE DOOR.
//!
//! The gate's declared harness identifier is load-bearing rather than decorative: the expansion writes one item that names the harness's own schema-identity type through BOTH the declared path and `$crate`, and the two must be one type.
//! So a delivery that hands the gate a crate which is not the harness is refused on that one item — before either seat reaches type checking — rather than as a cascade of unresolved paths somewhere inside a payload the consumer did not write.
//!
//! `macroonz` is a real crate this fixture depends on, and it is not the harness.
//! That is the shape a version-mixed or misnamed consumer presents.

fn main() {}

macroonz_harness::generated_support! {
    expected: [
        185, 251, 251, 45, 168, 146, 85, 42, 248, 177, 196, 48, 117, 229, 207, 5,
        84, 120, 104, 25, 150, 41, 202, 2, 243, 73, 31, 148, 241, 22, 122, 34,
    ],
    harness: macroonz,
    trials: { },
    deferred: { },
}
