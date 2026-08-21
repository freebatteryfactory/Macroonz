//! A generated delivery short one attachment does not match its carrier at all.
//!
//! Every declared row needs a callable and two revision commitments, and all
//! three live in the CONSUMPTION target. The carrier's matcher names one clause
//! per declared row, so a target that supplies fewer is refused at the
//! invocation — rather than expanding into a table with a row nothing runs, or a
//! row the expansion quietly dropped.

use threadpak_macros::RefusalFamily;
use threadpak_testpak::descriptor::RevisionBinding;
use threadpak_testpak::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use threadpak_testpak::report::TrialConclusion;
use threadpak_testpak::runner::Invocation;

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
        the_first_row {
            claim = named("fixture", "the-first-claim"),
            subject = named("fixture", "demo-subject"),
            check = named("fixture", "demo-check"),
            population = named("fixture", "demo-population"),
        },
        the_second_row {
            claim = named("fixture", "the-second-claim"),
            subject = named("fixture", "demo-subject"),
            check = named("fixture", "demo-check"),
            population = named("fixture", "demo-population"),
        },
    },
)]
enum DemoFamily {
    NotCanonical,
}

const REVISION_TAG: DomainTag =
    DomainTag::declared("fixture-revision", IdentityProfileVersion::declared(1));

fn committed() -> RevisionBinding {
    RevisionBinding::declared(ContentAddress::derived(REVISION_TAG, b"fixture/r1"))
}

fn the_first_row(_invocation: &Invocation) -> TrialConclusion {
    TrialConclusion::Passed
}

fn the_second_row(_invocation: &Invocation) -> TrialConclusion {
    TrialConclusion::Passed
}

demo_trials! {
    harness: threadpak_testpak,

    invocation: threadpak_testpak::report::InvocationProfile::declared(
        threadpak_testpak::report::CaseBudget::declared(1u32),
        threadpak_testpak::report::ByteBudget::declared(64u64),
        threadpak_testpak::report::TimeBudget::declared(1_000_000_000u64),
    ),

    target: threadpak_testpak::report::TargetBinding::bound(
        threadpak_testpak::report::TargetTriple::declared("x86_64-pc-windows-msvc"),
        threadpak_testpak::report::ToolchainIdentity::declared("1.97.1"),
    ),

    clock: threadpak_testpak::runner::HostClock::unmeasured(),

    attachments: {
        the_first_row {
            subject_revision: crate::committed(),
            check_revision: crate::committed(),
            call: crate::the_first_row,
        },
    },
}

fn main() {
    let _ = DemoFamily::NotCanonical;
}
