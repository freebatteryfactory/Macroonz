//! The reversal for the MINT half of the coupled seat: a services refusal family
//! body is produced only by a pass inside the plane that established its issues.
//!
//! A private seat closes the LITERAL and nothing else. A road that takes an
//! issue and hands back a refusal is the loading dock behind that fence: any
//! holder of an issue mints a body no pass raised, and any holder of the
//! borrowed body clones its issues out and reseats them through the same road.
//! The record either produces is indistinguishable from one a seam returned.
//!
//! # This fixture names exact roads
//!
//! The seven references below are the exact construction spellings this fixture attempts.
//! Making any one public changes this file's diagnostics and fails the fixture.
//! The lane proves those spellings are inaccessible; it does not prove that no alternate or later mint could exist.
//!
//! Each body is declared in a `seat`
//! module inside its home's `type_guard.rs`, whose entire content is that
//! record and inherent implementations of it and nothing else — the module is
//! the complete set of roads to the private seat.
//!
//! # Claim ceiling
//!
//! A compile-fail fixture observes name resolution at the exact paths it writes.
//! It does not enumerate methods the source never names.
//! It does not follow aliases to infer another construction road.
//! It does not infer that a receiver can or cannot be obtained.
//! It does not infer reachability from caller counts.
//! Those are separate source and owner questions.
//!
//! The structural privacy of each attempted method is still real.
//! The narrow claim keeps that evidence without turning this file into a complete public-surface census.
//! No wider capability closure follows.
//! So the set of code that can reach a private seat is
//! a module read in one screen rather than a file of dozens of types, and every
//! road outside it is `E0451` or `E0616` from the compiler.
//!
//! The paths are REFERENCED rather than called on purpose. Privacy is settled at
//! resolution, so a reference establishes the claim without constructing
//! arguments — and arguments constructed here would be testing the argument
//! types instead of the visibility.

use threadpak_macroc::{
    CompositionRootDeclaration, ExplanationCoverage, ProjectionClosureRefusal, ProjectionPlanning,
    RefusalDeriveRefusal, RenderedImplementation,
};

fn main() {
    let _planning_one = ProjectionPlanning::established;
    let _planning_many = ProjectionPlanning::co_established;
    let _planning_bound = ProjectionPlanning::bound_exceeded;
    let _closure = ProjectionClosureRefusal::<RenderedImplementation>::established;
    let _coverage = ExplanationCoverage::established;
    let _composition = CompositionRootDeclaration::established;
    let _capture = RefusalDeriveRefusal::established;
}
