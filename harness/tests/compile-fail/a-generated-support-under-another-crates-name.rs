//! A delivery that names the wrong crate as the harness refuses AT THE DOOR.
//!
//! The gate's declared harness identifier is load-bearing rather than
//! decorative: the expansion writes one item that names the harness's own
//! schema-identity type through BOTH the declared path and `$crate`, and the two
//! must be one type. So a delivery that hands the gate a crate which is not the
//! harness is refused on that one item — before either seat reaches type checking
//! — rather than as a cascade of unresolved paths somewhere inside a payload the
//! consumer did not write.
//!
//! `macroonz` is a real crate this fixture depends on, and it is not the harness. That is the shape a version-mixed or misnamed consumer presents.

fn main() {}

threadpak_testpak::generated_support! {
    expected: [
        222, 149, 109, 97, 135, 230, 254, 180, 55, 195, 41, 161, 180, 186, 130, 96,
        170, 30, 123, 48, 131, 30, 77, 129, 225, 115, 89, 175, 105, 68, 31, 161,
    ],
    harness: macroonz,
    trials: { },
    deferred: { },
}
