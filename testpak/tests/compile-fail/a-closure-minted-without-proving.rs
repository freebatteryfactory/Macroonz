//! A proof that did not prove anything is unwritable.
//!
//! `ProjectionClosure::proved` rebuilds the membership out of the rendered units
//! and compares it against the plan's. Holding a closure IS that proof, which
//! only holds if there is no other road to one — a closure assembled field by
//! field would carry the same type, satisfy the same signatures, and vouch for a
//! rendering nobody rebuilt.
//!
//! Every field is private and `proved` is the only associated function that
//! returns one, so the assembly below refuses twice over: the field it names is
//! not reachable, and the fields it does not name have no other road in either.
//!
//! No value is constructed below. The struct expression alone is the proof.

use threadpak_macroc::ProjectionClosure;
use threadpak_macroc::plane::PlanId;
use threadpak_macroc::planning::RenderedImplementation;

fn main() {
    let mint: fn(PlanId) -> ProjectionClosure<RenderedImplementation> =
        |plan| ProjectionClosure { plan };
    let _ = mint;
}
