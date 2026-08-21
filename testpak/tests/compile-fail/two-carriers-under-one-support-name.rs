//! Two declarations in one crate that choose one support name collide as an
//! ordinary duplicate macro definition.
//!
//! The physical carrier wears the PLAN's identity at full width, so two
//! declarations can never mint one hidden name. The alias is the author's choice,
//! and two authors choosing one spelling is a collision the consumer's own
//! compiler reports in the consumer's own words — which is why nothing in the
//! producer keeps a register of what it has already exported.

use threadpak_macros::RefusalFamily;

#[derive(RefusalFamily)]
#[refusal(
    family = "fixture.first",
    shape = single_cause,
    order(NotCanonical = "not-canonical")
)]
#[threadpak_trials(
    support = shared_trials,
    module = generated_first_trials,
    table = named("fixture", "first-trials"),

    suite construction = named("fixture", "construction") {
        the_first_row {
            claim = named("fixture", "the-first-claim"),
            subject = named("fixture", "demo-subject"),
            check = named("fixture", "demo-check"),
            population = named("fixture", "demo-population"),
        },
    },
)]
enum FirstFamily {
    NotCanonical,
}

#[derive(RefusalFamily)]
#[refusal(
    family = "fixture.second",
    shape = single_cause,
    order(NotCanonical = "not-canonical")
)]
#[threadpak_trials(
    support = shared_trials,
    module = generated_second_trials,
    table = named("fixture", "second-trials"),

    suite construction = named("fixture", "construction") {
        the_second_row {
            claim = named("fixture", "the-second-claim"),
            subject = named("fixture", "demo-subject"),
            check = named("fixture", "demo-check"),
            population = named("fixture", "demo-population"),
        },
    },
)]
enum SecondFamily {
    NotCanonical,
}

fn main() {
    let _ = FirstFamily::NotCanonical;
    let _ = SecondFamily::NotCanonical;
}
