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
//! Nothing in this lane re-derives the damaged text.
//! This file therefore supports one claim and no more: rustc refuses these checked-in bytes.
//! It carries no claim that the bytes are a damaged rendering from the current renderer.
//! `refusal_artifact_readings.rs` states the distinct evidence behind the lawful and shape-altered fixtures.
//!
//! The exact damage is named below; no other malformed shape is claimed.
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

impl :: macroonz :: RefusalFamily for DemoFamily {{{ const SHAPE : :: macroonz :: FamilyShape = :: macroonz :: FamilyShape :: SingleCause ; } impl :: macroonz :: CauseOrderDeclaration for DemoFamily { const DECLARED_ORDER : :: macroonz :: DeclaredCauseOrder = :: macroonz :: DeclaredCauseOrder :: declared ( & [ :: macroonz :: DeclaredCause :: declared ( :: macroonz :: CauseId :: declared ( :: macroonz :: RefusalFamilyId :: declared ( "testpak.demo" ) , :: macroonz :: LocalCauseKey :: declared ( "not-canonical" ) ) , "NotCanonical" ) , :: macroonz :: DeclaredCause :: declared ( :: macroonz :: CauseId :: declared ( :: macroonz :: RefusalFamilyId :: declared ( "testpak.demo" ) , :: macroonz :: LocalCauseKey :: declared ( "not-admitted" ) ) , "NotAdmitted" ) , :: macroonz :: DeclaredCause :: declared ( :: macroonz :: CauseId :: declared ( :: macroonz :: RefusalFamilyId :: declared ( "testpak.demo" ) , :: macroonz :: LocalCauseKey :: declared ( "unbounded" ) ) , "Unbounded" ) , ] ) ; }
