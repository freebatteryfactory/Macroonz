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
//! `threadpak` is a real crate this fixture really depends on, and it is not the
//! harness. That is exactly the shape a version-mixed or mis-renamed consumer
//! arrives in.

fn main() {}

threadpak_testpak::generated_support! {
    expected: [
        113, 22, 215, 27, 201, 83, 45, 177, 228, 123, 154, 255, 239, 17, 99, 56,
        150, 45, 78, 145, 144, 250, 75, 10, 60, 33, 74, 147, 17, 187, 77, 147,
    ],
    harness: threadpak,
    trials: { },
    deferred: { },
}
