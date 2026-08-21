//! A generated delivery carrying an attachment for a row nobody declared does
//! not match its carrier either.
//!
//! The matcher names exactly the rows the declaration states, in the order it
//! states them. A target that attaches a callable to a lens the declaration never
//! wrote is a target attaching a check to nothing — refused at the invocation
//! rather than silently ignored, because a dropped attachment reads exactly like
//! one that was accepted.

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
        the_only_row {
            claim = named("fixture", "the-only-claim"),
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

fn the_only_row(_invocation: &Invocation) -> TrialConclusion {
    TrialConclusion::Passed
}

fn a_row_nobody_declared(_invocation: &Invocation) -> TrialConclusion {
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
        threadpak_testpak::report::ToolchainIdentity::declared("1.98.0"),
    ),

    clock: threadpak_testpak::runner::HostClock::unmeasured(),

    attachments: {
        the_only_row {
            subject_revision: crate::committed(),
            check_revision: crate::committed(),
            call: crate::the_only_row,
        },
        a_row_nobody_declared {
            subject_revision: crate::committed(),
            check_revision: crate::committed(),
            call: crate::a_row_nobody_declared,
        },
    },
}

fn main() {
    let _ = DemoFamily::NotCanonical;
}
