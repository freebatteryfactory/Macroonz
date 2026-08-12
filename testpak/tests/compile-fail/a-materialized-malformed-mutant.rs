//! Lane C's reversal for `MalformedRust`: the mutated artifact does not
//! compile.
//!
//! # Provenance
//!
//! Everything below this header is a MATERIALIZED MUTANT. It is the byte-for-byte
//! output of this plane's own `mutated(lawful, MalformedRust)` over the lawful
//! rendering of the declaration restated below, and `compiled_behaviour.rs`
//! asserts that it still is on every run — a fixture whose provenance nobody
//! checks is a hand-written guess wearing a mutant's name.
//!
//! It is checked in rather than generated at test time on purpose. A fixture
//! written into the source tree by a running test is a fixture that agrees with
//! whatever the producer just did; a checked-in one whose provenance is stated
//! and verified is a fixture somebody can read.
//!
//! # The declaration it was rendered from
//!
//! ```text
//! #[refusal(family = "testpak.demo", shape = single_cause,
//!     order(NotCanonical = "not-canonical", NotAdmitted = "not-admitted",
//!     Unbounded = "unbounded"))]
//! enum DemoFamily { NotAdmitted, Unbounded, NotCanonical, }
//! ```
//!
//! # What the mutation did
//!
//! The first brace of the first implementation became three, so the artifact
//! stops being well-formed Rust. Lane B reports `Unparsable` on it and claims
//! nothing further; this file is where the claim that `rustc` REJECTS it is
//! made, by handing the text to `rustc`.

enum DemoFamily {
    NotAdmitted,
    Unbounded,
    NotCanonical,
}

fn main() {
    let _ = DemoFamily::NotAdmitted;
    let _ = DemoFamily::Unbounded;
    let _ = DemoFamily::NotCanonical;
}

impl :: threadpak :: refusal :: RefusalFamily for DemoFamily {{{ const SHAPE : :: threadpak :: refusal :: FamilyShape = :: threadpak :: refusal :: FamilyShape :: SingleCause ; const SELECTION_ORDER : & 'static [ & 'static str ] = & [ "NotCanonical" , "NotAdmitted" , "Unbounded" ] ; } impl :: threadpak :: refusal :: CauseOrderDeclaration for DemoFamily { const DECLARED_ORDER : :: threadpak :: refusal :: DeclaredCauseOrder = :: threadpak :: refusal :: DeclaredCauseOrder :: declared ( & [ :: threadpak :: refusal :: DeclaredCause :: declared ( :: threadpak :: refusal :: CauseId :: declared ( "testpak.demo.not-canonical" ) , "NotCanonical" ) , :: threadpak :: refusal :: DeclaredCause :: declared ( :: threadpak :: refusal :: CauseId :: declared ( "testpak.demo.not-admitted" ) , "NotAdmitted" ) , :: threadpak :: refusal :: DeclaredCause :: declared ( :: threadpak :: refusal :: CauseId :: declared ( "testpak.demo.unbounded" ) , "Unbounded" ) , ] ) ; }
