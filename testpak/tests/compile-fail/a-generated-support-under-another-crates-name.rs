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
        64, 247, 209, 126, 39, 187, 123, 191, 55, 210, 86, 156, 252, 110, 235, 212,
        119, 194, 33, 206, 138, 125, 70, 120, 179, 212, 187, 59, 69, 188, 29, 250,
    ],
    harness: threadpak,
    trials: { },
    deferred: { },
}
