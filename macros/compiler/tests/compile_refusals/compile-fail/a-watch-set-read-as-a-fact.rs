//! Reading a context's watch set is a road that can REFUSE, and no caller can take it as a fact.
//!
//! An account names up to the declared magnitude of captures, and a narrow reading that answered with the first one would produce a value byte for byte the shape of a complete watch set — so a plan committed to three declarations while watching one would read as current after the other two changed, and nothing downstream could tell the two apart.
//! The refusal is in the SIGNATURE: a caller cannot receive a watch set from this road without disposing of the case where the set cannot be watched.
//!
//! Restoring an unconditional answer makes the road total again and this file compiles, which is what makes the fixture the control for the shape rather than a note beside it.

use macroonz_compiler::{Account, Context, InvalidationSet, Kind, NoQuestions, SoleRole};

/// One kind, because an account is an account OF something.
struct Demo;

impl Kind for Demo {
    const NAME: &'static str = "fixture.demo";
    type Content = ();
    type Role = SoleRole;
    type Question = NoQuestions;
}

fn watched(context: &Context, account: &Account<Demo>) -> InvalidationSet {
    context.watch_set(account)
}

fn main() {
    let _road = watched;
}
