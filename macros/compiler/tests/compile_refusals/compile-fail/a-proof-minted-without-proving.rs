//! A proof that did not prove anything is unwritable.
//!
//! `Closure::proved` rebuilds the membership out of the rendered units and compares it against the plan's, so holding a closure IS that proof.
//! That only holds while there is no other road to one: a closure assembled field by field would carry the same type, satisfy the same signatures, and vouch for a rendering nobody rebuilt.
//!
//! No value is constructed below; the struct expression alone is the proof.

use macroonz::{Closure, PlanId, SoleRole};

fn main() {
    let mint: fn(PlanId) -> Closure<SoleRole> = |plan| Closure { plan };
    let _road = mint;
}
