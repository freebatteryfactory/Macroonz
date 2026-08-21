//! A trial declaration that reaches for the consumption target's own facts
//! refuses at the clause, in the compiler.
//!
//! `call` is a real seat of the road a generated row travels — it is what makes
//! the row executable — and it lives in the TEST TARGET, which is not the crate
//! this declaration sits in and has no crate binding a rendered path could be
//! rooted at. The grammar has no clause for it, and the refusal says so at the
//! exact key rather than at the declaration's opening.
//!
//! The same cause answers for every seat on either side of the wall: the origin,
//! the producer, the projection, and the schema a producer mints, and the
//! revisions, the budgets, the target binding, and the clock a target supplies.

use threadpak_macros::RefusalFamily;

#[derive(RefusalFamily)]
#[refusal(
    family = "fixture.demo",
    shape = single_cause,
    order(NotCanonical = "not-canonical")
)]
#[threadpak_trials(
    support = demo_trials,
    module = generated_demo_trials,
    table = named("fixture", "demo-trials"),

    suite construction = named("fixture", "construction") {
        the_only_row {
            claim = named("fixture", "the-only-claim"),
            subject = named("fixture", "demo-subject"),
            check = named("fixture", "demo-check"),
            population = named("fixture", "demo-population"),
            call = named("fixture", "demo-check"),
        },
    },
)]
enum DemoFamily {
    NotCanonical,
}

fn main() {
    let _ = DemoFamily::NotCanonical;
}
