//! The compiled read-back's reversal: a compiler REFUSES a malformed artifact.
//!
//! # Provenance
//!
//! Everything below this header is a MATERIALIZED DAMAGED ARTIFACT: the lawful
//! rendering of the declaration restated below, with one damage this harness
//! inflicted. The services render no defective artifact, so nothing here is a
//! defect anybody's producer produced. It is checked in rather than written by
//! a running test on purpose: a fixture a test writes agrees with whatever the
//! producer had just done.
//!
//! Nothing re-derives this text today — the road that cut it left the tree with
//! the retired judge seat — so this file supports one claim and no more: a
//! compiler refuses THIS text. That it is still today's renderer's output
//! damaged returns when the generator owns materialization and publishes each
//! fixture with its receipt; `compiled_behaviour.rs` states what stands behind
//! every fixture in this lane meanwhile.
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
//! The damage: the first brace of the first implementation became three, so the
//! artifact stops being well-formed Rust. A structural read recovers nothing
//! from it and claims nothing further; this is where the claim that `rustc`
//! REJECTS it is made, by handing the text to `rustc`.

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

impl :: threadpak :: refusal :: RefusalFamily for DemoFamily {{{ const SHAPE : :: threadpak :: refusal :: FamilyShape = :: threadpak :: refusal :: FamilyShape :: SingleCause ; const SELECTION_ORDER : & 'static [ & 'static str ] = & [ "NotCanonical" , "NotAdmitted" , "Unbounded" ] ; } impl :: threadpak :: refusal :: CauseOrderDeclaration for DemoFamily { const DECLARED_ORDER : :: threadpak :: refusal :: DeclaredCauseOrder = :: threadpak :: refusal :: DeclaredCauseOrder :: declared ( & [ :: threadpak :: refusal :: DeclaredCause :: declared ( :: threadpak :: refusal :: CauseId :: declared ( :: threadpak :: refusal :: RefusalFamilyId :: declared ( "testpak.demo" ) , :: threadpak :: refusal :: LocalCauseKey :: declared ( "not-canonical" ) ) , "NotCanonical" ) , :: threadpak :: refusal :: DeclaredCause :: declared ( :: threadpak :: refusal :: CauseId :: declared ( :: threadpak :: refusal :: RefusalFamilyId :: declared ( "testpak.demo" ) , :: threadpak :: refusal :: LocalCauseKey :: declared ( "not-admitted" ) ) , "NotAdmitted" ) , :: threadpak :: refusal :: DeclaredCause :: declared ( :: threadpak :: refusal :: CauseId :: declared ( :: threadpak :: refusal :: RefusalFamilyId :: declared ( "testpak.demo" ) , :: threadpak :: refusal :: LocalCauseKey :: declared ( "unbounded" ) ) , "Unbounded" ) , ] ) ; }
